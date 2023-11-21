#![feature(asm_const)]
#![feature(format_args_nl)]
#![feature(panic_info_message)]
#![no_main]
#![no_std]


mod bsp;
mod console;
mod cpu;
mod driver;
mod panic_wait;
mod print;
mod synchronization;

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
    println!("[0] {} version {}",
    env!("CARGO_PKG_NAME"),
    env!("CARGO_PKG_VERSION"));

    println!("[1] Booting on: {}", bsp::board_name());

    println!("[2] Drivers loaded:");
    driver::driver_manager().enumerate();

    println!("[3] Chars written: {}", console().chars_written());
    println!("[4] Echoing input now");

    // Discard any spurious received characters before going into echo mode.
    console().clear_rx();
    loop {
        let c = console().read_char();
        console().write_char(c);
    }

}