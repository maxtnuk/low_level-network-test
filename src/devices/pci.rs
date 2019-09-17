use std::error::Error;
use std::fs::{self, File, OpenOptions};
use std::io::{ErrorKind, Seek, SeekFrom, Write};
use std::os::unix::prelude::AsRawFd;
use std::ptr;

use byteorder::{NativeEndian, ReadBytesExt, WriteBytesExt};

use crate::prelude::*;

//this mod functions for my laptop network pcie
//pcie version pcie 3.0

pub const CMD_REG_OFFSET: u64 = 4;
pub const BUS_MS_ENABLE_BIT: u64 = 2;

const ROOT_PCI_DVICE: &str = "/sys/bus/pci/devices/";

type IOResult = Result<(), Box<dyn Error>>;

pub fn add_pci_unbind(pci_addr: &str) -> IOResult {
    let path = ROOT_PCI_DVICE.to_owned() + pci_addr + "/driver/unbind";

    match OpenOptions::new().write(true).open(path) {
        Ok(mut f) => {
            f.write_all(pci_addr.as_bytes()).unwrap();
            Ok(())
        }
        Err(ref e) if e.kind() == ErrorKind::NotFound => {
            println!("there is no unbind file");
            Ok(())
        }
        Err(ref e) if e.kind() == ErrorKind::Other => {
            println!("other io problum occur");
            Ok(())
        }
        Err(e) => Err(Box::new(e)),
    }
}

pub fn enable_dma(pci_addr: &str) -> IOResult {
    let path = ROOT_PCI_DVICE.to_owned() + pci_addr + "/config";
    let mut f = OpenOptions::new()
        .read(true)
        .write(true)
        .open(path)
        .unwrap();

    assert_eq!(f.seek(SeekFrom::Start(CMD_REG_OFFSET))?, CMD_REG_OFFSET);
    let mut dma = f.read_u16::<NativeEndian>()?;

    dma |= 1 << BUS_MS_ENABLE_BIT;

    assert_eq!(f.seek(SeekFrom::Start(CMD_REG_OFFSET))?, CMD_REG_OFFSET);
    f.write_u16::<NativeEndian>(dma)?;

    Ok(())
}

pub fn pci_map_resource(pci_addr: &str) -> Result<(*mut u8, usize), Box<dyn Error>> {
    let path = ROOT_PCI_DVICE.to_owned() + pci_addr + "/resource0";
    add_pci_unbind(pci_addr).unwrap();
    enable_dma(pci_addr).unwrap();

    let f = OpenOptions::new()
        .read(true)
        .write(true)
        .open(&path)
        .unwrap();
    let len = fs::metadata(path)?.len() as usize;

    let ptr = unsafe {
        libc::mmap(
            ptr::null_mut(),
            len,
            libc::PROT_READ | libc::PROT_WRITE,
            libc::MAP_SHARED,
            f.as_raw_fd(),
            0,
        ) as *mut u8
    };
    if ptr.is_null() || len == 0 {
        Err("pci mapping fail".into())
    } else {
        Ok((ptr, len))
    }
}
pub fn open_pci_file(pci_name: &str, resource: &str) -> Result<File, Box<dyn Error>> {
    let path = ROOT_PCI_DVICE.to_owned() + pci_name + "/" + resource;
    Ok(OpenOptions::new().read(true).open(path)?)
}
pub fn read_nbytes(file: &mut File, offset: usize, nbytes: usize) -> Result<u64, Box<dyn Error>> {
    file.seek(SeekFrom::Start(offset as u64))?;
    Ok(file.read_uint::<NativeEndian>(nbytes)?)
}
