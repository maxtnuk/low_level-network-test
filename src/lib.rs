pub mod prelude {}
mod test_prelude {
    pub use crate::realtekfe::constant::*;
    pub use crate::realtekfe::util::*;
}

mod devices;
mod realtekfe;
mod vfio;

#[cfg(test)]
mod tests {
    #[test]
    fn check_network() {
        use crate::test_prelude::*;
        let cur = check_kernel_version();
        let other = KernelVersion::new(2, 32, 12);
        assert!(cur >= other, "other is bigger than current kernel version");
    }
}
