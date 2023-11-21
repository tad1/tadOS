use crate::{synchronization::{NullLock, interface::Mutex}, println};


pub mod interface {
    pub trait DeviceDriver{
        fn compatible(&self) -> &'static str;

        unsafe fn init(&self) -> Result<(), &'static str>{
            Ok(())
        }
    }
}

pub type DeviceDriverPostInitCallback = unsafe fn() -> Result<(), &'static str>;
const NUM_DRIVERS: usize = 5;
static DRIVER_MANAGER: DriverManager = DriverManager::new();

#[derive(Copy, Clone)]
pub struct  DeviceDriverDescriptor{
    device_driver: &'static (dyn interface::DeviceDriver + Sync),
    post_init_callback: Option<DeviceDriverPostInitCallback>,
}


pub struct DriverManager{
    inner: NullLock<DriverManagerInner>,
}


struct DriverManagerInner{
    next_index: usize,
    descriptors: [Option<DeviceDriverDescriptor>; NUM_DRIVERS] 
}

impl DriverManagerInner{
    pub const fn new() -> Self{
        Self { next_index: 0, descriptors: [None; NUM_DRIVERS] }
    }
}

// Public Code

impl DeviceDriverDescriptor {
    pub fn new(
        device_driver: &'static (dyn interface::DeviceDriver + Sync),
        post_init_callback: Option<DeviceDriverPostInitCallback>
    ) -> Self {
            Self { device_driver: device_driver, post_init_callback: post_init_callback }
    }
}

pub fn driver_manager() -> &'static DriverManager{
    &DRIVER_MANAGER
}

impl DriverManager{
    pub const fn new() -> Self{
        Self { inner: NullLock::new(DriverManagerInner::new()) }
    }

    pub fn register_driver(&self, descriptor: DeviceDriverDescriptor){
        self.inner.lock(|inner| {
            inner.descriptors[inner.next_index] = Some(descriptor);
            inner.next_index += 1;
        })
    }

    fn for_each_descriptor<'a>(&'a self, f: impl FnMut(&'a DeviceDriverDescriptor)){
        self.inner.lock(|inner| 
            inner
                .descriptors
                .iter()
                .filter_map(|x| x.as_ref())
                .for_each(f)
            )
    }

    pub unsafe fn init_drivers(&self){
        self.for_each_descriptor(|descriptor| {
            if let Err(x) = descriptor.device_driver.init() {
                panic!(
                    "Error initializing driver: {}: {}",
                    descriptor.device_driver.compatible(),
                    x
                );
            }

            if let Some(callback) = descriptor.post_init_callback{
                if let Err(err) = callback() {
                    panic!(
                        "Error during driver post-init callback: {}: {}",
                        descriptor.device_driver.compatible(),
                        err
                    );
                }
            }

        })
    }

    pub fn enumerate(&self) {
        let mut i: usize = 1;
        self.for_each_descriptor(|descriptor| {
            println!("      {}. {}", i, descriptor.device_driver.compatible());

            i += 1;
        });
    }
}