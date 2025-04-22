//! MS and Second counter implemented using the TIM0 and TIM1 peripheral
#![no_main]
#![no_std]
// Import panic provider.
use panic_probe as _;
// Import logger.
use defmt_rtt as _;

use core::cell::Cell;
use cortex_m::asm;
use cortex_m_rt::entry;
use critical_section::Mutex;
use simple_examples::peb1;
use va416xx_hal::{
    irq_router::enable_and_init_irq_router,
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
    let mut dp = pac::Peripherals::take().unwrap();
    let mut last_ms = 0;
    defmt::println!("-- Vorago system ticks using timers --");
    // Use the external clock connected to XTAL_N.
    let clocks = dp
        .clkgen
        .constrain()
        .xtal_n_clk_with_src_freq(peb1::EXTCLK_FREQ)
        .freeze(&mut dp.sysconfig)
        .unwrap();
    enable_and_init_irq_router(&mut dp.sysconfig, &dp.irq_router);
    let _ = set_up_ms_tick(&mut dp.sysconfig, dp.tim0, &clocks);
    let mut second_timer = CountdownTimer::new(&mut dp.sysconfig, dp.tim1, &clocks);
    second_timer.listen();
    second_timer.start(1.Hz());
    loop {
        let current_ms = critical_section::with(|cs| MS_COUNTER.borrow(cs).get());
        if current_ms >= last_ms + 1000 {
            // To prevent drift.
            last_ms += 1000;
            defmt::info!("MS counter: {}", current_ms);
            let second = critical_section::with(|cs| SEC_COUNTER.borrow(cs).get());
            defmt::info!("Second counter: {}", second);
        }
        asm::delay(1000);
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
    critical_section::with(|cs| {
        let mut sec = SEC_COUNTER.borrow(cs).get();
        sec += 1;
        SEC_COUNTER.borrow(cs).set(sec);
    });
}
