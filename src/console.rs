use crate::bsp;

pub mod interface {
    pub use core::fmt::Write;
}

pub fn console() -> impl core::fmt::Write {
    bsp::console::console()
}