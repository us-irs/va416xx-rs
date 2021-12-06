// UART example application. Sends a test string over a UART and then enters
// echo mode
#![no_main]
#![no_std]

use cortex_m_rt::entry;
use embedded_hal_nb::serial::Read;
use embedded_io::Write;
use panic_rtt_target as _;
use rtt_target::{rprintln, rtt_init_print};
use simple_examples::peb1;
use va416xx_hal::clock::ClkgenExt;
use va416xx_hal::time::Hertz;
use va416xx_hal::{gpio::PinsG, pac, uart};

#[entry]
fn main() -> ! {
    rtt_init_print!();
    rprintln!("-- VA416xx UART example application--");

    let mut dp = pac::Peripherals::take().unwrap();

    // Use the external clock connected to XTAL_N.
    let clocks = dp
        .clkgen
        .constrain()
        .xtal_n_clk_with_src_freq(peb1::EXTCLK_FREQ)
        .freeze(&mut dp.sysconfig)
        .unwrap();

    let gpiob = PinsG::new(&mut dp.sysconfig, dp.portg);
    let tx = gpiob.pg0.into_funsel_1();
    let rx = gpiob.pg1.into_funsel_1();

    let uart0 = uart::Uart::new(
        dp.uart0,
        (tx, rx),
        Hertz::from_raw(115200),
        &mut dp.sysconfig,
        &clocks,
    );
    let (mut tx, mut rx) = uart0.split();
    writeln!(tx, "Hello World\n\r").unwrap();
    loop {
        // Echo what is received on the serial link.
        match nb::block!(rx.read()) {
            Ok(recvd) => {
                if let Err(e) = embedded_hal_nb::serial::Write::write(&mut tx, recvd) {
                    rprintln!("UART TX error: {:?}", e);
                }
            }
            Err(e) => {
                rprintln!("UART RX error {:?}", e);
            }
        }
    }
}
