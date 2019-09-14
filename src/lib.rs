#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
pub mod prelude {
    pub use crate::constant::*;
    pub use crate::util::check_kernel_version;
}

mod constant;
mod driver;
mod pci;
mod realtekfe;
pub(crate) mod util;
