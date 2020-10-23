use stm32f1xx_hal::gpio::{Edge, ExtiPin, Input, PullUp};

pub type BtnUp = stm32f1xx_hal::gpio::gpiob::PB0<Input<PullUp>>;
pub type BtnDown = stm32f1xx_hal::gpio::gpiob::PB1<Input<PullUp>>;
pub type BtnPlayPause = stm32f1xx_hal::gpio::gpiob::PB2<Input<PullUp>>;

pub fn config_interrupts(
    btn_up: &mut BtnUp,
    btn_down: &mut BtnDown,
    btn_playpause: &mut BtnPlayPause,
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
