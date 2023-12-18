mod null_card;

use core::fmt::{Error, Display};

use crate::synchronization::{self, NullLock, interface::Mutex};
use embedded_sdmmc::{self, FilenameError};

use self::null_card::NULL_SD_CARD;



pub mod interface {

    use embedded_sdmmc;

    use super::SdResult;
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

impl Display for SdResult{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "SdCard Driver Error: ");
        match self {
            SdResult::EMMC_OK => write!(f, "EMMC_OK"),
            SdResult::EMMC_ERROR => write!(f, "EMMC_ERROR"),
            SdResult::EMMC_TIMEOUT => write!(f, "EMMC_TIMEOUT"),
            SdResult::EMMC_BUSY => write!(f, "EMMC_BUSY"),
            SdResult::EMMC_NO_RESP => write!(f, "EMMC_NO_RESP"),
            SdResult::EMMC_ERROR_RESET => write!(f, "EMMC_ERROR_RESET"),
            SdResult::EMMC_ERROR_CLOCK => write!(f, "EMMC_ERROR_CLOCK"),
            SdResult::EMMC_ERROR_VOLTAGE => write!(f, "EMMC_ERROR_VOLTAGE"),
            SdResult::EMMC_ERROR_APP_CMD => write!(f, "EMMC_ERROR_APP_CMD"),
            SdResult::EMMC_CARD_ABSENT => write!(f, "EMMC_CARD_ABSENT"),
            SdResult::EMMC_READ_ERROR => write!(f, "EMMC_READ_ERROR"),
            SdResult::EMMC_MOUNT_FAIL => write!(f, "EMMC_MOUNT_FAIL"),
            SdResult::EMMC_CARD_STATE(_) => write!(f, "EMMC_CARD_STATE"),
            SdResult::NONE => write!(f, "NONE"),
        }
    }
}

pub struct SdmmcError(pub embedded_sdmmc::Error<SdResult>);
pub struct SdmmcFilenameError(pub FilenameError);

impl Display for SdmmcError{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f,"embedded_sdmmc crate error:");
        match &self.0 {
            embedded_sdmmc::Error::DeviceError(err) => write!(f,"{}", err),
            embedded_sdmmc::Error::FormatError(s) => write!(f, "FormatError {}", s),
            embedded_sdmmc::Error::NoSuchVolume => write!(f,"NoSuchVolume" ),
            embedded_sdmmc::Error::FilenameError(e) => write!(f,"{}", SdmmcFilenameError(e.clone())),
            embedded_sdmmc::Error::TooManyOpenVolumes => write!(f,"TooManyOpenVolumes" ),
            embedded_sdmmc::Error::TooManyOpenDirs => write!(f,"TooManyOpenDirs" ),
            embedded_sdmmc::Error::TooManyOpenFiles => write!(f,"TooManyOpenFiles" ),
            embedded_sdmmc::Error::BadHandle => write!(f,"BadHandle" ),
            embedded_sdmmc::Error::FileNotFound => write!(f,"FileNotFound" ),
            embedded_sdmmc::Error::FileAlreadyOpen => write!(f,"FileAlreadyOpen" ),
            embedded_sdmmc::Error::DirAlreadyOpen => write!(f,"DirAlreadyOpen" ),
            embedded_sdmmc::Error::OpenedDirAsFile => write!(f,"OpenedDirAsFile" ),
            embedded_sdmmc::Error::OpenedFileAsDir => write!(f,"OpenedFileAsDir" ),
            embedded_sdmmc::Error::DeleteDirAsFile => write!(f,"DeleteDirAsFile" ),
            embedded_sdmmc::Error::VolumeStillInUse => write!(f,"VolumeStillInUse" ),
            embedded_sdmmc::Error::VolumeAlreadyOpen => write!(f,"VolumeAlreadyOpen" ),
            embedded_sdmmc::Error::Unsupported => write!(f,"Unsupported" ),
            embedded_sdmmc::Error::EndOfFile => write!(f,"EndOfFile" ),
            embedded_sdmmc::Error::BadCluster => write!(f,"BadCluster" ),
            embedded_sdmmc::Error::ConversionError => write!(f,"ConversionError" ),
            embedded_sdmmc::Error::NotEnoughSpace => write!(f,"NotEnoughSpace" ),
            embedded_sdmmc::Error::AllocationError => write!(f,"AllocationError" ),
            embedded_sdmmc::Error::UnterminatedFatChain => write!(f,"UnterminatedFatChain" ),
            embedded_sdmmc::Error::ReadOnly => write!(f,"ReadOnly" ),
            embedded_sdmmc::Error::FileAlreadyExists => write!(f,"FileAlreadyExists" ),
            embedded_sdmmc::Error::BadBlockSize(s) => write!(f, "BadBlockSize({})", s),
            embedded_sdmmc::Error::InvalidOffset => write!(f,"InvalidOffset" ),
        }
    }
}

impl Display for SdmmcFilenameError{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "FilenameError: ");
        match self.0 {
            FilenameError::InvalidCharacter => write!(f, "InvalidCharacter"),
            FilenameError::FilenameEmpty => write!(f, "FilenameEmpty"),
            FilenameError::NameTooLong => write!(f, "NameTooLong"),
            FilenameError::MisplacedPeriod => write!(f, "MisplacedPeriod"),
            FilenameError::Utf8Error => write!(f, "Utf8Error"),
        }
    }
}

// static SDCARD: 
static CUR_SDCARD: NullLock<&'static (dyn embedded_sdmmc::BlockDevice<Error = SdResult> + Sync)> = NullLock::new(&NULL_SD_CARD);

pub fn register_sdcard(new_card: &'static (dyn embedded_sdmmc::BlockDevice<Error = SdResult> + Sync)){
    CUR_SDCARD.lock(|sdc| *sdc = new_card);
}

pub fn sdcard() -> &'static dyn embedded_sdmmc::BlockDevice<Error = SdResult> {
    CUR_SDCARD.lock(|sdc| *sdc)
}
