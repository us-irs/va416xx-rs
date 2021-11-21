#[doc = "Register `DMATTSEL` reader"]
pub struct R(crate::R<DMATTSEL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMATTSEL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMATTSEL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMATTSEL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DMATTSEL` writer"]
pub struct W(crate::W<DMATTSEL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMATTSEL_SPEC>;
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
impl From<crate::W<DMATTSEL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DMATTSEL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DMATTSEL` reader - DMA trigger type selection value"]
pub struct DMATTSEL_R(crate::FieldReader<u8, u8>);
impl DMATTSEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DMATTSEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DMATTSEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DMATTSEL` writer - DMA trigger type selection value"]
pub struct DMATTSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> DMATTSEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | (value as u32 & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - DMA trigger type selection value"]
    #[inline(always)]
    pub fn dmattsel(&self) -> DMATTSEL_R {
        DMATTSEL_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - DMA trigger type selection value"]
    #[inline(always)]
    pub fn dmattsel(&mut self) -> DMATTSEL_W {
        DMATTSEL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Trigger select for the DMA channels\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmattsel](index.html) module"]
pub struct DMATTSEL_SPEC;
impl crate::RegisterSpec for DMATTSEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dmattsel::R](R) reader structure"]
impl crate::Readable for DMATTSEL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dmattsel::W](W) writer structure"]
impl crate::Writable for DMATTSEL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DMATTSEL to value 0"]
impl crate::Resettable for DMATTSEL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
