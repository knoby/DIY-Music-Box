use stm32f1xx_hal::gpio::{Alternate, Floating, Input, Output, PushPull};
use stm32f1xx_hal::spi::*;

pub type PinCS = stm32f1xx_hal::gpio::gpioa::PA4<Output<PushPull>>;
pub type PinClock = stm32f1xx_hal::gpio::gpioa::PA5<Alternate<PushPull>>;
pub type PinMiso = stm32f1xx_hal::gpio::gpioa::PA6<Input<Floating>>;
pub type PinMosi = stm32f1xx_hal::gpio::gpioa::PA7<Alternate<PushPull>>;

pub type SpiDevice =
    Spi<stm32f1xx_hal::device::SPI1, Spi1NoRemap, (PinClock, PinMiso, PinMosi), u8>;

pub struct TagReader {
    spi: SpiDevice,
    cs: PinCS,
}

#[allow(clippy::too_many_arguments)]
impl TagReader {
    pub fn new(
        cs: PinCS,
        clock: PinClock,
        mosi: PinMosi,
        miso: PinMiso,
        spi_hw: stm32f1xx_hal::device::SPI1,
        clocks: stm32f1xx_hal::rcc::Clocks,
        apb: &mut stm32f1xx_hal::rcc::APB2,
        mapr: &mut stm32f1xx_hal::afio::MAPR,
    ) -> Self {
        use stm32f1xx_hal::spi::*;
        use stm32f1xx_hal::time::U32Ext;

        let spi_mode = Mode {
            polarity: Polarity::IdleLow,
            phase: Phase::CaptureOnFirstTransition,
        };

        let spi = stm32f1xx_hal::spi::Spi::spi1(
            spi_hw,
            (clock, miso, mosi),
            mapr,
            spi_mode,
            100_u32.khz(),
            clocks,
            apb,
        );

        Self { spi, cs }
    }
}
