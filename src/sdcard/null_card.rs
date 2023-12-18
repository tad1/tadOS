
use embedded_sdmmc::{BlockDevice, self};

use super::SdResult;

pub struct NullSdCard;

pub static NULL_SD_CARD: NullSdCard = NullSdCard {};

impl BlockDevice for NullSdCard{
    type Error = SdResult;

    fn read(
        &self,
        _blocks: &mut [embedded_sdmmc::Block],
        _start_block_idx: embedded_sdmmc::BlockIdx,
        _reason: &str,
    ) -> Result<(), Self::Error> {
        Ok(())
    }

    fn write(&self, _blocks: &[embedded_sdmmc::Block], _start_block_idx: embedded_sdmmc::BlockIdx) -> Result<(), Self::Error> {
        Ok(())
    }

    fn num_blocks(&self) -> Result<embedded_sdmmc::BlockCount, Self::Error> {
        Ok(embedded_sdmmc::BlockCount(0))
    }
}