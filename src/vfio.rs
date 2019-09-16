#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(clippy::all)]

use crate::realtekfe::ioctl::*;

pub const VFIO_API_VERSION: u64 = 0;
pub const VFIO_TYPE1_IOMMU: u64 = 1;
pub const VFIO_SPAPR_TCE_IOMMU: u64 = 2;
pub const VFIO_TYPE1v2_IOMMU: u64 = 3;
pub const VFIO_DMA_CC_IOMMU: u64 = 4;
pub const VFIO_EEH: u64 = 5;
pub const VFIO_TYPE1_NESTING_IOMMU: u64 = 6;
pub const VFIO_SPAPR_TCE_v2_IOMMU: u64 = 7;
pub const VFIO_NOIOMMU_IOMMU: u64 = 8;

pub const VFIO_TYPE: u8 = b';';
pub const VFIO_BASE: u64 = 100;

struct vfio_info_cap_header {
    id: u16,
    version: u16,
    next: u32,
}

pub const VFIO_GET_API_VERSION: u64 = IO(VFIO_TYPE as u64, VFIO_BASE + 0);
