#[doc = "Register `AUTOCORR_STATISTIC` reader"]
pub struct R(crate::R<AUTOCORR_STATISTIC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AUTOCORR_STATISTIC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AUTOCORR_STATISTIC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AUTOCORR_STATISTIC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AUTOCORR_STATISTIC` writer"]
pub struct W(crate::W<AUTOCORR_STATISTIC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AUTOCORR_STATISTIC_SPEC>;
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
impl From<crate::W<AUTOCORR_STATISTIC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AUTOCORR_STATISTIC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `AUTOCORR_FAILS` reader - Count each time an autocorrelation test fails"]
pub struct AUTOCORR_FAILS_R(crate::FieldReader<u8, u8>);
impl AUTOCORR_FAILS_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        AUTOCORR_FAILS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AUTOCORR_FAILS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AUTOCORR_FAILS` writer - Count each time an autocorrelation test fails"]
pub struct AUTOCORR_FAILS_W<'a> {
    w: &'a mut W,
}
impl<'a> AUTOCORR_FAILS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 14)) | ((value as u32 & 0xff) << 14);
        self.w
    }
}
#[doc = "Field `AUTOCORR_TRYS` reader - Count each time an autocorrelation test starts"]
pub struct AUTOCORR_TRYS_R(crate::FieldReader<u16, u16>);
impl AUTOCORR_TRYS_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        AUTOCORR_TRYS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AUTOCORR_TRYS_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AUTOCORR_TRYS` writer - Count each time an autocorrelation test starts"]
pub struct AUTOCORR_TRYS_W<'a> {
    w: &'a mut W,
}
impl<'a> AUTOCORR_TRYS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3fff) | (value as u32 & 0x3fff);
        self.w
    }
}
impl R {
    #[doc = "Bits 14:21 - Count each time an autocorrelation test fails"]
    #[inline(always)]
    pub fn autocorr_fails(&self) -> AUTOCORR_FAILS_R {
        AUTOCORR_FAILS_R::new(((self.bits >> 14) & 0xff) as u8)
    }
    #[doc = "Bits 0:13 - Count each time an autocorrelation test starts"]
    #[inline(always)]
    pub fn autocorr_trys(&self) -> AUTOCORR_TRYS_R {
        AUTOCORR_TRYS_R::new((self.bits & 0x3fff) as u16)
    }
}
impl W {
    #[doc = "Bits 14:21 - Count each time an autocorrelation test fails"]
    #[inline(always)]
    pub fn autocorr_fails(&mut self) -> AUTOCORR_FAILS_W {
        AUTOCORR_FAILS_W { w: self }
    }
    #[doc = "Bits 0:13 - Count each time an autocorrelation test starts"]
    #[inline(always)]
    pub fn autocorr_trys(&mut self) -> AUTOCORR_TRYS_W {
        AUTOCORR_TRYS_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Auto-correlator Statistic Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [autocorr_statistic](index.html) module"]
pub struct AUTOCORR_STATISTIC_SPEC;
impl crate::RegisterSpec for AUTOCORR_STATISTIC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [autocorr_statistic::R](R) reader structure"]
impl crate::Readable for AUTOCORR_STATISTIC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [autocorr_statistic::W](W) writer structure"]
impl crate::Writable for AUTOCORR_STATISTIC_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets AUTOCORR_STATISTIC to value 0"]
impl crate::Resettable for AUTOCORR_STATISTIC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
