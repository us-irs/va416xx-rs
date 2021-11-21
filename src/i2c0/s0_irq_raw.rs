#[doc = "Register `S0_IRQ_RAW` reader"]
pub struct R(crate::R<S0_IRQ_RAW_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<S0_IRQ_RAW_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<S0_IRQ_RAW_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<S0_IRQ_RAW_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `COMPLETED` reader - Controller Complted a Transaction"]
pub struct COMPLETED_R(crate::FieldReader<bool, bool>);
impl COMPLETED_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        COMPLETED_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for COMPLETED_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IDLE` reader - Controller is Idle"]
pub struct IDLE_R(crate::FieldReader<bool, bool>);
impl IDLE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        IDLE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IDLE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WAITING` reader - Controller is Waiting"]
pub struct WAITING_R(crate::FieldReader<bool, bool>);
impl WAITING_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        WAITING_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WAITING_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXSTALLED` reader - Controller is Tx Stalled"]
pub struct TXSTALLED_R(crate::FieldReader<bool, bool>);
impl TXSTALLED_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TXSTALLED_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXSTALLED_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXSTALLED` reader - Controller is Rx Stalled"]
pub struct RXSTALLED_R(crate::FieldReader<bool, bool>);
impl RXSTALLED_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RXSTALLED_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXSTALLED_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADDRESSMATCH` reader - I2C Address Match"]
pub struct ADDRESSMATCH_R(crate::FieldReader<bool, bool>);
impl ADDRESSMATCH_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ADDRESSMATCH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADDRESSMATCH_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NACKDATA` reader - I2C Data was not Acknowledged"]
pub struct NACKDATA_R(crate::FieldReader<bool, bool>);
impl NACKDATA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        NACKDATA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NACKDATA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXDATAFIRST` reader - Pending Data is first Byte following Address"]
pub struct RXDATAFIRST_R(crate::FieldReader<bool, bool>);
impl RXDATAFIRST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RXDATAFIRST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXDATAFIRST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `I2C_START` reader - I2C Start Condition"]
pub struct I2C_START_R(crate::FieldReader<bool, bool>);
impl I2C_START_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        I2C_START_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for I2C_START_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `I2C_STOP` reader - I2C Stop Condition"]
pub struct I2C_STOP_R(crate::FieldReader<bool, bool>);
impl I2C_STOP_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        I2C_STOP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for I2C_STOP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXUNDERFLOW` reader - TX FIFO Underflowed"]
pub struct TXUNDERFLOW_R(crate::FieldReader<bool, bool>);
impl TXUNDERFLOW_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TXUNDERFLOW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXUNDERFLOW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXOVERFLOW` reader - TX FIFO Overflowed"]
pub struct RXOVERFLOW_R(crate::FieldReader<bool, bool>);
impl RXOVERFLOW_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RXOVERFLOW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXOVERFLOW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXREADY` reader - TX FIFO Ready"]
pub struct TXREADY_R(crate::FieldReader<bool, bool>);
impl TXREADY_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TXREADY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXREADY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXREADY` reader - RX FIFO Ready"]
pub struct RXREADY_R(crate::FieldReader<bool, bool>);
impl RXREADY_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RXREADY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXREADY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXEMPTY` reader - TX FIFO Empty"]
pub struct TXEMPTY_R(crate::FieldReader<bool, bool>);
impl TXEMPTY_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TXEMPTY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXEMPTY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXFULL` reader - RX FIFO Full"]
pub struct RXFULL_R(crate::FieldReader<bool, bool>);
impl RXFULL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RXFULL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXFULL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - Controller Complted a Transaction"]
    #[inline(always)]
    pub fn completed(&self) -> COMPLETED_R {
        COMPLETED_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Controller is Idle"]
    #[inline(always)]
    pub fn idle(&self) -> IDLE_R {
        IDLE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Controller is Waiting"]
    #[inline(always)]
    pub fn waiting(&self) -> WAITING_R {
        WAITING_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Controller is Tx Stalled"]
    #[inline(always)]
    pub fn txstalled(&self) -> TXSTALLED_R {
        TXSTALLED_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Controller is Rx Stalled"]
    #[inline(always)]
    pub fn rxstalled(&self) -> RXSTALLED_R {
        RXSTALLED_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - I2C Address Match"]
    #[inline(always)]
    pub fn addressmatch(&self) -> ADDRESSMATCH_R {
        ADDRESSMATCH_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - I2C Data was not Acknowledged"]
    #[inline(always)]
    pub fn nackdata(&self) -> NACKDATA_R {
        NACKDATA_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Pending Data is first Byte following Address"]
    #[inline(always)]
    pub fn rxdatafirst(&self) -> RXDATAFIRST_R {
        RXDATAFIRST_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - I2C Start Condition"]
    #[inline(always)]
    pub fn i2c_start(&self) -> I2C_START_R {
        I2C_START_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - I2C Stop Condition"]
    #[inline(always)]
    pub fn i2c_stop(&self) -> I2C_STOP_R {
        I2C_STOP_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - TX FIFO Underflowed"]
    #[inline(always)]
    pub fn txunderflow(&self) -> TXUNDERFLOW_R {
        TXUNDERFLOW_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - TX FIFO Overflowed"]
    #[inline(always)]
    pub fn rxoverflow(&self) -> RXOVERFLOW_R {
        RXOVERFLOW_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - TX FIFO Ready"]
    #[inline(always)]
    pub fn txready(&self) -> TXREADY_R {
        TXREADY_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - RX FIFO Ready"]
    #[inline(always)]
    pub fn rxready(&self) -> RXREADY_R {
        RXREADY_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - TX FIFO Empty"]
    #[inline(always)]
    pub fn txempty(&self) -> TXEMPTY_R {
        TXEMPTY_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - RX FIFO Full"]
    #[inline(always)]
    pub fn rxfull(&self) -> RXFULL_R {
        RXFULL_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
#[doc = "Slave Raw Interrupt Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [s0_irq_raw](index.html) module"]
pub struct S0_IRQ_RAW_SPEC;
impl crate::RegisterSpec for S0_IRQ_RAW_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [s0_irq_raw::R](R) reader structure"]
impl crate::Readable for S0_IRQ_RAW_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets S0_IRQ_RAW to value 0"]
impl crate::Resettable for S0_IRQ_RAW_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
