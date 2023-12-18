mod api;
mod implementation;

pub use api::{KernelFunction, get_kernel_gate};
pub use implementation::kernel_call;
