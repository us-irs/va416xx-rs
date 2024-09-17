#![no_std]
#![no_main]
use embassy_executor::Spawner;
use embassy_time::{Duration, Instant, Ticker};
use embedded_hal::digital::StatefulOutputPin;
use panic_rtt_target as _;
use rtt_target::{rprintln, rtt_init_print};
use va416xx_hal::{gpio::PinsG, pac, prelude::*, time::Hertz};

const EXTCLK_FREQ: u32 = 40_000_000;

// main is itself an async function.
#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    rtt_init_print!();
    rprintln!("VA416xx Embassy Demo");

    let mut dp = pac::Peripherals::take().unwrap();

    // Initialize the systick interrupt & obtain the token to prove that we did
    // Use the external clock connected to XTAL_N.
    let clocks = dp
        .clkgen
        .constrain()
        .xtal_n_clk_with_src_freq(Hertz::from_raw(EXTCLK_FREQ))
        .freeze(&mut dp.sysconfig)
        .unwrap();
    // Safety: Only called once here.
    unsafe {
        embassy_example::init(
            &mut dp.sysconfig,
            &dp.irq_router,
            dp.tim15,
            dp.tim14,
            &clocks,
        )
    };
    let portg = PinsG::new(&mut dp.sysconfig, dp.portg);
    let mut led = portg.pg5.into_readable_push_pull_output();
    let mut ticker = Ticker::every(Duration::from_secs(1));
    loop {
        ticker.next().await;
        rprintln!("Current time: {}", Instant::now().as_secs());
        led.toggle().ok();
    }
}
