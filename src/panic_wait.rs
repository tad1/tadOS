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

    let timestamp = crate::time::time_manager().uptime();
    let (file, line, column) = match info.location() {
        Some(loc) => (loc.file(), loc.line(), loc.column()),
        _ => ("???", 0, 0)
    };

    println!(
        "[  {:>3}.{:06}] Kernel panic!\n\n\
        Parnic location:\n      File '{}', line {}, column {}\n\n\
        {}",timestamp.as_secs(), timestamp.subsec_micros(), file, line, column, info.message().unwrap_or(&format_args!("")));


    cpu::wait_forever()
}