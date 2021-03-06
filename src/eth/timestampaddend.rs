#[doc = "Register `TIMESTAMPADDEND` reader"]
pub struct R(crate::R<TIMESTAMPADDEND_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TIMESTAMPADDEND_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TIMESTAMPADDEND_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TIMESTAMPADDEND_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TIMESTAMPADDEND` writer"]
pub struct W(crate::W<TIMESTAMPADDEND_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TIMESTAMPADDEND_SPEC>;
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
impl From<crate::W<TIMESTAMPADDEND_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TIMESTAMPADDEND_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TSAR` reader - Timestamp Addend Register"]
pub struct TSAR_R(crate::FieldReader<u32, u32>);
impl TSAR_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        TSAR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TSAR_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TSAR` writer - Timestamp Addend Register"]
pub struct TSAR_W<'a> {
    w: &'a mut W,
}
impl<'a> TSAR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = value as u32;
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Timestamp Addend Register"]
    #[inline(always)]
    pub fn tsar(&self) -> TSAR_R {
        TSAR_R::new(self.bits as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Timestamp Addend Register"]
    #[inline(always)]
    pub fn tsar(&mut self) -> TSAR_W {
        TSAR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "This register is used by software to re-adjust the clock frequency linearly to match the Master clock frequency\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [timestampaddend](index.html) module"]
pub struct TIMESTAMPADDEND_SPEC;
impl crate::RegisterSpec for TIMESTAMPADDEND_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [timestampaddend::R](R) reader structure"]
impl crate::Readable for TIMESTAMPADDEND_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [timestampaddend::W](W) writer structure"]
impl crate::Writable for TIMESTAMPADDEND_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TIMESTAMPADDEND to value 0"]
impl crate::Resettable for TIMESTAMPADDEND_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
