#[doc = "Register `IRQ_END` reader"]
pub struct R(crate::R<IRQ_END_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IRQ_END_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IRQ_END_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IRQ_END_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `I2CIDLE` reader - I2C Bus is Idle"]
pub struct I2CIDLE_R(crate::FieldReader<bool, bool>);
impl I2CIDLE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        I2CIDLE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for I2CIDLE_R {
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
#[doc = "Field `STALLED` reader - Controller is Stalled"]
pub struct STALLED_R(crate::FieldReader<bool, bool>);
impl STALLED_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        STALLED_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for STALLED_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ARBLOST` reader - I2C Arbitration was lost"]
pub struct ARBLOST_R(crate::FieldReader<bool, bool>);
impl ARBLOST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ARBLOST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ARBLOST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NACKADDR` reader - I2C Address was not Acknowledged"]
pub struct NACKADDR_R(crate::FieldReader<bool, bool>);
impl NACKADDR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        NACKADDR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NACKADDR_R {
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
#[doc = "Field `CLKLOTO` reader - I2C Clock Low Timeout"]
pub struct CLKLOTO_R(crate::FieldReader<bool, bool>);
impl CLKLOTO_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CLKLOTO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CLKLOTO_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXOVERFLOW` reader - TX FIFO Overflowed"]
pub struct TXOVERFLOW_R(crate::FieldReader<bool, bool>);
impl TXOVERFLOW_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TXOVERFLOW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXOVERFLOW_R {
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
    #[doc = "Bit 0 - I2C Bus is Idle"]
    #[inline(always)]
    pub fn i2cidle(&self) -> I2CIDLE_R {
        I2CIDLE_R::new((self.bits & 0x01) != 0)
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
    #[doc = "Bit 3 - Controller is Stalled"]
    #[inline(always)]
    pub fn stalled(&self) -> STALLED_R {
        STALLED_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - I2C Arbitration was lost"]
    #[inline(always)]
    pub fn arblost(&self) -> ARBLOST_R {
        ARBLOST_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - I2C Address was not Acknowledged"]
    #[inline(always)]
    pub fn nackaddr(&self) -> NACKADDR_R {
        NACKADDR_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - I2C Data was not Acknowledged"]
    #[inline(always)]
    pub fn nackdata(&self) -> NACKDATA_R {
        NACKDATA_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - I2C Clock Low Timeout"]
    #[inline(always)]
    pub fn clkloto(&self) -> CLKLOTO_R {
        CLKLOTO_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 10 - TX FIFO Overflowed"]
    #[inline(always)]
    pub fn txoverflow(&self) -> TXOVERFLOW_R {
        TXOVERFLOW_R::new(((self.bits >> 10) & 0x01) != 0)
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
#[doc = "Enabled Interrupt Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [irq_end](index.html) module"]
pub struct IRQ_END_SPEC;
impl crate::RegisterSpec for IRQ_END_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [irq_end::R](R) reader structure"]
impl crate::Readable for IRQ_END_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets IRQ_END to value 0"]
impl crate::Resettable for IRQ_END_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
