
use crate::fs::blockdevice::{BlockDevice, self};

use super::{interface, SdResult};
use core::fmt;

pub struct NullSdCard;

pub static NULL_SD_CARD: NullSdCard = NullSdCard {};

impl BlockDevice for NullSdCard{
    type Error = SdResult;

    fn read(
        &self,
        blocks: &mut [blockdevice::Block],
        start_block_idx: blockdevice::BlockIdx,
        reason: &str,
    ) -> Result<(), Self::Error> {
        Ok(())
    }

    fn write(&self, blocks: &[blockdevice::Block], start_block_idx: blockdevice::BlockIdx) -> Result<(), Self::Error> {
        Ok(())
    }

    fn num_blocks(&self) -> Result<blockdevice::BlockCount, Self::Error> {
        Ok(blockdevice::BlockCount(0))
    }
}