//! Simple ADC example.
#![no_main]
#![no_std]

use cortex_m_rt::entry;
use embedded_hal::delay::DelayNs;
use panic_rtt_target as _;
use rtt_target::{rprintln, rtt_init_print};
use simple_examples::peb1;
use va416xx_hal::{
    adc::{Adc, ChannelSelect, ChannelValue, MultiChannelSelect},
    pac,
    prelude::*,
    timer::CountdownTimer,
};

// Quite spammy and disabled by default.
const ENABLE_BUF_PRINTOUT: bool = false;

#[entry]
fn main() -> ! {
    rtt_init_print!();
    rprintln!("VA416xx ADC example");

    let mut dp = pac::Peripherals::take().unwrap();
    // Use the external clock connected to XTAL_N.
    let clocks = dp
        .clkgen
        .constrain()
        .xtal_n_clk_with_src_freq(peb1::EXTCLK_FREQ)
        .freeze(&mut dp.sysconfig)
        .unwrap();

    let adc = Adc::new_with_channel_tag(&mut dp.sysconfig, dp.adc, &clocks);
    let mut delay_provider = CountdownTimer::new(&mut dp.sysconfig, dp.tim0, &clocks);
    let mut read_buf: [ChannelValue; 8] = [ChannelValue::default(); 8];
    loop {
        let single_value = adc
            .trigger_and_read_single_channel(va416xx_hal::adc::ChannelSelect::TempSensor)
            .expect("reading single channel value failed");
        rprintln!(
            "Read single ADC value on temperature sensor channel: {:?}",
            single_value
        );
        let read_num = adc
            .sweep_and_read_range(0, 7, &mut read_buf)
            .expect("ADC range read failed");
        if ENABLE_BUF_PRINTOUT {
            rprintln!("ADC Range Read (0-8) read {} values", read_num);
            rprintln!("ADC Range Read (0-8): {:?}", read_buf);
        }
        assert_eq!(read_num, 8);
        for (idx, ch_val) in read_buf.iter().enumerate() {
            assert_eq!(
                ch_val.channel(),
                ChannelSelect::try_from(idx as u8).unwrap()
            );
        }

        adc.sweep_and_read_multiselect(
            MultiChannelSelect::AnIn0 | MultiChannelSelect::AnIn2 | MultiChannelSelect::TempSensor,
            &mut read_buf[0..3],
        )
        .expect("ADC multiselect read failed");
        if ENABLE_BUF_PRINTOUT {
            rprintln!("ADC Multiselect Read(0, 2 and 10): {:?}", &read_buf[0..3]);
        }
        assert_eq!(read_buf[0].channel(), ChannelSelect::AnIn0);
        assert_eq!(read_buf[1].channel(), ChannelSelect::AnIn2);
        assert_eq!(read_buf[2].channel(), ChannelSelect::TempSensor);
        delay_provider.delay_ms(500);
    }
}
