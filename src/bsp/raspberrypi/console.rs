

use crate::{console, synchronization::{NullLock, interface::Mutex}};
use core::fmt;


struct QEMUConsoleInner {
    chars_written: usize,
}

struct QEMUConsole {
    inner: NullLock<QEMUConsoleInner>
}

static QEMU_CONSOLE: QEMUConsole = QEMUConsole::new();

impl QEMUConsoleInner{

    pub const fn new() -> Self{
        Self { chars_written: 0 }
    }

    pub fn write_char(&mut self, c: char){
        unsafe {
            core::ptr::write_volatile(0x3F20_1000 as *mut u8, c as u8)
        }
        self.chars_written += 1;
    }
}


impl fmt::Write for QEMUConsoleInner {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for c in s.chars(){
            if c == '\n' {
                self.write_char('\r')
            }

            self.write_char(c);
        }

        Ok(())
    }
}

impl QEMUConsole{
    pub const fn new() -> Self{
        QEMUConsole { 
            inner: NullLock::new(QEMUConsoleInner::new()) 
        }
    }

}

pub fn console() -> &'static dyn console::interface::All {
    &QEMU_CONSOLE
}


impl console::interface::Write for QEMUConsole{
    fn write_fmt(&self, args: fmt::Arguments) -> fmt::Result {
        self.inner.lock(|inner| fmt::Write::write_fmt(inner, args))
    }
}

impl console::interface::Statistics for QEMUConsole {
    fn chars_written(&self) -> usize {
        self.inner.lock(|inner| inner.chars_written)
    }
}

impl  console::interface::All for QEMUConsole {}