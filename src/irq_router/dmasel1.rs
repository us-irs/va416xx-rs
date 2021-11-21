#[doc = "Register `DMASEL1` reader"]
pub struct R(crate::R<DMASEL1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMASEL1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMASEL1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMASEL1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DMASEL1` writer"]
pub struct W(crate::W<DMASEL1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMASEL1_SPEC>;
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
impl From<crate::W<DMASEL1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DMASEL1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DMASEL` reader - DMA trigger source selection value"]
pub struct DMASEL_R(crate::FieldReader<u8, u8>);
impl DMASEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DMASEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DMASEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DMASEL` writer - DMA trigger source selection value"]
pub struct DMASEL_W<'a> {
    w: &'a mut W,
}
impl<'a> DMASEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7f) | (value as u32 & 0x7f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:6 - DMA trigger source selection value"]
    #[inline(always)]
    pub fn dmasel(&self) -> DMASEL_R {
        DMASEL_R::new((self.bits & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - DMA trigger source selection value"]
    #[inline(always)]
    pub fn dmasel(&mut self) -> DMASEL_W {
        DMASEL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt select for DMA channel 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmasel1](index.html) module"]
pub struct DMASEL1_SPEC;
impl crate::RegisterSpec for DMASEL1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dmasel1::R](R) reader structure"]
impl crate::Readable for DMASEL1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dmasel1::W](W) writer structure"]
impl crate::Writable for DMASEL1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DMASEL1 to value 0x7f"]
impl crate::Resettable for DMASEL1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x7f
    }
}
