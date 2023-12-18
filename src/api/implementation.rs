use core::{fmt::{self, Arguments}, time::Duration};

use crate::{info, console::console, bsp::driver::SDIO, time::time_manager};
use embedded_sdmmc::BlockDevice;
use super::api::{KernelFunction, ReadBlockArgs};


pub fn kernel_call(function: KernelFunction, arg:u64, resp:u64){
    match function {
        KernelFunction::ReadBlock => {let aguments : &mut ReadBlockArgs = unsafe { *(arg as *mut &mut ReadBlockArgs) }; let _ = (&SDIO).read(&mut aguments.blocks, aguments.start_block, aguments.reason);},
        KernelFunction::ReadChar => unsafe { *(arg as *mut char) = console().read_char() },
        KernelFunction::WriteChar => console().write_char(arg as u8 as char),
        KernelFunction::WriteFmt => {let _ = console().write_fmt(unsafe { *(arg as *const fmt::Arguments) });},
        KernelFunction::ReadCharNonBlocking => unsafe {*(arg as *mut Option<char>) = console().read_char_nonblocking()},
        KernelFunction::Spin => time_manager().spin_for(Duration::from_millis(arg)),
    }
}