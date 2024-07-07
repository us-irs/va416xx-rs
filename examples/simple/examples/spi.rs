//! SPI example application.
//!
//! If you do not use the loopback mode, MOSI and MISO need to be tied together on the board.
#![no_main]
#![no_std]

use cortex_m_rt::entry;
use embedded_hal::spi::{Mode, SpiBus, MODE_0};
use panic_rtt_target as _;
use rtt_target::{rprintln, rtt_init_print};
use simple_examples::peb1;
use va416xx_hal::spi::{clk_div_for_target_clock, Spi, TransferConfig};
use va416xx_hal::{
    gpio::{PinsB, PinsC},
    pac,
    prelude::*,
    spi::SpiConfig,
    time::Hertz,
};

#[derive(PartialEq, Debug)]
pub enum ExampleSelect {
    // Enter loopback mode. It is not necessary to tie MOSI/MISO together for this
    Loopback,
    // Send a test buffer and print everything received. You need to tie together MOSI/MISO in this
    // mode.
    TestBuffer,
}

const EXAMPLE_SEL: ExampleSelect = ExampleSelect::Loopback;
const SPI_SPEED_KHZ: u32 = 1000;
const SPI_MODE: Mode = MODE_0;
const BLOCKMODE: bool = true;
const FILL_WORD: u8 = 0x0f;

#[entry]
fn main() -> ! {
    rtt_init_print!();
    rprintln!("-- VA108xx SPI example application--");
    let cp = cortex_m::Peripherals::take().unwrap();
    let mut dp = pac::Peripherals::take().unwrap();
    // Use the external clock connected to XTAL_N.
    let clocks = dp
        .clkgen
        .constrain()
        .xtal_n_clk_with_src_freq(peb1::EXTCLK_FREQ)
        .freeze(&mut dp.sysconfig)
        .unwrap();
    let mut delay_sysclk = cortex_m::delay::Delay::new(cp.SYST, clocks.apb0().raw());

    let pins_b = PinsB::new(&mut dp.sysconfig, dp.portb);
    let pins_c = PinsC::new(&mut dp.sysconfig, dp.portc);
    // Configure SPI1 pins.
    let (sck, miso, mosi) = (
        pins_b.pb15.into_funsel_1(),
        pins_c.pc0.into_funsel_1(),
        pins_c.pc1.into_funsel_1(),
    );

    let mut spi_cfg = SpiConfig::default().clk_div(
        clk_div_for_target_clock(Hertz::from_raw(SPI_SPEED_KHZ), &clocks)
            .expect("invalid target clock"),
    );
    if EXAMPLE_SEL == ExampleSelect::Loopback {
        spi_cfg = spi_cfg.loopback(true)
    }
    let transfer_cfg = TransferConfig::new_no_hw_cs(None, Some(SPI_MODE), BLOCKMODE, false);
    // Create SPI peripheral.
    let mut spi0 = Spi::new(
        &mut dp.sysconfig,
        &clocks,
        dp.spi0,
        (sck, miso, mosi),
        spi_cfg,
        Some(&transfer_cfg.downgrade()),
    )
    .expect("creating SPI peripheral failed");
    spi0.set_fill_word(FILL_WORD);
    loop {
        let mut tx_buf: [u8; 3] = [1, 2, 3];
        let mut rx_buf: [u8; 3] = [0; 3];
        // Can't really verify correct reply here.
        spi0.write(&[0x42]).expect("write failed");
        // Need small delay.. otherwise we will read back the sent byte (which we don't want here).
        // The write function will return as soon as all bytes were shifted out, ignoring the
        // reply bytes.
        delay_sysclk.delay_us(50);
        // Because of the loopback mode, we should get back the fill word here.
        spi0.read(&mut rx_buf[0..1]).unwrap();
        assert_eq!(rx_buf[0], FILL_WORD);

        spi0.transfer_in_place(&mut tx_buf)
            .expect("SPI transfer_in_place failed");
        assert_eq!([1, 2, 3], tx_buf);
        spi0.transfer(&mut rx_buf, &tx_buf)
            .expect("SPI transfer failed");
        assert_eq!(rx_buf, tx_buf);
        delay_sysclk.delay_ms(500);
    }
}
