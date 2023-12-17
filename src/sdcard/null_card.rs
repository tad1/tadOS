
use embedded_sdmmc::{BlockDevice, self};

use super::{interface, SdResult};
use core::fmt;

pub struct NullSdCard;

pub static NULL_SD_CARD: NullSdCard = NullSdCard {};

impl BlockDevice for NullSdCard{
    type Error = SdResult;

    fn read(
        &self,
        blocks: &mut [embedded_sdmmc::Block],
        start_block_idx: embedded_sdmmc::BlockIdx,
        reason: &str,
    ) -> Result<(), Self::Error> {
        Ok(())
    }

    fn write(&self, blocks: &[embedded_sdmmc::Block], start_block_idx: embedded_sdmmc::BlockIdx) -> Result<(), Self::Error> {
        Ok(())
    }

    fn num_blocks(&self) -> Result<embedded_sdmmc::BlockCount, Self::Error> {
        Ok(embedded_sdmmc::BlockCount(0))
    }
}