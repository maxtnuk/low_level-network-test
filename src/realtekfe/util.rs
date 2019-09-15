use std::cmp::Ordering;
use std::ffi::CStr;

use libc::{uname, utsname};

#[inline]
fn digit(dst: &mut usize, b: u8) {
    *dst *= 10;
    *dst += (b - b'0') as usize;
}

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
#[derive(PartialEq)]
pub struct KernelVersion(usize, usize, usize);
impl KernelVersion {
    pub fn new(a: usize, b: usize, c: usize) -> Self {
        KernelVersion(a, b, c)
    }
}
impl PartialOrd for KernelVersion {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.0 == other.0 {
            if self.1 == other.1 {
                Some(self.2.cmp(&other.2))
            } else {
                Some(self.1.cmp(&other.1))
            }
        } else {
            Some(self.0.cmp(&other.0))
        }
    }
}

pub fn check_kernel_version() -> KernelVersion {
    let mut buf = make_utsname();
    unsafe {
        uname(&mut buf as *mut utsname);
    }

    let mut curr: usize = 0;
    let mut major: usize = 0;
    let mut minor: usize = 0;
    let mut patch: usize = 0;

    let release = unsafe { CStr::from_ptr(buf.release.as_mut_ptr()) };

    for b in release.to_bytes() {
        if curr >= 3 {
            break;
        }

        match b {
            b'.' | b'-' => {
                curr += 1;
            }
            b'0'..=b'9' => match curr {
                0 => digit(&mut major, *b),
                1 => digit(&mut minor, *b),
                _ => digit(&mut patch, *b),
            },
            _ => break,
        }
    }
    println!("{}.{}.{}", major, minor, patch);
    KernelVersion(major, minor, patch)
}
