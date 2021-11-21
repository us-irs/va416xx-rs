#[doc = "Register `SUBSEC_INC` reader"]
pub struct R(crate::R<SUBSEC_INC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SUBSEC_INC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SUBSEC_INC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SUBSEC_INC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SUBSEC_INC` writer"]
pub struct W(crate::W<SUBSEC_INC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SUBSEC_INC_SPEC>;
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
impl From<crate::W<SUBSEC_INC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SUBSEC_INC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SSINC` reader - Sub-Second Increment Valuee"]
pub struct SSINC_R(crate::FieldReader<u8, u8>);
impl SSINC_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SSINC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SSINC_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SSINC` writer - Sub-Second Increment Valuee"]
pub struct SSINC_W<'a> {
    w: &'a mut W,
}
impl<'a> SSINC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Sub-Second Increment Valuee"]
    #[inline(always)]
    pub fn ssinc(&self) -> SSINC_R {
        SSINC_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Sub-Second Increment Valuee"]
    #[inline(always)]
    pub fn ssinc(&mut self) -> SSINC_W {
        SSINC_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Holds the 8-bit value by which the Sub-Second register is incremented\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [subsec_inc](index.html) module"]
pub struct SUBSEC_INC_SPEC;
impl crate::RegisterSpec for SUBSEC_INC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [subsec_inc::R](R) reader structure"]
impl crate::Readable for SUBSEC_INC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [subsec_inc::W](W) writer structure"]
impl crate::Writable for SUBSEC_INC_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SUBSEC_INC to value 0"]
impl crate::Resettable for SUBSEC_INC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
