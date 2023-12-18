use core::arch::asm;

use embedded_sdmmc::{Block, BlockIdx};

use crate::bsp::driver::SDIO;


#[repr(C)]
#[derive(Debug, PartialEq)]

// note arguments should be passed as u64
pub enum KernelFunction {
    ReadBlock,
    ReadChar,
    ReadCharNonBlocking,
    WriteChar,
    WriteFmt,
    Spin,
}

pub struct ReadBlockArgs<'a>{
    pub blocks: &'a mut[Block],
    pub start_block: BlockIdx,
    pub reason: &'a str
}

pub type KernelCall = extern fn(KernelFunction, u64, u64);

// return to kernel_call function
pub unsafe fn get_kernel_gate() -> KernelCall {
    let mut addr: *const fn(KernelFunction) = core::ptr::null() as *const fn(KernelFunction);
    asm!(
        "mov x0, {addr}",
        "svc 0x666",
        "mov {addr}, x0",
        addr = inout(reg) addr);
    
    core::mem::transmute(*addr)
}