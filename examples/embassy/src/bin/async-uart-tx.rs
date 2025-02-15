//! Asynchronous UART transmission example application.
//!
//! This application receives sends 4 strings with different sizes permanently.
//! It uses PORTG0 as TX pin and PORTG1 as RX pin, which is the UART0 on the PEB1 board.
//!
//! Instructions:
//!
//! 1. Tie a USB to UART converter with RX to PORTG0 and TX to PORTG1.
//! 2. Connect to the serial interface by using an application like Putty or picocom. You can
//!    type something in the terminal and check if the data is echoed back. You can also check the
//!    RTT logs to see received data.
#![no_std]
#![no_main]
use embassy_example::EXTCLK_FREQ;
use embassy_executor::Spawner;
use embassy_time::{Duration, Instant, Ticker};
use embedded_io_async::Write;
use panic_rtt_target as _;
use rtt_target::{rprintln, rtt_init_print};
use va416xx_hal::{
    gpio::PinsG,
    pac::{self, interrupt},
    prelude::*,
    time::Hertz,
    uart::{
        self,
        tx_asynch::{on_interrupt_tx, TxAsync},
        Bank,
    },
};

const STR_LIST: &[&str] = &[
    "Hello World\r\n",
    "Smoll\r\n",
    "A string which is larger than the FIFO size\r\n",
    "A really large string which is significantly larger than the FIFO size\r\n",
];

// main is itself an async function.
#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    rtt_init_print!();
    rprintln!("-- VA108xx Async UART TX Demo --");

    let mut dp = pac::Peripherals::take().unwrap();

    // Initialize the systick interrupt & obtain the token to prove that we did
    // Use the external clock connected to XTAL_N.
    let clocks = dp
        .clkgen
        .constrain()
        .xtal_n_clk_with_src_freq(Hertz::from_raw(EXTCLK_FREQ))
        .freeze(&mut dp.sysconfig)
        .unwrap();
    // Safety: Only called once here.
    unsafe {
        va416xx_embassy::init(
            &mut dp.sysconfig,
            &dp.irq_router,
            dp.tim15,
            dp.tim14,
            &clocks,
        );
    }

    let portg = PinsG::new(&mut dp.sysconfig, dp.portg);
    let mut led = portg.pg5.into_readable_push_pull_output();

    let tx = portg.pg0.into_funsel_1();
    let rx = portg.pg1.into_funsel_1();

    let uarta = uart::Uart::new(&mut dp.sysconfig, dp.uart0, (tx, rx), 115200.Hz(), &clocks);
    let (tx, _rx) = uarta.split();
    let mut async_tx = TxAsync::new(tx);
    let mut ticker = Ticker::every(Duration::from_secs(1));
    let mut idx = 0;
    loop {
        rprintln!("Current time: {}", Instant::now().as_secs());
        led.toggle();
        let _written = async_tx
            .write(STR_LIST[idx].as_bytes())
            .await
            .expect("writing failed");
        idx += 1;
        if idx == STR_LIST.len() {
            idx = 0;
        }
        ticker.next().await;
    }
}

#[interrupt]
#[allow(non_snake_case)]
fn UART0_TX() {
    on_interrupt_tx(Bank::Uart0);
}
