use embedded_sdmmc::{TimeSource, Timestamp};

use crate::{synchronization::NullLock, bsp::{driver::SDIO, EMMCController}};
// use crate::bsp::driver::

pub struct TestClock{}

impl TimeSource for TestClock{
    fn get_timestamp(&self) -> embedded_sdmmc::Timestamp {
        Timestamp { year_since_1970: 0, zero_indexed_month: 0, zero_indexed_day: 0, hours: 0, minutes: 0, seconds: 0 }
    }
}
