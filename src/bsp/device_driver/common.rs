use core::{marker::PhantomData, ops};


pub struct MMIODerefWrapper<T> {
    address: usize,
    phantom_data: PhantomData<fn() -> T>
}

impl<T> MMIODerefWrapper<T>{
    pub const unsafe fn new(address: usize) -> MMIODerefWrapper<T>{
        Self { address: address, phantom_data: PhantomData }
    }
}

impl<T> ops::Deref for MMIODerefWrapper<T>{
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe {&*(self.address as *const _)}
    }
}