#[doc = "Register `WDOGINTCLR` reader"]
pub struct R(crate::R<WDOGINTCLR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WDOGINTCLR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WDOGINTCLR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WDOGINTCLR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `WDOGINTCLR` writer"]
pub struct W(crate::W<WDOGINTCLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WDOGINTCLR_SPEC>;
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
impl From<crate::W<WDOGINTCLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WDOGINTCLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CLEAR` reader - Write any value to clear interrupt"]
pub struct CLEAR_R(crate::FieldReader<u32, u32>);
impl CLEAR_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        CLEAR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CLEAR_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CLEAR` writer - Write any value to clear interrupt"]
pub struct CLEAR_W<'a> {
    w: &'a mut W,
}
impl<'a> CLEAR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | (value as u32 & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Write any value to clear interrupt"]
    #[inline(always)]
    pub fn clear(&self) -> CLEAR_R {
        CLEAR_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Write any value to clear interrupt"]
    #[inline(always)]
    pub fn clear(&mut self) -> CLEAR_W {
        CLEAR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "A write of any value clears the WDT module interrupt, and reloads the counter from the value in the WDOGLOAD Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wdogintclr](index.html) module"]
pub struct WDOGINTCLR_SPEC;
impl crate::RegisterSpec for WDOGINTCLR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [wdogintclr::R](R) reader structure"]
impl crate::Readable for WDOGINTCLR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [wdogintclr::W](W) writer structure"]
impl crate::Writable for WDOGINTCLR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets WDOGINTCLR to value 0"]
impl crate::Resettable for WDOGINTCLR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
