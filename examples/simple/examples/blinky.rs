//! Simple blinky example using the HAL
#![no_main]
#![no_std]

// Import panic provider.
use panic_probe as _;
// Import logger.
use defmt_rtt as _;

use cortex_m_rt::entry;
use va416xx_hal::{gpio::PinsG, pac};

#[entry]
fn main() -> ! {
    defmt::println!("VA416xx HAL blinky example");

    let mut dp = pac::Peripherals::take().unwrap();
    let portg = PinsG::new(&mut dp.sysconfig, dp.portg);
    let mut led = portg.pg5.into_readable_push_pull_output();
    loop {
        cortex_m::asm::delay(2_000_000);
        led.toggle();
    }
}
