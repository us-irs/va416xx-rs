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
use va416xx_hal::spi::{Spi, SpiClkConfig};
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
    // You need to tie together MOSI/MISO in this mode.
    MosiMisoTiedTogether,
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
    // Configure SPI0 pins.
    let (sck, miso, mosi) = (
        pins_b.pb15.into_funsel_1(),
        pins_c.pc0.into_funsel_1(),
        pins_c.pc1.into_funsel_1(),
    );

    let mut spi_cfg = SpiConfig::default()
        .clk_cfg(
            SpiClkConfig::from_clk(Hertz::from_raw(SPI_SPEED_KHZ), &clocks)
                .expect("invalid target clock"),
        )
        .mode(SPI_MODE)
        .blockmode(BLOCKMODE);
    if EXAMPLE_SEL == ExampleSelect::Loopback {
        spi_cfg = spi_cfg.loopback(true)
    }
    // Create SPI peripheral.
    let mut spi0 = Spi::new(
        &mut dp.sysconfig,
        &clocks,
        dp.spi0,
        (sck, miso, mosi),
        spi_cfg,
    );
    spi0.set_fill_word(FILL_WORD);
    loop {
        let tx_buf: [u8; 4] = [1, 2, 3, 0];
        let mut rx_buf: [u8; 4] = [0; 4];
        // Can't really verify correct behaviour here. Just verify nothing crazy happens or it hangs up.
        spi0.write(&[0x42, 0x43]).expect("write failed");

        // Can't really verify correct behaviour here. Just verify nothing crazy happens or it hangs up.
        spi0.read(&mut rx_buf[0..2]).unwrap();

        // If the pins are tied together, we should received exactly what we send.

        let mut inplace_buf = tx_buf;
        spi0.transfer_in_place(&mut inplace_buf)
            .expect("SPI transfer_in_place failed");
        assert_eq!([1, 2, 3, 0], inplace_buf);

        spi0.transfer(&mut rx_buf, &tx_buf)
            .expect("SPI transfer failed");
        assert_eq!(rx_buf, [1, 2, 3, 0]);
        delay_sysclk.delay_ms(500);
    }
}
