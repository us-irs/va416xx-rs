#[doc = "Register `STATUS` reader"]
pub struct R(crate::R<STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `STATUS` writer"]
pub struct W(crate::W<STATUS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<STATUS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<STATUS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `I2CIDLE` reader - I2C bus is idle"]
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
#[doc = "Field `I2CIDLE` writer - I2C bus is idle"]
pub struct I2CIDLE_W<'a> {
    w: &'a mut W,
}
impl<'a> I2CIDLE_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
#[doc = "Field `IDLE` reader - I2C controller is Idle"]
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
#[doc = "Field `IDLE` writer - I2C controller is Idle"]
pub struct IDLE_W<'a> {
    w: &'a mut W,
}
impl<'a> IDLE_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
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
#[doc = "Field `WAITING` writer - Controller is Waiting"]
pub struct WAITING_W<'a> {
    w: &'a mut W,
}
impl<'a> WAITING_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
        self.w
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
#[doc = "Field `STALLED` writer - Controller is Stalled"]
pub struct STALLED_W<'a> {
    w: &'a mut W,
}
impl<'a> STALLED_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u32 & 0x01) << 3);
        self.w
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
#[doc = "Field `ARBLOST` writer - I2C Arbitration was lost"]
pub struct ARBLOST_W<'a> {
    w: &'a mut W,
}
impl<'a> ARBLOST_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u32 & 0x01) << 4);
        self.w
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
#[doc = "Field `NACKADDR` writer - I2C Address was not Acknowledged"]
pub struct NACKADDR_W<'a> {
    w: &'a mut W,
}
impl<'a> NACKADDR_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u32 & 0x01) << 5);
        self.w
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
#[doc = "Field `NACKDATA` writer - I2C Data was not Acknowledged"]
pub struct NACKDATA_W<'a> {
    w: &'a mut W,
}
impl<'a> NACKDATA_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 6)) | ((value as u32 & 0x01) << 6);
        self.w
    }
}
#[doc = "Field `RXNEMPTY` reader - RX FIFO is Not Empty"]
pub struct RXNEMPTY_R(crate::FieldReader<bool, bool>);
impl RXNEMPTY_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RXNEMPTY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXNEMPTY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXNEMPTY` writer - RX FIFO is Not Empty"]
pub struct RXNEMPTY_W<'a> {
    w: &'a mut W,
}
impl<'a> RXNEMPTY_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 8)) | ((value as u32 & 0x01) << 8);
        self.w
    }
}
#[doc = "Field `RXFULL` reader - RX FIFO is Full"]
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
#[doc = "Field `RXFULL` writer - RX FIFO is Full"]
pub struct RXFULL_W<'a> {
    w: &'a mut W,
}
impl<'a> RXFULL_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 9)) | ((value as u32 & 0x01) << 9);
        self.w
    }
}
#[doc = "Field `RXTRIGGER` reader - RX FIFO Above Trigger Level"]
pub struct RXTRIGGER_R(crate::FieldReader<bool, bool>);
impl RXTRIGGER_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RXTRIGGER_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXTRIGGER_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXTRIGGER` writer - RX FIFO Above Trigger Level"]
pub struct RXTRIGGER_W<'a> {
    w: &'a mut W,
}
impl<'a> RXTRIGGER_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 11)) | ((value as u32 & 0x01) << 11);
        self.w
    }
}
#[doc = "Field `TXEMPTY` reader - TX FIFO is Empty"]
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
#[doc = "Field `TXEMPTY` writer - TX FIFO is Empty"]
pub struct TXEMPTY_W<'a> {
    w: &'a mut W,
}
impl<'a> TXEMPTY_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 12)) | ((value as u32 & 0x01) << 12);
        self.w
    }
}
#[doc = "Field `TXNFULL` reader - TX FIFO is Full"]
pub struct TXNFULL_R(crate::FieldReader<bool, bool>);
impl TXNFULL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TXNFULL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXNFULL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXNFULL` writer - TX FIFO is Full"]
pub struct TXNFULL_W<'a> {
    w: &'a mut W,
}
impl<'a> TXNFULL_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 13)) | ((value as u32 & 0x01) << 13);
        self.w
    }
}
#[doc = "Field `TXTRIGGER` reader - TX FIFO Below Trigger Level"]
pub struct TXTRIGGER_R(crate::FieldReader<bool, bool>);
impl TXTRIGGER_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TXTRIGGER_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXTRIGGER_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXTRIGGER` writer - TX FIFO Below Trigger Level"]
pub struct TXTRIGGER_W<'a> {
    w: &'a mut W,
}
impl<'a> TXTRIGGER_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 15)) | ((value as u32 & 0x01) << 15);
        self.w
    }
}
#[doc = "Field `RAW_SDA` reader - I2C Raw SDA value"]
pub struct RAW_SDA_R(crate::FieldReader<bool, bool>);
impl RAW_SDA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RAW_SDA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RAW_SDA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RAW_SDA` writer - I2C Raw SDA value"]
pub struct RAW_SDA_W<'a> {
    w: &'a mut W,
}
impl<'a> RAW_SDA_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 30)) | ((value as u32 & 0x01) << 30);
        self.w
    }
}
#[doc = "Field `RAW_SCL` reader - I2C Raw SCL value"]
pub struct RAW_SCL_R(crate::FieldReader<bool, bool>);
impl RAW_SCL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RAW_SCL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RAW_SCL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RAW_SCL` writer - I2C Raw SCL value"]
pub struct RAW_SCL_W<'a> {
    w: &'a mut W,
}
impl<'a> RAW_SCL_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 31)) | ((value as u32 & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - I2C bus is idle"]
    #[inline(always)]
    pub fn i2cidle(&self) -> I2CIDLE_R {
        I2CIDLE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - I2C controller is Idle"]
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
    #[doc = "Bit 8 - RX FIFO is Not Empty"]
    #[inline(always)]
    pub fn rxnempty(&self) -> RXNEMPTY_R {
        RXNEMPTY_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - RX FIFO is Full"]
    #[inline(always)]
    pub fn rxfull(&self) -> RXFULL_R {
        RXFULL_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 11 - RX FIFO Above Trigger Level"]
    #[inline(always)]
    pub fn rxtrigger(&self) -> RXTRIGGER_R {
        RXTRIGGER_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - TX FIFO is Empty"]
    #[inline(always)]
    pub fn txempty(&self) -> TXEMPTY_R {
        TXEMPTY_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - TX FIFO is Full"]
    #[inline(always)]
    pub fn txnfull(&self) -> TXNFULL_R {
        TXNFULL_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 15 - TX FIFO Below Trigger Level"]
    #[inline(always)]
    pub fn txtrigger(&self) -> TXTRIGGER_R {
        TXTRIGGER_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 30 - I2C Raw SDA value"]
    #[inline(always)]
    pub fn raw_sda(&self) -> RAW_SDA_R {
        RAW_SDA_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - I2C Raw SCL value"]
    #[inline(always)]
    pub fn raw_scl(&self) -> RAW_SCL_R {
        RAW_SCL_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - I2C bus is idle"]
    #[inline(always)]
    pub fn i2cidle(&mut self) -> I2CIDLE_W {
        I2CIDLE_W { w: self }
    }
    #[doc = "Bit 1 - I2C controller is Idle"]
    #[inline(always)]
    pub fn idle(&mut self) -> IDLE_W {
        IDLE_W { w: self }
    }
    #[doc = "Bit 2 - Controller is Waiting"]
    #[inline(always)]
    pub fn waiting(&mut self) -> WAITING_W {
        WAITING_W { w: self }
    }
    #[doc = "Bit 3 - Controller is Stalled"]
    #[inline(always)]
    pub fn stalled(&mut self) -> STALLED_W {
        STALLED_W { w: self }
    }
    #[doc = "Bit 4 - I2C Arbitration was lost"]
    #[inline(always)]
    pub fn arblost(&mut self) -> ARBLOST_W {
        ARBLOST_W { w: self }
    }
    #[doc = "Bit 5 - I2C Address was not Acknowledged"]
    #[inline(always)]
    pub fn nackaddr(&mut self) -> NACKADDR_W {
        NACKADDR_W { w: self }
    }
    #[doc = "Bit 6 - I2C Data was not Acknowledged"]
    #[inline(always)]
    pub fn nackdata(&mut self) -> NACKDATA_W {
        NACKDATA_W { w: self }
    }
    #[doc = "Bit 8 - RX FIFO is Not Empty"]
    #[inline(always)]
    pub fn rxnempty(&mut self) -> RXNEMPTY_W {
        RXNEMPTY_W { w: self }
    }
    #[doc = "Bit 9 - RX FIFO is Full"]
    #[inline(always)]
    pub fn rxfull(&mut self) -> RXFULL_W {
        RXFULL_W { w: self }
    }
    #[doc = "Bit 11 - RX FIFO Above Trigger Level"]
    #[inline(always)]
    pub fn rxtrigger(&mut self) -> RXTRIGGER_W {
        RXTRIGGER_W { w: self }
    }
    #[doc = "Bit 12 - TX FIFO is Empty"]
    #[inline(always)]
    pub fn txempty(&mut self) -> TXEMPTY_W {
        TXEMPTY_W { w: self }
    }
    #[doc = "Bit 13 - TX FIFO is Full"]
    #[inline(always)]
    pub fn txnfull(&mut self) -> TXNFULL_W {
        TXNFULL_W { w: self }
    }
    #[doc = "Bit 15 - TX FIFO Below Trigger Level"]
    #[inline(always)]
    pub fn txtrigger(&mut self) -> TXTRIGGER_W {
        TXTRIGGER_W { w: self }
    }
    #[doc = "Bit 30 - I2C Raw SDA value"]
    #[inline(always)]
    pub fn raw_sda(&mut self) -> RAW_SDA_W {
        RAW_SDA_W { w: self }
    }
    #[doc = "Bit 31 - I2C Raw SCL value"]
    #[inline(always)]
    pub fn raw_scl(&mut self) -> RAW_SCL_W {
        RAW_SCL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "I2C Controller Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [status](index.html) module"]
pub struct STATUS_SPEC;
impl crate::RegisterSpec for STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [status::R](R) reader structure"]
impl crate::Readable for STATUS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [status::W](W) writer structure"]
impl crate::Writable for STATUS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets STATUS to value 0"]
impl crate::Resettable for STATUS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
