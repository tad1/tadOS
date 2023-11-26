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

    println!("{:^37}", bsp::board_name());
    println!();
    println!("[ML] Requesting binary");
    console().flush();

    console().clear_rx();

    // we are sending a secret signal to notify: "Hey! I'm ready, gimme the binary now"
    for _ in 0..3{
        console().write_char(3 as char);
    }

    // oh, we can send only 8bit? let's fucking compose that!
    let mut size: u32 = u32::from(console().read_char() as u8);
    size |= u32::from(console().read_char() as u8) << 8;
    size |= u32::from(console().read_char() as u8) << 16;
    size |= u32::from(console().read_char() as u8) << 24;

    console().write_char('O');
    console().write_char('K');

    let kernel_addr: *mut u8 = bsp::memory::board_default_load_addr() as *mut u8;
    unsafe {
        for i in 0..size{
            core::ptr::write_volatile(kernel_addr.offset(i as isize), console().read_char() as u8)
        }
    }

    println!("[ML] Loaded! Executing the payload now\n");

    // Use black magic to create a function pointer
    let kernel: fn() -> ! = unsafe {
        core::mem::transmute(kernel_addr)
    };

    kernel()

}