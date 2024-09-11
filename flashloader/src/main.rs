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

const MAX_TC_SIZE: usize = 1024;
const MAX_TC_FRAME_SIZE: usize = cobs::max_encoding_length(MAX_TC_SIZE);

const MAX_TM_SIZE: usize = 128;
const MAX_TM_FRAME_SIZE: usize = cobs::max_encoding_length(MAX_TM_SIZE);

const UART_BAUDRATE: u32 = 115200;
const SERIAL_RX_WIRETAPPING: bool = false;
const COBS_RX_DEBUGGING: bool = false;

const BOOT_NVM_MEMORY_ID: u8 = 1;

pub enum ActionId {
    CorruptImageA = 128,
    CorruptImageB = 129,
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

use once_cell::sync::Lazy;
use ringbuf::{
    traits::{Consumer, Observer, Producer, SplitRef},
    CachingCons, StaticProd, StaticRb,
};

const BUF_RB_SIZE_TX: usize = 1024;
const SIZES_RB_SIZE_TX: usize = 16;

static mut BUF_RB_TX: Lazy<StaticRb<u8, BUF_RB_SIZE_TX>> =
    Lazy::new(StaticRb::<u8, BUF_RB_SIZE_TX>::default);
static mut SIZES_RB_TX: Lazy<StaticRb<usize, SIZES_RB_SIZE_TX>> =
    Lazy::new(StaticRb::<usize, SIZES_RB_SIZE_TX>::default);

pub struct DataProducer<const BUF_SIZE: usize, const SIZES_LEN: usize> {
    pub buf_prod: StaticProd<'static, u8, BUF_SIZE>,
    pub sizes_prod: StaticProd<'static, usize, SIZES_LEN>,
}

pub struct DataConsumer<const BUF_SIZE: usize, const SIZES_LEN: usize> {
    pub buf_cons: CachingCons<&'static StaticRb<u8, BUF_SIZE>>,
    pub sizes_cons: CachingCons<&'static StaticRb<usize, SIZES_LEN>>,
}

static CLOCKS: OnceCell<Clocks> = OnceCell::new();

pub const APP_A_START_ADDR: u32 = 0x4000;
pub const APP_A_END_ADDR: u32 = 0x22000;
pub const APP_B_START_ADDR: u32 = 0x22000;
pub const APP_B_END_ADDR: u32 = 0x40000;

#[rtic::app(device = pac, dispatchers = [U1, U2, U3])]
mod app {
    use super::*;
    use cortex_m::asm;
    use embedded_hal_nb::nb;
    use embedded_io::Write;
    use panic_rtt_target as _;
    use rtic::Mutex;
    use rtic_monotonics::systick::prelude::*;
    use rtic_sync::{
        channel::{Receiver, Sender},
        make_channel,
    };
    use rtt_target::rprintln;
    use satrs::pus::verification::VerificationReportCreator;
    use spacepackets::ecss::PusServiceId;
    use spacepackets::ecss::{
        tc::PusTcReader, tm::PusTmCreator, EcssEnumU8, PusPacket, WritablePusPacket,
    };
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
        tx_cons: DataConsumer<BUF_RB_SIZE_TX, SIZES_RB_SIZE_TX>,
        verif_reporter: VerificationReportCreator,
    }

    #[shared]
    struct Shared {
        decode_buffer_busy: bool,
        decode_buf: [u8; MAX_TC_SIZE],
        tx_prod: DataProducer<BUF_RB_SIZE_TX, SIZES_RB_SIZE_TX>,
    }

    pub type TcTx = Sender<'static, usize, 2>;
    pub type TcRx = Receiver<'static, usize, 2>;

    rtic_monotonics::systick_monotonic!(Mono, 10_000);

    #[init]
    fn init(mut cx: init::Context) -> (Shared, Local) {
        //rtt_init_default!();
        rtt_log::init();
        rprintln!("-- Vorago flashloader --");
        // Initialize the systick interrupt & obtain the token to prove that we did
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

        let verif_reporter = VerificationReportCreator::new(0).unwrap();

        let (buf_prod, buf_cons) = unsafe { BUF_RB_TX.split_ref() };
        let (sizes_prod, sizes_cons) = unsafe { SIZES_RB_TX.split_ref() };

        Mono::start(cx.core.SYST, clocks.sysclk().raw());
        CLOCKS.set(clocks).unwrap();
        pus_tc_handler::spawn().unwrap();
        uart_reader_task::spawn().unwrap();
        pus_tm_tx_handler::spawn().unwrap();
        (
            Shared {
                decode_buffer_busy: false,
                decode_buf: [0; MAX_TC_SIZE],
                tx_prod: DataProducer {
                    buf_prod,
                    sizes_prod,
                },
            },
            Local {
                uart_rx: rx,
                uart_tx: tx,
                cobs_reader_state: CobsReaderStates::default(),
                tc_tx,
                tc_rx,
                rom_spi: Some(cx.device.spi3),
                tx_cons: DataConsumer {
                    buf_cons,
                    sizes_cons,
                },
                verif_reporter,
            },
        )
    }

    // `shared` cannot be accessed from this context
    #[idle]
    fn idle(_cx: idle::Context) -> ! {
        loop {
            asm::nop();
        }
    }

    #[task(
        priority = 4,
        local=[
            read_buf: [u8;MAX_TC_FRAME_SIZE] = [0; MAX_TC_FRAME_SIZE],
            uart_rx,
            cobs_reader_state,
            tc_tx
        ],
        shared=[decode_buffer_busy, decode_buf]
    )]
    async fn uart_reader_task(mut cx: uart_reader_task::Context) {
        let mut current_idx = 0;
        loop {
            match cx.local.uart_rx.read() {
                Ok(byte) => {
                    if SERIAL_RX_WIRETAPPING {
                        log::debug!("RX Byte: 0x{:x?}", byte);
                    }
                    handle_single_rx_byte(&mut cx, byte, &mut current_idx)
                }
                Err(e) => {
                    match e {
                        nb::Error::Other(e) => {
                            log::warn!("UART error: {:?}", e);
                            match e {
                                uart::Error::Overrun => {
                                    cx.local.uart_rx.clear_fifo();
                                }
                                uart::Error::FramingError => (),
                                uart::Error::ParityError => (),
                                uart::Error::BreakCondition => (),
                                uart::Error::TransferPending => (),
                                uart::Error::BufferTooShort => (),
                            }
                        }
                        nb::Error::WouldBlock => {
                            // Delay for a short period before polling again.
                            Mono::delay(400.micros()).await;
                        }
                    }
                }
            }
        }
    }

    fn handle_single_rx_byte(
        cx: &mut uart_reader_task::Context,
        byte: u8,
        current_idx: &mut usize,
    ) {
        match cx.local.cobs_reader_state {
            CobsReaderStates::WaitingForStart => {
                if byte == COBS_FRAME_SEPARATOR {
                    if COBS_RX_DEBUGGING {
                        log::debug!("COBS start marker detected");
                    }
                    *cx.local.cobs_reader_state = CobsReaderStates::WatingForEnd;
                    *current_idx = 0;
                }
            }
            CobsReaderStates::WatingForEnd => {
                if byte == COBS_FRAME_SEPARATOR {
                    if COBS_RX_DEBUGGING {
                        log::debug!("COBS end marker detected");
                    }
                    let mut sending_failed = false;
                    let mut decoding_error = false;
                    let mut decode_buffer_busy = false;
                    cx.shared.decode_buffer_busy.lock(|busy| {
                        if *busy {
                            decode_buffer_busy = true;
                        } else {
                            cx.shared.decode_buf.lock(|buf| {
                                match cobs::decode(&cx.local.read_buf[..*current_idx], buf) {
                                    Ok(packet_len) => {
                                        if COBS_RX_DEBUGGING {
                                            log::debug!(
                                                "COBS decoded packet with length {}",
                                                packet_len
                                            );
                                        }
                                        if cx.local.tc_tx.try_send(packet_len).is_err() {
                                            sending_failed = true;
                                        }
                                        *busy = true;
                                    }
                                    Err(_) => {
                                        decoding_error = true;
                                    }
                                }
                            });
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
                    *cx.local.cobs_reader_state = CobsReaderStates::WaitingForStart;
                } else if *current_idx >= cx.local.read_buf.len() {
                    *cx.local.cobs_reader_state = CobsReaderStates::FrameOverflow;
                } else {
                    cx.local.read_buf[*current_idx] = byte;
                    *current_idx += 1;
                }
            }
            CobsReaderStates::FrameOverflow => {
                if byte == COBS_FRAME_SEPARATOR {
                    *cx.local.cobs_reader_state = CobsReaderStates::WaitingForStart;
                    *current_idx = 0;
                }
            }
        }
    }

    #[task(
        priority = 2,
        local=[
            read_buf: [u8;MAX_TC_FRAME_SIZE] = [0; MAX_TC_FRAME_SIZE],
            src_data_buf: [u8; 16] = [0; 16],
            verif_buf: [u8; 32] = [0; 32],
            tc_rx,
            rom_spi,
            verif_reporter
        ],
        shared=[decode_buffer_busy, decode_buf, tx_prod]
    )]
    async fn pus_tc_handler(mut cx: pus_tc_handler::Context) {
        loop {
            let packet_len = cx.local.tc_rx.recv().await.expect("all senders down");
            log::info!(target: "TC Handler", "received packet with length {}", packet_len);
            // We still copy the data to a local buffer, so the exchange buffer can already be used
            // for the next packet / decode process.
            cx.shared
                .decode_buf
                .lock(|buf| cx.local.read_buf[0..buf.len()].copy_from_slice(buf));
            cx.shared.decode_buffer_busy.lock(|busy| *busy = false);
            match PusTcReader::new(cx.local.read_buf) {
                Ok((pus_tc, _)) => {
                    let mut write_and_send = |tm: &PusTmCreator| {
                        let written_size = tm.write_to_bytes(cx.local.verif_buf).unwrap();
                        cx.shared.tx_prod.lock(|prod| {
                            prod.sizes_prod.try_push(tm.len_written()).unwrap();
                            prod.buf_prod
                                .push_slice(&cx.local.verif_buf[0..written_size]);
                        });
                    };
                    let token = cx.local.verif_reporter.add_tc(&pus_tc);
                    let (tm, accepted_token) = cx
                        .local
                        .verif_reporter
                        .acceptance_success(cx.local.src_data_buf, token, 0, 0, &[])
                        .expect("acceptance success failed");
                    write_and_send(&tm);

                    let (tm, started_token) = cx
                        .local
                        .verif_reporter
                        .start_success(cx.local.src_data_buf, accepted_token, 0, 0, &[])
                        .expect("acceptance success failed");
                    write_and_send(&tm);

                    if pus_tc.service() == PusServiceId::Action as u8 {
                        let mut corrupt_image = |base_addr: u32| {
                            // Safety: We only use this for NVM handling and we only do NVM
                            // handling here.
                            let mut sys_cfg = unsafe { pac::Sysconfig::steal() };
                            let nvm = Nvm::new(
                                &mut sys_cfg,
                                cx.local.rom_spi.take().unwrap(),
                                CLOCKS.get().as_ref().unwrap(),
                            );
                            let mut buf = [0u8; 4];
                            nvm.read_data(base_addr + 32, &mut buf);
                            buf[0] += 1;
                            nvm.write_data(base_addr + 32, &buf);
                            *cx.local.rom_spi = Some(nvm.release(&mut sys_cfg));
                            let tm = cx
                                .local
                                .verif_reporter
                                .completion_success(cx.local.src_data_buf, started_token, 0, 0, &[])
                                .expect("completion success failed");
                            write_and_send(&tm);
                        };
                        if pus_tc.subservice() == ActionId::CorruptImageA as u8 {
                            rprintln!("corrupting App Image A");
                            corrupt_image(APP_A_START_ADDR);
                        }
                        if pus_tc.subservice() == ActionId::CorruptImageB as u8 {
                            rprintln!("corrupting App Image B");
                            corrupt_image(APP_B_START_ADDR);
                        }
                    }
                    if pus_tc.service() == PusServiceId::Test as u8 && pus_tc.subservice() == 1 {
                        log::info!(target: "TC Handler", "received ping TC");
                    } else if pus_tc.service() == PusServiceId::MemoryManagement as u8 {
                        let tm = cx
                            .local
                            .verif_reporter
                            .step_success(
                                cx.local.src_data_buf,
                                &started_token,
                                0,
                                0,
                                &[],
                                EcssEnumU8::new(0),
                            )
                            .expect("step success failed");
                        write_and_send(&tm);
                        // Raw memory write TC
                        if pus_tc.subservice() == 2 {
                            let app_data = pus_tc.app_data();
                            if app_data.len() < 10 {
                                log::warn!(
                                    target: "TC Handler",
                                    "app data for raw memory write is too short: {}",
                                    app_data.len()
                                );
                            }
                            let memory_id = app_data[0];
                            if memory_id != BOOT_NVM_MEMORY_ID {
                                log::warn!(target: "TC Handler", "memory ID {} not supported", memory_id);
                                // TODO: Error reporting
                                return;
                            }
                            let offset = u32::from_be_bytes(app_data[2..6].try_into().unwrap());
                            let data_len = u32::from_be_bytes(app_data[6..10].try_into().unwrap());
                            if 10 + data_len as usize > app_data.len() {
                                log::warn!(
                                    target: "TC Handler",
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
                            let tm = cx
                                .local
                                .verif_reporter
                                .completion_success(cx.local.src_data_buf, started_token, 0, 0, &[])
                                .expect("completion success failed");
                            write_and_send(&tm);
                            log::info!("NVM operation done");
                        }
                    }
                }
                Err(e) => {
                    log::warn!("PUS TC error: {}", e);
                }
            }
        }
    }

    #[task(
        priority = 1,
        local=[
            read_buf: [u8;MAX_TM_SIZE] = [0; MAX_TM_SIZE],
            encoded_buf: [u8;MAX_TM_FRAME_SIZE] = [0; MAX_TM_FRAME_SIZE],
            uart_tx,
            tx_cons,
        ],
        shared=[]
    )]
    async fn pus_tm_tx_handler(cx: pus_tm_tx_handler::Context) {
        loop {
            while cx.local.tx_cons.sizes_cons.occupied_len() > 0 {
                let next_size = cx.local.tx_cons.sizes_cons.try_pop().unwrap();
                cx.local
                    .tx_cons
                    .buf_cons
                    .pop_slice(&mut cx.local.read_buf[0..next_size]);
                cx.local.encoded_buf[0] = 0;
                let send_size = cobs::encode(
                    &cx.local.read_buf[0..next_size],
                    &mut cx.local.encoded_buf[1..],
                );
                cx.local.encoded_buf[send_size + 1] = 0;
                cx.local
                    .uart_tx
                    .write(&cx.local.encoded_buf[0..send_size + 2])
                    .unwrap();
                Mono::delay(2.millis()).await;
            }
            Mono::delay(30.millis()).await;
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
