mod device_driver;

#[cfg(any(feature = "bsp_rpi3", feature = "bsp_rpi4"))]
mod raspberrypi;


#[cfg(any(feature = "bsp_rpi3", feature = "bsp_rpi4"))]
pub use self::raspberrypi::*;

pub use self::device_driver::EMMCController;