use stm32f1xx_hal::gpio::{Edge, ExtiPin, Input, PullUp};

use embedded_hal::digital::v2::InputPin;

pub type PinBtnUp = stm32f1xx_hal::gpio::gpiob::PB0<Input<PullUp>>;
pub type PinBtnDown = stm32f1xx_hal::gpio::gpiob::PB1<Input<PullUp>>;
pub type PinBtnPlayPause = stm32f1xx_hal::gpio::gpiob::PB2<Input<PullUp>>;

pub struct Button<T>
where
    T: InputPin + ExtiPin,
{
    enabled: bool,
    button: T,
}

impl<T: InputPin<Error = core::convert::Infallible> + ExtiPin> Button<T> {
    pub fn new(button: T) -> Self {
        Self {
            enabled: true,
            button,
        }
    }

    pub fn enable(&mut self) {
        self.enabled = true;
    }

    pub fn disable(&mut self) {
        self.enabled = false;
    }

    pub fn is_low(&mut self) -> bool {
        self.button.is_low().unwrap()
    }

    pub fn is_high(&mut self) -> bool {
        self.button.is_high().unwrap()
    }

    pub fn clear_interrupt_pending_bit(&mut self) {
        self.button.clear_interrupt_pending_bit();
    }

    pub fn is_enabled(&self) -> bool {
        self.enabled
    }
}

pub fn config_interrupts(
    btn_up: &mut PinBtnUp,
    btn_down: &mut PinBtnDown,
    btn_playpause: &mut PinBtnPlayPause,
    exti: &stm32f1xx_hal::device::EXTI,
    afio: &mut stm32f1xx_hal::afio::Parts,
) {
    // The Buttons are wired to ground --> press is a falling edge on the input pin
    btn_up.make_interrupt_source(afio);
    btn_up.trigger_on_edge(exti, Edge::FALLING);
    btn_up.enable_interrupt(exti);

    btn_down.make_interrupt_source(afio);
    btn_down.trigger_on_edge(exti, Edge::FALLING);
    btn_down.enable_interrupt(exti);

    btn_playpause.make_interrupt_source(afio);
    btn_playpause.trigger_on_edge(exti, Edge::FALLING);
    btn_playpause.enable_interrupt(exti);
}
