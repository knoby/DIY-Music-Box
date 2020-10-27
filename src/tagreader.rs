use stm32f1xx_hal::gpio::{Alternate, Floating, Input, Output, PushPull};
use stm32f1xx_hal::spi::*;

pub type PinCS = stm32f1xx_hal::gpio::gpioa::PA4<Output<PushPull>>;
pub type PinClock = stm32f1xx_hal::gpio::gpiob::PB13<Alternate<PushPull>>;
pub type PinMiso = stm32f1xx_hal::gpio::gpiob::PB14<Input<Floating>>;
pub type PinMosi = stm32f1xx_hal::gpio::gpiob::PB15<Alternate<PushPull>>;

pub type SpiDevice =
    Spi<stm32f1xx_hal::device::SPI2, Spi2NoRemap, (PinClock, PinMiso, PinMosi), u8>;

use rtt_target::rprintln;

pub struct TagReader {
    device: mfrc522::Mfrc522<SpiDevice, PinCS>,
    last_tag: Option<mfrc522::Uid>,
}

#[allow(clippy::too_many_arguments)]
impl TagReader {
    pub fn new(
        cs: PinCS,
        clock: PinClock,
        mosi: PinMosi,
        miso: PinMiso,
        spi_hw: stm32f1xx_hal::device::SPI2,
        clocks: stm32f1xx_hal::rcc::Clocks,
        apb: &mut stm32f1xx_hal::rcc::APB1,
    ) -> Self {
        use stm32f1xx_hal::time::U32Ext;

        let spi_mode = mfrc522::MODE;

        let spi = stm32f1xx_hal::spi::Spi::spi2(
            spi_hw,
            (clock, miso, mosi),
            spi_mode,
            1.khz(),
            clocks,
            apb,
        );

        let mut device = mfrc522::Mfrc522::new(spi, cs).unwrap();

        match device.version().unwrap() {
            0x91 => rprintln!("Detected MFRC522 Version 1.0"),
            0x92 => rprintln!("Detected MFRC522 Version 2.0"),
            _ => rprintln!("Detected unknown MFRC522 Version"),
        }

        Self {
            device,
            last_tag: None,
        }
    }

    /// Check if a tag is in the field
    /// Returns Option<UID> if one is in the field and None if none is in the field
    pub fn check_for_new_tag(&mut self) -> Option<mfrc522::Uid> {
        // Safe current state of the tag
        let last_tag = self.last_tag.take();

        // Try to select a TAG and send it to HALT state
        let uid = self
            .device
            .wupa()
            .and_then(|atqa| self.device.select(&atqa))
            .and_then(|uid| self.device.hlta().map(|_| uid))
            .ok();

        // Safe for next time
        self.last_tag = uid;

        // Check if tag is new
        if uid != last_tag && uid.is_some() {
            uid
        } else {
            None
        }
    }
}
