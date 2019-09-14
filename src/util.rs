use libc::{uname, utsname};
use std::ffi::CStr;

fn make_utsname() -> utsname {
    utsname {
        sysname: [0; 65],
        nodename: [0; 65],
        release: [0; 65],
        version: [0; 65],
        machine: [0; 65],
        domainname: [0; 65],
    }
}

pub fn check_kernel_version() {
    let mut buf = make_utsname();
    unsafe {
        uname(&mut buf as *mut utsname);
    }
    let release = unsafe { CStr::from_ptr(buf.release.as_mut_ptr()) };
    println!("sysname: {}", release.to_str().unwrap());
}
