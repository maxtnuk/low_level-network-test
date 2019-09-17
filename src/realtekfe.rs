use std::collections::VecDeque;
use std::os::unix::io::RawFd;
use std::rc::Rc;

use crate::devices::memory::Mempool;

//Realtek Fast Ethernet Controller mod
pub(crate) mod constant;
pub(crate) mod util;

use constant::*;

pub struct RfeDevice {
    pci_addr: String,
    addr: *mut u8,
    len: usize,
    num_rx_queues: u16,
    num_tx_queues: u16,
    rx_queues: Vec<RfeRxQueue>,
    tx_queues: Vec<RfeTxQueue>,
    vfio_fd: Option<Rc<RawFd>>,
}
struct RfeRxQueue {
    descriptors: *mut rfe_fd,
    num_descriptors: usize,
    pool: Rc<Mempool>,
    bufs_in_use: Vec<usize>,
    rx_index: usize,
}

struct RfeTxQueue {
    descriptors: *mut rfe_fd,
    num_descriptors: usize,
    pool: Option<Rc<Mempool>>,
    bufs_in_use: VecDeque<usize>,
    clean_index: usize,
    tx_index: usize,
}
