use lr_network::prelude::*;

//Don't use this to test library
//This example for test library inner functions
fn main() {
    let cur = check_kernel_version();
    let other = KernelVersion::new(5, 32, 12);
    assert!(cur >= other, "compare faile");
    println!("Hello world");
}
