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

unsafe fn kernel_init() -> !{
    println!("Hello from tadOS!");

    panic!("Oh no! It's a panic attack! Anyway..")
}