//! RTIC minimal blinky
#![no_main]
#![no_std]

use va416xx_hal::time::Hertz;

const EXTCLK_FREQ: Hertz = Hertz::from_raw(40_000_000);

#[rtic::app(device = pac, dispatchers = [U1, U2, U3])]
mod app {
    use super::*;
    // Import panic provider.
    use panic_probe as _;
    // Import logger.
    use cortex_m::asm;
    use defmt_rtt as _;
    use rtic_monotonics::systick::prelude::*;
    use rtic_monotonics::Monotonic;
    use va416xx_hal::{
        gpio::{OutputReadablePushPull, Pin, PinsG, PG5},
        pac,
        prelude::*,
    };

    #[local]
    struct Local {
        led: Pin<PG5, OutputReadablePushPull>,
    }

    #[shared]
    struct Shared {}

    rtic_monotonics::systick_monotonic!(Mono, 1_000);

    #[init]
    fn init(mut cx: init::Context) -> (Shared, Local) {
        defmt::println!("-- Vorago RTIC example application --");
        // Use the external clock connected to XTAL_N.
        let clocks = cx
            .device
            .clkgen
            .constrain()
            .xtal_n_clk_with_src_freq(EXTCLK_FREQ)
            .freeze(&mut cx.device.sysconfig)
            .unwrap();
        Mono::start(cx.core.SYST, clocks.sysclk().raw());
        let portg = PinsG::new(&mut cx.device.sysconfig, cx.device.portg);
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
            cx.local.led.toggle();
            Mono::delay(200.millis()).await;
        }
    }
}
