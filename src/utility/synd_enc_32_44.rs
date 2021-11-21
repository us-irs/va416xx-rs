#[doc = "Register `SYND_ENC_32_44` reader"]
pub struct R(crate::R<SYND_ENC_32_44_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SYND_ENC_32_44_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SYND_ENC_32_44_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SYND_ENC_32_44_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SYND_ENC_32_44` writer"]
pub struct W(crate::W<SYND_ENC_32_44_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SYND_ENC_32_44_SPEC>;
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
impl From<crate::W<SYND_ENC_32_44_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SYND_ENC_32_44_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SYND_ENC_31_16` reader - Computed syndrome value for bits 31-16"]
pub struct SYND_ENC_31_16_R(crate::FieldReader<u8, u8>);
impl SYND_ENC_31_16_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SYND_ENC_31_16_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SYND_ENC_31_16_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SYND_ENC_31_16` writer - Computed syndrome value for bits 31-16"]
pub struct SYND_ENC_31_16_W<'a> {
    w: &'a mut W,
}
impl<'a> SYND_ENC_31_16_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 6)) | ((value as u32 & 0x3f) << 6);
        self.w
    }
}
#[doc = "Field `SYND_ENC_7_0` reader - Computed syndrome value for bits 15-0"]
pub struct SYND_ENC_7_0_R(crate::FieldReader<u8, u8>);
impl SYND_ENC_7_0_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SYND_ENC_7_0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SYND_ENC_7_0_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SYND_ENC_7_0` writer - Computed syndrome value for bits 15-0"]
pub struct SYND_ENC_7_0_W<'a> {
    w: &'a mut W,
}
impl<'a> SYND_ENC_7_0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | (value as u32 & 0x3f);
        self.w
    }
}
impl R {
    #[doc = "Bits 6:11 - Computed syndrome value for bits 31-16"]
    #[inline(always)]
    pub fn synd_enc_31_16(&self) -> SYND_ENC_31_16_R {
        SYND_ENC_31_16_R::new(((self.bits >> 6) & 0x3f) as u8)
    }
    #[doc = "Bits 0:5 - Computed syndrome value for bits 15-0"]
    #[inline(always)]
    pub fn synd_enc_7_0(&self) -> SYND_ENC_7_0_R {
        SYND_ENC_7_0_R::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 6:11 - Computed syndrome value for bits 31-16"]
    #[inline(always)]
    pub fn synd_enc_31_16(&mut self) -> SYND_ENC_31_16_W {
        SYND_ENC_31_16_W { w: self }
    }
    #[doc = "Bits 0:5 - Computed syndrome value for bits 15-0"]
    #[inline(always)]
    pub fn synd_enc_7_0(&mut self) -> SYND_ENC_7_0_W {
        SYND_ENC_7_0_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "EDAC Encode\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [synd_enc_32_44](index.html) module"]
pub struct SYND_ENC_32_44_SPEC;
impl crate::RegisterSpec for SYND_ENC_32_44_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [synd_enc_32_44::R](R) reader structure"]
impl crate::Readable for SYND_ENC_32_44_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [synd_enc_32_44::W](W) writer structure"]
impl crate::Writable for SYND_ENC_32_44_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SYND_ENC_32_44 to value 0"]
impl crate::Resettable for SYND_ENC_32_44_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
