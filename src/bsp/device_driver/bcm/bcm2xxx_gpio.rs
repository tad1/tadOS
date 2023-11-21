use crate::{
    bsp::device_driver::{common::MMIODerefWrapper, bcm::bcm2xxx_gpio::GPPUDCLK0::{PUDCLK15, PUDCLK14}}, driver, synchronization,
    synchronization::{NullLock, interface::Mutex}, cpu,
};
use tock_registers::{
    interfaces::{ReadWriteable, Writeable},
    register_bitfields, register_structs,
    registers::ReadWrite,
};


register_bitfields!{
    u32,
    GPFSEL1[
        FSEL15 OFFSET(15) NUMBITS(3) [
            Input = 0b000,
            Output = 0b001,
            AltFunc0 = 0b100
        ],
        FSEL14 OFFSET(12) NUMBITS(3) [
            Input = 0b000,
            Output = 0b001,
            AltFunc0 = 0b100
        ]
    ],

    GPPUD[
        PUD OFFSET(0) NUMBITS(2) [
            Off = 0b00,
            PullDown = 0b01,
            PullUp = 0b10
        ]
    ],

    GPPUDCLK0[
        PUDCLK15 OFFSET(15) NUMBITS(1)[
            NoEffect = 0b0,
            AssertClock = 0b1
        ],
        PUDCLK14 OFFSET(14) NUMBITS(1)[
            NoEffect = 0b0,
            AssertClock = 0b1
        ]
    ],
    GPIO_PUP_PDN_CNTRL_REG0 [
        /// Pin 15
        GPIO_PUP_PDN_CNTRL15 OFFSET(30) NUMBITS(2) [
            NoResistor = 0b00,
            PullUp = 0b01
        ],

        /// Pin 14
        GPIO_PUP_PDN_CNTRL14 OFFSET(28) NUMBITS(2) [
            NoResistor = 0b00,
            PullUp = 0b01
        ]
    ]
}

register_structs! {
    #[allow(non_snake_case)]
    RegisterBlock {
        (0x00 => _reserved1),
        (0x04 => GPFSEL1: ReadWrite<u32, GPFSEL1::Register>),
        (0x08 => _reserved2),
        (0x94 => GPPUD: ReadWrite<u32, GPPUD::Register>),
        (0x98 => GPPUDCLK0: ReadWrite<u32, GPPUDCLK0::Register>),
        (0x9C => _reserved3),
        (0xE4 => GPIO_PUP_PDN_CNTRL_REG0: ReadWrite<u32, GPIO_PUP_PDN_CNTRL_REG0::Register>),
        (0xE8 => @END),
    }
}

type Registers = MMIODerefWrapper<RegisterBlock>;

struct GPIOInner{
    registers: Registers
}

pub struct GPIO{
    inner: NullLock<GPIOInner>
}

impl GPIOInner{
    pub const unsafe fn new(mmio_start_addr: usize) -> GPIOInner{
        Self { registers: Registers::new(mmio_start_addr) }
    }

    fn disable_pub_14_15_bcm2837(&mut self){
        const DELAY: usize = 2000;

        self.registers.GPPUD.write(GPPUD::PUD::Off);
        cpu::spin_for_cycles(DELAY);

        self.registers.GPPUDCLK0.write(PUDCLK15::AssertClock + PUDCLK14::AssertClock);
        cpu::spin_for_cycles(DELAY);

        self.registers.GPPUD.write(GPPUD::PUD::Off);
        self.registers.GPPUDCLK0.set(0);
    }

    pub fn map_pl011_uart(&mut self){
        self.registers.GPFSEL1.modify(GPFSEL1::FSEL14::AltFunc0 + GPFSEL1::FSEL15::AltFunc0);

        #[cfg(feature = "bsp_rpi3")]
        self.disable_pub_14_15_bcm2837();
    }

}

impl GPIO{
    const COMPATIBLE: &'static str = "BCM GPIO";

    pub const unsafe fn new(mmio_start_addr: usize) -> GPIO{
        Self { inner: NullLock::new(GPIOInner::new(mmio_start_addr)) }
    }

    pub fn map_pl011_uart(&self){
        self.inner.lock(|inner| inner.map_pl011_uart())
    }
}

impl driver::interface::DeviceDriver for GPIO{
    fn compatible(&self) -> &'static str {
        Self::COMPATIBLE
    }
}