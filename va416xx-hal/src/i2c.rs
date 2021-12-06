//! API for the I2C peripheral
//!
//! ## Examples
//!
//! - [PEB1 accelerometer example](https://egit.irs.uni-stuttgart.de/rust/va416xx-rs/src/branch/main/examples/simple/examples/peb1-accelerometer.rs)
use crate::{
    clock::{
        assert_periph_reset, deassert_periph_reset, enable_peripheral_clock, Clocks,
        PeripheralSelect,
    },
    pac,
    time::Hertz,
    typelevel::Sealed,
};
use core::{marker::PhantomData, ops::Deref};
use embedded_hal::i2c::{self, Operation, SevenBitAddress, TenBitAddress};

//==================================================================================================
// Defintions
//==================================================================================================

const CLK_100K: Hertz = Hertz::from_raw(100_000);
const CLK_400K: Hertz = Hertz::from_raw(400_000);
const MIN_CLK_400K: Hertz = Hertz::from_raw(10_000_000);

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FifoEmptyMode {
    Stall = 0,
    EndTransaction = 1,
}

#[derive(Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct ClockTooSlowForFastI2c;

#[derive(Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Error {
    InvalidTimingParams,
    ArbitrationLost,
    NackAddr,
    /// Data not acknowledged in write operation
    NackData,
    /// Not enough data received in read operation
    InsufficientDataReceived,
    /// Number of bytes in transfer too large (larger than 0x7fe)
    DataTooLarge,
}

#[derive(Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum InitError {
    /// Wrong address used in constructor
    WrongAddrMode,
    /// APB1 clock is too slow for fast I2C mode.
    ClkTooSlow(ClockTooSlowForFastI2c),
}

impl From<ClockTooSlowForFastI2c> for InitError {
    fn from(value: ClockTooSlowForFastI2c) -> Self {
        Self::ClkTooSlow(value)
    }
}

impl embedded_hal::i2c::Error for Error {
    fn kind(&self) -> embedded_hal::i2c::ErrorKind {
        match self {
            Error::ArbitrationLost => embedded_hal::i2c::ErrorKind::ArbitrationLoss,
            Error::NackAddr => {
                embedded_hal::i2c::ErrorKind::NoAcknowledge(i2c::NoAcknowledgeSource::Address)
            }
            Error::NackData => {
                embedded_hal::i2c::ErrorKind::NoAcknowledge(i2c::NoAcknowledgeSource::Data)
            }
            Error::DataTooLarge | Error::InsufficientDataReceived | Error::InvalidTimingParams => {
                embedded_hal::i2c::ErrorKind::Other
            }
        }
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
enum I2cCmd {
    Start = 0b00,
    Stop = 0b10,
    StartWithStop = 0b11,
    Cancel = 0b100,
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum I2cSpeed {
    Regular100khz = 0,
    Fast400khz = 1,
}

#[derive(Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum I2cDirection {
    Send = 0,
    Read = 1,
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum I2cAddress {
    Regular(u8),
    TenBit(u16),
}

pub type I2cRegBlock = pac::i2c0::RegisterBlock;

/// Common trait implemented by all PAC peripheral access structures. The register block
/// format is the same for all SPI blocks.
pub trait Instance: Deref<Target = I2cRegBlock> {
    const IDX: u8;
    const PERIPH_SEL: PeripheralSelect;

    fn ptr() -> *const I2cRegBlock;
}

impl Instance for pac::I2c0 {
    const IDX: u8 = 0;
    const PERIPH_SEL: PeripheralSelect = PeripheralSelect::I2c0;

    fn ptr() -> *const I2cRegBlock {
        Self::ptr()
    }
}

impl Instance for pac::I2c1 {
    const IDX: u8 = 1;
    const PERIPH_SEL: PeripheralSelect = PeripheralSelect::I2c1;

    fn ptr() -> *const I2cRegBlock {
        Self::ptr()
    }
}

impl Instance for pac::I2c2 {
    const IDX: u8 = 2;
    const PERIPH_SEL: PeripheralSelect = PeripheralSelect::I2c2;

    fn ptr() -> *const I2cRegBlock {
        Self::ptr()
    }
}

//==================================================================================================
// Config
//==================================================================================================

pub struct TrTfThighTlow(u8, u8, u8, u8);
pub struct TsuStoTsuStaThdStaTBuf(u8, u8, u8, u8);

pub struct TimingCfg {
    // 4 bit max width
    tr: u8,
    // 4 bit max width
    tf: u8,
    // 4 bit max width
    thigh: u8,
    // 4 bit max width
    tlow: u8,
    // 4 bit max width
    tsu_sto: u8,
    // 4 bit max width
    tsu_sta: u8,
    // 4 bit max width
    thd_sta: u8,
    // 4 bit max width
    tbuf: u8,
}

impl TimingCfg {
    pub fn new(
        first_16_bits: TrTfThighTlow,
        second_16_bits: TsuStoTsuStaThdStaTBuf,
    ) -> Result<Self, Error> {
        if first_16_bits.0 > 0xf
            || first_16_bits.1 > 0xf
            || first_16_bits.2 > 0xf
            || first_16_bits.3 > 0xf
            || second_16_bits.0 > 0xf
            || second_16_bits.1 > 0xf
            || second_16_bits.2 > 0xf
            || second_16_bits.3 > 0xf
        {
            return Err(Error::InvalidTimingParams);
        }
        Ok(TimingCfg {
            tr: first_16_bits.0,
            tf: first_16_bits.1,
            thigh: first_16_bits.2,
            tlow: first_16_bits.3,
            tsu_sto: second_16_bits.0,
            tsu_sta: second_16_bits.1,
            thd_sta: second_16_bits.2,
            tbuf: second_16_bits.3,
        })
    }

    pub fn reg(&self) -> u32 {
        (self.tbuf as u32) << 28
            | (self.thd_sta as u32) << 24
            | (self.tsu_sta as u32) << 20
            | (self.tsu_sto as u32) << 16
            | (self.tlow as u32) << 12
            | (self.thigh as u32) << 8
            | (self.tf as u32) << 4
            | (self.tr as u32)
    }
}

impl Default for TimingCfg {
    fn default() -> Self {
        TimingCfg {
            tr: 0x02,
            tf: 0x01,
            thigh: 0x08,
            tlow: 0x09,
            tsu_sto: 0x8,
            tsu_sta: 0x0a,
            thd_sta: 0x8,
            tbuf: 0xa,
        }
    }
}

pub struct MasterConfig {
    pub tx_fe_mode: FifoEmptyMode,
    pub rx_fe_mode: FifoEmptyMode,
    /// Enable the analog delay glitch filter
    pub alg_filt: bool,
    /// Enable the digital glitch filter
    pub dlg_filt: bool,
    pub tm_cfg: Option<TimingCfg>,
    // Loopback mode
    // lbm: bool,
}

impl Default for MasterConfig {
    fn default() -> Self {
        MasterConfig {
            tx_fe_mode: FifoEmptyMode::Stall,
            rx_fe_mode: FifoEmptyMode::Stall,
            alg_filt: false,
            dlg_filt: false,
            tm_cfg: None,
        }
    }
}

impl Sealed for MasterConfig {}

pub struct SlaveConfig {
    pub tx_fe_mode: FifoEmptyMode,
    pub rx_fe_mode: FifoEmptyMode,
    /// Maximum number of words before issuing a negative acknowledge.
    /// Range should be 0 to 0x7fe. Setting the value to 0x7ff has the same effect as not setting
    /// the enable bit since RXCOUNT stops counting at 0x7fe.
    pub max_words: Option<usize>,
    /// A received address is compared to the ADDRESS register (addr) using the address mask
    /// (addr_mask). Those bits with a 1 in the address mask must match for there to be an address
    /// match
    pub addr: I2cAddress,
    /// The default address mask will be 0x3ff to only allow full matches
    pub addr_mask: Option<u16>,
    /// Optionally specify a second I2C address the slave interface responds to
    pub addr_b: Option<I2cAddress>,
    pub addr_b_mask: Option<u16>,
}

impl SlaveConfig {
    /// Build a default slave config given a specified slave address to respond to
    pub fn new(addr: I2cAddress) -> Self {
        SlaveConfig {
            tx_fe_mode: FifoEmptyMode::Stall,
            rx_fe_mode: FifoEmptyMode::Stall,
            max_words: None,
            addr,
            addr_mask: None,
            addr_b: None,
            addr_b_mask: None,
        }
    }
}

impl Sealed for SlaveConfig {}

//==================================================================================================
// I2C Base
//==================================================================================================

pub struct I2cBase<I2c> {
    i2c: I2c,
    clock: Hertz,
}

impl<I2C> I2cBase<I2C> {
    #[inline]
    fn unwrap_addr(addr: I2cAddress) -> (u16, u32) {
        match addr {
            I2cAddress::Regular(addr) => (addr as u16, 0 << 15),
            I2cAddress::TenBit(addr) => (addr, 1 << 15),
        }
    }
}

impl<I2c: Instance> I2cBase<I2c> {
    pub fn new(
        i2c: I2c,
        sys_cfg: &mut pac::Sysconfig,
        clocks: &Clocks,
        speed_mode: I2cSpeed,
        ms_cfg: Option<&MasterConfig>,
        sl_cfg: Option<&SlaveConfig>,
    ) -> Result<Self, ClockTooSlowForFastI2c> {
        enable_peripheral_clock(sys_cfg, I2c::PERIPH_SEL);
        assert_periph_reset(sys_cfg, I2c::PERIPH_SEL);
        cortex_m::asm::nop();
        cortex_m::asm::nop();
        deassert_periph_reset(sys_cfg, I2c::PERIPH_SEL);

        let mut i2c_base = I2cBase {
            i2c,
            clock: clocks.apb1(),
        };
        if let Some(ms_cfg) = ms_cfg {
            i2c_base.cfg_master(ms_cfg);
        }

        if let Some(sl_cfg) = sl_cfg {
            i2c_base.cfg_slave(sl_cfg);
        }
        i2c_base.cfg_clk_scale(speed_mode)?;
        Ok(i2c_base)
    }

    fn cfg_master(&mut self, ms_cfg: &MasterConfig) {
        let (txfemd, rxfemd) = match (ms_cfg.tx_fe_mode, ms_cfg.rx_fe_mode) {
            (FifoEmptyMode::Stall, FifoEmptyMode::Stall) => (false, false),
            (FifoEmptyMode::Stall, FifoEmptyMode::EndTransaction) => (false, true),
            (FifoEmptyMode::EndTransaction, FifoEmptyMode::Stall) => (true, false),
            (FifoEmptyMode::EndTransaction, FifoEmptyMode::EndTransaction) => (true, true),
        };
        self.i2c.ctrl().modify(|_, w| {
            w.txfemd().bit(txfemd);
            w.rxffmd().bit(rxfemd);
            w.dlgfilter().bit(ms_cfg.dlg_filt);
            w.algfilter().bit(ms_cfg.alg_filt)
        });
        if let Some(ref tm_cfg) = ms_cfg.tm_cfg {
            self.i2c
                .tmconfig()
                .write(|w| unsafe { w.bits(tm_cfg.reg()) });
        }
        self.i2c.fifo_clr().write(|w| {
            w.rxfifo().set_bit();
            w.txfifo().set_bit()
        });
    }

    fn cfg_slave(&mut self, sl_cfg: &SlaveConfig) {
        let (txfemd, rxfemd) = match (sl_cfg.tx_fe_mode, sl_cfg.rx_fe_mode) {
            (FifoEmptyMode::Stall, FifoEmptyMode::Stall) => (false, false),
            (FifoEmptyMode::Stall, FifoEmptyMode::EndTransaction) => (false, true),
            (FifoEmptyMode::EndTransaction, FifoEmptyMode::Stall) => (true, false),
            (FifoEmptyMode::EndTransaction, FifoEmptyMode::EndTransaction) => (true, true),
        };
        self.i2c.s0_ctrl().modify(|_, w| {
            w.txfemd().bit(txfemd);
            w.rxffmd().bit(rxfemd)
        });
        self.i2c.s0_fifo_clr().write(|w| {
            w.rxfifo().set_bit();
            w.txfifo().set_bit()
        });
        let max_words = sl_cfg.max_words;
        if let Some(max_words) = max_words {
            self.i2c
                .s0_maxwords()
                .write(|w| unsafe { w.bits(1 << 31 | max_words as u32) });
        }
        let (addr, addr_mode_mask) = Self::unwrap_addr(sl_cfg.addr);
        // The first bit is the read/write value. Normally, both read and write are matched
        // using the RWMASK bit of the address mask register
        self.i2c
            .s0_address()
            .write(|w| unsafe { w.bits((addr << 1) as u32 | addr_mode_mask) });
        if let Some(addr_mask) = sl_cfg.addr_mask {
            self.i2c
                .s0_addressmask()
                .write(|w| unsafe { w.bits((addr_mask << 1) as u32) });
        }
        if let Some(addr_b) = sl_cfg.addr_b {
            let (addr, addr_mode_mask) = Self::unwrap_addr(addr_b);
            self.i2c
                .s0_addressb()
                .write(|w| unsafe { w.bits((addr << 1) as u32 | addr_mode_mask) })
        }
        if let Some(addr_b_mask) = sl_cfg.addr_b_mask {
            self.i2c
                .s0_addressmaskb()
                .write(|w| unsafe { w.bits((addr_b_mask << 1) as u32) })
        }
    }

    #[inline]
    pub fn filters(&mut self, digital_filt: bool, analog_filt: bool) {
        self.i2c.ctrl().modify(|_, w| {
            w.dlgfilter().bit(digital_filt);
            w.algfilter().bit(analog_filt)
        });
    }

    #[inline]
    pub fn fifo_empty_mode(&mut self, rx: FifoEmptyMode, tx: FifoEmptyMode) {
        self.i2c.ctrl().modify(|_, w| {
            w.txfemd().bit(tx as u8 != 0);
            w.rxffmd().bit(rx as u8 != 0)
        });
    }

    fn calc_clk_div(&self, speed_mode: I2cSpeed) -> Result<u8, ClockTooSlowForFastI2c> {
        if speed_mode == I2cSpeed::Regular100khz {
            Ok(((self.clock.raw() / CLK_100K.raw() / 20) - 1) as u8)
        } else {
            if self.clock.raw() < MIN_CLK_400K.raw() {
                return Err(ClockTooSlowForFastI2c);
            }
            Ok(((self.clock.raw() / CLK_400K.raw() / 25) - 1) as u8)
        }
    }

    /// Configures the clock scale for a given speed mode setting
    pub fn cfg_clk_scale(&mut self, speed_mode: I2cSpeed) -> Result<(), ClockTooSlowForFastI2c> {
        let clk_div = self.calc_clk_div(speed_mode)?;
        self.i2c
            .clkscale()
            .write(|w| unsafe { w.bits((speed_mode as u32) << 31 | clk_div as u32) });
        Ok(())
    }

    pub fn load_address(&mut self, addr: u16) {
        // Load address
        self.i2c
            .address()
            .write(|w| unsafe { w.bits((addr << 1) as u32) });
    }

    #[inline]
    fn stop_cmd(&mut self) {
        self.i2c
            .cmd()
            .write(|w| unsafe { w.bits(I2cCmd::Stop as u32) });
    }
}

//==================================================================================================
// I2C Master
//==================================================================================================

pub struct I2cMaster<I2c, Addr = SevenBitAddress> {
    i2c_base: I2cBase<I2c>,
    addr: PhantomData<Addr>,
}

impl<I2c: Instance, Addr> I2cMaster<I2c, Addr> {
    pub fn new(
        i2c: I2c,
        sys_cfg: &mut pac::Sysconfig,
        cfg: MasterConfig,
        clocks: &Clocks,
        speed_mode: I2cSpeed,
    ) -> Result<Self, ClockTooSlowForFastI2c> {
        Ok(I2cMaster {
            i2c_base: I2cBase::new(i2c, sys_cfg, clocks, speed_mode, Some(&cfg), None)?,
            addr: PhantomData,
        }
        .enable_master())
    }

    #[inline]
    pub fn cancel_transfer(&self) {
        self.i2c_base
            .i2c
            .cmd()
            .write(|w| unsafe { w.bits(I2cCmd::Cancel as u32) });
    }

    #[inline]
    pub fn clear_tx_fifo(&self) {
        self.i2c_base.i2c.fifo_clr().write(|w| w.txfifo().set_bit());
    }

    #[inline]
    pub fn clear_rx_fifo(&self) {
        self.i2c_base.i2c.fifo_clr().write(|w| w.rxfifo().set_bit());
    }

    #[inline]
    pub fn enable_master(self) -> Self {
        self.i2c_base.i2c.ctrl().modify(|_, w| w.enable().set_bit());
        self
    }

    #[inline]
    pub fn disable_master(self) -> Self {
        self.i2c_base
            .i2c
            .ctrl()
            .modify(|_, w| w.enable().clear_bit());
        self
    }

    #[inline(always)]
    fn load_fifo(&self, word: u8) {
        self.i2c_base
            .i2c
            .data()
            .write(|w| unsafe { w.bits(word as u32) });
    }

    #[inline(always)]
    fn read_fifo(&self) -> u8 {
        self.i2c_base.i2c.data().read().bits() as u8
    }

    fn error_handler_write(&mut self, init_cmd: &I2cCmd) {
        self.clear_tx_fifo();
        if *init_cmd == I2cCmd::Start {
            self.i2c_base.stop_cmd()
        }
    }

    fn write_base(
        &mut self,
        addr: I2cAddress,
        init_cmd: I2cCmd,
        bytes: impl IntoIterator<Item = u8>,
    ) -> Result<(), Error> {
        let mut iter = bytes.into_iter();
        // Load address
        let (addr, addr_mode_bit) = I2cBase::<I2c>::unwrap_addr(addr);
        self.i2c_base.i2c.address().write(|w| unsafe {
            w.bits(I2cDirection::Send as u32 | (addr << 1) as u32 | addr_mode_bit)
        });

        self.i2c_base
            .i2c
            .cmd()
            .write(|w| unsafe { w.bits(init_cmd as u32) });
        let mut load_if_next_available = || {
            if let Some(next_byte) = iter.next() {
                self.load_fifo(next_byte);
            }
        };
        loop {
            let status_reader = self.i2c_base.i2c.status().read();
            if status_reader.arblost().bit_is_set() {
                self.error_handler_write(&init_cmd);
                return Err(Error::ArbitrationLost);
            } else if status_reader.nackaddr().bit_is_set() {
                self.error_handler_write(&init_cmd);
                return Err(Error::NackAddr);
            } else if status_reader.nackdata().bit_is_set() {
                self.error_handler_write(&init_cmd);
                return Err(Error::NackData);
            } else if status_reader.idle().bit_is_set() {
                return Ok(());
            } else {
                while !status_reader.txnfull().bit_is_set() {
                    load_if_next_available();
                }
            }
        }
    }

    fn write_from_buffer(
        &mut self,
        init_cmd: I2cCmd,
        addr: I2cAddress,
        output: &[u8],
    ) -> Result<(), Error> {
        let len = output.len();
        // It should theoretically possible to transfer larger data sizes by tracking
        // the number of sent words and setting it to 0x7fe as soon as only that many
        // bytes are remaining. However, large transfer like this are not common. This
        // feature will therefore not be supported for now.
        if len > 0x7fe {
            return Err(Error::DataTooLarge);
        }
        // Load number of words
        self.i2c_base
            .i2c
            .words()
            .write(|w| unsafe { w.bits(len as u32) });
        let mut bytes = output.iter();
        // FIFO has a depth of 16. We load slightly above the trigger level
        // but not all of it because the transaction might fail immediately
        const FILL_DEPTH: usize = 12;

        // load the FIFO
        for _ in 0..core::cmp::min(FILL_DEPTH, len) {
            self.load_fifo(*bytes.next().unwrap());
        }

        self.write_base(addr, init_cmd, output.iter().cloned())
    }

    fn read_internal(&mut self, addr: I2cAddress, buffer: &mut [u8]) -> Result<(), Error> {
        let len = buffer.len();
        // It should theoretically possible to transfer larger data sizes by tracking
        // the number of sent words and setting it to 0x7fe as soon as only that many
        // bytes are remaining. However, large transfer like this are not common. This
        // feature will therefore not be supported for now.
        if len > 0x7fe {
            return Err(Error::DataTooLarge);
        }
        // Clear the receive FIFO
        self.clear_rx_fifo();

        // Load number of words
        self.i2c_base
            .i2c
            .words()
            .write(|w| unsafe { w.bits(len as u32) });
        let (addr, addr_mode_bit) = match addr {
            I2cAddress::Regular(addr) => (addr as u16, 0 << 15),
            I2cAddress::TenBit(addr) => (addr, 1 << 15),
        };
        // Load address
        self.i2c_base.i2c.address().write(|w| unsafe {
            w.bits(I2cDirection::Read as u32 | (addr << 1) as u32 | addr_mode_bit)
        });

        let mut buf_iter = buffer.iter_mut();
        let mut read_bytes = 0;
        // Start receive transfer
        self.i2c_base
            .i2c
            .cmd()
            .write(|w| unsafe { w.bits(I2cCmd::StartWithStop as u32) });
        let mut read_if_next_available = || {
            if let Some(next_byte) = buf_iter.next() {
                *next_byte = self.read_fifo();
            }
        };
        loop {
            let status_reader = self.i2c_base.i2c.status().read();
            if status_reader.arblost().bit_is_set() {
                self.clear_rx_fifo();
                return Err(Error::ArbitrationLost);
            } else if status_reader.nackaddr().bit_is_set() {
                self.clear_rx_fifo();
                return Err(Error::NackAddr);
            } else if status_reader.idle().bit_is_set() {
                if read_bytes != len {
                    return Err(Error::InsufficientDataReceived);
                }
                return Ok(());
            } else if status_reader.rxnempty().bit_is_set() {
                read_if_next_available();
                read_bytes += 1;
            }
        }
    }
}

//======================================================================================
// Embedded HAL I2C implementations
//======================================================================================

impl<I2c> embedded_hal::i2c::ErrorType for I2cMaster<I2c, SevenBitAddress> {
    type Error = Error;
}

impl<I2c: Instance> embedded_hal::i2c::I2c for I2cMaster<I2c, SevenBitAddress> {
    fn transaction(
        &mut self,
        address: SevenBitAddress,
        operations: &mut [Operation<'_>],
    ) -> Result<(), Self::Error> {
        for operation in operations {
            match operation {
                Operation::Read(buf) => self.read_internal(I2cAddress::Regular(address), buf)?,
                Operation::Write(buf) => self.write_from_buffer(
                    I2cCmd::StartWithStop,
                    I2cAddress::Regular(address),
                    buf,
                )?,
            }
        }
        Ok(())
    }
}

impl<I2c> embedded_hal::i2c::ErrorType for I2cMaster<I2c, TenBitAddress> {
    type Error = Error;
}

impl<I2c: Instance> embedded_hal::i2c::I2c<TenBitAddress> for I2cMaster<I2c, TenBitAddress> {
    fn transaction(
        &mut self,
        address: TenBitAddress,
        operations: &mut [Operation<'_>],
    ) -> Result<(), Self::Error> {
        for operation in operations {
            match operation {
                Operation::Read(buf) => self.read_internal(I2cAddress::TenBit(address), buf)?,
                Operation::Write(buf) => {
                    self.write_from_buffer(I2cCmd::StartWithStop, I2cAddress::TenBit(address), buf)?
                }
            }
        }
        Ok(())
    }
}

//==================================================================================================
// I2C Slave
//==================================================================================================

pub struct I2cSlave<I2c, Addr = SevenBitAddress> {
    i2c_base: I2cBase<I2c>,
    addr: PhantomData<Addr>,
}

impl<I2c: Instance, Addr> I2cSlave<I2c, Addr> {
    fn new_generic(
        i2c: I2c,
        sys_cfg: &mut pac::Sysconfig,
        cfg: SlaveConfig,
        clocks: &Clocks,
        speed_mode: I2cSpeed,
    ) -> Result<Self, ClockTooSlowForFastI2c> {
        Ok(I2cSlave {
            i2c_base: I2cBase::new(i2c, sys_cfg, clocks, speed_mode, None, Some(&cfg))?,
            addr: PhantomData,
        }
        .enable_slave())
    }

    #[inline]
    pub fn enable_slave(self) -> Self {
        self.i2c_base
            .i2c
            .s0_ctrl()
            .modify(|_, w| w.enable().set_bit());
        self
    }

    #[inline]
    pub fn disable_slave(self) -> Self {
        self.i2c_base
            .i2c
            .s0_ctrl()
            .modify(|_, w| w.enable().clear_bit());
        self
    }

    #[inline(always)]
    fn load_fifo(&self, word: u8) {
        self.i2c_base
            .i2c
            .s0_data()
            .write(|w| unsafe { w.bits(word as u32) });
    }

    #[inline(always)]
    fn read_fifo(&self) -> u8 {
        self.i2c_base.i2c.s0_data().read().bits() as u8
    }

    #[inline]
    fn clear_tx_fifo(&self) {
        self.i2c_base
            .i2c
            .s0_fifo_clr()
            .write(|w| w.txfifo().set_bit());
    }

    #[inline]
    fn clear_rx_fifo(&self) {
        self.i2c_base
            .i2c
            .s0_fifo_clr()
            .write(|w| w.rxfifo().set_bit());
    }

    /// Get the last address that was matched by the slave control and the corresponding
    /// master direction
    pub fn last_address(&self) -> (I2cDirection, u32) {
        let bits = self.i2c_base.i2c.s0_lastaddress().read().bits();
        match bits & 0x01 {
            0 => (I2cDirection::Send, bits >> 1),
            1 => (I2cDirection::Read, bits >> 1),
            _ => (I2cDirection::Send, bits >> 1),
        }
    }

    pub fn write(&mut self, output: &[u8]) -> Result<(), Error> {
        let len = output.len();
        // It should theoretically possible to transfer larger data sizes by tracking
        // the number of sent words and setting it to 0x7fe as soon as only that many
        // bytes are remaining. However, large transfer like this are not common. This
        // feature will therefore not be supported for now.
        if len > 0x7fe {
            return Err(Error::DataTooLarge);
        }
        let mut bytes = output.iter();
        // FIFO has a depth of 16. We load slightly above the trigger level
        // but not all of it because the transaction might fail immediately
        const FILL_DEPTH: usize = 12;

        // load the FIFO
        for _ in 0..core::cmp::min(FILL_DEPTH, len) {
            self.load_fifo(*bytes.next().unwrap());
        }

        let status_reader = self.i2c_base.i2c.s0_status().read();
        let mut load_if_next_available = || {
            if let Some(next_byte) = bytes.next() {
                self.load_fifo(*next_byte);
            }
        };
        loop {
            if status_reader.nackdata().bit_is_set() {
                self.clear_tx_fifo();
                return Err(Error::NackData);
            } else if status_reader.idle().bit_is_set() {
                return Ok(());
            } else {
                while !status_reader.txnfull().bit_is_set() {
                    load_if_next_available();
                }
            }
        }
    }

    pub fn read(&mut self, buffer: &mut [u8]) -> Result<(), Error> {
        let len = buffer.len();
        // It should theoretically possible to transfer larger data sizes by tracking
        // the number of sent words and setting it to 0x7fe as soon as only that many
        // bytes are remaining. However, large transfer like this are not common. This
        // feature will therefore not be supported for now.
        if len > 0x7fe {
            return Err(Error::DataTooLarge);
        }
        // Clear the receive FIFO
        self.clear_rx_fifo();

        let mut buf_iter = buffer.iter_mut();
        let mut read_bytes = 0;
        let mut read_if_next_available = || {
            if let Some(next_byte) = buf_iter.next() {
                *next_byte = self.read_fifo();
            }
        };
        loop {
            let status_reader = self.i2c_base.i2c.s0_status().read();
            if status_reader.idle().bit_is_set() {
                if read_bytes != len {
                    return Err(Error::InsufficientDataReceived);
                }
                return Ok(());
            } else if status_reader.rxnempty().bit_is_set() {
                read_bytes += 1;
                read_if_next_available();
            }
        }
    }
}

impl<I2c: Instance> I2cSlave<I2c, SevenBitAddress> {
    /// Create a new I2C slave for seven bit addresses
    pub fn new(
        i2c: I2c,
        sys_cfg: &mut pac::Sysconfig,
        cfg: SlaveConfig,
        clocks: &Clocks,
        speed_mode: I2cSpeed,
    ) -> Result<Self, InitError> {
        if let I2cAddress::TenBit(_) = cfg.addr {
            return Err(InitError::WrongAddrMode);
        }
        Ok(Self::new_generic(i2c, sys_cfg, cfg, clocks, speed_mode)?)
    }
}

impl<I2c: Instance> I2cSlave<I2c, TenBitAddress> {
    pub fn new_ten_bit_addr(
        i2c: I2c,
        sys_cfg: &mut pac::Sysconfig,
        cfg: SlaveConfig,
        clocks: &Clocks,
        speed_mode: I2cSpeed,
    ) -> Result<Self, ClockTooSlowForFastI2c> {
        Self::new_generic(i2c, sys_cfg, cfg, clocks, speed_mode)
    }
}
