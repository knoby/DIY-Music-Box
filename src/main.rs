#![no_std]
#![no_main]

// The panic behaviour
#[cfg(feature = "panic-stop")]
use panic_halt as _;
#[cfg(feature = "panic-rtt")]
use panic_rtt_target as _;

// The RTT logging feature
use rtt_target::{rprintln, rtt_init_print};

// Some global imports
use rtic::app;
use rtic::cyccnt::U32Ext;
use stm32f1xx_hal::prelude::*;

// Mods that are used in the application
mod app;
mod buttons;
mod player;
mod tagreader;

const CYCLES_10_MS: u32 = 32_000_000 / 100;

/// On board LED type alias
type OnBoardLED =
    stm32f1xx_hal::gpio::gpioc::PC13<stm32f1xx_hal::gpio::Output<stm32f1xx_hal::gpio::PushPull>>;

#[app(device=stm32f1xx_hal::device, monotonic=rtic::cyccnt::CYCCNT, peripherals=true)]
const APP: () = {
    struct Resources {
        /// Onboard LED on PA5 pin
        led: OnBoardLED,
        /// Buttons
        buttons: (
            buttons::Button<buttons::PinBtnUp>,
            buttons::Button<buttons::PinBtnDown>,
            buttons::Button<buttons::PinBtnPlayPause>,
        ),
        /// RFID Tag reader
        tagreader: tagreader::TagReader,
        /// DFPlayer
        player: player::DFPlayer,
        /// Event Reciver
        event_cons: heapless::spsc::Consumer<'static, app::Events, heapless::consts::U8>,
        /// Event Sender
        event_prod: heapless::spsc::Producer<'static, app::Events, heapless::consts::U8>,
    }

    #[init()]
    fn init(mut cx: init::Context) -> init::LateResources {
        // Create Que
        static mut QUEUE: Option<heapless::spsc::Queue<app::Events, heapless::consts::U8>> = None;
        *QUEUE = Some(heapless::spsc::Queue::new());

        let (event_prod, event_cons) = QUEUE.as_mut().unwrap().split();

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
        let mut afio = dp.AFIO.constrain(&mut rcc.apb2);

        // Freez clockslet clocks = rcc
        rprintln!("Setting up Clocks");
        let clocks = rcc
            .cfgr
            .sysclk(32.mhz())
            .pclk1(16.mhz())
            .pclk2(32.mhz())
            .freeze(&mut flash.acr);

        // Get the gpios
        let mut gpioa = dp.GPIOA.split(&mut rcc.apb2);
        let mut gpiob = dp.GPIOB.split(&mut rcc.apb2);
        let mut gpioc = dp.GPIOC.split(&mut rcc.apb2);
        let mut _gpiod = dp.GPIOD.split(&mut rcc.apb2);
        let mut _gpioe = dp.GPIOE.split(&mut rcc.apb2);

        // Config Onboard LED
        rprintln!("Setup Onboard LED");
        let led = gpioc.pc13.into_push_pull_output(&mut gpioc.crh);

        // Config of the buttons
        rprintln!("Setup Inputs for buttons");
        let mut btn_up = gpiob.pb0.into_pull_up_input(&mut gpiob.crl);
        let mut btn_down = gpiob.pb1.into_pull_up_input(&mut gpiob.crl);
        let mut btn_playpause = gpiob.pb2.into_pull_up_input(&mut gpiob.crl);
        buttons::config_interrupts(
            &mut btn_up,
            &mut btn_down,
            &mut btn_playpause,
            &dp.EXTI,
            &mut afio,
        );
        let btn_up = buttons::Button::new(btn_up);
        let btn_down = buttons::Button::new(btn_down);
        let btn_playpause = buttons::Button::new(btn_playpause);

        // Config of the tagreader
        rprintln!("Setup Tagreader");
        let spi_cs = gpioa.pa4.into_push_pull_output(&mut gpioa.crl);
        let spi_clock = gpioa.pa5.into_alternate_push_pull(&mut gpioa.crl);
        let spi_miso = gpioa.pa6.into_floating_input(&mut gpioa.crl);
        let spi_mosi = gpioa.pa7.into_alternate_push_pull(&mut gpioa.crl);

        let tagreader = tagreader::TagReader::new(
            spi_cs,
            spi_clock,
            spi_mosi,
            spi_miso,
            dp.SPI1,
            clocks,
            &mut rcc.apb2,
            &mut afio.mapr,
        );

        // Init the Dfplayer
        rprintln!("Setup DFPlayer");
        let serial_tx = gpioa.pa9.into_alternate_push_pull(&mut gpioa.crh);
        let serial_rx = gpioa.pa10.into_floating_input(&mut gpioa.crh);

        let player = player::DFPlayer::new(
            dp.USART1,
            serial_tx,
            serial_rx,
            &mut afio.mapr,
            clocks,
            &mut rcc.apb2,
        );

        rprintln!("Setup done");

        // Return late resources
        init::LateResources {
            led,
            buttons: (btn_up, btn_down, btn_playpause),
            tagreader,
            player,
            event_prod,
            event_cons,
        }
    }

    #[idle(resources=[led, event_cons])]
    fn idle(cx: idle::Context) -> ! {
        rprintln!("Entering Idle Loop");
        loop {
            match cx.resources.event_cons.dequeue() {
                Some(app::Events::NewTag) => rprintln!("Event: New Tag detected"),
                Some(app::Events::ButtonPressedLong(button)) => match button {
                    app::Button::Up => rprintln!("Event: Button Up Pressed Long"),
                    app::Button::Down => rprintln!("Event: Button Down Pressed Long"),
                    app::Button::PlayPause => rprintln!("Event: Button PlayPause Pressed Long"),
                },
                Some(app::Events::ButtonPressedShort(button)) => match button {
                    app::Button::Up => rprintln!("Event: Button Up Pressed Short"),
                    app::Button::Down => rprintln!("Event: Button Down Pressed Short"),
                    app::Button::PlayPause => rprintln!("Event: Button PlayPause Pressed Short"),
                },
                None => (),
            }
        }
    }

    //===============================================================================================
    //==== Handling of the Buttons =====
    //===============================================================================================

    //==== Button Up=====
    #[task(binds=EXTI0, priority=5, resources=[buttons], schedule=[btn_check])]
    fn btn_up_pressed(cx: btn_up_pressed::Context) {
        if cx.resources.buttons.0.is_enabled() {
            cx.resources.buttons.0.disable();
            cx.schedule
                .btn_check(cx.start + CYCLES_10_MS.cycles(), app::Button::Up, 0)
                .ok();
        }
        cx.resources.buttons.0.clear_interrupt_pending_bit();
    }

    //==== Button Down =====
    #[task(binds=EXTI1, priority=5, resources=[buttons], schedule=[btn_check])]
    fn btn_down_pressed(cx: btn_down_pressed::Context) {
        if cx.resources.buttons.1.is_enabled() {
            cx.resources.buttons.1.disable();
            cx.schedule
                .btn_check(cx.start + CYCLES_10_MS.cycles(), app::Button::Down, 0)
                .ok();
        }
        cx.resources.buttons.1.clear_interrupt_pending_bit();
    }

    //==== Button PlayPause =====
    #[task(binds=EXTI2, priority=5, resources=[buttons], schedule=[btn_check])]
    fn btn_playpause_pressed(cx: btn_playpause_pressed::Context) {
        if cx.resources.buttons.2.is_enabled() {
            cx.resources.buttons.2.disable();
            cx.schedule
                .btn_check(cx.start + CYCLES_10_MS.cycles(), app::Button::PlayPause, 0)
                .ok();
        }
        cx.resources.buttons.2.clear_interrupt_pending_bit();
    }

    // ==== Button Evaluation ====
    #[task(priority=5, capacity = 3, resources=[buttons, event_prod], schedule=[btn_check, btn_enable])]
    fn btn_check(cx: btn_check::Context, btn: app::Button, iteration: u8) {
        use app::{Button::*, Events::*};
        // Check Iteration
        if iteration >= 100 {
            cx.resources
                .event_prod
                .enqueue(ButtonPressedLong(btn))
                .unwrap();
            // Schedule btn enable
            cx.schedule
                .btn_enable(cx.scheduled + CYCLES_10_MS.cycles(), btn)
                .unwrap();
        } else {
            // Check if button is pressed
            if match btn {
                Up => cx.resources.buttons.0.is_low(),
                Down => cx.resources.buttons.1.is_low(),
                PlayPause => cx.resources.buttons.2.is_low(),
            } {
                // Schedule next test
                cx.schedule
                    .btn_check(cx.scheduled + CYCLES_10_MS.cycles(), btn, iteration + 1)
                    .unwrap();
            } else {
                // Emit Event
                cx.resources
                    .event_prod
                    .enqueue(ButtonPressedShort(btn))
                    .unwrap();
                // Schedule btn enable
                cx.schedule
                    .btn_enable(cx.scheduled + CYCLES_10_MS.cycles(), btn)
                    .unwrap();
            }
        }
    }

    #[task(priority=5, capacity=3, resources=[buttons], schedule=[btn_enable])]
    fn btn_enable(cx: btn_enable::Context, btn: app::Button) {
        use app::Button::*;
        // Check if button is pressed
        if match btn {
            Up => cx.resources.buttons.0.is_high(),
            Down => cx.resources.buttons.1.is_high(),
            PlayPause => cx.resources.buttons.2.is_high(),
        } {
            match btn {
                Up => cx.resources.buttons.0.enable(),
                Down => cx.resources.buttons.1.enable(),
                PlayPause => cx.resources.buttons.2.enable(),
            }
        } else {
            // Schedule btn enable
            cx.schedule
                .btn_enable(cx.scheduled + CYCLES_10_MS.cycles(), btn)
                .unwrap();
        }
    }

    extern "C" {
        fn CAN_RX1();
    }
};
