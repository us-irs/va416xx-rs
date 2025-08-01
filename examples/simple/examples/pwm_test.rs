//! Simple PWM example
//!
//! Outputs a PWM waveform on pin PG2.
#![no_main]
#![no_std]

use cortex_m_rt::entry;
use embedded_hal::pwm::SetDutyCycle;
// Import panic provider.
use panic_probe as _;
// Import logger.
use defmt_rtt as _;
use simple_examples::peb1;
use va416xx_hal::{
    clock::ClockConfigurator,
    pac,
    pins::PinsG,
    prelude::*,
    pwm::{get_duty_from_percent, PwmPin},
};

#[entry]
fn main() -> ! {
    defmt::println!("-- VA108xx PWM example application--");
    let dp = pac::Peripherals::take().unwrap();

    // Use the external clock connected to XTAL_N.
    let clocks = ClockConfigurator::new(dp.clkgen)
        .xtal_n_clk_with_src_freq(peb1::EXTCLK_FREQ)
        .freeze()
        .unwrap();

    let pinsg = PinsG::new(dp.portg);
    let mut pwm = PwmPin::new(pinsg.pg2, dp.tim9, &clocks, 10.kHz()).unwrap();
    //let mut delay_timer = CountdownTimer::new(dp.tim0, &clocks);
    //let mut current_duty_cycle = 0.0;
    pwm.set_duty_cycle(get_duty_from_percent(0.5)).unwrap();
    pwm.enable();

    // Delete type information, increased code readibility for the rest of the code
    loop {
        cortex_m::asm::nop();
    }
}
