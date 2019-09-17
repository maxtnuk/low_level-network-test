use std::error::Error;
use std::fs::{read_link, OpenOptions};
use std::mem::size_of;
use std::os::unix::io::RawFd;
use std::os::unix::io::{AsRawFd, IntoRawFd};
use std::ptr;
use std::rc::Rc;

pub(crate) mod ioctl;
pub(crate) mod vfio;

use crate::devices::pci::{BUS_MS_ENABLE_BIT, CMD_REG_OFFSET};
use crate::devices::{DeviceInfo, DeviceInit};
use vfio::*;

//create vfio with iommu
pub struct VfioStruct {
    vfio_fd: Option<RawFd>,
}
impl VfioStruct {
    pub fn new() -> Self {
        VfioStruct { vfio_fd: None }
    }
    pub fn get_vfio_fd(&self) -> Option<RawFd> {
        self.vfio_fd.clone()
    }
    fn vfio_init(&mut self) -> Result<(), Box<dyn Error>> {
        let container_file = OpenOptions::new()
            .read(true)
            .write(true)
            .open("/dev/vfio/vfio")
            .unwrap();
        let cfd = container_file.into_raw_fd();

        // check if the container's API version is the same as the VFIO API's
        if unsafe { libc::ioctl(cfd, VFIO_GET_API_VERSION) } != VFIO_API_VERSION as i32 {
            return Err("unknown VFIO API Version".into());
        }

        // check if type1 is supported
        if unsafe { libc::ioctl(cfd, VFIO_CHECK_EXTENSION, VFIO_TYPE1_IOMMU) } != 1 {
            return Err("container doesn't support Type1 IOMMU".into());
        }

        if unsafe { libc::ioctl(cfd, VFIO_SET_IOMMU, VFIO_TYPE1_IOMMU) } == -1 {
            return Err(format!(
                "failed to VFIO_SET_IOMMU to VFIO_TYPE1_IOMMU. Errno: {}",
                unsafe { *libc::__errno_location() }
            )
            .into());
        }

        self.vfio_fd = Some(cfd);
        Ok(())
    }
    pub fn get_dfd(&self, pci_addr: &str) -> Result<RawFd, Box<dyn Error>> {
        if self.vfio_fd.is_none() {
            return Err(format!("failed to no vfio container").into());
        }
        let group_status: vfio_group_status = vfio_group_status {
            argsz: size_of::<vfio_group_status>() as u32,
            flags: 0,
        };
        let link = read_link(format!("/sys/bus/pci/devices/{}/iommu_group", pci_addr)).unwrap();
        let group = link
            .file_name()
            .unwrap()
            .to_str()
            .unwrap()
            .parse::<i32>()
            .unwrap();

        // open the devices' group
        let group_file = OpenOptions::new()
            .read(true)
            .write(true)
            .open(format!("/dev/vfio/{}", group))
            .unwrap();
        let gfd = group_file.as_raw_fd();
        if unsafe { libc::ioctl(gfd, VFIO_GROUP_SET_CONTAINER, &self.vfio_fd.unwrap()) } == -1 {
            return Err(
                format!("failed to VFIO_GROUP_SET_CONTAINER. Errno: {}", unsafe {
                    *libc::__errno_location()
                })
                .into(),
            );
        }

        if unsafe { libc::ioctl(gfd, VFIO_GROUP_GET_STATUS, &group_status) } == -1 {
            return Err(
                format!("failed to VFIO_GROUP_GET_STATUS. Errno: {}", unsafe {
                    *libc::__errno_location()
                })
                .into(),
            );
        }
        if (group_status.flags & VFIO_GROUP_FLAGS_VIABLE) != 1 {
            return Err(
                "group is not viable (ie, not all devices in this group are bound to vfio)".into(),
            );
        }

        let dfd = unsafe { libc::ioctl(gfd, VFIO_GROUP_GET_DEVICE_FD, pci_addr) };
        if dfd == -1 {
            return Err(
                format!("failed to VFIO_GROUP_GET_DEVICE_FD. Errno: {}", unsafe {
                    *libc::__errno_location()
                })
                .into(),
            );
        }
        let conf_reg: vfio_region_info = vfio_region_info {
            argsz: size_of::<vfio_region_info>() as u32,
            flags: 0,
            index: VFIO_PCI_CONFIG_REGION_INDEX,
            cap_offset: 0,
            size: 0,
            offset: 0,
        };
        if unsafe { libc::ioctl(dfd, VFIO_DEVICE_GET_REGION_INFO, &conf_reg) } == -1 {
            return Err(format!(
            "failed to VFIO_DEVICE_GET_REGION_INFO for index VFIO_PCI_CONFIG_REGION_INDEX. Errno: {}",
            unsafe { *libc::__errno_location() }
        ).into());
        }

        let mut dma: u16 = 0;
        if unsafe {
            libc::pread(
                dfd,
                &mut dma as *mut _ as *mut libc::c_void,
                2,
                (conf_reg.offset + CMD_REG_OFFSET) as i64,
            )
        } == -1
        {
            return Err(format!("failed to pread DMA bit. Errno: {}", unsafe {
                *libc::__errno_location()
            })
            .into());
        }

        dma |= 1 << BUS_MS_ENABLE_BIT;

        if unsafe {
            libc::pwrite(
                dfd,
                &mut dma as *mut _ as *mut libc::c_void,
                2,
                (conf_reg.offset + CMD_REG_OFFSET) as i64,
            )
        } == -1
        {
            return Err(format!("failed to pwrite DMA bit. Errno: {}", unsafe {
                *libc::__errno_location()
            })
            .into());
        }
        Ok(dfd)
    }
    pub fn make_vfio(&mut self) {
        if self.vfio_fd.is_none() {
            self.vfio_init();
        }
    }
    pub fn vfio_map_region(fd: RawFd, index: u32) -> Result<(*mut u8, usize), Box<dyn Error>> {
        let region_info: vfio_region_info = vfio_region_info {
            argsz: size_of::<vfio_region_info>() as u32,
            flags: 0,
            index,
            cap_offset: 0,
            size: 0,
            offset: 0,
        };
        if unsafe { libc::ioctl(fd, VFIO_DEVICE_GET_REGION_INFO, &region_info) } == -1 {
            return Err(
                format!("failed to VFIO_DEVICE_GET_REGION_INFO. Errno: {}", unsafe {
                    *libc::__errno_location()
                })
                .into(),
            );
        }

        let len = region_info.size as usize;

        let ptr = unsafe {
            libc::mmap(
                ptr::null_mut(),
                len,
                libc::PROT_READ | libc::PROT_WRITE,
                libc::MAP_SHARED,
                fd,
                region_info.offset as i64,
            ) as *mut u8
        };
        if ptr == libc::MAP_FAILED as *mut u8 {
            return Err(format!("failed to mmap region. Errno: {}", unsafe {
                *libc::__errno_location()
            })
            .into());
        }
        let addr = ptr as *mut u8;

        Ok((addr, len))
    }
    pub fn vfio_map_dma(&self, ptr: *mut u8, size: usize) -> Result<usize, Box<dyn Error>> {
        if self.vfio_fd.is_none() {
            return Err("none vfio container".into());
        }
        let iommu_dma_map: vfio_iommu_type1_dma_map = vfio_iommu_type1_dma_map {
            argsz: size_of::<vfio_iommu_type1_dma_map>() as u32,
            vaddr: ptr,
            size,
            iova: ptr,
            flags: VFIO_DMA_MAP_FLAG_READ | VFIO_DMA_MAP_FLAG_WRITE,
        };

        if unsafe { libc::ioctl(self.vfio_fd.unwrap(), VFIO_IOMMU_MAP_DMA, &iommu_dma_map) } != -1 {
            Ok(iommu_dma_map.iova as usize)
        } else {
            Err("failed to map the DMA memory - ulimit set for this user?".into())
        }
    }
}
