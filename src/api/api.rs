use core::{arch::asm, ptr, fmt};

use crate::info;

#[repr(C)]
#[derive(Debug, PartialEq)]
pub enum KernelFunction {
    ReadBlock
}


pub fn kernel_call(function: KernelFunction){
    info!("Calling kernel function!");
    match function {
        KernelFunction::ReadBlock => {
            info!("trying to read block!")
        },
    }
}

// return to kernel_call function
pub unsafe extern "C" fn get_kernel_gate() -> extern fn(KernelFunction){
    let mut addr: *const fn(KernelFunction) = core::ptr::null() as *const fn(KernelFunction);
    asm!(
        "mov x0, {addr}",
        "svc 0x666",
        "mov {addr}, x0",
        addr = inout(reg) addr);
    
    info!("Address is: {:x}", addr as u64);

    core::mem::transmute(*addr)
}