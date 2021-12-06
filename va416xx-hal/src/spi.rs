//! API for the SPI peripheral
//!
//! ## Examples
//!
//! - [Blocking SPI example](https://egit.irs.uni-stuttgart.de/rust/va416xx-rs/src/branch/main/examples/simple/examples/spi.rs)
use core::{convert::Infallible, marker::PhantomData, ops::Deref};

use embedded_hal::spi::Mode;

use crate::{
    clock::PeripheralSelect,
    gpio::{
        AltFunc1, AltFunc2, AltFunc3, Pin, PA0, PA1, PA2, PA3, PA4, PA5, PA6, PA7, PA8, PA9, PB0,
        PB1, PB10, PB11, PB12, PB13, PB14, PB15, PB2, PB3, PB4, PB5, PB6, PB7, PB8, PB9, PC0, PC1,
        PC10, PC11, PC7, PC8, PC9, PE10, PE11, PE12, PE13, PE14, PE15, PE5, PE6, PE7, PE8, PE9,
        PF0, PF1, PF2, PF3, PF4, PF5, PF6, PF7, PG2, PG3, PG4,
    },
    pac,
    time::Hertz,
    typelevel::{NoneT, Sealed},
};

//==================================================================================================
// Defintions
//==================================================================================================

// FIFO has a depth of 16.
const FILL_DEPTH: usize = 12;

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum HwChipSelectId {
    Id0 = 0,
    Id1 = 1,
    Id2 = 2,
    Id3 = 3,
    Id4 = 4,
    Id5 = 5,
    Id6 = 6,
    Id7 = 7,
    Invalid = 0xff,
}

#[derive(Debug)]
pub enum SpiId {
    Spi0,
    Spi1,
    Spi2,
    Spi3,
    Invalid,
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum WordSize {
    OneBit = 0x00,
    FourBits = 0x03,
    EightBits = 0x07,
    SixteenBits = 0x0f,
}

//==================================================================================================
// Pin type definitions
//==================================================================================================

pub trait PinSck<SPI>: Sealed {}
pub trait PinMosi<SPI>: Sealed {}
pub trait PinMiso<SPI>: Sealed {}

pub trait HwCsProvider: Sealed {
    const CS_ID: HwChipSelectId;
    const SPI_ID: SpiId;
}

pub trait OptionalHwCs<Spi>: HwCsProvider + Sealed {}

macro_rules! hw_cs_pins {
    ($SPIx:path, $portId: path:
        $(
            ($PXx:ident, $AFx:ident, $HwCsIdent:path, $typedef:ident),
        )+
    ) => {
        $(
            impl HwCsProvider for Pin<$PXx, $AFx> {
                const CS_ID: HwChipSelectId = $HwCsIdent;
                const SPI_ID: SpiId = $portId;
            }
            impl OptionalHwCs<$SPIx> for Pin<$PXx, $AFx> {}
            pub type $typedef = Pin<$PXx, $AFx>;
        )+
    };
}

impl HwCsProvider for NoneT {
    const CS_ID: HwChipSelectId = HwChipSelectId::Invalid;
    const SPI_ID: SpiId = SpiId::Invalid;
}

impl OptionalHwCs<pac::Spi0> for NoneT {}
impl OptionalHwCs<pac::Spi1> for NoneT {}
impl OptionalHwCs<pac::Spi2> for NoneT {}
impl OptionalHwCs<pac::Spi3> for NoneT {}

// SPI 0

impl PinSck<pac::Spi0> for Pin<PB15, AltFunc1> {}
impl PinMosi<pac::Spi0> for Pin<PC1, AltFunc1> {}
impl PinMiso<pac::Spi0> for Pin<PC0, AltFunc1> {}

// SPI 1

impl PinSck<pac::Spi1> for Pin<PB8, AltFunc3> {}
impl PinMosi<pac::Spi1> for Pin<PB10, AltFunc3> {}
impl PinMiso<pac::Spi1> for Pin<PB9, AltFunc3> {}

impl PinSck<pac::Spi1> for Pin<PC9, AltFunc2> {}
impl PinMosi<pac::Spi1> for Pin<PC11, AltFunc2> {}
impl PinMiso<pac::Spi1> for Pin<PC10, AltFunc2> {}

impl PinSck<pac::Spi1> for Pin<PG3, AltFunc2> {}
impl PinMiso<pac::Spi1> for Pin<PG4, AltFunc2> {}

impl PinSck<pac::Spi1> for Pin<PE13, AltFunc2> {}
impl PinMosi<pac::Spi1> for Pin<PE15, AltFunc2> {}
impl PinMiso<pac::Spi1> for Pin<PE14, AltFunc2> {}

impl PinSck<pac::Spi1> for Pin<PF3, AltFunc1> {}
impl PinMosi<pac::Spi1> for Pin<PF5, AltFunc1> {}
impl PinMiso<pac::Spi1> for Pin<PF4, AltFunc1> {}

// SPI 2

impl PinSck<pac::Spi2> for Pin<PA5, AltFunc2> {}
impl PinMosi<pac::Spi2> for Pin<PA7, AltFunc2> {}
impl PinMiso<pac::Spi2> for Pin<PA6, AltFunc2> {}

impl PinSck<pac::Spi2> for Pin<PF5, AltFunc2> {}
impl PinMosi<pac::Spi2> for Pin<PF7, AltFunc2> {}
impl PinMiso<pac::Spi2> for Pin<PF6, AltFunc2> {}

// SPI3 is shared with the ROM SPI pins and has its own dedicated pins.

// SPI 0 HW CS pins

hw_cs_pins!(
    pac::Spi0, SpiId::Spi0:
    (PB14, AltFunc1, HwChipSelectId::Id0, HwCs0Spi0),
    (PB13, AltFunc1, HwChipSelectId::Id1, HwCs1Spi0),
    (PB12, AltFunc1, HwChipSelectId::Id2, HwCs2Spi0),
    (PB11, AltFunc1, HwChipSelectId::Id3, HwCs3Spi0),
);

hw_cs_pins!(
    pac::Spi1, SpiId::Spi1:
    (PB7, AltFunc3, HwChipSelectId::Id0, HwCs0Spi1Pb),
    (PB6, AltFunc3, HwChipSelectId::Id1, HwCs1Spi1Pb),
    (PB5, AltFunc3, HwChipSelectId::Id2, HwCs2Spi1Pb),
    (PB4, AltFunc3, HwChipSelectId::Id3, HwCs3Spi1Pb),
    (PB3, AltFunc3, HwChipSelectId::Id4, HwCs4Spi1Pb),
    (PB2, AltFunc3, HwChipSelectId::Id5, HwCs5Spi1Pb),
    (PB1, AltFunc3, HwChipSelectId::Id6, HwCs6Spi1Pb),
    (PB0, AltFunc3, HwChipSelectId::Id7, HwCs7Spi1Pb),
    (PC8, AltFunc2, HwChipSelectId::Id0, HwCs0Spi1Pc),
    (PC7, AltFunc2, HwChipSelectId::Id1, HwCs1Spi1Pc),
    (PE12, AltFunc2, HwChipSelectId::Id0, HwCs0Spi1Pe),
    (PE11, AltFunc2, HwChipSelectId::Id1, HwCs1Spi1Pe),
    (PE10, AltFunc2, HwChipSelectId::Id2, HwCs2Spi1Pe),
    (PE9, AltFunc2, HwChipSelectId::Id3, HwCs3Spi1Pe),
    (PE8, AltFunc2, HwChipSelectId::Id4, HwCs4Spi1Pe),
    (PE7, AltFunc3, HwChipSelectId::Id5, HwCs5Spi1Pe),
    (PE6, AltFunc3, HwChipSelectId::Id6, HwCs6Spi1Pe),
    (PE5, AltFunc3, HwChipSelectId::Id7, HwCs7Spi1Pe),
    (PF2, AltFunc1, HwChipSelectId::Id0, HwCs0Spi1Pf),
    (PG2, AltFunc2, HwChipSelectId::Id0, HwCs0Spi1Pg),
);

hw_cs_pins!(
    pac::Spi2, SpiId::Spi2:
    (PA4, AltFunc2, HwChipSelectId::Id0, HwCs0Spi2Pa),
    (PA3, AltFunc2, HwChipSelectId::Id1, HwCs1Spi2Pa),
    (PA2, AltFunc2, HwChipSelectId::Id2, HwCs2Spi2Pa),
    (PA1, AltFunc2, HwChipSelectId::Id3, HwCs3Spi2Pa),
    (PA0, AltFunc2, HwChipSelectId::Id4, HwCs4Spi2Pa),
    (PA8, AltFunc2, HwChipSelectId::Id6, HwCs6Spi2Pa),
    (PA9, AltFunc2, HwChipSelectId::Id5, HwCs5Spi2Pa),
    (PF0, AltFunc2, HwChipSelectId::Id4, HwCs4Spi2Pf),
    (PF1, AltFunc2, HwChipSelectId::Id3, HwCs3Spi2Pf),
    (PF2, AltFunc2, HwChipSelectId::Id2, HwCs2Spi2Pf),
    (PF3, AltFunc2, HwChipSelectId::Id1, HwCs1Spi2Pf),
    (PF4, AltFunc2, HwChipSelectId::Id0, HwCs0Spi2Pf),
);

//==================================================================================================
// Config
//==================================================================================================

pub trait TransferConfigProvider {
    fn sod(&mut self, sod: bool);
    fn blockmode(&mut self, blockmode: bool);
    fn mode(&mut self, mode: Mode);
    fn frequency(&mut self, spi_clk: Hertz);
    fn hw_cs_id(&self) -> u8;
}

/// This struct contains all configuration parameter which are transfer specific
/// and might change for transfers to different SPI slaves
#[derive(Copy, Clone)]
pub struct TransferConfig<HwCs> {
    pub spi_clk: Hertz,
    pub mode: Mode,
    /// This only works if the Slave Output Disable (SOD) bit of the [`SpiConfig`] is set to
    /// false
    pub hw_cs: Option<HwCs>,
    pub sod: bool,
    /// If this is enabled, all data in the FIFO is transmitted in a single frame unless
    /// the BMSTOP bit is set on a dataword. A frame is defined as CSn being active for the
    /// duration of multiple data words
    pub blockmode: bool,
}

/// Type erased variant of the transfer configuration. This is required to avoid generics in
/// the SPI constructor.
pub struct ErasedTransferConfig {
    pub spi_clk: Hertz,
    pub mode: Mode,
    pub sod: bool,
    /// If this is enabled, all data in the FIFO is transmitted in a single frame unless
    /// the BMSTOP bit is set on a dataword. A frame is defined as CSn being active for the
    /// duration of multiple data words
    pub blockmode: bool,
    pub hw_cs: HwChipSelectId,
}

impl TransferConfig<NoneT> {
    pub fn new_no_hw_cs(spi_clk: impl Into<Hertz>, mode: Mode, blockmode: bool, sod: bool) -> Self {
        TransferConfig {
            spi_clk: spi_clk.into(),
            mode,
            hw_cs: None,
            sod,
            blockmode,
        }
    }
}

impl<HwCs: HwCsProvider> TransferConfig<HwCs> {
    pub fn new(
        spi_clk: impl Into<Hertz>,
        mode: Mode,
        hw_cs: Option<HwCs>,
        blockmode: bool,
        sod: bool,
    ) -> Self {
        TransferConfig {
            spi_clk: spi_clk.into(),
            mode,
            hw_cs,
            sod,
            blockmode,
        }
    }

    pub fn downgrade(self) -> ErasedTransferConfig {
        ErasedTransferConfig {
            spi_clk: self.spi_clk,
            mode: self.mode,
            sod: self.sod,
            blockmode: self.blockmode,
            hw_cs: HwCs::CS_ID,
        }
    }
}

impl<HwCs: HwCsProvider> TransferConfigProvider for TransferConfig<HwCs> {
    /// Slave Output Disable
    fn sod(&mut self, sod: bool) {
        self.sod = sod;
    }

    fn blockmode(&mut self, blockmode: bool) {
        self.blockmode = blockmode;
    }

    fn mode(&mut self, mode: Mode) {
        self.mode = mode;
    }

    fn frequency(&mut self, spi_clk: Hertz) {
        self.spi_clk = spi_clk;
    }

    fn hw_cs_id(&self) -> u8 {
        HwCs::CS_ID as u8
    }
}

#[derive(Default)]
/// Configuration options for the whole SPI bus. See Programmer Guide p.92 for more details
pub struct SpiConfig {
    /// Serial clock rate divider. Together with the CLKPRESCALE register, it determines
    /// the SPI clock rate in master mode. 0 by default. Specifying a higher value
    /// limits the maximum attainable SPI speed
    pub ser_clock_rate_div: u8,
    /// By default, configure SPI for master mode (ms == false)
    ms: bool,
    /// Slave output disable. Useful if separate GPIO pins or decoders are used for CS control
    pub slave_output_disable: bool,
    /// Loopback mode. If you use this, don't connect MISO to MOSI, they will be tied internally
    pub loopback_mode: bool,
    /// Enable Master Delayer Capture Mode. See Programmers Guide p.92 for more details
    pub master_delayer_capture: bool,
}

impl SpiConfig {
    pub fn loopback(mut self, enable: bool) -> Self {
        self.loopback_mode = enable;
        self
    }

    pub fn master_mode(mut self, master: bool) -> Self {
        self.ms = !master;
        self
    }

    pub fn slave_output_disable(mut self, sod: bool) -> Self {
        self.slave_output_disable = sod;
        self
    }
}

//==================================================================================================
// Word Size
//==================================================================================================

/// Configuration trait for the Word Size used by the SPI peripheral
pub trait WordProvider: Copy + Default + Into<u32> + TryFrom<u32> + 'static {
    const MASK: u32;
    fn word_reg() -> u8;
}

impl WordProvider for u8 {
    const MASK: u32 = 0xff;
    fn word_reg() -> u8 {
        0x07
    }
}

impl WordProvider for u16 {
    const MASK: u32 = 0xffff;
    fn word_reg() -> u8 {
        0x0f
    }
}

pub type SpiRegBlock = pac::spi0::RegisterBlock;

/// Common trait implemented by all PAC peripheral access structures. The register block
/// format is the same for all SPI blocks.
pub trait Instance: Deref<Target = SpiRegBlock> {
    const IDX: u8;
    const PERIPH_SEL: PeripheralSelect;

    fn ptr() -> *const SpiRegBlock;
}

impl Instance for pac::Spi0 {
    const IDX: u8 = 0;
    const PERIPH_SEL: PeripheralSelect = PeripheralSelect::Spi0;

    fn ptr() -> *const SpiRegBlock {
        Self::ptr()
    }
}

impl Instance for pac::Spi1 {
    const IDX: u8 = 1;
    const PERIPH_SEL: PeripheralSelect = PeripheralSelect::Spi1;

    fn ptr() -> *const SpiRegBlock {
        Self::ptr()
    }
}

impl Instance for pac::Spi2 {
    const IDX: u8 = 2;
    const PERIPH_SEL: PeripheralSelect = PeripheralSelect::Spi2;

    fn ptr() -> *const SpiRegBlock {
        Self::ptr()
    }
}

//==================================================================================================
// Spi
//==================================================================================================

pub struct SpiBase<SpiInstance, Word = u8> {
    spi: SpiInstance,
    cfg: SpiConfig,
    apb1_clk: Hertz,
    /// Fill word for read-only SPI transactions.
    pub fill_word: Word,
    blockmode: bool,
    word: PhantomData<Word>,
}

pub struct Spi<SpiInstance, Pins, Word = u8> {
    inner: SpiBase<SpiInstance, Word>,
    pins: Pins,
}

fn mode_to_cpo_cph_bit(mode: embedded_hal::spi::Mode) -> (bool, bool) {
    match mode {
        embedded_hal::spi::MODE_0 => (false, false),
        embedded_hal::spi::MODE_1 => (false, true),
        embedded_hal::spi::MODE_2 => (true, false),
        embedded_hal::spi::MODE_3 => (true, true),
    }
}

impl<SpiInstance: Instance, Word: WordProvider> SpiBase<SpiInstance, Word>
where
    <Word as TryFrom<u32>>::Error: core::fmt::Debug,
{
    #[inline]
    pub fn cfg_clock(&mut self, spi_clk: impl Into<Hertz>) {
        let clk_prescale =
            self.apb1_clk.raw() / (spi_clk.into().raw() * (self.cfg.ser_clock_rate_div as u32 + 1));
        self.spi
            .clkprescale()
            .write(|w| unsafe { w.bits(clk_prescale) });
    }

    #[inline]
    pub fn cfg_mode(&mut self, mode: Mode) {
        let (cpo_bit, cph_bit) = mode_to_cpo_cph_bit(mode);
        self.spi.ctrl0().modify(|_, w| {
            w.spo().bit(cpo_bit);
            w.sph().bit(cph_bit)
        });
    }

    #[inline]
    pub fn clear_tx_fifo(&self) {
        self.spi.fifo_clr().write(|w| w.txfifo().set_bit());
    }

    #[inline]
    pub fn clear_rx_fifo(&self) {
        self.spi.fifo_clr().write(|w| w.rxfifo().set_bit());
    }

    #[inline]
    pub fn perid(&self) -> u32 {
        self.spi.perid().read().bits()
    }

    #[inline]
    pub fn cfg_hw_cs(&mut self, hw_cs: HwChipSelectId) {
        if hw_cs == HwChipSelectId::Invalid {
            return;
        }
        self.spi.ctrl1().modify(|_, w| {
            w.sod().clear_bit();
            unsafe {
                w.ss().bits(hw_cs as u8);
            }
            w
        });
    }

    #[inline]
    pub fn cfg_hw_cs_with_pin<HwCs: OptionalHwCs<SpiInstance>>(&mut self, _: &HwCs) {
        self.cfg_hw_cs(HwCs::CS_ID);
    }

    pub fn cfg_hw_cs_disable(&mut self) {
        self.spi.ctrl1().modify(|_, w| {
            w.sod().set_bit();
            w
        });
    }

    pub fn cfg_transfer<HwCs: OptionalHwCs<SpiInstance>>(
        &mut self,
        transfer_cfg: &TransferConfig<HwCs>,
    ) {
        self.cfg_clock(transfer_cfg.spi_clk);
        self.cfg_mode(transfer_cfg.mode);
        self.blockmode = transfer_cfg.blockmode;
        self.spi.ctrl1().modify(|_, w| {
            if transfer_cfg.sod {
                w.sod().set_bit();
            } else if transfer_cfg.hw_cs.is_some() {
                w.sod().clear_bit();
                unsafe {
                    w.ss().bits(HwCs::CS_ID as u8);
                }
            } else {
                w.sod().clear_bit();
            }
            if transfer_cfg.blockmode {
                w.blockmode().set_bit();
            } else {
                w.blockmode().clear_bit();
            }
            w
        });
    }

    /// Sends a word to the slave
    #[inline(always)]
    fn send_blocking(&self, word: Word) {
        // TODO: Upper limit for wait cycles to avoid complete hangups?
        while self.spi.status().read().tnf().bit_is_clear() {}
        self.send(word)
    }

    #[inline(always)]
    fn send(&self, word: Word) {
        self.spi.data().write(|w| unsafe { w.bits(word.into()) });
    }

    /// Read a word from the slave. Must be preceeded by a [`send`](Self::send) call
    #[inline(always)]
    fn read_blocking(&self) -> Word {
        // TODO: Upper limit for wait cycles to avoid complete hangups?
        while self.spi.status().read().rne().bit_is_clear() {}
        self.read_single_word()
    }

    #[inline(always)]
    fn read_single_word(&self) -> Word {
        (self.spi.data().read().bits() & Word::MASK)
            .try_into()
            .unwrap()
    }

    fn transfer_preparation(&self, words: &[Word]) -> Result<(), Infallible> {
        if words.is_empty() {
            return Ok(());
        }
        let mut status_reg = self.spi.status().read();
        // Wait until all bytes have been transferred.
        while status_reg.tfe().bit_is_clear() {
            // Ignore all received read words.
            if status_reg.rne().bit_is_set() {
                self.clear_rx_fifo();
            }
            status_reg = self.spi.status().read();
        }
        // Ignore all received read words.
        if status_reg.rne().bit_is_set() {
            self.clear_rx_fifo();
        }
        Ok(())
    }

    fn initial_send_fifo_pumping(&self, words: Option<&[Word]>) -> usize {
        if self.blockmode {
            self.spi.ctrl1().modify(|_, w| w.mtxpause().set_bit())
        }
        // Fill the first half of the write FIFO
        let mut current_write_idx = 0;
        for _ in 0..core::cmp::min(FILL_DEPTH, words.map_or(0, |words| words.len())) {
            self.send_blocking(words.map_or(self.fill_word, |words| words[current_write_idx]));
            current_write_idx += 1;
        }
        if self.blockmode {
            self.spi.ctrl1().modify(|_, w| w.mtxpause().clear_bit())
        }
        current_write_idx
    }
}

/*
macro_rules! spi_ctor {
    ($spiI:ident, $PeriphSel: path) => {
        /// Create a new SPI struct
        ///
        /// You can delete the pin type information by calling the
        /// [`downgrade`](Self::downgrade) function
        ///
        /// ## Arguments
        /// * `spi` - SPI bus to use
        /// * `pins` - Pins to be used for SPI transactions. These pins are consumed
        ///     to ensure the pins can not be used for other purposes anymore
        /// * `spi_cfg` - Configuration specific to the SPI bus
        /// * `transfer_cfg` - Optional initial transfer configuration which includes
        ///     configuration which can change across individual SPI transfers like SPI mode
        ///     or SPI clock. If only one device is connected, this configuration only needs
        ///     to be done once.
        /// * `syscfg` - Can be passed optionally to enable the peripheral clock
        pub fn $spiI(
            spi: SpiI,
            pins: (Sck, Miso, Mosi),
            clocks: &crate::clock::Clocks,
            spi_cfg: SpiConfig,
            syscfg: &mut pac::Sysconfig,
            transfer_cfg: Option<&ErasedTransferConfig>,
        ) -> Self {
            crate::clock::enable_peripheral_clock(syscfg, $PeriphSel);
            let SpiConfig {
                ser_clock_rate_div,
                ms,
                slave_output_disable,
                loopback_mode,
                master_delayer_capture,
            } = spi_cfg;
            let mut mode = embedded_hal::spi::MODE_0;
            let mut clk_prescale = 0x02;
            let mut ss = 0;
            let mut init_blockmode = false;
            let apb1_clk = clocks.apb1();
            if let Some(transfer_cfg) = transfer_cfg {
                mode = transfer_cfg.mode;
                clk_prescale =
                    apb1_clk.raw() / (transfer_cfg.spi_clk.raw() * (ser_clock_rate_div as u32 + 1));
                if transfer_cfg.hw_cs != HwChipSelectId::Invalid {
                    ss = transfer_cfg.hw_cs as u8;
                }
                init_blockmode = transfer_cfg.blockmode;
            }

            let (cpo_bit, cph_bit) = mode_to_cpo_cph_bit(mode);
            spi.ctrl0().write(|w| {
                unsafe {
                    w.size().bits(Word::word_reg());
                    w.scrdv().bits(ser_clock_rate_div);
                    // Clear clock phase and polarity. Will be set to correct value for each
                    // transfer
                    w.spo().bit(cpo_bit);
                    w.sph().bit(cph_bit)
                }
            });
            spi.ctrl1().write(|w| {
                w.lbm().bit(loopback_mode);
                w.sod().bit(slave_output_disable);
                w.ms().bit(ms);
                w.mdlycap().bit(master_delayer_capture);
                w.blockmode().bit(init_blockmode);
                unsafe { w.ss().bits(ss) }
            });

            spi.fifo_clr().write(|w| {
                w.rxfifo().set_bit();
                w.txfifo().set_bit()
            });
            spi.clkprescale().write(|w| unsafe { w.bits(clk_prescale) });
            // Enable the peripheral as the last step as recommended in the
            // programmers guide
            spi.ctrl1().modify(|_, w| w.enable().set_bit());
            Spi {
                inner: SpiBase {
                    spi,
                    cfg: spi_cfg,
                    apb1_clk,
                    fill_word: Default::default(),
                    blockmode: init_blockmode,
                    word: PhantomData,
                },
                pins,
            }
        }
    };
}
*/

impl<
        SpiI: Instance,
        Sck: PinSck<SpiI>,
        Miso: PinMiso<SpiI>,
        Mosi: PinMosi<SpiI>,
        Word: WordProvider,
    > Spi<SpiI, (Sck, Miso, Mosi), Word>
where
    <Word as TryFrom<u32>>::Error: core::fmt::Debug,
{
    /// Create a new SPI struct
    ///
    /// You can delete the pin type information by calling the
    /// [`downgrade`](Self::downgrade) function
    ///
    /// ## Arguments
    /// * `spi` - SPI bus to use
    /// * `pins` - Pins to be used for SPI transactions. These pins are consumed
    ///     to ensure the pins can not be used for other purposes anymore
    /// * `spi_cfg` - Configuration specific to the SPI bus
    /// * `transfer_cfg` - Optional initial transfer configuration which includes
    ///     configuration which can change across individual SPI transfers like SPI mode
    ///     or SPI clock. If only one device is connected, this configuration only needs
    ///     to be done once.
    /// * `syscfg` - Can be passed optionally to enable the peripheral clock
    pub fn new(
        spi: SpiI,
        pins: (Sck, Miso, Mosi),
        clocks: &crate::clock::Clocks,
        spi_cfg: SpiConfig,
        syscfg: &mut pac::Sysconfig,
        transfer_cfg: Option<&ErasedTransferConfig>,
    ) -> Self {
        crate::clock::enable_peripheral_clock(syscfg, SpiI::PERIPH_SEL);
        let SpiConfig {
            ser_clock_rate_div,
            ms,
            slave_output_disable,
            loopback_mode,
            master_delayer_capture,
        } = spi_cfg;
        let mut mode = embedded_hal::spi::MODE_0;
        let mut clk_prescale = 0x02;
        let mut ss = 0;
        let mut init_blockmode = false;
        let apb1_clk = clocks.apb1();
        if let Some(transfer_cfg) = transfer_cfg {
            mode = transfer_cfg.mode;
            clk_prescale =
                apb1_clk.raw() / (transfer_cfg.spi_clk.raw() * (ser_clock_rate_div as u32 + 1));
            if transfer_cfg.hw_cs != HwChipSelectId::Invalid {
                ss = transfer_cfg.hw_cs as u8;
            }
            init_blockmode = transfer_cfg.blockmode;
        }

        let (cpo_bit, cph_bit) = mode_to_cpo_cph_bit(mode);
        spi.ctrl0().write(|w| {
            unsafe {
                w.size().bits(Word::word_reg());
                w.scrdv().bits(ser_clock_rate_div);
                // Clear clock phase and polarity. Will be set to correct value for each
                // transfer
                w.spo().bit(cpo_bit);
                w.sph().bit(cph_bit)
            }
        });
        spi.ctrl1().write(|w| {
            w.lbm().bit(loopback_mode);
            w.sod().bit(slave_output_disable);
            w.ms().bit(ms);
            w.mdlycap().bit(master_delayer_capture);
            w.blockmode().bit(init_blockmode);
            unsafe { w.ss().bits(ss) }
        });

        spi.fifo_clr().write(|w| {
            w.rxfifo().set_bit();
            w.txfifo().set_bit()
        });
        spi.clkprescale().write(|w| unsafe { w.bits(clk_prescale) });
        // Enable the peripheral as the last step as recommended in the
        // programmers guide
        spi.ctrl1().modify(|_, w| w.enable().set_bit());
        Spi {
            inner: SpiBase {
                spi,
                cfg: spi_cfg,
                apb1_clk,
                fill_word: Default::default(),
                blockmode: init_blockmode,
                word: PhantomData,
            },
            pins,
        }
    }

    #[inline]
    pub fn cfg_clock(&mut self, spi_clk: impl Into<Hertz>) {
        self.inner.cfg_clock(spi_clk);
    }

    #[inline]
    pub fn cfg_mode(&mut self, mode: Mode) {
        self.inner.cfg_mode(mode);
    }

    pub fn set_fill_word(&mut self, fill_word: Word) {
        self.inner.fill_word = fill_word;
    }

    pub fn fill_word(&self) -> Word {
        self.inner.fill_word
    }

    #[inline]
    pub fn perid(&self) -> u32 {
        self.inner.perid()
    }

    pub fn cfg_transfer<HwCs: OptionalHwCs<SpiI>>(&mut self, transfer_cfg: &TransferConfig<HwCs>) {
        self.inner.cfg_transfer(transfer_cfg);
    }

    /// Releases the SPI peripheral and associated pins
    pub fn release(self) -> (SpiI, (Sck, Miso, Mosi), SpiConfig) {
        (self.inner.spi, self.pins, self.inner.cfg)
    }

    pub fn downgrade(self) -> SpiBase<SpiI, Word> {
        self.inner
    }
}

/// Changing the word size also requires a type conversion
impl<SpiI: Instance, Sck: PinSck<SpiI>, Miso: PinMiso<SpiI>, Mosi: PinMosi<SpiI>>
    From<Spi<SpiI, (Sck, Miso, Mosi), u8>> for Spi<SpiI, (Sck, Miso, Mosi), u16>
{
    fn from(old_spi: Spi<SpiI, (Sck, Miso, Mosi), u8>) -> Self {
        old_spi
            .inner
            .spi
            .ctrl0()
            .modify(|_, w| unsafe { w.size().bits(WordSize::SixteenBits as u8) });
        Spi {
            inner: SpiBase {
                spi: old_spi.inner.spi,
                cfg: old_spi.inner.cfg,
                blockmode: old_spi.inner.blockmode,
                fill_word: Default::default(),
                apb1_clk: old_spi.inner.apb1_clk,
                word: PhantomData,
            },
            pins: old_spi.pins,
        }
    }
}

/// Changing the word size also requires a type conversion
impl<SpiI: Instance, Sck: PinSck<SpiI>, Miso: PinMiso<SpiI>, Mosi: PinMosi<SpiI>>
    From<Spi<SpiI, (Sck, Miso, Mosi), u16>> for Spi<SpiI, (Sck, Miso, Mosi), u8>
{
    fn from(old_spi: Spi<SpiI, (Sck, Miso, Mosi), u16>) -> Self {
        old_spi
            .inner
            .spi
            .ctrl0()
            .modify(|_, w| unsafe { w.size().bits(WordSize::EightBits as u8) });
        Spi {
            inner: SpiBase {
                spi: old_spi.inner.spi,
                cfg: old_spi.inner.cfg,
                blockmode: old_spi.inner.blockmode,
                apb1_clk: old_spi.inner.apb1_clk,
                fill_word: Default::default(),
                word: PhantomData,
            },
            pins: old_spi.pins,
        }
    }
}

impl<SpiI: Instance, Word: WordProvider> embedded_hal::spi::ErrorType for SpiBase<SpiI, Word> {
    type Error = Infallible;
}

impl<SpiI: Instance, Word: WordProvider> embedded_hal::spi::SpiBus<Word> for SpiBase<SpiI, Word>
where
    <Word as TryFrom<u32>>::Error: core::fmt::Debug,
{
    fn read(&mut self, words: &mut [Word]) -> Result<(), Self::Error> {
        self.transfer_preparation(words)?;
        let mut current_read_idx = 0;
        let mut current_write_idx = self.initial_send_fifo_pumping(None);
        loop {
            if current_write_idx < words.len() {
                self.send_blocking(self.fill_word);
                current_write_idx += 1;
            }
            if current_read_idx < words.len() {
                words[current_read_idx] = self.read_blocking();
                current_read_idx += 1;
            }
            if current_read_idx >= words.len() && current_write_idx >= words.len() {
                break;
            }
        }
        Ok(())
    }

    fn write(&mut self, words: &[Word]) -> Result<(), Self::Error> {
        self.transfer_preparation(words)?;
        let mut current_write_idx = self.initial_send_fifo_pumping(Some(words));
        while current_write_idx < words.len() {
            self.send_blocking(words[current_write_idx]);
            current_write_idx += 1;
            // Ignore received words.
            if self.spi.status().read().rne().bit_is_set() {
                self.clear_rx_fifo();
            }
        }
        Ok(())
    }

    fn transfer(&mut self, read: &mut [Word], write: &[Word]) -> Result<(), Self::Error> {
        self.transfer_preparation(write)?;
        let mut current_read_idx = 0;
        let mut current_write_idx = self.initial_send_fifo_pumping(Some(write));
        while current_read_idx < read.len() || current_write_idx < write.len() {
            if current_write_idx < write.len() {
                self.send_blocking(write[current_write_idx]);
                current_write_idx += 1;
            }
            if current_read_idx < read.len() {
                read[current_read_idx] = self.read_blocking();
                current_read_idx += 1;
            }
        }

        Ok(())
    }

    fn transfer_in_place(&mut self, words: &mut [Word]) -> Result<(), Self::Error> {
        self.transfer_preparation(words)?;
        let mut current_read_idx = 0;
        let mut current_write_idx = self.initial_send_fifo_pumping(Some(words));

        while current_read_idx < words.len() || current_write_idx < words.len() {
            if current_write_idx < words.len() {
                self.send_blocking(words[current_write_idx]);
                current_write_idx += 1;
            }
            if current_read_idx < words.len() && current_read_idx < current_write_idx {
                words[current_read_idx] = self.read_blocking();
                current_read_idx += 1;
            }
        }
        Ok(())
    }

    fn flush(&mut self) -> Result<(), Self::Error> {
        let status_reg = self.spi.status().read();
        while status_reg.tfe().bit_is_clear() || status_reg.rne().bit_is_set() {
            if status_reg.rne().bit_is_set() {
                self.read_single_word();
            }
        }
        Ok(())
    }
}

impl<
        SpiI: Instance,
        Word: WordProvider,
        Sck: PinSck<SpiI>,
        Miso: PinMiso<SpiI>,
        Mosi: PinMosi<SpiI>,
    > embedded_hal::spi::ErrorType for Spi<SpiI, (Sck, Miso, Mosi), Word>
{
    type Error = Infallible;
}

impl<
        SpiI: Instance,
        Word: WordProvider,
        Sck: PinSck<SpiI>,
        Miso: PinMiso<SpiI>,
        Mosi: PinMosi<SpiI>,
    > embedded_hal::spi::SpiBus<Word> for Spi<SpiI, (Sck, Miso, Mosi), Word>
where
    <Word as TryFrom<u32>>::Error: core::fmt::Debug,
{
    fn read(&mut self, words: &mut [Word]) -> Result<(), Self::Error> {
        self.inner.read(words)
    }

    fn write(&mut self, words: &[Word]) -> Result<(), Self::Error> {
        self.inner.write(words)
    }

    fn transfer(&mut self, read: &mut [Word], write: &[Word]) -> Result<(), Self::Error> {
        self.inner.transfer(read, write)
    }

    fn transfer_in_place(&mut self, words: &mut [Word]) -> Result<(), Self::Error> {
        self.inner.transfer_in_place(words)
    }

    fn flush(&mut self) -> Result<(), Self::Error> {
        self.inner.flush()
    }
}
