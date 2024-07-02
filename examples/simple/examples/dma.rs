//! Simple DMA example
#![no_main]
#![no_std]

use core::cell::Cell;

use cortex_m::interrupt::Mutex;
use cortex_m_rt::entry;
use embedded_hal::delay::DelayNs;
use panic_rtt_target as _;
use rtt_target::{rprintln, rtt_init_print};
use simple_examples::peb1;
use va416xx_hal::dma::{Dma, DmaCfg, DmaCtrlBlock};
use va416xx_hal::pwm::CountdownTimer;
use va416xx_hal::{
    pac::{self, interrupt},
    prelude::*,
};

// Place the DMA control block in SRAM1
const DMA_CTRL_BLOCK_ADDR: u32 = 0x2000_0000;
static DMA_DONE_FLAG: Mutex<Cell<bool>> = Mutex::new(Cell::new(false));

#[entry]
fn main() -> ! {
    rtt_init_print!();
    rprintln!("VA416xx DMA example");

    let mut dp = pac::Peripherals::take().unwrap();
    // Use the external clock connected to XTAL_N.
    let clocks = dp
        .clkgen
        .constrain()
        .xtal_n_clk_with_src_freq(peb1::EXTCLK_FREQ)
        .freeze(&mut dp.sysconfig)
        .unwrap();
    let dma_ctrl_block =
        DmaCtrlBlock::new_at_addr(DMA_CTRL_BLOCK_ADDR).expect("error creating DMA control block");
    let dma = Dma::new(&mut dp.sysconfig, dp.dma, DmaCfg::default(), dma_ctrl_block)
        .expect("error creating DMA");
    let (mut dma0, _, _, _) = dma.split();
    let mut delay_ms = CountdownTimer::new(&mut dp.sysconfig, dp.tim0, &clocks);
    let mut src_buf: [u8; 64] = [0; 64];
    let mut dest_buf: [u8; 64] = [0; 64];
    let dma_ctrl_block_ptr = DMA_CTRL_BLOCK_ADDR as *const DmaCtrlBlock;
    let dma_ctrl_block_ref = unsafe { &*dma_ctrl_block_ptr };
    loop {
        (0..64).for_each(|i| {
            src_buf[i] = i as u8;
        });
        cortex_m::interrupt::free(|cs| {
            DMA_DONE_FLAG.borrow(cs).set(false);
        });
        dma0.prepare_mem_to_mem_transfer_8_bit(&src_buf, &mut dest_buf)
            .expect("error preparing transfer");
        rprintln!("ch0 cfg: {:?}", dma_ctrl_block_ref.pri[0].cfg);
        dma0.select_primary_structure();
        // Safety: Not using mask based critical sections.
        unsafe {
            dma0.enable_done_interrupt();
            dma0.enable_active_interrupt();
        };
        dma0.enable();
        //dma0.sw_request();
        let state = dma0.state_raw();
        rprintln!("dma state: {}", state);
        // Use polling for completion status.
        loop {
            let mut dma_done = false;
            cortex_m::interrupt::free(|cs| {
                if DMA_DONE_FLAG.borrow(cs).get() {
                    dma_done = true;
                }
            });
            if dma_done {
                rprintln!("DMA transfer done");
                break;
            }
            let state = dma0.state_raw();
            if state != 0 {
                rprintln!("dma state: {}", state);
            }
            delay_ms.delay_ms(1);
        }
        (0..64).for_each(|i| {
            assert_eq!(dest_buf[i], i as u8);
        });
        dest_buf.fill(0);
        delay_ms.delay_ms(200);
    }
}

#[interrupt]
#[allow(non_snake_case)]
fn DMA_DONE0() {
    rprintln!("dma done interrupt");
    // Notify the main loop that the DMA transfer is finished.
    cortex_m::interrupt::free(|cs| {
        DMA_DONE_FLAG.borrow(cs).set(true);
    });
}

#[interrupt]
#[allow(non_snake_case)]
fn DMA_ACTIVE0() {
    rprintln!("dma active interrupt");
}
