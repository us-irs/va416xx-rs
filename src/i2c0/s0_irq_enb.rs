#[doc = "Register `S0_IRQ_ENB` reader"]
pub struct R(crate::R<S0_IRQ_ENB_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<S0_IRQ_ENB_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<S0_IRQ_ENB_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<S0_IRQ_ENB_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `S0_IRQ_ENB` writer"]
pub struct W(crate::W<S0_IRQ_ENB_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<S0_IRQ_ENB_SPEC>;
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
impl From<crate::W<S0_IRQ_ENB_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<S0_IRQ_ENB_SPEC>) -> Self {
        W(writer)
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
#[doc = "Field `COMPLETED` writer - Controller Complted a Transaction"]
pub struct COMPLETED_W<'a> {
    w: &'a mut W,
}
impl<'a> COMPLETED_W<'a> {
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
#[doc = "Field `TXSTALLED` writer - Controller is Tx Stalled"]
pub struct TXSTALLED_W<'a> {
    w: &'a mut W,
}
impl<'a> TXSTALLED_W<'a> {
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
#[doc = "Field `RXSTALLED` writer - Controller is Rx Stalled"]
pub struct RXSTALLED_W<'a> {
    w: &'a mut W,
}
impl<'a> RXSTALLED_W<'a> {
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
#[doc = "Field `ADDRESSMATCH` writer - I2C Address Match"]
pub struct ADDRESSMATCH_W<'a> {
    w: &'a mut W,
}
impl<'a> ADDRESSMATCH_W<'a> {
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
#[doc = "Field `RXDATAFIRST` writer - Pending Data is first Byte following Address"]
pub struct RXDATAFIRST_W<'a> {
    w: &'a mut W,
}
impl<'a> RXDATAFIRST_W<'a> {
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
#[doc = "Field `I2C_START` writer - I2C Start Condition"]
pub struct I2C_START_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C_START_W<'a> {
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
#[doc = "Field `I2C_STOP` writer - I2C Stop Condition"]
pub struct I2C_STOP_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C_STOP_W<'a> {
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
#[doc = "Field `TXUNDERFLOW` writer - TX FIFO Underflowed"]
pub struct TXUNDERFLOW_W<'a> {
    w: &'a mut W,
}
impl<'a> TXUNDERFLOW_W<'a> {
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
impl W {
    #[doc = "Bit 0 - Controller Complted a Transaction"]
    #[inline(always)]
    pub fn completed(&mut self) -> COMPLETED_W {
        COMPLETED_W { w: self }
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
    #[doc = "Bit 3 - Controller is Tx Stalled"]
    #[inline(always)]
    pub fn txstalled(&mut self) -> TXSTALLED_W {
        TXSTALLED_W { w: self }
    }
    #[doc = "Bit 4 - Controller is Rx Stalled"]
    #[inline(always)]
    pub fn rxstalled(&mut self) -> RXSTALLED_W {
        RXSTALLED_W { w: self }
    }
    #[doc = "Bit 5 - I2C Address Match"]
    #[inline(always)]
    pub fn addressmatch(&mut self) -> ADDRESSMATCH_W {
        ADDRESSMATCH_W { w: self }
    }
    #[doc = "Bit 6 - I2C Data was not Acknowledged"]
    #[inline(always)]
    pub fn nackdata(&mut self) -> NACKDATA_W {
        NACKDATA_W { w: self }
    }
    #[doc = "Bit 7 - Pending Data is first Byte following Address"]
    #[inline(always)]
    pub fn rxdatafirst(&mut self) -> RXDATAFIRST_W {
        RXDATAFIRST_W { w: self }
    }
    #[doc = "Bit 8 - I2C Start Condition"]
    #[inline(always)]
    pub fn i2c_start(&mut self) -> I2C_START_W {
        I2C_START_W { w: self }
    }
    #[doc = "Bit 9 - I2C Stop Condition"]
    #[inline(always)]
    pub fn i2c_stop(&mut self) -> I2C_STOP_W {
        I2C_STOP_W { w: self }
    }
    #[doc = "Bit 10 - TX FIFO Underflowed"]
    #[inline(always)]
    pub fn txunderflow(&mut self) -> TXUNDERFLOW_W {
        TXUNDERFLOW_W { w: self }
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
#[doc = "Slave Interrupt Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [s0_irq_enb](index.html) module"]
pub struct S0_IRQ_ENB_SPEC;
impl crate::RegisterSpec for S0_IRQ_ENB_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [s0_irq_enb::R](R) reader structure"]
impl crate::Readable for S0_IRQ_ENB_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [s0_irq_enb::W](W) writer structure"]
impl crate::Writable for S0_IRQ_ENB_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets S0_IRQ_ENB to value 0"]
impl crate::Resettable for S0_IRQ_ENB_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
