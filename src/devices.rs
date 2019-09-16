use std::error::Error;
use std::os::unix::io::RawFd;

pub(crate) mod memory;
pub(crate) mod net;
pub(crate) mod pci;

pub trait DeviceInfo {
    fn init(pci_addr: &str, num_rx_queues: u16, num_tx_queues: u16) -> Result<Self, Box<dyn Error>>
    where
        Self: Sized;
    fn get_driver_name(&self) -> &str;
    fn is_card_iommu_capable(&self) -> bool;
    fn get_vfio_container(&self) -> Option<RawFd>;
    fn get_pci_addr(&self) -> &str;
    fn get_mac_addr(&self) -> [u8; 6];
    fn set_mac_addr(&self, mac: [u8; 6]);
    fn get_link_speed(&self) -> u16;
}
