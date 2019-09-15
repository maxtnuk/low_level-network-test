pub mod prelude {
    pub use crate::constant::*;
    pub use crate::util::*;
}

mod constant;
mod driver;
mod pci;
mod realtekfe;
pub(crate) mod util;

#[cfg(test)]
mod tests {
    #[test]
    fn check_network() {
        use crate::prelude::*;
        let cur = check_kernel_version();
        let other = KernelVersion::new(2, 32, 12);
        assert!(cur >= other, "other is bigger than current kernel version");
    }
}
