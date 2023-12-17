#![feature(asm_const)]
#![feature(const_option)]
#![feature(nonzero_min_max)]
#![feature(format_args_nl)]
#![feature(slice_as_chunks)]
#![feature(panic_info_message)]
#![feature(unchecked_math)]
#![no_main]
#![no_std]


use core::arch::asm;

use api::{kernel_call, KernelFunction};
use embedded_sdmmc::{VolumeManager, TimeSource, VolumeIdx, Volume, Timestamp};
use exception::set_kernel_gate;
use sdcard::SdResult;
use time::time_manager;

use crate::{bsp::driver::{SDIO, new_sdcard}, api::get_kernel_gate};


mod bsp;
mod console;
mod sdcard;
mod cpu;
mod driver;
mod panic_wait;
mod print;
mod synchronization;
mod time;
mod api;
mod exception;


unsafe fn kernel_init() -> !{

    exception::handling_init();

    let gate_addr = &(kernel_call as fn(KernelFunction)) as *const fn(KernelFunction);
    set_kernel_gate(unsafe{gate_addr as u64});
    
    
    if let Err(x) = bsp::driver::init() {
        panic!("Error initializing BSP driver subsystem: {}", x)
    }

    driver::driver_manager().init_drivers();

    kernel_main()
}

fn kernel_main() -> !{
    use console::console;

    println!("
    _            _  ___  ____  
   | |_ __ _  __| |/ _ \\/ ___| 
   | __/ _` |/ _` | | | \\___ \\ 
   | || (_| | (_| | |_| |___) |
    \\__\\__,_|\\__,_|\\___/|____/ ");

    println!("{:^37}", bsp::board_name());
    
    use core::time::Duration;

    info!(
        "{} version {}",
        env!("CARGO_PKG_NAME"),
        env!("CARGO_PKG_VERSION")
    );

    info!("Booting on: {}", bsp::board_name());

    let (_, privilege_level) = exception::current_privilege_level();
    info!("Current privilege level: {}", privilege_level);

    info!("Exception handling state:");
    exception::asynchronous::print_state();

    info!(
        "Architectural timer resolution: {} ns",
        time::time_manager().resolution().as_nanos()
    );

    info!("Drivers loaded:");
    driver::driver_manager().enumerate();

    info!("Timer test, spinning for 1 second");
    time::time_manager().spin_for(Duration::from_secs(1));

    // set interrupt vector table

    // test_sdcard();

    let gate = unsafe { get_kernel_gate() };
    gate(KernelFunction::ReadBlock);

    console().clear_rx();
    loop {
        let c = console().read_char();
        console().write_char(c);
    }



}

fn test_sdcard(){
    
    let timesource = TestClock{};

    let mut volume_controller = VolumeManager::new(&SDIO, timesource);
    info!("Getting volume");
    let volume0 = volume_controller.open_volume(VolumeIdx(0)).unwrap();

    info!("Getting dir");
    let dir = volume_controller.open_root_dir(volume0).unwrap();
    info!("Iterating..");
    let _ = volume_controller.iterate_dir(dir, |entry| {
        println!(">> {}",entry.name);
    });

    let file = volume_controller.open_file_in_dir(dir, "CONFIG.TXT", embedded_sdmmc::Mode::ReadOnly).unwrap();
    while !volume_controller.file_eof(file).unwrap() {
        let mut buffer = [0u8; 32];
        let num_read = volume_controller.read(file, &mut buffer).unwrap();
        for b in &buffer[0..num_read] {
            print!("{}", *b as char);
        }
    }

    let _ = volume_controller.close_file(file);
    let _ = volume_controller.close_dir(dir);


}

struct TestClock{}

impl TimeSource for TestClock{
    fn get_timestamp(&self) -> embedded_sdmmc::Timestamp {
        Timestamp { year_since_1970: 0, zero_indexed_month: 0, zero_indexed_day: 0, hours: 0, minutes: 0, seconds: 0 }
    }
}
