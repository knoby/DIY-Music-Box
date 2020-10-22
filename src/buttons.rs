use stm32f1xx_hal::gpio::{Input, PullUp};

pub type BtnPlayPause = stm32f1xx_hal::gpio::gpiob::PB12<Input<PullUp>>;
pub type BtnUp = stm32f1xx_hal::gpio::gpiob::PB13<Input<PullUp>>;
pub type BtnDown = stm32f1xx_hal::gpio::gpiob::PB14<Input<PullUp>>;
