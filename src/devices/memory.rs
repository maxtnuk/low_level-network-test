use std::cell::RefCell;

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
pub struct Mempool {
    base_addr: *mut u8,
    num_entries: usize,
    entry_size: usize,
    phys_addresses: Vec<usize>,
    pub(crate) free_stack: RefCell<Vec<usize>>,
}
