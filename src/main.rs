#![feature(asm_const)]
#![feature(format_args_nl)]
#![feature(panic_info_message)]
#![no_main]
#![no_std]

mod bsp;
mod cpu;
mod panic_wait;
mod console;
mod print;
mod synchronization;

unsafe fn kernel_init() -> !{
    use console::console;

    println!("[0] Hello from tadOS!");
    println!("[1] Chars written {}", console().chars_written());

    panic!("[2] Oh no! It's a panic attack! Anyway..")
}