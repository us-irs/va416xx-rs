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
#[doc = "Field `IRQ_RX` reader - RX Interrupt"]
pub struct IRQ_RX_R(crate::FieldReader<bool, bool>);
impl IRQ_RX_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        IRQ_RX_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IRQ_RX_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IRQ_RX` writer - RX Interrupt"]
pub struct IRQ_RX_W<'a> {
    w: &'a mut W,
}
impl<'a> IRQ_RX_W<'a> {
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
#[doc = "Field `IRQ_RX_STATUS` reader - RX Status Interrupt"]
pub struct IRQ_RX_STATUS_R(crate::FieldReader<bool, bool>);
impl IRQ_RX_STATUS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        IRQ_RX_STATUS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IRQ_RX_STATUS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IRQ_RX_STATUS` writer - RX Status Interrupt"]
pub struct IRQ_RX_STATUS_W<'a> {
    w: &'a mut W,
}
impl<'a> IRQ_RX_STATUS_W<'a> {
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
#[doc = "Field `IRQ_RX_TO` reader - RX Timeout Interrupt"]
pub struct IRQ_RX_TO_R(crate::FieldReader<bool, bool>);
impl IRQ_RX_TO_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        IRQ_RX_TO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IRQ_RX_TO_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IRQ_RX_TO` writer - RX Timeout Interrupt"]
pub struct IRQ_RX_TO_W<'a> {
    w: &'a mut W,
}
impl<'a> IRQ_RX_TO_W<'a> {
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
#[doc = "Field `IRQ_TX` reader - TX Interrupt"]
pub struct IRQ_TX_R(crate::FieldReader<bool, bool>);
impl IRQ_TX_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        IRQ_TX_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IRQ_TX_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IRQ_TX` writer - TX Interrupt"]
pub struct IRQ_TX_W<'a> {
    w: &'a mut W,
}
impl<'a> IRQ_TX_W<'a> {
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
#[doc = "Field `IRQ_TX_STATUS` reader - TX Status Interrupt"]
pub struct IRQ_TX_STATUS_R(crate::FieldReader<bool, bool>);
impl IRQ_TX_STATUS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        IRQ_TX_STATUS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IRQ_TX_STATUS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IRQ_TX_STATUS` writer - TX Status Interrupt"]
pub struct IRQ_TX_STATUS_W<'a> {
    w: &'a mut W,
}
impl<'a> IRQ_TX_STATUS_W<'a> {
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
#[doc = "Field `IRQ_TX_EMPTY` reader - TX Empty Interrupt"]
pub struct IRQ_TX_EMPTY_R(crate::FieldReader<bool, bool>);
impl IRQ_TX_EMPTY_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        IRQ_TX_EMPTY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IRQ_TX_EMPTY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IRQ_TX_EMPTY` writer - TX Empty Interrupt"]
pub struct IRQ_TX_EMPTY_W<'a> {
    w: &'a mut W,
}
impl<'a> IRQ_TX_EMPTY_W<'a> {
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
#[doc = "Field `IRQ_TX_CTS` reader - TX CTS Change Interrupt"]
pub struct IRQ_TX_CTS_R(crate::FieldReader<bool, bool>);
impl IRQ_TX_CTS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        IRQ_TX_CTS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IRQ_TX_CTS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IRQ_TX_CTS` writer - TX CTS Change Interrupt"]
pub struct IRQ_TX_CTS_W<'a> {
    w: &'a mut W,
}
impl<'a> IRQ_TX_CTS_W<'a> {
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
impl R {
    #[doc = "Bit 0 - RX Interrupt"]
    #[inline(always)]
    pub fn irq_rx(&self) -> IRQ_RX_R {
        IRQ_RX_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - RX Status Interrupt"]
    #[inline(always)]
    pub fn irq_rx_status(&self) -> IRQ_RX_STATUS_R {
        IRQ_RX_STATUS_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - RX Timeout Interrupt"]
    #[inline(always)]
    pub fn irq_rx_to(&self) -> IRQ_RX_TO_R {
        IRQ_RX_TO_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 4 - TX Interrupt"]
    #[inline(always)]
    pub fn irq_tx(&self) -> IRQ_TX_R {
        IRQ_TX_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - TX Status Interrupt"]
    #[inline(always)]
    pub fn irq_tx_status(&self) -> IRQ_TX_STATUS_R {
        IRQ_TX_STATUS_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - TX Empty Interrupt"]
    #[inline(always)]
    pub fn irq_tx_empty(&self) -> IRQ_TX_EMPTY_R {
        IRQ_TX_EMPTY_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - TX CTS Change Interrupt"]
    #[inline(always)]
    pub fn irq_tx_cts(&self) -> IRQ_TX_CTS_R {
        IRQ_TX_CTS_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - RX Interrupt"]
    #[inline(always)]
    pub fn irq_rx(&mut self) -> IRQ_RX_W {
        IRQ_RX_W { w: self }
    }
    #[doc = "Bit 1 - RX Status Interrupt"]
    #[inline(always)]
    pub fn irq_rx_status(&mut self) -> IRQ_RX_STATUS_W {
        IRQ_RX_STATUS_W { w: self }
    }
    #[doc = "Bit 2 - RX Timeout Interrupt"]
    #[inline(always)]
    pub fn irq_rx_to(&mut self) -> IRQ_RX_TO_W {
        IRQ_RX_TO_W { w: self }
    }
    #[doc = "Bit 4 - TX Interrupt"]
    #[inline(always)]
    pub fn irq_tx(&mut self) -> IRQ_TX_W {
        IRQ_TX_W { w: self }
    }
    #[doc = "Bit 5 - TX Status Interrupt"]
    #[inline(always)]
    pub fn irq_tx_status(&mut self) -> IRQ_TX_STATUS_W {
        IRQ_TX_STATUS_W { w: self }
    }
    #[doc = "Bit 6 - TX Empty Interrupt"]
    #[inline(always)]
    pub fn irq_tx_empty(&mut self) -> IRQ_TX_EMPTY_W {
        IRQ_TX_EMPTY_W { w: self }
    }
    #[doc = "Bit 7 - TX CTS Change Interrupt"]
    #[inline(always)]
    pub fn irq_tx_cts(&mut self) -> IRQ_TX_CTS_W {
        IRQ_TX_CTS_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "IRQ Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [irq_enb](index.html) module"]
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
