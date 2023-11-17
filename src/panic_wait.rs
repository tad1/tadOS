use crate::{cpu, println};
use core::panic::PanicInfo;


fn preventreenter(){
    use core::sync::atomic::{AtomicBool, Ordering};
    static HANDLED: AtomicBool = AtomicBool::new(false);

    if !HANDLED.load(Ordering::Relaxed) {
        HANDLED.store(true, Ordering::Relaxed);
        return;
    }

    cpu::wait_forever();
}

#[panic_handler]
fn panic(info: &PanicInfo) -> !{
    preventreenter();

    let (file, line, column) = match info.location() {
        Some(loc) => (loc.file(), loc.line(), loc.column()),
        _ => ("???", 0, 0)
    };

    println!(
        "Kernel panic!\n\n\
        Parnic location:\n      File '{}', line {}, column {}\n\n\
        {}", file, line, column, info.message().unwrap_or(&format_args!("")));


    cpu::wait_forever()
}