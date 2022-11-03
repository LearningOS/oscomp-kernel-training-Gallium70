use alloc::vec::Vec;
use easy_fs::BlockDevice;

use crate::sync::UPSafeCell;

pub struct RamDisk(UPSafeCell<Vec<u8>>);

const BLK_SIZE: usize = 512;

impl RamDisk {
    pub fn new() -> Self {
        unsafe { Self(UPSafeCell::new(include_bytes!("../../../fs.img").to_vec())) }
    }
}

impl BlockDevice for RamDisk {
    fn read_block(&self, block_id: usize, buf: &mut [u8]) {
        buf.copy_from_slice(
            &self.0.exclusive_access()[block_id * BLK_SIZE..block_id * BLK_SIZE + BLK_SIZE],
        );
    }
    fn write_block(&self, block_id: usize, buf: &[u8]) {
        self.0.exclusive_access()[block_id * BLK_SIZE..block_id * BLK_SIZE + BLK_SIZE]
            .copy_from_slice(buf);
    }
}
