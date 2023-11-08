#![no_main]
#![no_std]

use core::panic::PanicInfo;


#[no_mangle]
pub unsafe extern "C" fn Loop() -> ! {
    let _x = 42;

    loop{}
}

#[panic_handler]
fn panic(_panic: &PanicInfo<'_>) -> ! {
    loop {}
}
