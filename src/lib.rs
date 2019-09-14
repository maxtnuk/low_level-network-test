#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
pub mod prelude {
    pub use crate::constant::*;
}

mod constant;
mod driver;
mod pci;
