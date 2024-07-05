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

use embedded_hal_nb::serial::Read;
use once_cell::sync::OnceCell;
use panic_rtt_target as _;
use va416xx_hal::{clock::Clocks, edac, pac, time::Hertz, wdt::Wdt};

const EXTCLK_FREQ: u32 = 40_000_000;
const COBS_FRAME_SEPARATOR: u8 = 0x0;
const MAX_PACKET_SIZE: usize = 1024;
const MAX_FRAME_SIZE: usize = cobs::max_encoding_length(MAX_PACKET_SIZE);
const UART_BAUDRATE: u32 = 115200;

const BOOT_NVM_MEMORY_ID: u8 = 1;

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

static CLOCKS: OnceCell<Clocks> = OnceCell::new();

#[rtic::app(device = pac, dispatchers = [U1, U2, U3])]
mod app {
    use super::*;
    use embedded_hal_nb::nb;
    use panic_rtt_target as _;
    use rtic_monotonics::systick::ExtU32;
    use rtic_monotonics::systick::Systick;
    use rtic_sync::{
        channel::{Receiver, Sender},
        make_channel,
    };
    use rtt_target::rprintln;
    use spacepackets::ecss::PusServiceId;
    use spacepackets::ecss::{tc::PusTcReader, PusPacket};
    use va416xx_hal::{
        clock::ClkgenExt,
        edac,
        gpio::PinsG,
        nvm::Nvm,
        pac,
        uart::{self, Uart},
    };

    use crate::{setup_edac, EXTCLK_FREQ};

    #[derive(Default, Debug, Copy, Clone, PartialEq, Eq)]
    pub enum CobsReaderStates {
        #[default]
        WaitingForStart,
        WatingForEnd,
        FrameOverflow,
    }

    #[local]
    struct Local {
        uart_rx: uart::Rx<pac::Uart0>,
        uart_tx: uart::Tx<pac::Uart0>,
        cobs_reader_state: CobsReaderStates,
        tc_tx: TcTx,
        tc_rx: TcRx,
        rom_spi: Option<pac::Spi3>,
    }

    #[shared]
    struct Shared {
        decode_buffer_busy: bool,
        decode_buf: [u8; MAX_PACKET_SIZE],
    }

    pub type TcTx = Sender<'static, usize, 2>;
    pub type TcRx = Receiver<'static, usize, 2>;

    #[init]
    fn init(mut cx: init::Context) -> (Shared, Local) {
        //rtt_init_default!();
        rtt_log::init();
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

        let gpiob = PinsG::new(&mut cx.device.sysconfig, cx.device.portg);
        let tx = gpiob.pg0.into_funsel_1();
        let rx = gpiob.pg1.into_funsel_1();

        let uart0 = Uart::new(
            cx.device.uart0,
            (tx, rx),
            Hertz::from_raw(UART_BAUDRATE),
            &mut cx.device.sysconfig,
            &clocks,
        );
        let (tx, rx) = uart0.split();
        let (tc_tx, tc_rx) = make_channel!(usize, 2);

        Systick::start(cx.core.SYST, clocks.sysclk().raw(), systick_mono_token);
        CLOCKS.set(clocks).unwrap();
        (
            Shared {
                decode_buffer_busy: false,
                decode_buf: [0; MAX_PACKET_SIZE],
            },
            Local {
                uart_rx: rx,
                uart_tx: tx,
                cobs_reader_state: CobsReaderStates::default(),
                tc_tx,
                tc_rx,
                rom_spi: Some(cx.device.spi3),
            },
        )
    }

    // `shared` cannot be accessed from this context
    #[idle]
    fn idle(_cx: idle::Context) -> ! {
        #[allow(clippy::empty_loop)]
        loop {}
    }

    #[task(
        priority = 3,
        local=[
            read_buf: [u8;MAX_FRAME_SIZE] = [0; MAX_FRAME_SIZE],
            uart_rx,
            cobs_reader_state,
            tc_tx
        ],
        shared=[decode_buffer_busy, decode_buf]
    )]
    async fn uart_reader_task(mut cx: uart_reader_task::Context) {
        let mut current_idx = 0;
        loop {
            match nb::block!(cx.local.uart_rx.read()) {
                Ok(byte) => match cx.local.cobs_reader_state {
                    CobsReaderStates::WaitingForStart => {
                        if byte == COBS_FRAME_SEPARATOR {
                            *cx.local.cobs_reader_state = CobsReaderStates::WatingForEnd;
                        }
                    }
                    CobsReaderStates::WatingForEnd => {
                        if byte == COBS_FRAME_SEPARATOR {
                            let mut sending_failed = false;
                            let mut decoding_error = false;
                            let mut decode_buffer_busy = false;
                            cx.shared.decode_buffer_busy.lock(|busy| {
                                if *busy {
                                    decode_buffer_busy = true;
                                } else {
                                    cx.shared.decode_buf.lock(|buf| {
                                        match cobs::decode(&cx.local.read_buf[..current_idx], buf) {
                                            Ok(packet_len) => {
                                                if cx.local.tc_tx.try_send(packet_len).is_err() {
                                                    sending_failed = true;
                                                }
                                            }
                                            Err(_) => {
                                                decoding_error = true;
                                            }
                                        }
                                    });
                                    *busy = true;
                                }
                            });
                            if sending_failed {
                                log::warn!("sending TC packet failed, queue full");
                            }
                            if decoding_error {
                                log::warn!("decoding error");
                            }
                            if decode_buffer_busy {
                                log::warn!("decode buffer busy. data arriving too fast");
                            }
                        } else if current_idx >= cx.local.read_buf.len() {
                            *cx.local.cobs_reader_state = CobsReaderStates::FrameOverflow;
                        } else {
                            cx.local.read_buf[current_idx] = byte;
                            current_idx += 1;
                        }
                    }
                    CobsReaderStates::FrameOverflow => {
                        if byte == COBS_FRAME_SEPARATOR {
                            *cx.local.cobs_reader_state = CobsReaderStates::WaitingForStart;
                            current_idx = 0;
                        }
                    }
                },
                Err(e) => match e {
                    uart::Error::Overrun => todo!(),
                    uart::Error::FramingError => todo!(),
                    uart::Error::ParityError => todo!(),
                    uart::Error::BreakCondition => todo!(),
                    uart::Error::TransferPending => todo!(),
                    uart::Error::BufferTooShort => todo!(),
                },
            }
        }
    }

    #[task(
        priority = 2,
        local=[
            read_buf: [u8;MAX_FRAME_SIZE] = [0; MAX_FRAME_SIZE],
            tc_rx,
            rom_spi
        ],
        shared=[decode_buffer_busy, decode_buf]
    )]
    async fn pus_tc_handler(mut cx: pus_tc_handler::Context) {
        loop {
            let _ = cx.local.tc_rx.recv().await;
            // We still copy the data to a local buffer, so the exchange buffer can already be used
            // for the next packet / decode process.
            cx.shared
                .decode_buf
                .lock(|buf| cx.local.read_buf[0..buf.len()].copy_from_slice(buf));
            cx.shared.decode_buffer_busy.lock(|busy| *busy = false);
            match PusTcReader::new(cx.local.read_buf) {
                Ok((pus_tc, _)) => {
                    if pus_tc.service() == PusServiceId::Test as u8 && pus_tc.subservice() == 1 {
                        log::info!("Received ping TC");
                    } else if pus_tc.service() == PusServiceId::MemoryManagement as u8 {
                        // Raw memory write TC
                        if pus_tc.subservice() == 2 {
                            let app_data = pus_tc.app_data();
                            if app_data.len() < 10 {
                                log::warn!(
                                    "app data for raw memory write is too short: {}",
                                    app_data.len()
                                );
                            }
                            let memory_id = app_data[0];
                            if memory_id != BOOT_NVM_MEMORY_ID {
                                log::warn!("memory ID {} not supported", memory_id);
                                // TODO: Error reporting
                                return;
                            }
                            let offset = u32::from_be_bytes(app_data[2..6].try_into().unwrap());
                            let data_len = u32::from_be_bytes(app_data[6..10].try_into().unwrap());
                            if 10 + data_len as usize > app_data.len() {
                                log::warn!(
                                    "invalid data length {} for raw mem write detected",
                                    data_len
                                );
                                // TODO: Error reporting
                                return;
                            }
                            let data = &app_data[10..10 + data_len as usize];
                            log::info!("writing {} bytes at offset {} to NVM", data_len, offset);
                            // Safety: We only use this for NVM handling and we only do NVM
                            // handling here.
                            let mut sys_cfg = unsafe { pac::Sysconfig::steal() };
                            let nvm = Nvm::new(
                                &mut sys_cfg,
                                cx.local.rom_spi.take().unwrap(),
                                CLOCKS.get().as_ref().unwrap(),
                            );
                            nvm.write_data(offset, data);
                            *cx.local.rom_spi = Some(nvm.release(&mut sys_cfg));
                            log::info!("NVM operation done");
                        }
                    }
                }
                Err(e) => {
                    log::warn!("PUS TC error: {}", e);
                },
            }
        }
    }

    #[task(
        priority = 1,
        local=[
            uart_tx,
        ],
        shared=[]
    )]
    async fn pus_tm_tx_handler(_cx: pus_tm_tx_handler::Context) {
        loop {
            Systick::delay(500.millis()).await;
        }
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
