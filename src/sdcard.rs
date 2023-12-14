mod null_card;

use core::fmt::Error;

use crate::synchronization::{self, NullLock, interface::Mutex};
use crate::fs::blockdevice;

use self::null_card::NULL_SD_CARD;



pub mod interface {

    use crate::fs::blockdevice;

    use super::SdResult;

    pub trait BlockDevice: blockdevice::BlockDevice {}
}

#[allow(non_camel_case_types)]
#[derive(PartialEq, Debug, Clone, Copy)]
pub enum SdResult{
    EMMC_OK,            // NO error
    EMMC_ERROR,         // General non specific SD error
    EMMC_TIMEOUT,       // SD Timeout error
    EMMC_BUSY,          // SD Card is busy
    EMMC_NO_RESP,       // SD Card did not respond
    EMMC_ERROR_RESET,   // SD Card did not reset
    EMMC_ERROR_CLOCK,   // SD Card clock change failed
    EMMC_ERROR_VOLTAGE, // SD Card does not support requested voltage
    EMMC_ERROR_APP_CMD, // SD Card app command failed
    EMMC_CARD_ABSENT,   // SD Card not present
    EMMC_READ_ERROR,
    EMMC_MOUNT_FAIL,
    EMMC_CARD_STATE(u32),
    NONE,
}

// static SDCARD: 
static CUR_SDCARD: NullLock<&'static (dyn blockdevice::BlockDevice<Error = SdResult> + Sync)> = NullLock::new(&NULL_SD_CARD);

pub fn register_sdcard(new_card: &'static (dyn blockdevice::BlockDevice<Error = SdResult> + Sync)){
    CUR_SDCARD.lock(|sdc| *sdc = new_card);
}

pub fn sdcard() -> &'static dyn blockdevice::BlockDevice<Error = SdResult> {
    CUR_SDCARD.lock(|sdc| *sdc)
}
