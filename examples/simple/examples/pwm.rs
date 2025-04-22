//! Simple PWM example
#![no_main]
#![no_std]

use cortex_m_rt::entry;
use embedded_hal::{delay::DelayNs, pwm::SetDutyCycle};
// Import panic provider.
use panic_probe as _;
// Import logger.
use defmt_rtt as _;
use simple_examples::peb1;
use va416xx_hal::{
    gpio::PinsA,
    pac,
    prelude::*,
    pwm::{self, get_duty_from_percent, PwmA, PwmB, ReducedPwmPin},
    timer::CountdownTimer,
};

#[entry]
fn main() -> ! {
    defmt::println!("-- VA108xx PWM example application--");
    let mut dp = pac::Peripherals::take().unwrap();

    // Use the external clock connected to XTAL_N.
    let clocks = dp
        .clkgen
        .constrain()
        .xtal_n_clk_with_src_freq(peb1::EXTCLK_FREQ)
        .freeze(&mut dp.sysconfig)
        .unwrap();

    let pinsa = PinsA::new(&mut dp.sysconfig, dp.porta);
    let mut pwm = pwm::PwmPin::new(
        (pinsa.pa3.into_funsel_1(), dp.tim3),
        &mut dp.sysconfig,
        &clocks,
        10.Hz(),
    );
    let mut delay_timer = CountdownTimer::new(&mut dp.sysconfig, dp.tim0, &clocks);
    let mut current_duty_cycle = 0.0;
    pwm.set_duty_cycle(get_duty_from_percent(current_duty_cycle))
        .unwrap();
    pwm.enable();

    // Delete type information, increased code readibility for the rest of the code
    let mut reduced_pin = ReducedPwmPin::from(pwm);
    loop {
        let mut counter = 0;
        // Increase duty cycle continuously
        while current_duty_cycle < 1.0 {
            delay_timer.delay_ms(400);
            current_duty_cycle += 0.02;
            counter += 1;
            if counter % 10 == 0 {
                defmt::info!("current duty cycle: {}", current_duty_cycle);
            }

            reduced_pin
                .set_duty_cycle(get_duty_from_percent(current_duty_cycle))
                .unwrap();
        }

        // Switch to PWMB and decrease the window with a high signal from 100 % to 0 %
        // continously
        current_duty_cycle = 0.0;
        let mut upper_limit = 1.0;
        let mut lower_limit = 0.0;
        let mut pwmb: ReducedPwmPin<PwmB> = ReducedPwmPin::from(reduced_pin);
        pwmb.set_pwmb_lower_limit(get_duty_from_percent(lower_limit));
        pwmb.set_pwmb_upper_limit(get_duty_from_percent(upper_limit));
        while lower_limit < 0.5 {
            delay_timer.delay_ms(400);
            lower_limit += 0.01;
            upper_limit -= 0.01;
            pwmb.set_pwmb_lower_limit(get_duty_from_percent(lower_limit));
            pwmb.set_pwmb_upper_limit(get_duty_from_percent(upper_limit));
            defmt::info!("Lower limit: {}", pwmb.pwmb_lower_limit());
            defmt::info!("Upper limit: {}", pwmb.pwmb_upper_limit());
        }
        reduced_pin = ReducedPwmPin::<PwmA>::from(pwmb);
    }
}
