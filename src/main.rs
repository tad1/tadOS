#![no_main]
#![no_std]

use core::{panic::PanicInfo, arch::asm};
use aarch64_cpu::{asm, registers};


#[no_mangle]
pub unsafe extern "C" fn Init() -> ! {
    asm!(include_str!("single_core_init.s"));
    Loop();
}

#[no_mangle]
pub unsafe extern "C" fn Loop() -> ! {
    let mut _x = 42;
    
    asm!("wfi"); // halts CPU
    loop {}
}

#[panic_handler]
fn panic(_panic: &PanicInfo<'_>) -> ! {
    loop {}
}
