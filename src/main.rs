#![no_std]
#![no_main]

use panic_halt as _;

use rtt_target::{rprintln, rtt_init_print};

use rtic::app;
use stm32f1xx_hal::prelude::*;

const LED_PERIOD: u32 = 8_000_000;

#[app(device=stm32f1xx_hal::device, monotonic=rtic::cyccnt::CYCCNT, peripherals=true)]
const APP: () = {
    struct Resources {
        /// Onboard LED on PA5 pin
        led: stm32f1xx_hal::gpio::gpioa::PA5<
            stm32f1xx_hal::gpio::Output<stm32f1xx_hal::gpio::PushPull>,
        >,
    }

    #[init(schedule=[set_led])]
    fn init(mut cx: init::Context) -> init::LateResources {
        // Init the RTT Channel for logging
        rtt_init_print!();

        rprintln!("Rustiebox Init ");

        // Get device peripherals
        let dp = cx.device;

        // Enable the monotonic timer
        cx.core.DWT.enable_cycle_counter();

        // Get flash and rcc registers
        let mut flash = dp.FLASH.constrain();
        let mut rcc = dp.RCC.constrain();

        // Freez clocks
        let _clocks = rcc.cfgr.freeze(&mut flash.acr);

        // Get the gpios
        let mut gpioa = dp.GPIOA.split(&mut rcc.apb2);
        let mut _gpiob = dp.GPIOB.split(&mut rcc.apb2);
        let mut _gpioc = dp.GPIOC.split(&mut rcc.apb2);
        let mut _gpiod = dp.GPIOD.split(&mut rcc.apb2);
        let mut _gpioe = dp.GPIOE.split(&mut rcc.apb2);

        // Config Onboard LED
        let led = gpioa.pa5.into_push_pull_output(&mut gpioa.crl);

        // Schedule LED Task
        cx.schedule.set_led(cx.start, true).unwrap();

        // Return late resources
        init::LateResources { led }
    }

    #[idle(resources=[led])]
    fn idle(_cx: idle::Context) -> ! {
        loop {
            cortex_m::asm::nop();
        }
    }

    #[task(resources=[led], schedule=[set_led])]
    fn set_led(cx: set_led::Context, state: bool) {
        use embedded_hal::digital::v2::OutputPin;
        use rtic::cyccnt::U32Ext;

        // set LED State
        if state {
            cx.resources.led.set_high().unwrap();
        } else {
            cx.resources.led.set_low().unwrap();
        }
        // Spawn task
        cx.schedule
            .set_led(cx.scheduled + LED_PERIOD.cycles(), !state)
            .unwrap();
    }

    extern "C" {
        fn CAN_RX1();
    }
};
