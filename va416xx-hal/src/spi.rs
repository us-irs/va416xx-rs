//! API for the SPI peripheral
//!
//! The main abstraction provided by this module are the [Spi] and the [SpiBase] structure.
//! These provide the [embedded_hal::spi] traits, but also offer a low level interface
//! via the [SpiLowLevel] trait.
//!
//! ## Examples
//!
//! - [Blocking SPI example](https://egit.irs.uni-stuttgart.de/rust/va416xx-rs/src/branch/main/examples/simple/examples/spi.rs)
use core::{convert::Infallible, marker::PhantomData, ops::Deref};

use embedded_hal::spi::{Mode, MODE_0};

use crate::{
    clock::{Clocks, PeripheralSelect, SyscfgExt},
    gpio::{
        AltFunc1, AltFunc2, AltFunc3, Pin, PA0, PA1, PA2, PA3, PA4, PA5, PA6, PA7, PA8, PA9, PB0,
        PB1, PB12, PB13, PB14, PB15, PB2, PB3, PB4, PC0, PC1, PC10, PC11, PC7, PC8, PC9, PE12,
        PE13, PE14, PE15, PE5, PE6, PE7, PE8, PE9, PF0, PF1, PG2, PG3, PG4,
    },
    pac,
    time::Hertz,
    typelevel::{NoneT, Sealed},
};

#[cfg(not(feature = "va41628"))]
use crate::gpio::{PB10, PB11, PB5, PB6, PB7, PB8, PB9, PE10, PE11, PF2, PF3, PF4, PF5, PF6, PF7};

//==================================================================================================
// Defintions
//==================================================================================================

// FIFO has a depth of 16.
const FILL_DEPTH: usize = 12;

pub const DEFAULT_CLK_DIV: u16 = 2;

pub const BMSTART_BMSTOP_MASK: u32 = 1 << 31;
pub const BMSKIPDATA_MASK: u32 = 1 << 30;

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
            ($PXx:ident, $AFx:ident, $HwCsIdent:path, $typedef:ident $(, $meta: meta)?),
        )+
    ) => {
        $(
            $(#[$meta])?
            impl HwCsProvider for Pin<$PXx, $AFx> {
                const CS_ID: HwChipSelectId = $HwCsIdent;
                const SPI_ID: SpiId = $portId;
            }

            $(#[$meta])?
            impl OptionalHwCs<$SPIx> for Pin<$PXx, $AFx> {}

            $(#[$meta])?
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

pub struct RomSpiSck;
pub struct RomSpiMiso;
pub struct RomSpiMosi;

impl Sealed for RomSpiSck {}
impl Sealed for RomSpiMosi {}
impl Sealed for RomSpiMiso {}

// SPI 0

impl PinSck<pac::Spi0> for Pin<PB15, AltFunc1> {}
impl PinMosi<pac::Spi0> for Pin<PC1, AltFunc1> {}
impl PinMiso<pac::Spi0> for Pin<PC0, AltFunc1> {}

// SPI 1
#[cfg(not(feature = "va41628"))]
impl PinSck<pac::Spi1> for Pin<PB8, AltFunc3> {}
#[cfg(not(feature = "va41628"))]
impl PinMosi<pac::Spi1> for Pin<PB10, AltFunc3> {}
#[cfg(not(feature = "va41628"))]
impl PinMiso<pac::Spi1> for Pin<PB9, AltFunc3> {}

impl PinSck<pac::Spi1> for Pin<PC9, AltFunc2> {}
impl PinMosi<pac::Spi1> for Pin<PC11, AltFunc2> {}
impl PinMiso<pac::Spi1> for Pin<PC10, AltFunc2> {}

impl PinSck<pac::Spi1> for Pin<PG3, AltFunc2> {}
impl PinMiso<pac::Spi1> for Pin<PG4, AltFunc2> {}

impl PinSck<pac::Spi1> for Pin<PE13, AltFunc2> {}
impl PinMosi<pac::Spi1> for Pin<PE15, AltFunc2> {}
impl PinMiso<pac::Spi1> for Pin<PE14, AltFunc2> {}

#[cfg(not(feature = "va41628"))]
impl PinSck<pac::Spi1> for Pin<PF3, AltFunc1> {}
#[cfg(not(feature = "va41628"))]
impl PinMosi<pac::Spi1> for Pin<PF5, AltFunc1> {}
#[cfg(not(feature = "va41628"))]
impl PinMiso<pac::Spi1> for Pin<PF4, AltFunc1> {}

// SPI 2

impl PinSck<pac::Spi2> for Pin<PA5, AltFunc2> {}
impl PinMosi<pac::Spi2> for Pin<PA7, AltFunc2> {}
impl PinMiso<pac::Spi2> for Pin<PA6, AltFunc2> {}

#[cfg(not(feature = "va41628"))]
impl PinSck<pac::Spi2> for Pin<PF5, AltFunc2> {}
#[cfg(not(feature = "va41628"))]
impl PinMosi<pac::Spi2> for Pin<PF7, AltFunc2> {}
#[cfg(not(feature = "va41628"))]
impl PinMiso<pac::Spi2> for Pin<PF6, AltFunc2> {}

// SPI3 is shared with the ROM SPI pins and has its own dedicated pins.
//
impl PinSck<pac::Spi3> for RomSpiSck {}
impl PinMosi<pac::Spi3> for RomSpiMosi {}
impl PinMiso<pac::Spi3> for RomSpiMiso {}

// SPI 0 HW CS pins

hw_cs_pins!(
    pac::Spi0, SpiId::Spi0:
    (PB14, AltFunc1, HwChipSelectId::Id0, HwCs0Spi0),
    (PB13, AltFunc1, HwChipSelectId::Id1, HwCs1Spi0),
    (PB12, AltFunc1, HwChipSelectId::Id2, HwCs2Spi0),
    (PB11, AltFunc1, HwChipSelectId::Id3, HwCs3Spi0, cfg(not(feature="va41628"))),
);

hw_cs_pins!(
    pac::Spi1, SpiId::Spi1:
    (PB7, AltFunc3, HwChipSelectId::Id0, HwCs0Spi1Pb, cfg(not(feature="va41628"))),
    (PB6, AltFunc3, HwChipSelectId::Id1, HwCs1Spi1Pb, cfg(not(feature="va41628"))),
    (PB5, AltFunc3, HwChipSelectId::Id2, HwCs2Spi1Pb, cfg(not(feature="va41628"))),
    (PB4, AltFunc3, HwChipSelectId::Id3, HwCs3Spi1Pb),
    (PB3, AltFunc3, HwChipSelectId::Id4, HwCs4Spi1Pb),
    (PB2, AltFunc3, HwChipSelectId::Id5, HwCs5Spi1Pb),
    (PB1, AltFunc3, HwChipSelectId::Id6, HwCs6Spi1Pb),
    (PB0, AltFunc3, HwChipSelectId::Id7, HwCs7Spi1Pb),
    (PC8, AltFunc2, HwChipSelectId::Id0, HwCs0Spi1Pc),
    (PC7, AltFunc2, HwChipSelectId::Id1, HwCs1Spi1Pc),
    (PE12, AltFunc2, HwChipSelectId::Id0, HwCs0Spi1Pe),
    (PE11, AltFunc2, HwChipSelectId::Id1, HwCs1Spi1Pe, cfg(not(feature="va41628"))),
    (PE10, AltFunc2, HwChipSelectId::Id2, HwCs2Spi1Pe, cfg(not(feature="va41628"))),
    (PE9, AltFunc2, HwChipSelectId::Id3, HwCs3Spi1Pe),
    (PE8, AltFunc2, HwChipSelectId::Id4, HwCs4Spi1Pe),
    (PE7, AltFunc3, HwChipSelectId::Id5, HwCs5Spi1Pe),
    (PE6, AltFunc3, HwChipSelectId::Id6, HwCs6Spi1Pe),
    (PE5, AltFunc3, HwChipSelectId::Id7, HwCs7Spi1Pe),
    (PF2, AltFunc1, HwChipSelectId::Id0, HwCs0Spi1Pf, cfg(not(feature="va41628"))),
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
    (PF2, AltFunc2, HwChipSelectId::Id2, HwCs2Spi2Pf, cfg(not(feature="va41628"))),
    (PF3, AltFunc2, HwChipSelectId::Id1, HwCs1Spi2Pf, cfg(not(feature="va41628"))),
    (PF4, AltFunc2, HwChipSelectId::Id0, HwCs0Spi2Pf, cfg(not(feature="va41628"))),
);

//==================================================================================================
// Config
//==================================================================================================

pub trait TransferConfigProvider {
    fn sod(&mut self, sod: bool);
    fn blockmode(&mut self, blockmode: bool);
    fn mode(&mut self, mode: Mode);
    fn clk_cfg(&mut self, clk_cfg: SpiClkConfig);
    fn hw_cs_id(&self) -> u8;
}

/// This struct contains all configuration parameter which are transfer specific
/// and might change for transfers to different SPI slaves
#[derive(Copy, Clone, Debug)]
pub struct TransferConfigWithHwcs<HwCs> {
    pub hw_cs: Option<HwCs>,
    pub cfg: TransferConfig,
}

/// Type erased variant of the transfer configuration. This is required to avoid generics in
/// the SPI constructor.
#[derive(Copy, Clone, Debug)]
pub struct TransferConfig {
    pub clk_cfg: Option<SpiClkConfig>,
    pub mode: Option<Mode>,
    pub sod: bool,
    /// If this is enabled, all data in the FIFO is transmitted in a single frame unless
    /// the BMSTOP bit is set on a dataword. A frame is defined as CSn being active for the
    /// duration of multiple data words
    pub blockmode: bool,
    /// Only used when blockmode is used. The SCK will be stalled until an explicit stop bit
    /// is set on a written word.
    pub bmstall: bool,
    pub hw_cs: HwChipSelectId,
}

impl TransferConfigWithHwcs<NoneT> {
    pub fn new_no_hw_cs(
        clk_cfg: Option<SpiClkConfig>,
        mode: Option<Mode>,
        blockmode: bool,
        bmstall: bool,
        sod: bool,
    ) -> Self {
        TransferConfigWithHwcs {
            hw_cs: None,
            cfg: TransferConfig {
                clk_cfg,
                mode,
                sod,
                blockmode,
                bmstall,
                hw_cs: HwChipSelectId::Invalid,
            },
        }
    }
}

impl<HwCs: HwCsProvider> TransferConfigWithHwcs<HwCs> {
    pub fn new(
        clk_cfg: Option<SpiClkConfig>,
        mode: Option<Mode>,
        hw_cs: Option<HwCs>,
        blockmode: bool,
        bmstall: bool,
        sod: bool,
    ) -> Self {
        TransferConfigWithHwcs {
            hw_cs,
            cfg: TransferConfig {
                clk_cfg,
                mode,
                sod,
                blockmode,
                bmstall,
                hw_cs: HwCs::CS_ID,
            },
        }
    }

    pub fn downgrade(self) -> TransferConfig {
        self.cfg
    }
}

impl<HwCs: HwCsProvider> TransferConfigProvider for TransferConfigWithHwcs<HwCs> {
    /// Slave Output Disable
    fn sod(&mut self, sod: bool) {
        self.cfg.sod = sod;
    }

    fn blockmode(&mut self, blockmode: bool) {
        self.cfg.blockmode = blockmode;
    }

    fn mode(&mut self, mode: Mode) {
        self.cfg.mode = Some(mode);
    }

    fn clk_cfg(&mut self, clk_cfg: SpiClkConfig) {
        self.cfg.clk_cfg = Some(clk_cfg);
    }

    fn hw_cs_id(&self) -> u8 {
        HwCs::CS_ID as u8
    }
}

/// Configuration options for the whole SPI bus. See Programmer Guide p.92 for more details
pub struct SpiConfig {
    clk: SpiClkConfig,
    // SPI mode configuration
    pub init_mode: Mode,
    /// If this is enabled, all data in the FIFO is transmitted in a single frame unless
    /// the BMSTOP bit is set on a dataword. A frame is defined as CSn being active for the
    /// duration of multiple data words. Defaults to true.
    pub blockmode: bool,
    /// This enables the stalling of the SPI SCK if in blockmode and the FIFO is empty.
    /// Currently enabled by default.
    pub bmstall: bool,
    /// By default, configure SPI for master mode (ms == false)
    ms: bool,
    /// Slave output disable. Useful if separate GPIO pins or decoders are used for CS control
    pub slave_output_disable: bool,
    /// Loopback mode. If you use this, don't connect MISO to MOSI, they will be tied internally
    pub loopback_mode: bool,
    /// Enable Master Delayer Capture Mode. See Programmers Guide p.92 for more details
    pub master_delayer_capture: bool,
}

impl Default for SpiConfig {
    fn default() -> Self {
        Self {
            init_mode: MODE_0,
            blockmode: true,
            bmstall: true,
            // Default value is definitely valid.
            clk: SpiClkConfig::from_div(DEFAULT_CLK_DIV).unwrap(),
            ms: Default::default(),
            slave_output_disable: Default::default(),
            loopback_mode: Default::default(),
            master_delayer_capture: Default::default(),
        }
    }
}

impl SpiConfig {
    pub fn loopback(mut self, enable: bool) -> Self {
        self.loopback_mode = enable;
        self
    }

    pub fn blockmode(mut self, enable: bool) -> Self {
        self.blockmode = enable;
        self
    }

    pub fn bmstall(mut self, enable: bool) -> Self {
        self.bmstall = enable;
        self
    }

    pub fn mode(mut self, mode: Mode) -> Self {
        self.init_mode = mode;
        self
    }

    pub fn clk_cfg(mut self, clk_cfg: SpiClkConfig) -> Self {
        self.clk = clk_cfg;
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

    #[inline(always)]
    fn ptr() -> *const SpiRegBlock {
        Self::ptr()
    }
}

impl Instance for pac::Spi1 {
    const IDX: u8 = 1;
    const PERIPH_SEL: PeripheralSelect = PeripheralSelect::Spi1;

    #[inline(always)]
    fn ptr() -> *const SpiRegBlock {
        Self::ptr()
    }
}

impl Instance for pac::Spi2 {
    const IDX: u8 = 2;
    const PERIPH_SEL: PeripheralSelect = PeripheralSelect::Spi2;

    #[inline(always)]
    fn ptr() -> *const SpiRegBlock {
        Self::ptr()
    }
}

impl Instance for pac::Spi3 {
    const IDX: u8 = 3;
    const PERIPH_SEL: PeripheralSelect = PeripheralSelect::Spi3;

    #[inline(always)]
    fn ptr() -> *const SpiRegBlock {
        Self::ptr()
    }
}

//==================================================================================================
// Spi
//==================================================================================================

/// Low level access trait for the SPI peripheral.
pub trait SpiLowLevel {
    /// Low level function to write a word to the SPI FIFO but also checks whether
    /// there is actually data in the FIFO.
    ///
    /// Uses the [nb] API to allow usage in blocking and non-blocking contexts.
    fn write_fifo(&self, data: u32) -> nb::Result<(), Infallible>;

    /// Low level function to write a word to the SPI FIFO without checking whether
    /// there FIFO is full.
    ///
    /// This does not necesarily mean there is a space in the FIFO available.
    /// Use [Self::write_fifo] function to write a word into the FIFO reliably.
    fn write_fifo_unchecked(&self, data: u32);

    /// Low level function to read a word from the SPI FIFO. Must be preceeded by a
    /// [Self::write_fifo] call.
    ///
    /// Uses the [nb] API to allow usage in blocking and non-blocking contexts.
    fn read_fifo(&self) -> nb::Result<u32, Infallible>;

    /// Low level function to read a word from from the SPI FIFO.
    ///
    /// This does not necesarily mean there is a word in the FIFO available.
    /// Use the [Self::read_fifo] function to read a word from the FIFO reliably using the [nb]
    /// API.
    /// You might also need to mask the value to ignore the BMSTART/BMSTOP bit.
    fn read_fifo_unchecked(&self) -> u32;
}

pub struct SpiBase<SpiInstance, Word = u8> {
    spi: SpiInstance,
    cfg: SpiConfig,
    apb1_clk: Hertz,
    /// Fill word for read-only SPI transactions.
    pub fill_word: Word,
    blockmode: bool,
    bmstall: bool,
    word: PhantomData<Word>,
}

pub struct Spi<SpiInstance, Pins, Word = u8> {
    inner: SpiBase<SpiInstance, Word>,
    pins: Pins,
}

pub fn mode_to_cpo_cph_bit(mode: embedded_hal::spi::Mode) -> (bool, bool) {
    match mode {
        embedded_hal::spi::MODE_0 => (false, false),
        embedded_hal::spi::MODE_1 => (false, true),
        embedded_hal::spi::MODE_2 => (true, false),
        embedded_hal::spi::MODE_3 => (true, true),
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct SpiClkConfig {
    prescale_val: u16,
    scrdv: u8,
}

impl SpiClkConfig {
    pub fn prescale_val(&self) -> u16 {
        self.prescale_val
    }
    pub fn scrdv(&self) -> u8 {
        self.scrdv
    }
}

impl SpiClkConfig {
    pub fn new(prescale_val: u16, scrdv: u8) -> Self {
        Self {
            prescale_val,
            scrdv,
        }
    }

    pub fn from_div(div: u16) -> Result<Self, SpiClkConfigError> {
        spi_clk_config_from_div(div)
    }

    pub fn from_clk(spi_clk: Hertz, clocks: &Clocks) -> Option<Self> {
        clk_div_for_target_clock(spi_clk, clocks).map(|div| spi_clk_config_from_div(div).unwrap())
    }
}

#[derive(Debug)]
pub enum SpiClkConfigError {
    DivIsZero,
    DivideValueNotEven,
    ScrdvValueTooLarge,
}

#[inline]
pub fn spi_clk_config_from_div(mut div: u16) -> Result<SpiClkConfig, SpiClkConfigError> {
    if div == 0 {
        return Err(SpiClkConfigError::DivIsZero);
    }
    if div % 2 != 0 {
        return Err(SpiClkConfigError::DivideValueNotEven);
    }
    let mut prescale_val = 0;

    // find largest (even) prescale value that divides into div
    for i in (2..=0xfe).rev().step_by(2) {
        if div % i == 0 {
            prescale_val = i;
            break;
        }
    }

    if prescale_val == 0 {
        return Err(SpiClkConfigError::DivideValueNotEven);
    }

    div /= prescale_val;
    if div > u8::MAX as u16 + 1 {
        return Err(SpiClkConfigError::ScrdvValueTooLarge);
    }
    Ok(SpiClkConfig {
        prescale_val,
        scrdv: (div - 1) as u8,
    })
}

#[inline]
pub fn clk_div_for_target_clock(spi_clk: impl Into<Hertz>, clocks: &Clocks) -> Option<u16> {
    let spi_clk = spi_clk.into();
    if spi_clk > clocks.apb1() {
        return None;
    }

    // Step 1: Calculate raw divider.
    let raw_div = clocks.apb1().raw() / spi_clk.raw();
    let remainder = clocks.apb1().raw() % spi_clk.raw();

    // Step 2: Round up if necessary.
    let mut rounded_div = if remainder * 2 >= spi_clk.raw() {
        raw_div + 1
    } else {
        raw_div
    };

    if rounded_div % 2 != 0 {
        // Take slower clock conservatively.
        rounded_div += 1;
    }
    if rounded_div > u16::MAX as u32 {
        return None;
    }
    Some(rounded_div as u16)
}

impl<SpiInstance: Instance, Word: WordProvider> SpiBase<SpiInstance, Word>
where
    <Word as TryFrom<u32>>::Error: core::fmt::Debug,
{
    #[inline]
    pub fn cfg_clock(&mut self, cfg: SpiClkConfig) {
        self.spi
            .ctrl0()
            .modify(|_, w| unsafe { w.scrdv().bits(cfg.scrdv) });
        self.spi
            .clkprescale()
            .write(|w| unsafe { w.bits(cfg.prescale_val as u32) });
    }

    #[inline]
    pub fn cfg_clock_from_div(&mut self, div: u16) -> Result<(), SpiClkConfigError> {
        let val = spi_clk_config_from_div(div)?;
        self.cfg_clock(val);
        Ok(())
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
    pub fn spi(&self) -> &SpiInstance {
        &self.spi
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

    #[inline]
    pub fn cfg_hw_cs_disable(&mut self) {
        self.spi.ctrl1().modify(|_, w| {
            w.sod().set_bit();
            w
        });
    }

    pub fn cfg_transfer<HwCs: OptionalHwCs<SpiInstance>>(
        &mut self,
        transfer_cfg: &TransferConfigWithHwcs<HwCs>,
    ) {
        if let Some(trans_clk_div) = transfer_cfg.cfg.clk_cfg {
            self.cfg_clock(trans_clk_div);
        }
        if let Some(mode) = transfer_cfg.cfg.mode {
            self.cfg_mode(mode);
        }
        self.blockmode = transfer_cfg.cfg.blockmode;
        self.spi.ctrl1().modify(|_, w| {
            if transfer_cfg.cfg.sod {
                w.sod().set_bit();
            } else if transfer_cfg.hw_cs.is_some() {
                w.sod().clear_bit();
                unsafe {
                    w.ss().bits(HwCs::CS_ID as u8);
                }
            } else {
                w.sod().clear_bit();
            }
            w.blockmode().bit(transfer_cfg.cfg.blockmode);
            w.bmstall().bit(transfer_cfg.cfg.bmstall)
        });
    }

    /// Low level function to write a word to the SPI FIFO but also checks whether
    /// there is actually data in the FIFO.
    ///
    /// Uses the [nb] API to allow usage in blocking and non-blocking contexts.
    #[inline(always)]
    pub fn write_fifo(&self, data: u32) -> nb::Result<(), Infallible> {
        if self.spi.status().read().tnf().bit_is_clear() {
            return Err(nb::Error::WouldBlock);
        }
        self.write_fifo_unchecked(data);
        Ok(())
    }

    /// Low level function to write a word to the SPI FIFO without checking whether
    /// there FIFO is full.
    ///
    /// This does not necesarily mean there is a space in the FIFO available.
    /// Use [Self::write_fifo] function to write a word into the FIFO reliably.
    #[inline(always)]
    pub fn write_fifo_unchecked(&self, data: u32) {
        self.spi.data().write(|w| unsafe { w.bits(data) });
    }

    /// Low level function to read a word from the SPI FIFO. Must be preceeded by a
    /// [Self::write_fifo] call.
    ///
    /// Uses the [nb] API to allow usage in blocking and non-blocking contexts.
    #[inline(always)]
    pub fn read_fifo(&self) -> nb::Result<u32, Infallible> {
        if self.spi.status().read().rne().bit_is_clear() {
            return Err(nb::Error::WouldBlock);
        }
        Ok(self.read_fifo_unchecked())
    }

    /// Low level function to read a word from from the SPI FIFO.
    ///
    /// This does not necesarily mean there is a word in the FIFO available.
    /// Use the [Self::read_fifo] function to read a word from the FIFO reliably using the [nb]
    /// API.
    /// You might also need to mask the value to ignore the BMSTART/BMSTOP bit.
    #[inline(always)]
    pub fn read_fifo_unchecked(&self) -> u32 {
        self.spi.data().read().bits()
    }

    fn flush_internal(&self) {
        let mut status_reg = self.spi.status().read();
        while status_reg.tfe().bit_is_clear()
            || status_reg.rne().bit_is_set()
            || status_reg.busy().bit_is_set()
        {
            if status_reg.rne().bit_is_set() {
                self.read_fifo_unchecked();
            }
            status_reg = self.spi.status().read();
        }
    }

    fn transfer_preparation(&self, words: &[Word]) -> Result<(), Infallible> {
        if words.is_empty() {
            return Ok(());
        }
        self.flush_internal();
        Ok(())
    }

    // The FIFO can hold a guaranteed amount of data, so we can pump it on transfer
    // initialization. Returns the amount of written bytes.
    fn initial_send_fifo_pumping_with_words(&self, words: &[Word]) -> usize {
        if self.blockmode {
            self.spi.ctrl1().modify(|_, w| w.mtxpause().set_bit())
        }
        // Fill the first half of the write FIFO
        let mut current_write_idx = 0;
        let smaller_idx = core::cmp::min(FILL_DEPTH, words.len());
        for _ in 0..smaller_idx {
            if current_write_idx == smaller_idx.saturating_sub(1) && self.bmstall {
                self.write_fifo_unchecked(words[current_write_idx].into() | BMSTART_BMSTOP_MASK);
            } else {
                self.write_fifo_unchecked(words[current_write_idx].into());
            }
            current_write_idx += 1;
        }
        if self.blockmode {
            self.spi.ctrl1().modify(|_, w| w.mtxpause().clear_bit())
        }
        current_write_idx
    }

    // The FIFO can hold a guaranteed amount of data, so we can pump it on transfer
    // initialization.
    fn initial_send_fifo_pumping_with_fill_words(&self, send_len: usize) -> usize {
        if self.blockmode {
            self.spi.ctrl1().modify(|_, w| w.mtxpause().set_bit())
        }
        // Fill the first half of the write FIFO
        let mut current_write_idx = 0;
        let smaller_idx = core::cmp::min(FILL_DEPTH, send_len);
        for _ in 0..smaller_idx {
            if current_write_idx == smaller_idx.saturating_sub(1) && self.bmstall {
                self.write_fifo_unchecked(self.fill_word.into() | BMSTART_BMSTOP_MASK);
            } else {
                self.write_fifo_unchecked(self.fill_word.into());
            }
            current_write_idx += 1;
        }
        if self.blockmode {
            self.spi.ctrl1().modify(|_, w| w.mtxpause().clear_bit())
        }
        current_write_idx
    }
}

impl<SpiInstance: Instance, Word: WordProvider> SpiLowLevel for SpiBase<SpiInstance, Word>
where
    <Word as TryFrom<u32>>::Error: core::fmt::Debug,
{
    #[inline(always)]
    fn write_fifo(&self, data: u32) -> nb::Result<(), Infallible> {
        if self.spi.status().read().tnf().bit_is_clear() {
            return Err(nb::Error::WouldBlock);
        }
        self.write_fifo_unchecked(data);
        Ok(())
    }

    #[inline(always)]
    fn write_fifo_unchecked(&self, data: u32) {
        self.spi.data().write(|w| unsafe { w.bits(data) });
    }

    #[inline(always)]
    fn read_fifo(&self) -> nb::Result<u32, Infallible> {
        if self.spi.status().read().rne().bit_is_clear() {
            return Err(nb::Error::WouldBlock);
        }
        Ok(self.read_fifo_unchecked())
    }

    #[inline(always)]
    fn read_fifo_unchecked(&self) -> u32 {
        self.spi.data().read().bits()
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
        let mut current_write_idx = self.initial_send_fifo_pumping_with_fill_words(words.len());
        loop {
            if current_read_idx < words.len() {
                words[current_read_idx] = (nb::block!(self.read_fifo())? & Word::MASK)
                    .try_into()
                    .unwrap();
                current_read_idx += 1;
            }
            if current_write_idx < words.len() {
                if current_write_idx == words.len() - 1 && self.bmstall {
                    nb::block!(self.write_fifo(self.fill_word.into() | BMSTART_BMSTOP_MASK))?;
                } else {
                    nb::block!(self.write_fifo(self.fill_word.into()))?;
                }
                current_write_idx += 1;
            }
            if current_read_idx >= words.len() && current_write_idx >= words.len() {
                break;
            }
        }
        Ok(())
    }

    fn write(&mut self, words: &[Word]) -> Result<(), Self::Error> {
        self.transfer_preparation(words)?;
        let mut current_write_idx = self.initial_send_fifo_pumping_with_words(words);
        while current_write_idx < words.len() {
            if current_write_idx == words.len() - 1 && self.bmstall {
                nb::block!(self.write_fifo(words[current_write_idx].into() | BMSTART_BMSTOP_MASK))?;
            } else {
                nb::block!(self.write_fifo(words[current_write_idx].into()))?;
            }
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
        let mut current_write_idx = self.initial_send_fifo_pumping_with_words(write);
        while current_read_idx < read.len() || current_write_idx < write.len() {
            if current_write_idx < write.len() {
                if current_write_idx == write.len() - 1 && self.bmstall {
                    nb::block!(
                        self.write_fifo(write[current_write_idx].into() | BMSTART_BMSTOP_MASK)
                    )?;
                } else {
                    nb::block!(self.write_fifo(write[current_write_idx].into()))?;
                }
                current_write_idx += 1;
            }
            if current_read_idx < read.len() {
                read[current_read_idx] = (nb::block!(self.read_fifo())? & Word::MASK)
                    .try_into()
                    .unwrap();
                current_read_idx += 1;
            }
        }

        Ok(())
    }

    fn transfer_in_place(&mut self, words: &mut [Word]) -> Result<(), Self::Error> {
        self.transfer_preparation(words)?;
        let mut current_read_idx = 0;
        let mut current_write_idx = self.initial_send_fifo_pumping_with_words(words);

        while current_read_idx < words.len() || current_write_idx < words.len() {
            if current_write_idx < words.len() {
                if current_write_idx == words.len() - 1 && self.bmstall {
                    nb::block!(
                        self.write_fifo(words[current_write_idx].into() | BMSTART_BMSTOP_MASK)
                    )?;
                } else {
                    nb::block!(self.write_fifo(words[current_write_idx].into()))?;
                }
                current_write_idx += 1;
            }
            if current_read_idx < words.len() && current_read_idx < current_write_idx {
                words[current_read_idx] = (nb::block!(self.read_fifo())? & Word::MASK)
                    .try_into()
                    .unwrap();
                current_read_idx += 1;
            }
        }
        Ok(())
    }

    fn flush(&mut self) -> Result<(), Self::Error> {
        self.flush_internal();
        Ok(())
    }
}

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
        syscfg: &mut pac::Sysconfig,
        clocks: &crate::clock::Clocks,
        spi: SpiI,
        pins: (Sck, Miso, Mosi),
        spi_cfg: SpiConfig,
    ) -> Self {
        crate::clock::enable_peripheral_clock(syscfg, SpiI::PERIPH_SEL);
        // This is done in the C HAL.
        syscfg.assert_periph_reset_for_two_cycles(SpiI::PERIPH_SEL);
        let SpiConfig {
            clk,
            init_mode,
            blockmode,
            bmstall,
            ms,
            slave_output_disable,
            loopback_mode,
            master_delayer_capture,
        } = spi_cfg;

        let (cpo_bit, cph_bit) = mode_to_cpo_cph_bit(init_mode);
        spi.ctrl0().write(|w| {
            unsafe {
                w.size().bits(Word::word_reg());
                w.scrdv().bits(clk.scrdv);
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
            w.blockmode().bit(blockmode);
            w.bmstall().bit(bmstall);
            unsafe { w.ss().bits(0) }
        });
        spi.clkprescale()
            .write(|w| unsafe { w.bits(clk.prescale_val as u32) });

        spi.fifo_clr().write(|w| {
            w.rxfifo().set_bit();
            w.txfifo().set_bit()
        });
        // Enable the peripheral as the last step as recommended in the
        // programmers guide
        spi.ctrl1().modify(|_, w| w.enable().set_bit());
        Spi {
            inner: SpiBase {
                spi,
                cfg: spi_cfg,
                apb1_clk: clocks.apb1(),
                fill_word: Default::default(),
                bmstall,
                blockmode,
                word: PhantomData,
            },
            pins,
        }
    }

    delegate::delegate! {
        to self.inner {
            #[inline]
            pub fn cfg_clock(&mut self, cfg: SpiClkConfig);

            #[inline]
            pub fn cfg_clock_from_div(&mut self, div: u16) -> Result<(), SpiClkConfigError>;

            #[inline]
            pub fn spi(&self) -> &SpiI;

            #[inline]
            pub fn cfg_mode(&mut self, mode: Mode);

            #[inline]
            pub fn perid(&self) -> u32;

            pub fn cfg_transfer<HwCs: OptionalHwCs<SpiI>>(
                &mut self, transfer_cfg: &TransferConfigWithHwcs<HwCs>
            );
        }
    }

    #[inline]
    pub fn set_fill_word(&mut self, fill_word: Word) {
        self.inner.fill_word = fill_word;
    }

    #[inline]
    pub fn fill_word(&self) -> Word {
        self.inner.fill_word
    }

    /// Releases the SPI peripheral and associated pins
    pub fn release(self) -> (SpiI, (Sck, Miso, Mosi), SpiConfig) {
        (self.inner.spi, self.pins, self.inner.cfg)
    }

    pub fn downgrade(self) -> SpiBase<SpiI, Word> {
        self.inner
    }
}

impl<
        SpiI: Instance,
        Sck: PinSck<SpiI>,
        Miso: PinMiso<SpiI>,
        Mosi: PinMosi<SpiI>,
        Word: WordProvider,
    > SpiLowLevel for Spi<SpiI, (Sck, Miso, Mosi), Word>
where
    <Word as TryFrom<u32>>::Error: core::fmt::Debug,
{
    delegate::delegate! {
        to self.inner {
            fn write_fifo(&self, data: u32) -> nb::Result<(), Infallible>;
            fn write_fifo_unchecked(&self, data: u32);
            fn read_fifo(&self) -> nb::Result<u32, Infallible>;
            fn read_fifo_unchecked(&self) -> u32;
        }
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
    delegate::delegate! {
        to self.inner {
            fn read(&mut self, words: &mut [Word]) -> Result<(), Self::Error>;
            fn write(&mut self, words: &[Word]) -> Result<(), Self::Error>;
            fn transfer(&mut self, read: &mut [Word], write: &[Word]) -> Result<(), Self::Error>;
            fn transfer_in_place(&mut self, words: &mut [Word]) -> Result<(), Self::Error>;
            fn flush(&mut self) -> Result<(), Self::Error>;
        }
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
                bmstall: old_spi.inner.bmstall,
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
                bmstall: old_spi.inner.bmstall,
                apb1_clk: old_spi.inner.apb1_clk,
                fill_word: Default::default(),
                word: PhantomData,
            },
            pins: old_spi.pins,
        }
    }
}
