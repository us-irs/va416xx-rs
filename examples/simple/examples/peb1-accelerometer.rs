//! Example code for the PEB1 development board accelerometer.
#![no_main]
#![no_std]

use accelerometer::{Accelerometer, RawAccelerometer};
use cortex_m_rt::entry;
use embedded_hal::delay::DelayNs;
use panic_rtt_target as _;
use rtt_target::{rprintln, rtt_init_print};
use simple_examples::peb1;
use va416xx_hal::{
    i2c,
    pac::{self},
    prelude::*,
    pwm::CountdownTimer,
};
use vorago_peb1::lis2dh12::{self, detect_i2c_addr, FullScale, Odr};

pub enum DisplayMode {
    Raw,
    Normalized,
}

const DISPLAY_MODE: DisplayMode = DisplayMode::Normalized;

#[entry]
fn main() -> ! {
    rtt_init_print!();
    let mut dp = pac::Peripherals::take().unwrap();
    rprintln!("-- Vorago PEB1 accelerometer example --");
    // Use the external clock connected to XTAL_N.
    let clocks = dp
        .clkgen
        .constrain()
        .xtal_n_clk_with_src_freq(peb1::EXTCLK_FREQ)
        .freeze(&mut dp.sysconfig)
        .unwrap();
    let mut i2c_master = i2c::I2cMaster::new(
        dp.i2c0,
        &mut dp.sysconfig,
        i2c::MasterConfig::default(),
        &clocks,
        i2c::I2cSpeed::Regular100khz,
    )
    .expect("creating I2C master failed");
    let mut delay_provider = CountdownTimer::new(&mut dp.sysconfig, dp.tim1, &clocks);
    // Detect the I2C address of the accelerometer by scanning all possible values.
    let slave_addr = detect_i2c_addr(&mut i2c_master).expect("detecting I2C address failed");
    // Create the accelerometer driver using the PEB1 BSP.
    let mut accelerometer = vorago_peb1::accelerometer::new_with_i2cm(i2c_master, slave_addr)
        .expect("creating accelerometer driver failed");
    let device_id = accelerometer.get_device_id().unwrap();
    accelerometer
        .set_mode(lis2dh12::reg::Mode::Normal)
        .expect("setting mode failed");
    accelerometer
        .set_odr(Odr::Hz100)
        .expect("setting ODR failed");
    accelerometer
        .set_fs(FullScale::G4)
        .expect("setting full scale failed");
    // This function also enabled BDU.
    accelerometer
        .enable_temp(true)
        .expect("enabling temperature sensor failed");
    rprintln!("Device ID: 0x{:02X}", device_id);
    // Start reading the accelerometer periodically.
    loop {
        let temperature = accelerometer
            .get_temp_outf()
            .expect("reading temperature failed");
        match DISPLAY_MODE {
            DisplayMode::Normalized => {
                let value = accelerometer
                    .accel_norm()
                    .expect("reading normalized accelerometer data failed");
                rprintln!("Accel Norm F32x3: {:.06?} | Temp {} °C", value, temperature);
            }
            DisplayMode::Raw => {
                let value_raw = accelerometer
                    .accel_raw()
                    .expect("reading raw accelerometer data failed");
                rprintln!("Accel Raw F32x3: {:?} | Temp {} °C", value_raw, temperature);
            }
        }
        delay_provider.delay_ms(100);
    }
}
