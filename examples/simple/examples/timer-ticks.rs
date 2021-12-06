//! MS and Second counter implemented using the TIM0 and TIM1 peripheral
#![no_main]
#![no_std]

use core::cell::Cell;
use cortex_m::interrupt::Mutex;
use cortex_m_rt::entry;
use panic_rtt_target as _;
use rtt_target::{rprintln, rtt_init_print};
use simple_examples::peb1;
use va416xx_hal::{
    pac::{self, interrupt},
    prelude::*,
    timer::{default_ms_irq_handler, set_up_ms_tick, CountdownTimer, MS_COUNTER},
};

#[allow(dead_code)]
enum LibType {
    Pac,
    Hal,
}

static SEC_COUNTER: Mutex<Cell<u32>> = Mutex::new(Cell::new(0));

#[entry]
fn main() -> ! {
    rtt_init_print!();
    let mut dp = pac::Peripherals::take().unwrap();
    let mut last_ms = 0;
    rprintln!("-- Vorago system ticks using timers --");
    // Use the external clock connected to XTAL_N.
    let clocks = dp
        .clkgen
        .constrain()
        .xtal_n_clk_with_src_freq(peb1::EXTCLK_FREQ)
        .freeze(&mut dp.sysconfig)
        .unwrap();
    let _ = set_up_ms_tick(&mut dp.sysconfig, dp.tim0, &clocks);
    let mut second_timer = CountdownTimer::new(&mut dp.sysconfig, dp.tim1, &clocks);
    second_timer.start(1.Hz());
    second_timer.listen();
    loop {
        let current_ms = cortex_m::interrupt::free(|cs| MS_COUNTER.borrow(cs).get());
        if current_ms - last_ms >= 1000 {
            last_ms = current_ms;
            rprintln!("MS counter: {}", current_ms);
            let second = cortex_m::interrupt::free(|cs| SEC_COUNTER.borrow(cs).get());
            rprintln!("Second counter: {}", second);
        }
        cortex_m::asm::delay(10000);
    }
}

#[interrupt]
#[allow(non_snake_case)]
fn TIM0() {
    default_ms_irq_handler()
}

#[interrupt]
#[allow(non_snake_case)]
fn TIM1() {
    cortex_m::interrupt::free(|cs| {
        let mut sec = SEC_COUNTER.borrow(cs).get();
        sec += 1;
        SEC_COUNTER.borrow(cs).set(sec);
    });
}
