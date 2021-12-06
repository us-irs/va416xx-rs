//! Simple blinky example using the HAL
#![no_main]
#![no_std]

use cortex_m_rt::entry;
use embedded_hal::digital::StatefulOutputPin;
use panic_rtt_target as _;
use rtt_target::{rprintln, rtt_init_print};
use va416xx_hal::{gpio::PinsG, pac};

#[entry]
fn main() -> ! {
    rtt_init_print!();
    rprintln!("VA416xx HAL blinky example");

    let mut dp = pac::Peripherals::take().unwrap();
    let portg = PinsG::new(&mut dp.sysconfig, dp.portg);
    let mut led = portg.pg5.into_readable_push_pull_output();
    //let mut delay = CountDownTimer::new(&mut dp.SYSCONFIG, 50.mhz(), dp.TIM0);
    loop {
        cortex_m::asm::delay(2_000_000);
        led.toggle().ok();
    }
}
