#![feature(asm_const)]
#![feature(const_option)]
#![feature(nonzero_min_max)]
#![feature(format_args_nl)]
#![feature(slice_as_chunks)]
#![feature(panic_info_message)]
#![feature(unchecked_math)]
#![no_main]
#![no_std]


use core::{arch::asm, fmt::Display};

use api::{kernel_call, KernelFunction};
use bsp::EMMCController;
use embedded_sdmmc::{VolumeManager, VolumeIdx};
use exception::set_kernel_gate;
use sdcard::SdResult;
use time::time_manager;

use crate::{bsp::driver::{SDIO, new_sdcard}, api::get_kernel_gate, fs::TestClock, elf::load_elf, sdcard::SdmmcError};


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
mod elf;
mod fs;

unsafe fn kernel_init() -> !{

    exception::handling_init();

    let gate_addr = &(kernel_call as fn(KernelFunction, u64, u64)) as *const fn(KernelFunction, u64, u64);
    set_kernel_gate(gate_addr as u64);
    
    
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

    console().clear_rx();

    const MAX_COMMAND_SIZE: usize = 11;
    let mut buffer: [u8; MAX_COMMAND_SIZE] = [b' '; MAX_COMMAND_SIZE];
    let mut i = 0;
    
    let mut volume_controller = VolumeManager::new(&SDIO, TestClock{});
    
    test_sdcard(&mut volume_controller);
    let volume0 = volume_controller.open_volume(VolumeIdx(0)).unwrap();
    let dir = volume_controller.open_root_dir(volume0).unwrap();
    
    print!("[{}kernel{} /] >", "\x1b[32m", "\x1b[0m");
    loop {
        let c = console().read_char();

        if c.is_control() {
            match c as u8 {
                23 => while i > 0 { // Ctrl + Backspace
                    console().write_char(8 as char);
                    console().write_char(' ');
                    console().write_char(8 as char);
                    
                    
                    i = i - 1;
                    if buffer[i] == ' ' as u8{
                        break;
                    }
                }
                127 => if i > 0 { // Backspace
                    console().write_char(8 as char);
                    console().write_char(' ');
                    console().write_char(8 as char);
                    i = i - 1;
                }
                9 => { // Tab
                    let typed_name: &str = unsafe {core::mem::transmute(&buffer[..i] as &[u8])};
                    println!("");
                    let _ = volume_controller.iterate_dir(dir, |entry| if entry.name.base_name().starts_with(typed_name.as_bytes()) {
                            println!("{}", entry.name);
                        }
                    );
                    print!("[{}kernel{} /] >{}", "\x1b[32m", "\x1b[0m", typed_name);



                }
                13 => { // Enter
                    console().write_char('\n');
                    // 1. check if file exists
                    let name: &str = unsafe {core::mem::transmute(&buffer[..i] as &[u8])};
                    let res = volume_controller.open_file_in_dir(dir, name, embedded_sdmmc::Mode::ReadOnly);
                    

                    match res {
                        Ok(file) => {
                            let res = unsafe { load_elf(&mut volume_controller, file) };
                            let _ = volume_controller.close_file(file);

                            if let Some(function) = res {
                                function();
                            }

                        },
                        Err(err) => {
                            info!("{}", SdmmcError(err));
                            i = 0;
                            print!("[{}kernel{} /] >", "\x1b[32m", "\x1b[0m");
                            continue;
                        },
                    }
                    let _ = volume_controller.close_dir(dir);
                    
                    i = 0;
                    print!("[{}kernel{} /] >", "\x1b[32m", "\x1b[0m");
                }
                _ => {}            
            }
        } else {
            if i < MAX_COMMAND_SIZE{
                console().write_char(c);
                buffer[i] = c as u8;
                i = i + 1;
            }
        }
        // if(c as u8 == 127){
        // }
        // console().write_char(c);
    }

    let _ = volume_controller.close_dir(dir);
    let _ = volume_controller.close_volume(volume0);



}

fn test_sdcard(volume_controller:&mut VolumeManager<&EMMCController, TestClock>){
    

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
    let _ = volume_controller.close_volume(volume0);


}

