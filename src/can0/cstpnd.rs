#[doc = "Register `CSTPND` reader"]
pub struct R(crate::R<CSTPND_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CSTPND_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CSTPND_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CSTPND_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CSTPND` writer"]
pub struct W(crate::W<CSTPND_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CSTPND_SPEC>;
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
impl From<crate::W<CSTPND_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CSTPND_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `NS` reader - CAN Node Status"]
pub struct NS_R(crate::FieldReader<u8, u8>);
impl NS_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        NS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NS` writer - CAN Node Status"]
pub struct NS_W<'a> {
    w: &'a mut W,
}
impl<'a> NS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 5)) | ((value as u32 & 0x07) << 5);
        self.w
    }
}
#[doc = "Field `IRQ` reader - Interrupt Request portion of Interrupt Code"]
pub struct IRQ_R(crate::FieldReader<bool, bool>);
impl IRQ_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        IRQ_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IRQ_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IRQ` writer - Interrupt Request portion of Interrupt Code"]
pub struct IRQ_W<'a> {
    w: &'a mut W,
}
impl<'a> IRQ_W<'a> {
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
#[doc = "Field `IST` reader - Interrupt Source portion of Interrupt Code"]
pub struct IST_R(crate::FieldReader<u8, u8>);
impl IST_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        IST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IST_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IST` writer - Interrupt Source portion of Interrupt Code"]
pub struct IST_W<'a> {
    w: &'a mut W,
}
impl<'a> IST_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | (value as u32 & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bits 5:7 - CAN Node Status"]
    #[inline(always)]
    pub fn ns(&self) -> NS_R {
        NS_R::new(((self.bits >> 5) & 0x07) as u8)
    }
    #[doc = "Bit 4 - Interrupt Request portion of Interrupt Code"]
    #[inline(always)]
    pub fn irq(&self) -> IRQ_R {
        IRQ_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bits 0:3 - Interrupt Source portion of Interrupt Code"]
    #[inline(always)]
    pub fn ist(&self) -> IST_R {
        IST_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 5:7 - CAN Node Status"]
    #[inline(always)]
    pub fn ns(&mut self) -> NS_W {
        NS_W { w: self }
    }
    #[doc = "Bit 4 - Interrupt Request portion of Interrupt Code"]
    #[inline(always)]
    pub fn irq(&mut self) -> IRQ_W {
        IRQ_W { w: self }
    }
    #[doc = "Bits 0:3 - Interrupt Source portion of Interrupt Code"]
    #[inline(always)]
    pub fn ist(&mut self) -> IST_W {
        IST_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CAN Status Pending Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cstpnd](index.html) module"]
pub struct CSTPND_SPEC;
impl crate::RegisterSpec for CSTPND_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cstpnd::R](R) reader structure"]
impl crate::Readable for CSTPND_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cstpnd::W](W) writer structure"]
impl crate::Writable for CSTPND_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CSTPND to value 0"]
impl crate::Resettable for CSTPND_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
