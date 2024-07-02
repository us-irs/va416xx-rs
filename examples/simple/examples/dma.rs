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
use va416xx_hal::dma::{Dma, DmaCfg, DmaChannel, DmaCtrlBlock};
use va416xx_hal::pwm::CountdownTimer;
use va416xx_hal::{
    pac::{self, interrupt},
    prelude::*,
};

// Place the DMA control block in SRAM1
const DMA_CTRL_BLOCK_ADDR: u32 = 0x2000_0000;
static DMA_DONE_FLAG: Mutex<Cell<bool>> = Mutex::new(Cell::new(false));
static DMA_ACTIVE_FLAG: Mutex<Cell<bool>> = Mutex::new(Cell::new(false));

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
    let mut src_buf_8_bit: [u8; 65] = [0; 65];
    let mut dest_buf_8_bit: [u8; 65] = [0; 65];
    let mut src_buf_16_bit: [u16; 33] = [0; 33];
    let mut dest_buf_16_bit: [u16; 33] = [0; 33];
    let mut src_buf_32_bit: [u32; 17] = [0; 17];
    let mut dest_buf_32_bit: [u32; 17] = [0; 17];
    loop {
        transfer_example_8_bit(
            &mut src_buf_8_bit,
            &mut dest_buf_8_bit,
            &mut dma0,
            &mut delay_ms,
        );
        delay_ms.delay_ms(500);
        transfer_example_16_bit(
            &mut src_buf_16_bit,
            &mut dest_buf_16_bit,
            &mut dma0,
            &mut delay_ms,
        );
        delay_ms.delay_ms(500);
        transfer_example_32_bit(
            &mut src_buf_32_bit,
            &mut dest_buf_32_bit,
            &mut dma0,
            &mut delay_ms,
        );
        delay_ms.delay_ms(500);
    }
}

fn transfer_example_8_bit(
    src_buf: &mut [u8; 65],
    dest_buf: &mut [u8; 65],
    dma0: &mut DmaChannel,
    delay_ms: &mut CountdownTimer<pac::Tim0>,
) {
    (0..64).for_each(|i| {
        src_buf[i] = i as u8;
    });
    cortex_m::interrupt::free(|cs| {
        DMA_DONE_FLAG.borrow(cs).set(false);
    });
    cortex_m::interrupt::free(|cs| {
        DMA_ACTIVE_FLAG.borrow(cs).set(false);
    });
    dma0.prepare_mem_to_mem_transfer_8_bit(src_buf, dest_buf)
        .expect("error preparing transfer");
    // Enable all interrupts.
    // Safety: Not using mask based critical sections.
    unsafe {
        dma0.enable_done_interrupt();
        dma0.enable_active_interrupt();
    };
    // Enable the individual channel.
    dma0.enable();
    // We still need to manually trigger the DMA request.
    dma0.trigger_with_sw_request();
    // Use polling for completion status.
    loop {
        let mut dma_done = false;
        cortex_m::interrupt::free(|cs| {
            if DMA_ACTIVE_FLAG.borrow(cs).get() {
                rprintln!("DMA0 is active with 8 bit transfer");
                DMA_ACTIVE_FLAG.borrow(cs).set(false);
            }
            if DMA_DONE_FLAG.borrow(cs).get() {
                dma_done = true;
            }
        });
        if dma_done {
            rprintln!("8-bit transfer done");
            break;
        }
        delay_ms.delay_ms(1);
    }
    (0..64).for_each(|i| {
        assert_eq!(dest_buf[i], i as u8);
    });
    // Sentinel value, should be 0.
    assert_eq!(dest_buf[64], 0);
    dest_buf.fill(0);
}

fn transfer_example_16_bit(
    src_buf: &mut [u16; 33],
    dest_buf: &mut [u16; 33],
    dma0: &mut DmaChannel,
    delay_ms: &mut CountdownTimer<pac::Tim0>,
) {
    // Set values scaled from 0 to 65535 to verify this is really a 16-bit transfer.
    (0..32).for_each(|i| {
        src_buf[i] = (i as u32 * u16::MAX as u32 / (src_buf.len() - 1) as u32) as u16;
    });
    cortex_m::interrupt::free(|cs| {
        DMA_DONE_FLAG.borrow(cs).set(false);
    });
    cortex_m::interrupt::free(|cs| {
        DMA_ACTIVE_FLAG.borrow(cs).set(false);
    });
    dma0.prepare_mem_to_mem_transfer_16_bit(src_buf, dest_buf)
        .expect("error preparing transfer");
    // Enable all interrupts.
    // Safety: Not using mask based critical sections.
    unsafe {
        dma0.enable_done_interrupt();
        dma0.enable_active_interrupt();
    };
    // Enable the individual channel.
    dma0.enable();
    // We still need to manually trigger the DMA request.
    dma0.trigger_with_sw_request();
    // Use polling for completion status.
    loop {
        let mut dma_done = false;
        cortex_m::interrupt::free(|cs| {
            if DMA_ACTIVE_FLAG.borrow(cs).get() {
                rprintln!("DMA0 is active with 16-bit transfer");
                DMA_ACTIVE_FLAG.borrow(cs).set(false);
            }
            if DMA_DONE_FLAG.borrow(cs).get() {
                dma_done = true;
            }
        });
        if dma_done {
            rprintln!("16-bit transfer done");
            break;
        }
        delay_ms.delay_ms(1);
    }
    (0..32).for_each(|i| {
        assert_eq!(
            dest_buf[i],
            (i as u32 * u16::MAX as u32 / (src_buf.len() - 1) as u32) as u16
        );
    });
    // Sentinel value, should be 0.
    assert_eq!(dest_buf[32], 0);
    dest_buf.fill(0);
}

fn transfer_example_32_bit(
    src_buf: &mut [u32; 17],
    dest_buf: &mut [u32; 17],
    dma0: &mut DmaChannel,
    delay_ms: &mut CountdownTimer<pac::Tim0>,
) {
    // Set values scaled from 0 to 65535 to verify this is really a 16-bit transfer.
    (0..16).for_each(|i| {
        src_buf[i] = (i as u64 * u32::MAX as u64 / (src_buf.len() - 1) as u64) as u32;
    });
    cortex_m::interrupt::free(|cs| {
        DMA_DONE_FLAG.borrow(cs).set(false);
    });
    cortex_m::interrupt::free(|cs| {
        DMA_ACTIVE_FLAG.borrow(cs).set(false);
    });
    dma0.prepare_mem_to_mem_transfer_32_bit(src_buf, dest_buf)
        .expect("error preparing transfer");
    // Enable all interrupts.
    // Safety: Not using mask based critical sections.
    unsafe {
        dma0.enable_done_interrupt();
        dma0.enable_active_interrupt();
    };
    // Enable the individual channel.
    dma0.enable();
    // We still need to manually trigger the DMA request.
    dma0.trigger_with_sw_request();
    // Use polling for completion status.
    loop {
        let mut dma_done = false;
        cortex_m::interrupt::free(|cs| {
            if DMA_ACTIVE_FLAG.borrow(cs).get() {
                rprintln!("DMA0 is active with 32-bit transfer");
                DMA_ACTIVE_FLAG.borrow(cs).set(false);
            }
            if DMA_DONE_FLAG.borrow(cs).get() {
                dma_done = true;
            }
        });
        if dma_done {
            rprintln!("32-bit transfer done");
            break;
        }
        delay_ms.delay_ms(1);
    }
    (0..16).for_each(|i| {
        assert_eq!(
            dest_buf[i],
            (i as u64 * u32::MAX as u64 / (src_buf.len() - 1) as u64) as u32
        );
    });
    // Sentinel value, should be 0.
    assert_eq!(dest_buf[16], 0);
    dest_buf.fill(0);
}

#[interrupt]
#[allow(non_snake_case)]
fn DMA_DONE0() {
    // Notify the main loop that the DMA transfer is finished.
    cortex_m::interrupt::free(|cs| {
        DMA_DONE_FLAG.borrow(cs).set(true);
    });
}

#[interrupt]
#[allow(non_snake_case)]
fn DMA_ACTIVE0() {
    // Notify the main loop that the DMA 0 is active now.
    cortex_m::interrupt::free(|cs| {
        DMA_ACTIVE_FLAG.borrow(cs).set(true);
    });
}
