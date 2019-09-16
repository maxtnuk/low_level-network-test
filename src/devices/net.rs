use std::collections::VecDeque;
use std::rc::Rc;

use super::memory::Mempool;

pub struct Packet {
    pub(crate) addr_virt: *mut u8,
    pub(crate) addr_phys: usize,
    pub(crate) len: usize,
    pub(crate) pool: Rc<Mempool>,
    pub(crate) pool_entry: usize,
}
pub trait PacketQue {
    fn rx_batch(
        &mut self,
        queue_id: u32,
        buffer: &mut VecDeque<Packet>,
        num_packets: usize,
    ) -> usize;
    fn tx_batch(&mut self, queue_id: u32, buffer: &mut VecDeque<Packet>) -> usize;
}
