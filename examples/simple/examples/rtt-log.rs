// Code to test RTT logger functionality.
#![no_main]
#![no_std]

use cortex_m_rt::entry;
use panic_rtt_target as _;
use rtt_target::{rprintln, rtt_init_print};
use va416xx_hal::pac;

// Mask for the LED
const LED_PG5: u32 = 1 << 5;

#[entry]
fn main() -> ! {
    let dp = pac::Peripherals::take().unwrap();
    // Enable all peripheral clocks
    dp.sysconfig
        .peripheral_clk_enable()
        .modify(|_, w| unsafe { w.bits(0xffffffff) });
    dp.portg.dir().modify(|_, w| unsafe { w.bits(LED_PG5) });
    dp.portg
        .datamask()
        .modify(|_, w| unsafe { w.bits(LED_PG5) });

    rtt_init_print!();
    rprintln!("VA416xx RTT Demo");
    let mut counter = 0;
    loop {
        rprintln!("{}: Hello, world!", counter);
        // Still toggle LED. If there are issues with the RTT log, the LED
        // blinking ensures that the application is actually running.
        dp.portg.togout().write(|w| unsafe { w.bits(LED_PG5) });
        counter += 1;
        cortex_m::asm::delay(10_000_000);
    }
}
