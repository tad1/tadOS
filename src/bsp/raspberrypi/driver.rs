// SPDX-License-Identifier: MIT OR Apache-2.0
//
// Copyright (c) 2018-2023 Andre Richter <andre.o.richter@gmail.com>

//! BSP driver support.

use super::memory::map::mmio;
use crate::{bsp::device_driver, console, driver as generic_driver, sdcard, synchronization::interface::Mutex, info};
use core::sync::atomic::{AtomicBool, Ordering};

//--------------------------------------------------------------------------------------------------
// Global instances
//--------------------------------------------------------------------------------------------------

static PL011_UART: device_driver::PL011Uart =
    unsafe { device_driver::PL011Uart::new(mmio::PL011_UART_START) };
static GPIO: device_driver::GPIO = unsafe { device_driver::GPIO::new(mmio::GPIO_START) };
pub static SDIO: device_driver::EMMCController = unsafe { device_driver::EMMCController::new(mmio::EMMC_START) };
//--------------------------------------------------------------------------------------------------
// Private Code
//--------------------------------------------------------------------------------------------------

/// This must be called only after successful init of the UART driver.
fn post_init_uart() -> Result<(), &'static str> {
    console::register_console(&PL011_UART);

    Ok(())
}

/// This must be called only after successful init of the GPIO driver.
fn post_init_gpio() -> Result<(), &'static str> {
    GPIO.map_pl011_uart();
    Ok(())
}

fn post_init_emmc() -> Result<(), &'static str> {
    use device_driver::EMMCController;
    // sdcard::register_sdcard(&SDIO);
    match unsafe { &SDIO.emmc_init_card() } {
        &crate::sdcard::SdResult::EMMC_OK => {
            info!("EMMC2 driver initialized...\n")
        }
        _ => {
            info!("failed to initialize EMMC2...\n")
        }
    }

    Ok(())
}

fn driver_uart() -> Result<(), &'static str> {
    let uart_descriptor =
        generic_driver::DeviceDriverDescriptor::new(&PL011_UART, Some(post_init_uart));
    generic_driver::driver_manager().register_driver(uart_descriptor);

    Ok(())
}

fn driver_gpio() -> Result<(), &'static str> {
    let gpio_descriptor = generic_driver::DeviceDriverDescriptor::new(&GPIO, Some(post_init_gpio));
    generic_driver::driver_manager().register_driver(gpio_descriptor);

    Ok(())
}

fn driver_emmc() -> Result<(), &'static str> {
    let emmc_descriptor = generic_driver::DeviceDriverDescriptor::new(&SDIO, Some(post_init_emmc));
    generic_driver::driver_manager().register_driver(emmc_descriptor);

    Ok(())
}

//--------------------------------------------------------------------------------------------------
// Public Code
//--------------------------------------------------------------------------------------------------

/// Initialize the driver subsystem.
///
/// # Safety
///
/// See child function calls.
pub unsafe fn init() -> Result<(), &'static str> {
    static INIT_DONE: AtomicBool = AtomicBool::new(false);
    if INIT_DONE.load(Ordering::Relaxed) {
        return Err("Init already done");
    }

    driver_uart()?;
    driver_gpio()?;
    driver_emmc()?;

    INIT_DONE.store(true, Ordering::Relaxed);
    Ok(())
}

pub unsafe fn new_sdcard() -> device_driver::EMMCController {
    device_driver::EMMCController::new(mmio::EMMC_START)
}