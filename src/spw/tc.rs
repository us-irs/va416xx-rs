#[doc = "Register `TC` reader"]
pub struct R(crate::R<TC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TC` writer"]
pub struct W(crate::W<TC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TC_SPEC>;
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
impl From<crate::W<TC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TIRQ_END` reader - The current value of the time control flags"]
pub struct TIRQ_END_R(crate::FieldReader<u8, u8>);
impl TIRQ_END_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        TIRQ_END_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIRQ_END_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIRQ_END` writer - The current value of the time control flags"]
pub struct TIRQ_END_W<'a> {
    w: &'a mut W,
}
impl<'a> TIRQ_END_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | ((value as u32 & 0x03) << 6);
        self.w
    }
}
#[doc = "Field `TIMECNT` reader - The current value of the system time counter"]
pub struct TIMECNT_R(crate::FieldReader<u8, u8>);
impl TIMECNT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        TIMECNT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIMECNT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIMECNT` writer - The current value of the system time counter"]
pub struct TIMECNT_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMECNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | (value as u32 & 0x3f);
        self.w
    }
}
impl R {
    #[doc = "Bits 6:7 - The current value of the time control flags"]
    #[inline(always)]
    pub fn tirq_end(&self) -> TIRQ_END_R {
        TIRQ_END_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bits 0:5 - The current value of the system time counter"]
    #[inline(always)]
    pub fn timecnt(&self) -> TIMECNT_R {
        TIMECNT_R::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 6:7 - The current value of the time control flags"]
    #[inline(always)]
    pub fn tirq_end(&mut self) -> TIRQ_END_W {
        TIRQ_END_W { w: self }
    }
    #[doc = "Bits 0:5 - The current value of the system time counter"]
    #[inline(always)]
    pub fn timecnt(&mut self) -> TIMECNT_W {
        TIMECNT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Time Code Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tc](index.html) module"]
pub struct TC_SPEC;
impl crate::RegisterSpec for TC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tc::R](R) reader structure"]
impl crate::Readable for TC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tc::W](W) writer structure"]
impl crate::Writable for TC_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TC to value 0"]
impl crate::Resettable for TC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
