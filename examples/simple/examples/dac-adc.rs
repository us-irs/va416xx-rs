//! Simple DAC-ADC example.
#![no_main]
#![no_std]

use cortex_m_rt::entry;
use embedded_hal::delay::DelayNs;
use panic_rtt_target as _;
use rtt_target::{rprintln, rtt_init_print};
use simple_examples::peb1;
use va416xx_hal::{adc::Adc, dac::Dac, pac, prelude::*, timer::CountdownTimer};

const DAC_INCREMENT: u16 = 256;

#[derive(Debug, PartialEq, Eq)]
pub enum AppMode {
    // Measurements on AIN0.
    AdcOnly,
    // AOUT0. You can use a multi-meter to measure the changing voltage on the pin.
    DacOnly,
    /// AOUT0 needs to be wired to AIN0.
    DacAndAdc,
}

const APP_MODE: AppMode = AppMode::DacAndAdc;

#[entry]
fn main() -> ! {
    rtt_init_print!();
    rprintln!("VA416xx DAC/ADC example");

    let mut dp = pac::Peripherals::take().unwrap();
    // Use the external clock connected to XTAL_N.
    let clocks = dp
        .clkgen
        .constrain()
        .xtal_n_clk_with_src_freq(peb1::EXTCLK_FREQ)
        .freeze(&mut dp.sysconfig)
        .unwrap();
    let mut dac = None;
    if APP_MODE == AppMode::DacOnly || APP_MODE == AppMode::DacAndAdc {
        dac = Some(Dac::new(
            &mut dp.sysconfig,
            dp.dac0,
            va416xx_hal::dac::DacSettling::Apb2Times100,
            &clocks,
        ));
    }
    let mut adc = None;
    if APP_MODE == AppMode::AdcOnly || APP_MODE == AppMode::DacAndAdc {
        adc = Some(Adc::new(&mut dp.sysconfig, dp.adc, &clocks));
    }
    let mut delay_provider = CountdownTimer::new(&mut dp.sysconfig, dp.tim0, &clocks);
    let mut current_val = 0;
    loop {
        if let Some(dac) = &dac {
            rprintln!("loading DAC with value {}", current_val);
            dac.load_and_trigger_manually(current_val)
                .expect("loading DAC value failed");
            if current_val + DAC_INCREMENT >= 4096 {
                current_val = 0;
            } else {
                current_val += DAC_INCREMENT;
            }
        }
        if let Some(dac) = &dac {
            // This should never block.
            nb::block!(dac.is_settled()).unwrap();
        }
        if let Some(adc) = &adc {
            let ch_value = adc
                .trigger_and_read_single_channel(va416xx_hal::adc::ChannelSelect::AnIn0)
                .expect("reading ADC channel 0 failed");
            rprintln!("Received channel value {:?}", ch_value);
        }

        delay_provider.delay_ms(500);
    }
}
