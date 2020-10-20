#![no_std]
#![no_main]

use panic_halt as _;

use embedded_hal::digital::v2::OutputPin;
use rtic::app;
use stm32f1xx_hal::prelude::*;

#[app(device=stm32f1xx_hal::device, peripherals=true)]
const APP: () = {
    struct Resources {
        led: stm32f1xx_hal::gpio::gpioa::PA5<
            stm32f1xx_hal::gpio::Output<stm32f1xx_hal::gpio::PushPull>,
        >,
    }
    #[init]
    fn init(cx: init::Context) -> init::LateResources {
        let device = cx.device;

        let mut flash = device.FLASH.constrain();
        let mut rcc = device.RCC.constrain();

        let _clocks = rcc.cfgr.freeze(&mut flash.acr);

        let mut gpioa = device.GPIOA.split(&mut rcc.apb2);

        let led = gpioa.pa5.into_push_pull_output(&mut gpioa.crl);

        init::LateResources { led }
    }

    #[idle(resources=[led])]
    fn idle(cx: idle::Context) -> ! {
        loop {
            cortex_m::asm::delay(320_000);
            cx.resources.led.set_high().unwrap();
            cortex_m::asm::delay(320_000);
            cx.resources.led.set_low().unwrap();
        }
    }
};
