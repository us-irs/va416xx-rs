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
#[doc = "Field `RORIM` reader - RX Overrun"]
pub struct RORIM_R(crate::FieldReader<bool, bool>);
impl RORIM_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RORIM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RORIM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RORIM` writer - RX Overrun"]
pub struct RORIM_W<'a> {
    w: &'a mut W,
}
impl<'a> RORIM_W<'a> {
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
#[doc = "Field `RTIM` reader - RX Timeout"]
pub struct RTIM_R(crate::FieldReader<bool, bool>);
impl RTIM_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RTIM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RTIM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RTIM` writer - RX Timeout"]
pub struct RTIM_W<'a> {
    w: &'a mut W,
}
impl<'a> RTIM_W<'a> {
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
#[doc = "Field `RXIM` reader - RX Fifo is at least half full"]
pub struct RXIM_R(crate::FieldReader<bool, bool>);
impl RXIM_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RXIM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXIM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXIM` writer - RX Fifo is at least half full"]
pub struct RXIM_W<'a> {
    w: &'a mut W,
}
impl<'a> RXIM_W<'a> {
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
#[doc = "Field `TXIM` reader - TX Fifo is at least half empty"]
pub struct TXIM_R(crate::FieldReader<bool, bool>);
impl TXIM_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TXIM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXIM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXIM` writer - TX Fifo is at least half empty"]
pub struct TXIM_W<'a> {
    w: &'a mut W,
}
impl<'a> TXIM_W<'a> {
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
impl R {
    #[doc = "Bit 0 - RX Overrun"]
    #[inline(always)]
    pub fn rorim(&self) -> RORIM_R {
        RORIM_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - RX Timeout"]
    #[inline(always)]
    pub fn rtim(&self) -> RTIM_R {
        RTIM_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - RX Fifo is at least half full"]
    #[inline(always)]
    pub fn rxim(&self) -> RXIM_R {
        RXIM_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - TX Fifo is at least half empty"]
    #[inline(always)]
    pub fn txim(&self) -> TXIM_R {
        TXIM_R::new(((self.bits >> 3) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - RX Overrun"]
    #[inline(always)]
    pub fn rorim(&mut self) -> RORIM_W {
        RORIM_W { w: self }
    }
    #[doc = "Bit 1 - RX Timeout"]
    #[inline(always)]
    pub fn rtim(&mut self) -> RTIM_W {
        RTIM_W { w: self }
    }
    #[doc = "Bit 2 - RX Fifo is at least half full"]
    #[inline(always)]
    pub fn rxim(&mut self) -> RXIM_W {
        RXIM_W { w: self }
    }
    #[doc = "Bit 3 - TX Fifo is at least half empty"]
    #[inline(always)]
    pub fn txim(&mut self) -> TXIM_W {
        TXIM_W { w: self }
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
