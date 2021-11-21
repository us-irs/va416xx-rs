#[doc = "Register `IRQ_ENB` reader"]
pub struct R(crate::R<IRQ_ENB_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IRQ_ENB_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IRQ_ENB_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IRQ_ENB_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IRQ_ENB` writer"]
pub struct W(crate::W<IRQ_ENB_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IRQ_ENB_SPEC>;
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
impl From<crate::W<IRQ_ENB_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IRQ_ENB_SPEC>) -> Self {
        W(writer)
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
#[doc = "Field `I2CIDLE` writer - I2C Bus is Idle"]
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
#[doc = "Field `IDLE` writer - Controller is Idle"]
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
#[doc = "Field `CLKLOTO` writer - I2C Clock Low Timeout"]
pub struct CLKLOTO_W<'a> {
    w: &'a mut W,
}
impl<'a> CLKLOTO_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | ((value as u32 & 0x01) << 7);
        self.w
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
#[doc = "Field `TXOVERFLOW` writer - TX FIFO Overflowed"]
pub struct TXOVERFLOW_W<'a> {
    w: &'a mut W,
}
impl<'a> TXOVERFLOW_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | ((value as u32 & 0x01) << 10);
        self.w
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
#[doc = "Field `RXOVERFLOW` writer - TX FIFO Overflowed"]
pub struct RXOVERFLOW_W<'a> {
    w: &'a mut W,
}
impl<'a> RXOVERFLOW_W<'a> {
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
#[doc = "Field `TXREADY` writer - TX FIFO Ready"]
pub struct TXREADY_W<'a> {
    w: &'a mut W,
}
impl<'a> TXREADY_W<'a> {
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
#[doc = "Field `RXREADY` writer - RX FIFO Ready"]
pub struct RXREADY_W<'a> {
    w: &'a mut W,
}
impl<'a> RXREADY_W<'a> {
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
#[doc = "Field `TXEMPTY` writer - TX FIFO Empty"]
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | ((value as u32 & 0x01) << 14);
        self.w
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
#[doc = "Field `RXFULL` writer - RX FIFO Full"]
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | ((value as u32 & 0x01) << 15);
        self.w
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
impl W {
    #[doc = "Bit 0 - I2C Bus is Idle"]
    #[inline(always)]
    pub fn i2cidle(&mut self) -> I2CIDLE_W {
        I2CIDLE_W { w: self }
    }
    #[doc = "Bit 1 - Controller is Idle"]
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
    #[doc = "Bit 7 - I2C Clock Low Timeout"]
    #[inline(always)]
    pub fn clkloto(&mut self) -> CLKLOTO_W {
        CLKLOTO_W { w: self }
    }
    #[doc = "Bit 10 - TX FIFO Overflowed"]
    #[inline(always)]
    pub fn txoverflow(&mut self) -> TXOVERFLOW_W {
        TXOVERFLOW_W { w: self }
    }
    #[doc = "Bit 11 - TX FIFO Overflowed"]
    #[inline(always)]
    pub fn rxoverflow(&mut self) -> RXOVERFLOW_W {
        RXOVERFLOW_W { w: self }
    }
    #[doc = "Bit 12 - TX FIFO Ready"]
    #[inline(always)]
    pub fn txready(&mut self) -> TXREADY_W {
        TXREADY_W { w: self }
    }
    #[doc = "Bit 13 - RX FIFO Ready"]
    #[inline(always)]
    pub fn rxready(&mut self) -> RXREADY_W {
        RXREADY_W { w: self }
    }
    #[doc = "Bit 14 - TX FIFO Empty"]
    #[inline(always)]
    pub fn txempty(&mut self) -> TXEMPTY_W {
        TXEMPTY_W { w: self }
    }
    #[doc = "Bit 15 - RX FIFO Full"]
    #[inline(always)]
    pub fn rxfull(&mut self) -> RXFULL_W {
        RXFULL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [irq_enb](index.html) module"]
pub struct IRQ_ENB_SPEC;
impl crate::RegisterSpec for IRQ_ENB_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [irq_enb::R](R) reader structure"]
impl crate::Readable for IRQ_ENB_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [irq_enb::W](W) writer structure"]
impl crate::Writable for IRQ_ENB_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets IRQ_ENB to value 0"]
impl crate::Resettable for IRQ_ENB_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
