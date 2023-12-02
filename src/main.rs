#![feature(asm_const)]
#![feature(const_option)]
#![feature(nonzero_min_max)]
#![feature(format_args_nl)]
#![feature(panic_info_message)]
#![feature(unchecked_math)]
#![no_main]
#![no_std]


mod bsp;
mod console;
mod cpu;
mod driver;
mod panic_wait;
mod print;
mod synchronization;
mod time;

unsafe fn kernel_init() -> !{

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

    info!(
        "Architectural timer resolution: {} ns",
        time::time_manager().resolution().as_nanos()
    );

    info!("Drivers loaded:");
    driver::driver_manager().enumerate();
    time::time_manager().spin_for(Duration::from_nanos(1));

    loop {
        info!("Spinning for 1 second");
        time::time_manager().spin_for(Duration::from_secs(1));
    }

}