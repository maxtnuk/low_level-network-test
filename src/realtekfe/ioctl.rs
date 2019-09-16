#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(clippy::all)]

//grabed from linux/ioctl.h

pub const IOC_NRBITS: u64 = 8;
pub const IOC_TYPEBITS: u64 = 8;
pub const IOC_SIZEBITS: u64 = 14;
pub const IOC_DIRBITS: u64 = 2;

pub const IOC_NRMASK: u64 = (1 << IOC_NRBITS) - 1;
pub const IOC_TYPEMASK: u64 = (1 << IOC_TYPEBITS) - 1;
pub const IOC_SIZEMASK: u64 = (1 << IOC_SIZEBITS) - 1;
pub const IOC_DIRMASK: u64 = (1 << IOC_DIRBITS) - 1;

pub const IOC_NRSHIFT: u64 = 0;
pub const IOC_TYPESHIFT: u64 = IOC_NRSHIFT + IOC_NRBITS;
pub const IOC_SIZESHIFT: u64 = IOC_TYPESHIFT + IOC_TYPEBITS;
pub const IOC_DIRSHIFT: u64 = IOC_SIZESHIFT + IOC_SIZEBITS;

pub const IOC_NONE: u64 = 0;
pub const IOC_WRITE: u64 = 1;
pub const IOC_READ: u64 = 2;

pub fn IOC(dir: u64, ioctype: u64, nr: u64, size: u64) -> u64 {
    (dir << IOC_DIRSHIFT)
        | (ioctype << IOC_TYPESHIFT)
        | (nr << IOC_NRSHIFT)
        | (size << IOC_SIZESHIFT)
}

pub fn IO(ioctype: u64, nr: u64) -> u64 {
    IOC(IOC_NONE, ioctype, nr, 0)
}

pub fn IOC_DIR(nr: u64) -> u64 {
    (nr >> IOC_DIRSHIFT) & IOC_DIRMASK
}
pub fn IOC_TYPE(nr: u64) -> u64 {
    (nr >> IOC_TYPESHIFT) & IOC_TYPEMASK
}
pub fn IOC_NR(nr: u64) -> u64 {
    (nr >> IOC_NRSHIFT) & IOC_NRMASK
}
pub fn IOC_SIZE(nr: u64) -> u64 {
    (nr >> IOC_SIZESHIFT) & IOC_SIZEMASK
}

pub const IOC_IN: u64 = (IOC_WRITE << IOC_DIRSHIFT);
pub const IOC_OUT: u64 = (IOC_READ << IOC_DIRSHIFT);
pub const IOC_INOUT: u64 = ((IOC_WRITE | IOC_READ) << IOC_DIRSHIFT);
pub const IOCSIZE_MASK: u64 = (IOC_SIZEMASK << IOC_SIZESHIFT);
pub const IOCSIZE_SHIFT: u64 = (IOC_SIZESHIFT);
