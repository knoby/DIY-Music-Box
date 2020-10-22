use stm32f1xx_hal::gpio::{Alternate, Floating, Input, PushPull};

pub type PinSerialTx = stm32f1xx_hal::gpio::gpioa::PA9<Alternate<PushPull>>;
pub type PinSerialRx = stm32f1xx_hal::gpio::gpioa::PA10<Input<Floating>>;

pub type SerialDevice =
    stm32f1xx_hal::serial::Serial<stm32f1xx_hal::device::USART1, (PinSerialTx, PinSerialRx)>;

pub struct DFPlayer {
    serial: SerialDevice,
}

impl DFPlayer {
    pub fn new(
        serial_hw: stm32f1xx_hal::device::USART1,
        tx: PinSerialTx,
        rx: PinSerialRx,
        mapr: &mut stm32f1xx_hal::afio::MAPR,
        clocks: stm32f1xx_hal::rcc::Clocks,
        apb: &mut stm32f1xx_hal::rcc::APB2,
    ) -> Self {
        use stm32f1xx_hal::time::U32Ext;

        let config = stm32f1xx_hal::serial::Config::default().baudrate(9600_u32.bps());

        let serial =
            stm32f1xx_hal::serial::Serial::usart1(serial_hw, (tx, rx), mapr, config, clocks, apb);

        Self { serial }
    }
}
