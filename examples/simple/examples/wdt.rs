// Code to test the watchdog timer.
#![no_main]
#![no_std]

use core::cell::Cell;
use cortex_m::interrupt::Mutex;
use cortex_m_rt::entry;
use panic_rtt_target as _;
use rtt_target::{rprintln, rtt_init_print};
use simple_examples::peb1;
use va416xx_hal::pac::{self, interrupt};
use va416xx_hal::prelude::*;
use va416xx_hal::wdt::WdtController;

static WDT_INTRPT_COUNT: Mutex<Cell<u32>> = Mutex::new(Cell::new(0));

#[derive(Debug, PartialEq, Eq)]
#[allow(dead_code)]
enum TestMode {
    // Watchdog is fed by main loop, which runs with high period.
    FedByMain,
    // Watchdog is fed by watchdog IRQ.
    FedByIrq,
    AllowReset,
}
const TEST_MODE: TestMode = TestMode::FedByMain;
const WDT_ROLLOVER_MS: u32 = 100;

#[entry]
fn main() -> ! {
    rtt_init_print!();
    rprintln!("-- VA416xx WDT example application--");
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

    let mut last_interrupt_counter = 0;
    let mut wdt_ctrl =
        WdtController::start(&mut dp.sysconfig, dp.watch_dog, &clocks, WDT_ROLLOVER_MS);
    wdt_ctrl.enable_reset();
    loop {
        if TEST_MODE != TestMode::AllowReset {
            wdt_ctrl.feed();
        }
        let interrupt_counter = cortex_m::interrupt::free(|cs| WDT_INTRPT_COUNT.borrow(cs).get());
        if interrupt_counter > last_interrupt_counter {
            rprintln!("interrupt counter has increased to {}", interrupt_counter);
            last_interrupt_counter = interrupt_counter;
        }
        match TEST_MODE {
            TestMode::FedByMain => delay_sysclk.delay_ms(WDT_ROLLOVER_MS / 5),
            TestMode::FedByIrq => delay_sysclk.delay_ms(WDT_ROLLOVER_MS),
            _ => (),
        }
    }
}

#[interrupt]
#[allow(non_snake_case)]
fn WATCHDOG() {
    cortex_m::interrupt::free(|cs| {
        WDT_INTRPT_COUNT
            .borrow(cs)
            .set(WDT_INTRPT_COUNT.borrow(cs).get() + 1);
    });
    let wdt = unsafe { pac::WatchDog::steal() };
    // Clear interrupt.
    if TEST_MODE != TestMode::AllowReset {
        wdt.wdogintclr().write(|w| unsafe { w.bits(1) });
    }
}
