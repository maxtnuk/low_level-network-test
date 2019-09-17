#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]

use super::ioctl::*;
//grabed from linux/vfio.h
pub const VFIO_API_VERSION: u32 = 0;
pub const VFIO_TYPE1_IOMMU: u32 = 1;
pub const VFIO_SPAPR_TCE_IOMMU: u32 = 2;
pub const VFIO_TYPE1v2_IOMMU: u32 = 3;
pub const VFIO_DMA_CC_IOMMU: u32 = 4;
pub const VFIO_EEH: u32 = 5;
pub const VFIO_TYPE1_NESTING_IOMMU: u32 = 6;
pub const VFIO_SPAPR_TCE_v2_IOMMU: u32 = 7;
pub const VFIO_NOIOMMU_IOMMU: u32 = 8;

pub const VFIO_TYPE: u8 = b';';
pub const VFIO_BASE: u64 = 100;

pub struct vfio_info_cap_header {
    pub id: u16,
    pub version: u16,
    pub next: u32,
}
pub struct vfio_group_status {
    pub argsz: u32,
    pub flags: u32,
}
pub struct vfio_device_info {
    pub argsz: u32,
    pub flags: u32,
    pub num_regions: u32,
    pub num_irqs: u32,
}

pub const VFIO_GET_API_VERSION: u64 = IO(VFIO_TYPE as u64, VFIO_BASE + 0);
pub const VFIO_CHECK_EXTENSION: u64 = IO(VFIO_TYPE as u64, VFIO_BASE + 1);
pub const VFIO_SET_IOMMU: u64 = IO(VFIO_TYPE as u64, VFIO_BASE + 2);
pub const VFIO_GROUP_GET_STATUS: u64 = IO(VFIO_TYPE as u64, VFIO_BASE + 3);
pub const VFIO_GROUP_SET_CONTAINER: u64 = IO(VFIO_TYPE as u64, VFIO_BASE + 4);
pub const VFIO_GROUP_UNSET_CONTAINER: u64 = IO(VFIO_TYPE as u64, VFIO_BASE + 5);
pub const VFIO_GROUP_GET_DEVICE_FD: u64 = IO(VFIO_TYPE as u64, VFIO_BASE + 6);
pub const VFIO_DEVICE_GET_INFO: u64 = IO(VFIO_TYPE as u64, VFIO_BASE + 7);
pub const VFIO_DEVICE_GET_REGION_INFO: u64 = IO(VFIO_TYPE as u64, VFIO_BASE + 8);
pub const VFIO_DEVICE_GET_IRQ_INFO: u64 = IO(VFIO_TYPE as u64, VFIO_BASE + 9);
pub const VFIO_DEVICE_SET_IRQS: u64 = IO(VFIO_TYPE as u64, VFIO_BASE + 10);
pub const VFIO_DEVICE_RESET: u64 = IO(VFIO_TYPE as u64, VFIO_BASE + 11);
pub const VFIO_DEVICE_GET_PCI_HOT_RESET_INFO: u64 = IO(VFIO_TYPE as u64, VFIO_BASE + 12);
pub const VFIO_DEVICE_PCI_HOT_RESET: u64 = IO(VFIO_TYPE as u64, VFIO_BASE + 13);
pub const VFIO_DEVICE_QUERY_GFX_PLANE: u64 = IO(VFIO_TYPE as u64, VFIO_BASE + 14);
pub const VFIO_DEVICE_GET_GFX_DMABUF: u64 = IO(VFIO_TYPE as u64, VFIO_BASE + 15);
pub const VFIO_DEVICE_IOEVENTFD: u64 = IO(VFIO_TYPE as u64, VFIO_BASE + 16);

pub const VFIO_IOMMU_GET_INFO: u64 = IO(VFIO_TYPE as u64, VFIO_BASE + 12);
pub const VFIO_IOMMU_MAP_DMA: u64 = IO(VFIO_TYPE as u64, VFIO_BASE + 13);
pub const VFIO_IOMMU_UNMAP_DMA: u64 = IO(VFIO_TYPE as u64, VFIO_BASE + 14);
pub const VFIO_IOMMU_ENABLE: u64 = IO(VFIO_TYPE as u64, VFIO_BASE + 15);
pub const VFIO_IOMMU_DISABLE: u64 = IO(VFIO_TYPE as u64, VFIO_BASE + 16);

pub const VFIO_IOMMU_SPAPR_TCE_GET_INFO: u64 = IO(VFIO_TYPE as u64, VFIO_BASE + 12);

pub const VFIO_EEH_PE_OP: u64 = IO(VFIO_TYPE as u64, VFIO_BASE + 21);

pub const VFIO_IOMMU_SPAPR_REGISTER_MEMORY: u64 = IO(VFIO_TYPE as u64, VFIO_BASE + 17);
pub const VFIO_IOMMU_SPAPR_UNREGISTER_MEMORY: u64 = IO(VFIO_TYPE as u64, VFIO_BASE + 18);
pub const VFIO_IOMMU_SPAPR_TCE_CREATE: u64 = IO(VFIO_TYPE as u64, VFIO_BASE + 19);
pub const VFIO_IOMMU_SPAPR_TCE_REMOVE: u64 = IO(VFIO_TYPE as u64, VFIO_BASE + 20);

pub const VFIO_GROUP_FLAGS_VIABLE: u32 = 1 << 0;
pub const VFIO_GROUP_FLAGS_CONTAINER_SET: u32 = 1 << 1;

pub const VFIO_DEVICE_FLAGS_RESET: u32 = 1 << 0;
pub const VFIO_DEVICE_FLAGS_PCI: u32 = 1 << 1;
pub const VFIO_DEVICE_FLAGS_PLATFORM: u32 = 1 << 2;
pub const VFIO_DEVICE_FLAGS_AMBA: u32 = 1 << 3;
pub const VFIO_DEVICE_FLAGS_CCW: u32 = 1 << 4;
pub const VFIO_DEVICE_FLAGS_AP: u32 = 1 << 5;

pub const VFIO_DEVICE_API_PCI_STRING: &str = "vfio-pci";
pub const VFIO_DEVICE_API_PLATFORM_STRING: &str = "vfio-platform";
pub const VFIO_DEVICE_API_AMBA_STRING: &str = "vfio-amba";
pub const VFIO_DEVICE_API_CCW_STRING: &str = "vfio-ccw";
pub const VFIO_DEVICE_API_AP_STRING: &str = "vfio-ap";

pub struct vfio_region_info {
    pub argsz: u32,
    pub flags: u32,
    pub index: u32,
    pub cap_offset: u32,
    pub size: usize,
    pub offset: u64,
}

//enum part
pub const VFIO_PCI_CONFIG_REGION_INDEX: u32 = 7;

pub const VFIO_REGION_INFO_FLAG_READ: u32 = 1 << 0;
pub const VFIO_REGION_INFO_FLAG_WRITE: u32 = 1 << 1;
pub const VFIO_REGION_INFO_FLAG_MMAP: u32 = 1 << 2;
pub const VFIO_REGION_INFO_FLAG_CAPS: u32 = 1 << 3;

pub const VFIO_REGION_INFO_CAP_SPARSE_MMAP: u32 = 1;
pub const VFIO_REGION_INFO_CAP_TYPE: u32 = 2;

pub struct vfio_region_sparse_mmap_area {
    pub offset: u64,
    pub size: usize,
}
pub struct vfio_region_info_cap_sparse_mmap {
    pub header: vfio_info_cap_header,
    pub nr_areas: u32,
    pub reserved: u32,
    pub areas: Vec<vfio_region_sparse_mmap_area>,
}

pub struct vfio_region_info_cap_type {
    pub header: vfio_info_cap_header,
    pub mtype: u32, //mtype = type
    pub subtype: u32,
}

pub const VFIO_REGION_TYPE_PCI_VENDOR_TYPE: u32 = 1 << 31;
pub const VFIO_REGION_TYPE_PCI_VENDOR_MASK: u32 = 0xffff;

pub const VFIO_REGION_SUBTYPE_INTEL_IGD_OPREGION: u32 = 1;
pub const VFIO_REGION_SUBTYPE_INTEL_IGD_HOST_CFG: u32 = 2;
pub const VFIO_REGION_SUBTYPE_INTEL_IGD_LPC_CFG: u32 = 3;

pub const VFIO_REGION_TYPE_GFX: u32 = 1;
pub const VFIO_REGION_SUBTYPE_GFX_EDID: u32 = 1;

pub struct vfio_region_gfx_edid {
    pub edid_offset: u32,
    pub edid_max_size: usize,
    pub edid_size: usize,
    pub max_xres: u32,
    pub max_yres: u32,
    pub link_state: u32,
}

pub const VFIO_DEVICE_GFX_LINK_STATE_UP: u32 = 1;
pub const VFIO_DEVICE_GFX_LINK_STATE_DOWN: u32 = 2;

pub const VFIO_REGION_TYPE_CCW: u32 = 2;
pub const VFIO_REGION_SUBTYPE_CCW_ASYNC_CMD: u32 = 1;
pub const VFIO_REGION_SUBTYPE_NVIDIA_NVLINK2_RAM: u32 = 1;
pub const VFIO_REGION_SUBTYPE_IBM_NVLINK2_ATSD: u32 = 1;

pub const VFIO_REGION_INFO_CAP_MSIX_MAPPABLE: u32 = 3;
pub const VFIO_REGION_INFO_CAP_NVLINK2_SSATGT: u32 = 4;
pub const VFIO_REGION_INFO_CAP_NVLINK2_LNKSPD: u32 = 5;

pub struct vfio_region_info_cap_nvlink2_ssatgt {
    pub header: vfio_info_cap_header,
    pub tgt: u64,
}

pub struct vfio_region_info_cap_nvlink2_lnkspd {
    pub header: vfio_info_cap_header,
    pub link_speed: u32,
    pub pad: u32,
}

pub struct vfio_irq_info {
    pub argsz: u32,
    pub flags: u32,
    pub index: u32,
    pub count: u32,
}

pub const VFIO_IRQ_INFO_EVENTFD: u32 = 1 << 0;
pub const VFIO_IRQ_INFO_MASKABLE: u32 = 1 << 1;
pub const VFIO_IRQ_INFO_AUTOMASKED: u32 = 1 << 2;
pub const VFIO_IRQ_INFO_NORESIZE: u32 = 1 << 3;

pub struct vfio_irq_set {
    pub argsz: u32,
    pub flags: u32,
    pub index: u32,
    pub start: u32,
    pub count: u32,
    pub data: *mut u8,
}

pub const VFIO_IRQ_SET_DATA_NONE: u32 = 1 << 0;
pub const VFIO_IRQ_SET_DATA_BOOL: u32 = 1 << 1;
pub const VFIO_IRQ_SET_DATA_EVENTFD: u32 = 1 << 2;
pub const VFIO_IRQ_SET_ACTION_MASK: u32 = 1 << 3;
pub const VFIO_IRQ_SET_ACTION_UNMASK: u32 = 1 << 4;
pub const VFIO_IRQ_SET_ACTION_TRIGGER: u32 = 1 << 5;

pub const VFIO_IRQ_SET_DATA_TYPE_MASK: u32 =
    VFIO_IRQ_SET_DATA_NONE | VFIO_IRQ_SET_DATA_BOOL | VFIO_IRQ_SET_DATA_EVENTFD;
pub const VFIO_IRQ_SET_ACTION_TYPE_MASK: u32 =
    VFIO_IRQ_SET_ACTION_MASK | VFIO_IRQ_SET_ACTION_UNMASK | VFIO_IRQ_SET_ACTION_TRIGGER;

pub struct vfio_pci_dependent_device {
    pub group_id: u32,
    pub segment: u16,
    pub bus: u8,
    pub devfn: u8,
}
pub struct vfio_pci_hot_reset_info {
    pub argsz: u32,
    pub flags: u32,
    pub count: u32,
    pub devices: Vec<vfio_pci_dependent_device>,
}
pub struct vfio_pci_hot_reset {
    pub argsz: u32,
    pub flags: u32,
    pub count: u32,
    pub group_fds: *mut i32,
}

pub struct vfio_device_gfx_plane_info {
    pub argsz: u32,
    pub flags: u32,
    pub drm_plane_type: u32,
    pub drm_format: u32,
    pub drm_format_mod: u64,
    pub width: u32,
    pub height: u32,
    pub stride: u32,
    pub size: usize,
    pub x_pos: u32,
    pub y_pos: u32,
    pub x_hot: u32,
    pub y_hot: u32,
    pub id_sec: section,
}
pub union section {
    pub region_index: u32,
    pub dmabuf_id: u32,
}
pub const VFIO_DEVICE_IOEVENTFD_8: u8 = 1 << 0;
pub const VFIO_DEVICE_IOEVENTFD_16: u16 = 1 << 1;
pub const VFIO_DEVICE_IOEVENTFD_32: u32 = 1 << 2;
pub const VFIO_DEVICE_IOEVENTFD_64: u64 = 1 << 3;

pub const VFIO_DEVICE_IOEVENTFD_SIZE_MASK: u8 = 0xf;

pub struct vfio_device_ioeventfd {
    pub argsz: u32,
    pub flags: u32,
    pub offset: u64,
    pub data: u64,
    pub fd: i32,
}

pub struct vfio_iommu_type1_info {
    pub argsz: u32,
    pub flags: u32,
    pub iova_pgsizes: usize,
}
pub const VFIO_IOMMU_INFO_PGSIZES: u32 = 1 << 0;
pub struct vfio_iommu_type1_dma_map {
    pub argsz: u32,
    pub flags: u32,
    pub vaddr: *mut u8,
    pub iova: *mut u8,
    pub size: usize,
}

pub const VFIO_DMA_MAP_FLAG_READ: u32 = 1 << 0;
pub const VFIO_DMA_MAP_FLAG_WRITE: u32 = 1 << 1;

pub struct vfio_iommu_type1_dma_unmap {
    pub argsz: u32,
    pub flags: u32,
    pub vaddr: u64,
    pub iova: u64,
}

pub struct vfio_iommu_spapr_tce_ddw_info {
    pub pgsizes: usize,
    pub max_dynamic_windows_supported: u32,
    pub levels: u32,
}

pub struct vfio_eeh_pe_err {
    pub mtype: u32,
    pub func: u32,
    pub addr: u64,
    pub mask: u64,
}
pub struct vfio_eeh_pe_op {
    pub argsz: u32,
    pub flags: u32,
    pub op: u32,
    pub err: vfio_eeh_pe_err,
}

pub const VFIO_EEH_PE_DISABLE: u32 = 0;
pub const VFIO_EEH_PE_ENABLE: u32 = 1;
pub const VFIO_EEH_PE_UNFREEZE_IO: u32 = 2;
pub const VFIO_EEH_PE_UNFREEZE_DMA: u32 = 3;
pub const VFIO_EEH_PE_GET_STATE: u32 = 4;
pub const VFIO_EEH_PE_STATE_NORMAL: u32 = 0;
pub const VFIO_EEH_PE_STATE_RESET: u32 = 1;
pub const VFIO_EEH_PE_STATE_STOPPED: u32 = 2;
pub const VFIO_EEH_PE_STATE_STOPPED_DMA: u32 = 3;
pub const VFIO_EEH_PE_STATE_UNAVAIL: u32 = 4;
pub const VFIO_EEH_PE_RESET_DEACTIVATE: u32 = 5;
pub const VFIO_EEH_PE_RESET_HOT: u32 = 6;
pub const VFIO_EEH_PE_RESET_FUNDAMENTAL: u32 = 7;
pub const VFIO_EEH_PE_CONFIGURE: u32 = 8;
pub const VFIO_EEH_PE_INJECT_ERR: u32 = 9;

pub struct vfio_iommu_spapr_register_memory {
    pub argsz: u32,
    pub flags: u32,
    pub vaddr: u64,
    pub size: usize,
}
pub struct vfio_iommu_spapr_tce_create {
    pub argsz: u32,
    pub flags: u32,
    /* in */
    pub page_shift: u32,
    pub resv1: u32,
    pub window_size: usize,
    pub levels: u32,
    pub resv2: u32,
    /* out */
    pub start_addr: u64,
}
pub struct vfio_iommu_spapr_tce_remove {
    pub argsz: u32,
    pub flags: u32,
    pub start_addr: u64,
}
