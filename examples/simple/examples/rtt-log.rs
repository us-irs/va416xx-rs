// Code to test RTT logger functionality.
#![no_main]
#![no_std]

// Import panic provider.
use panic_probe as _;
// Import logger.
use defmt_rtt as _;

use cortex_m_rt::entry;
use va416xx_hal::pac;

// Mask for the LED
const LED_PG5: u32 = 1 << 5;

#[entry]
fn main() -> ! {
    defmt::println!("VA416xx RTT Demo");
    let dp = pac::Peripherals::take().unwrap();
    // Enable all peripheral clocks
    dp.sysconfig
        .peripheral_clk_enable()
        .modify(|_, w| unsafe { w.bits(0xffffffff) });
    dp.portg.dir().modify(|_, w| unsafe { w.bits(LED_PG5) });
    dp.portg
        .datamask()
        .modify(|_, w| unsafe { w.bits(LED_PG5) });

    let mut counter = 0;
    loop {
        defmt::info!("{}: Hello, world!", counter);
        // Still toggle LED. If there are issues with the RTT log, the LED
        // blinking ensures that the application is actually running.
        dp.portg.togout().write(|w| unsafe { w.bits(LED_PG5) });
        counter += 1;
        cortex_m::asm::delay(10_000_000);
    }
}
