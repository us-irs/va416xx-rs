//! Asynchronous UART reception example application.
//!
//! This application receives data on two UARTs permanently using a ring buffer.
//! The ring buffer are read them asynchronously.
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
use core::cell::RefCell;

use critical_section::Mutex;
use embassy_example::EXTCLK_FREQ;
use embassy_executor::Spawner;
use embassy_time::Instant;
use embedded_io::Write;
use embedded_io_async::Read;
use heapless::spsc::{Producer, Queue};
use panic_rtt_target as _;
use rtt_target::{rprintln, rtt_init_print};
use va416xx_hal::{
    gpio::PinsG,
    pac::{self, interrupt},
    prelude::*,
    time::Hertz,
    uart::{
        self,
        rx_asynch::{on_interrupt_rx, RxAsync},
        Bank,
    },
};

static QUEUE_UART_A: static_cell::ConstStaticCell<Queue<u8, 256>> =
    static_cell::ConstStaticCell::new(Queue::new());
static PRODUCER_UART_A: Mutex<RefCell<Option<Producer<u8, 256>>>> = Mutex::new(RefCell::new(None));

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    rtt_init_print!();
    rprintln!("-- VA108xx Async UART RX Demo --");

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

    let (mut tx_uart_a, rx_uart_a) = uarta.split();
    let (prod_uart_a, cons_uart_a) = QUEUE_UART_A.take().split();
    // Pass the producer to the interrupt handler.
    critical_section::with(|cs| {
        *PRODUCER_UART_A.borrow(cs).borrow_mut() = Some(prod_uart_a);
    });

    // TODO: Add example for RxAsyncOverwriting using another UART.
    let mut async_uart_rx = RxAsync::new(rx_uart_a, cons_uart_a);
    let mut buf = [0u8; 256];
    loop {
        rprintln!("Current time UART A: {}", Instant::now().as_secs());
        led.toggle();
        let read_bytes = async_uart_rx.read(&mut buf).await.unwrap();
        let read_str = core::str::from_utf8(&buf[..read_bytes]).unwrap();
        rprintln!(
            "Read {} bytes asynchronously on UART A: {:?}",
            read_bytes,
            read_str
        );
        tx_uart_a.write_all(read_str.as_bytes()).unwrap();
    }
}

#[interrupt]
#[allow(non_snake_case)]
fn UART0_RX() {
    let mut prod =
        critical_section::with(|cs| PRODUCER_UART_A.borrow(cs).borrow_mut().take().unwrap());
    let errors = on_interrupt_rx(Bank::Uart0, &mut prod);
    critical_section::with(|cs| *PRODUCER_UART_A.borrow(cs).borrow_mut() = Some(prod));
    // In a production app, we could use a channel to send the errors to the main task.
    if let Err(errors) = errors {
        rprintln!("UART A errors: {:?}", errors);
    }
}
