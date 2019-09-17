use std::collections::VecDeque;
use std::ops::{Deref, DerefMut};
use std::ptr;
use std::rc::Rc;
use std::slice;

use super::memory::Mempool;

pub struct Packet {
    pub(crate) addr_virt: *mut u8,
    pub(crate) addr_phys: usize,
    pub(crate) len: usize,
    pub(crate) pool: Rc<Mempool>,
    pub(crate) pool_entry: usize,
}
impl Clone for Packet {
    fn clone(&self) -> Self {
        let mut p = alloc_pkt(&self.pool, self.len).expect("no buffer available");
        p.clone_from_slice(&self);

        p
    }
}

impl Deref for Packet {
    type Target = [u8];

    fn deref(&self) -> &[u8] {
        unsafe { slice::from_raw_parts(self.addr_virt, self.len) }
    }
}

impl DerefMut for Packet {
    fn deref_mut(&mut self) -> &mut [u8] {
        unsafe { slice::from_raw_parts_mut(self.addr_virt, self.len) }
    }
}

impl Drop for Packet {
    fn drop(&mut self) {
        //println!("drop");
        self.pool.free_buf(self.pool_entry);
    }
}
impl Packet {
    pub(crate) fn new(
        addr_virt: *mut u8,
        addr_phys: usize,
        len: usize,
        pool: Rc<Mempool>,
        pool_entry: usize,
    ) -> Packet {
        Packet {
            addr_virt,
            addr_phys,
            len,
            pool,
            pool_entry,
        }
    }

    pub fn get_virt_addr(&self) -> *mut u8 {
        self.addr_virt
    }

    pub fn get_phys_addr(&self) -> usize {
        self.addr_phys
    }

    pub fn get_pool(&self) -> &Rc<Mempool> {
        &self.pool
    }

    pub fn truncate(&mut self, len: usize) {
        // Validity invariant: the referred to memory range is a proper subset of the previous one.
        self.len = self.len.min(len)
    }
}
pub fn alloc_pkt_batch(
    pool: &Rc<Mempool>,
    buffer: &mut VecDeque<Packet>,
    num_packets: usize,
    packet_size: usize,
) -> usize {
    let mut allocated = 0;

    while let Some(p) = alloc_pkt(pool, packet_size) {
        buffer.push_back(p);

        allocated += 1;
        if allocated >= num_packets {
            break;
        }
    }

    allocated
}

/// Returns a free packet from the `pool`, or [`None`] if the requested packet size exceeds the
/// maximum size for that pool or if the pool is empty.
pub fn alloc_pkt(pool: &Rc<Mempool>, size: usize) -> Option<Packet> {
    if size > pool.entry_size {
        return None;
    }

    match pool.alloc_buf() {
        Some(packet) => unsafe {
            Some(Packet::new(
                pool.get_virt_addr(packet),
                pool.get_phys_addr(packet),
                size,
                pool.clone(),
                packet,
            ))
        },
        _ => None,
    }
}

/// Initializes `len` fields of type `T` at `addr` with `value`.
pub(crate) unsafe fn memset<T: Copy>(addr: *mut T, len: usize, value: T) {
    for i in 0..len {
        ptr::write_volatile(addr.add(i) as *mut T, value);
    }
}
