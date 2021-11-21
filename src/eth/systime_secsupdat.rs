#[doc = "Register `SYSTIME_SECSUPDAT` reader"]
pub struct R(crate::R<SYSTIME_SECSUPDAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SYSTIME_SECSUPDAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SYSTIME_SECSUPDAT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SYSTIME_SECSUPDAT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SYSTIME_SECSUPDAT` writer"]
pub struct W(crate::W<SYSTIME_SECSUPDAT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SYSTIME_SECSUPDAT_SPEC>;
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
impl From<crate::W<SYSTIME_SECSUPDAT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SYSTIME_SECSUPDAT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TSS` reader - Timestamp Second"]
pub struct TSS_R(crate::FieldReader<u32, u32>);
impl TSS_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        TSS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TSS_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TSS` writer - Timestamp Second"]
pub struct TSS_W<'a> {
    w: &'a mut W,
}
impl<'a> TSS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | (value as u32 & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Timestamp Second"]
    #[inline(always)]
    pub fn tss(&self) -> TSS_R {
        TSS_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Timestamp Second"]
    #[inline(always)]
    pub fn tss(&mut self) -> TSS_W {
        TSS_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Holds the lower 32 bits of the second field to be written to, added to, or subtracted from the system time value\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [systime_secsupdat](index.html) module"]
pub struct SYSTIME_SECSUPDAT_SPEC;
impl crate::RegisterSpec for SYSTIME_SECSUPDAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [systime_secsupdat::R](R) reader structure"]
impl crate::Readable for SYSTIME_SECSUPDAT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [systime_secsupdat::W](W) writer structure"]
impl crate::Writable for SYSTIME_SECSUPDAT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SYSTIME_SECSUPDAT to value 0"]
impl crate::Resettable for SYSTIME_SECSUPDAT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
