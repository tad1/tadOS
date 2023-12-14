
pub(super) mod map{
    pub const GPIO_OFFSET:         usize = 0x0020_0000;
    pub const UART_OFFSET:         usize = 0x0020_1000;
    pub const EMMC_OFFSET:         usize = 0x0030_0000;

    #[cfg(feature = "bsp_rpi3")]
    pub mod mmio {
        use super::*;

        pub const START:    usize = 0x3F00_0000;
        pub const GPIO_START: usize = START + GPIO_OFFSET;
        pub const PL011_UART_START: usize = START + UART_OFFSET;
        pub const EMMC_START: usize = START + EMMC_OFFSET;
    }
}
