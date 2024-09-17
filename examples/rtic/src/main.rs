//! RTIC minimal blinky
#![no_main]
#![no_std]

#[rtic::app(device = pac, dispatchers = [U1, U2, U3])]
mod app {
    use cortex_m::asm;
    use embedded_hal::digital::StatefulOutputPin;
    use panic_rtt_target as _;
    use rtic_monotonics::systick::prelude::*;
    use rtic_monotonics::Monotonic;
    use rtt_target::{rprintln, rtt_init_default};
    use va416xx_hal::{
        gpio::{OutputReadablePushPull, Pin, PinsG, PG5},
        pac,
    };

    #[local]
    struct Local {
        led: Pin<PG5, OutputReadablePushPull>,
    }

    #[shared]
    struct Shared {}

    rtic_monotonics::systick_monotonic!(Mono, 10_000);

    #[init]
    fn init(_ctx: init::Context) -> (Shared, Local) {
        rtt_init_default!();
        rprintln!("-- Vorago RTIC template --");
        let mut dp = pac::Peripherals::take().unwrap();
        let portg = PinsG::new(&mut dp.sysconfig, dp.portg);
        let led = portg.pg5.into_readable_push_pull_output();
        blinky::spawn().ok();
        (Shared {}, Local { led })
    }

    // `shared` cannot be accessed from this context
    #[idle]
    fn idle(_cx: idle::Context) -> ! {
        loop {
            asm::nop();
        }
    }

    #[task(
        priority = 3,
        local=[led],
    )]
    async fn blinky(cx: blinky::Context) {
        loop {
            cx.local.led.toggle().ok();
            Mono::delay(200.millis()).await;
        }
    }
}
