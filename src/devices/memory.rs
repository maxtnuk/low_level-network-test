use std::cell::RefCell;
use std::error::Error;
use std::io::{self, Read, Seek};
use std::os::unix::io::AsRawFd;
use std::rc::Rc;
use std::{fs, mem, ptr};

use crate::include::VfioStruct;

const HUGE_PAGE_BITS: u32 = 21;
const HUGE_PAGE_SIZE: usize = 1 << HUGE_PAGE_BITS;

const MAP_HUGE_2MB: i32 = 0x5400_0000; // 21 << 26

trait RegOp {
    fn get_reg32(&self, reg: u32) -> u32;
    fn set_reg32(&self, reg: u32, value: u32);
    fn set_flags32(&self, reg: u32, flags: u32);
    fn clear_flags32(&self, reg: u32, flags: u32);
    fn wait_clear_reg32(&self, reg: u32, value: u32);
    fn wait_set_reg32(&self, reg: u32, value: u32);
}
struct Dma<T> {
    pub virt: *mut T,
    pub phys: usize,
}
impl<T> Dma<T> {
    /// Allocates dma memory on a huge page.
    #[allow(dead_code)]
    pub fn allocate(
        vfio_fd: &VfioStruct,
        size: usize,
        require_contigous: bool,
        path: &str,
    ) -> Result<Dma<T>, Box<dyn Error>> {
        let size = if size % HUGE_PAGE_SIZE != 0 {
            ((size >> HUGE_PAGE_BITS) + 1) << HUGE_PAGE_BITS
        } else {
            size
        }; //make with 2MB

        if vfio_fd.get_vfio_fd().is_none() {
            //debug!("allocating dma memory via VFIO");

            let ptr = unsafe {
                libc::mmap(
                    ptr::null_mut(),
                    size,
                    libc::PROT_READ | libc::PROT_WRITE,
                    libc::MAP_SHARED | libc::MAP_ANONYMOUS | libc::MAP_HUGETLB | MAP_HUGE_2MB,
                    -1,
                    0,
                ) as *mut u8
            };

            // This is the main IOMMU work: IOMMU DMA MAP the memory...
            if ptr == libc::MAP_FAILED as *mut u8 {
                Err("failed to memory map ".into())
            } else {
                let iova = vfio_fd.vfio_map_dma(ptr, size)?;

                let memory = Dma {
                    virt: ptr as *mut T,
                    phys: iova,
                };

                Ok(memory)
            }
        } else {
            //debug!("allocating dma memory via huge page");

            if require_contigous && size > HUGE_PAGE_SIZE {
                return Err("failed to map physically contigous memory".into());
            }

            match fs::OpenOptions::new()
                .read(true)
                .write(true)
                .create(true)
                .open(path.clone())
            {
                Ok(f) => {
                    let ptr = unsafe {
                        libc::mmap(
                            ptr::null_mut(),
                            size,
                            libc::PROT_READ | libc::PROT_WRITE,
                            libc::MAP_SHARED | libc::MAP_HUGETLB,
                            f.as_raw_fd(),
                            0,
                        )
                    };

                    if ptr == libc::MAP_FAILED {
                        Err("failed to memory map huge page - huge pages enabled and free?".into())
                    } else if unsafe { libc::mlock(ptr as *mut libc::c_void, size) } == 0 {
                        let memory = Dma {
                            virt: ptr as *mut T,
                            phys: virt_to_phys(ptr as usize)?,
                        };

                        Ok(memory)
                    } else {
                        Err("failed to memory lock huge page".into())
                    }
                }
                Err(ref e) if e.kind() == io::ErrorKind::NotFound => Err(Box::new(io::Error::new(
                    io::ErrorKind::NotFound,
                    format!(
                        "huge page {} could not be created - huge pages enabled?",
                        path
                    ),
                ))),
                Err(e) => Err(Box::new(e)),
            }
        }
    }
}

pub struct Mempool {
    base_addr: *mut u8,
    num_entries: usize,
    pub(crate) entry_size: usize,
    phys_addresses: Vec<usize>,
    pub(crate) free_stack: RefCell<Vec<usize>>,
}
impl Mempool {
    pub fn allocate(
        vfio_fd: &VfioStruct,
        entries: usize,
        size: usize,
        path: &str,
    ) -> Result<Rc<Mempool>, Box<dyn Error>> {
        let entry_size = match size {
            0 => 2048,
            x => x,
        };

        if vfio_fd.get_vfio_fd().is_none() && HUGE_PAGE_SIZE % entry_size != 0 {
            panic!("entry size must be a divisor of the page size");
        }

        let dma: Dma<u8> = Dma::allocate(vfio_fd, entries * entry_size, false, path)?;
        let mut phys_addresses = Vec::with_capacity(entries);

        for i in 0..entries {
            if vfio_fd.get_vfio_fd().is_some() {
                phys_addresses.push(unsafe { dma.virt.add(i * entry_size) } as usize);
            } else {
                phys_addresses
                    .push(unsafe { virt_to_phys(dma.virt.add(i * entry_size) as usize)? });
            }
        }

        let pool = Mempool {
            base_addr: dma.virt,
            num_entries: entries,
            entry_size,
            phys_addresses,
            free_stack: RefCell::new(Vec::with_capacity(entries)),
        };

        unsafe {
            libc::memset(
                pool.base_addr as *mut core::ffi::c_void,
                (pool.num_entries * pool.entry_size) as i32,
                0x00,
            );
        }

        let pool = Rc::new(pool);
        pool.free_stack.borrow_mut().extend(0..entries);

        Ok(pool)
    }

    /// Removes a packet from the packet pool and returns it, or [`None`] if the pool is empty.
    pub(crate) fn alloc_buf(&self) -> Option<usize> {
        self.free_stack.borrow_mut().pop()
    }

    /// Returns a packet to the packet pool.
    pub(crate) fn free_buf(&self, id: usize) {
        self.free_stack.borrow_mut().push(id);
    }

    /// Returns a packet to the packet pool.
    pub(crate) unsafe fn get_virt_addr(&self, id: usize) -> *mut u8 {
        self.base_addr.add(id * self.entry_size)
    }

    /// Returns a packet to the packet pool.
    pub(crate) unsafe fn get_phys_addr(&self, id: usize) -> usize {
        self.phys_addresses[id]
    }

    pub fn entry_size(&self) -> usize {
        self.entry_size
    }
}
/// Translates a virtual address to its physical counterpart.
pub(crate) fn virt_to_phys(addr: usize) -> Result<usize, Box<dyn Error>> {
    let pagesize = unsafe { libc::sysconf(libc::_SC_PAGE_SIZE) } as usize;

    let mut file = fs::OpenOptions::new()
        .read(true)
        .open("/proc/self/pagemap")?;

    file.seek(io::SeekFrom::Start(
        (addr / pagesize * mem::size_of::<usize>()) as u64,
    ))?;

    let mut buffer = [0; mem::size_of::<usize>()];
    file.read_exact(&mut buffer)?;

    let phys = unsafe { mem::transmute::<[u8; mem::size_of::<usize>()], usize>(buffer) };
    Ok((phys & 0x007f_ffff_ffff_ffff) * pagesize + addr % pagesize)
}
