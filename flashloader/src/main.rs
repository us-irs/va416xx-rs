//! Vorago flashloader which can be used to flash image A and image B via a simple
//! low-level CCSDS memory interface via a UART wire.
//!
//! This flash loader can be used after the bootloader was flashed to flash the images.
//! You can also use this as an starting application for a software update mechanism.
//!
//! Bootloader memory map
//!
//! * <0x0>     Bootloader start                         <code up to 0x3FFE bytes>
//! * <0x3FFE>  Bootloader CRC                           <halfword>
//! * <0x4000>  App image A start                        <code up to 0x1DFFC (~120K) bytes>
//! * <0x21FFC> App image A CRC check length             <halfword>
//! * <0x21FFE> App image A CRC check value              <halfword>
//! * <0x22000> App image B start                        <code up to 0x1DFFC (~120K) bytes>
//! * <0x3FFFC> App image B CRC check length             <halfword>
//! * <0x3FFFE> App image B CRC check value              <halfword>
//! * <0x40000>                                          <end>
#![no_main]
#![no_std]

use crc::{Crc, CRC_16_IBM_3740};
use panic_rtt_target as _;
use va416xx_hal::{
    edac,
    pac::{self},
    time::Hertz,
    wdt::Wdt,
};

const EXTCLK_FREQ: u32 = 40_000_000;
const WITH_WDT: bool = true;
const WDT_FREQ_MS: u32 = 50;
const DEBUG_PRINTOUTS: bool = true;

// Important bootloader addresses and offsets, vector table information.

const BOOTLOADER_START_ADDR: u32 = 0x0;
const BOOTLOADER_END_ADDR: u32 = 0x4000;
const BOOTLOADER_CRC_ADDR: u32 = 0x3FFE;
const APP_A_START_ADDR: u32 = 0x4000;
pub const APP_A_END_ADDR: u32 = 0x22000;
// The actual size of the image which is relevant for CRC calculation.
const APP_A_SIZE_ADDR: u32 = 0x21FF8;
const APP_A_CRC_ADDR: u32 = 0x21FFC;
const APP_B_START_ADDR: u32 = 0x22000;
pub const APP_B_END_ADDR: u32 = 0x40000;
// The actual size of the image which is relevant for CRC calculation.
const APP_B_SIZE_ADDR: u32 = 0x3FFF8;
const APP_B_CRC_ADDR: u32 = 0x3FFFC;
pub const APP_IMG_SZ: u32 = 0x1E000;

pub const VECTOR_TABLE_OFFSET: u32 = 0x0;
pub const VECTOR_TABLE_LEN: u32 = 0x350;
pub const RESET_VECTOR_OFFSET: u32 = 0x4;

const CRC_ALGO: Crc<u16> = Crc::<u16>::new(&CRC_16_IBM_3740);

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum AppSel {
    A,
    B,
}

pub trait WdtInterface {
    fn feed(&self);
}

pub struct OptWdt(Option<Wdt>);

impl WdtInterface for OptWdt {
    fn feed(&self) {
        if self.0.is_some() {
            self.0.as_ref().unwrap().feed();
        }
    }
}

#[rtic::app(device = pac)]
mod app {
    use super::*;
    use panic_rtt_target as _;
    use rtic_monotonics::systick::Systick;
    use rtt_target::{rprintln, rtt_init_default};
    use va416xx_hal::{clock::ClkgenExt, edac, pac};

    use crate::{setup_edac, EXTCLK_FREQ};

    #[local]
    struct Local {}

    #[shared]
    struct Shared {}

    #[init]
    fn init(mut cx: init::Context) -> (Shared, Local) {
        rtt_init_default!();
        rprintln!("-- Vorago flashloader --");
        // Initialize the systick interrupt & obtain the token to prove that we did
        let systick_mono_token = rtic_monotonics::create_systick_token!();
        // Use the external clock connected to XTAL_N.
        let clocks = cx
            .device
            .clkgen
            .constrain()
            .xtal_n_clk_with_src_freq(Hertz::from_raw(EXTCLK_FREQ))
            .freeze(&mut cx.device.sysconfig)
            .unwrap();
        setup_edac(&mut cx.device.sysconfig);

        Systick::start(cx.core.SYST, clocks.sysclk().raw(), systick_mono_token);
        (Shared {}, Local {})
    }

    // `shared` cannot be accessed from this context
    #[idle]
    fn idle(_cx: idle::Context) -> ! {
        #[allow(clippy::empty_loop)]
        loop {}
    }

    #[task(binds = EDAC_SBE, priority = 1)]
    fn edac_sbe_isr(_cx: edac_sbe_isr::Context) {
        // TODO: Send some command via UART for notification purposes. Also identify the problematic
        // memory.
        edac::clear_sbe_irq();
    }

    #[task(binds = EDAC_MBE, priority = 1)]
    fn edac_mbe_isr(_cx: edac_mbe_isr::Context) {
        // TODO: Send some command via UART for notification purposes.
        edac::clear_mbe_irq();
        // TODO: Reset like the vorago example?
    }

    #[task(binds = WATCHDOG, priority = 1)]
    fn watchdog_isr(_cx: watchdog_isr::Context) {
        let wdt = unsafe { pac::WatchDog::steal() };
        // Clear interrupt.
        wdt.wdogintclr().write(|w| unsafe { w.bits(1) });
    }
}

fn setup_edac(syscfg: &mut pac::Sysconfig) {
    // The scrub values are based on the Vorago provided bootloader.
    edac::enable_rom_scrub(syscfg, 125);
    edac::enable_ram0_scrub(syscfg, 1000);
    edac::enable_ram1_scrub(syscfg, 1000);
    edac::enable_sbe_irq();
    edac::enable_mbe_irq();
}
