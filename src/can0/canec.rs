#[doc = "Register `CANEC` reader"]
pub struct R(crate::R<CANEC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CANEC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CANEC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CANEC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CANEC` writer"]
pub struct W(crate::W<CANEC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CANEC_SPEC>;
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
impl From<crate::W<CANEC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CANEC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `REC` reader - Receive Error Counter"]
pub struct REC_R(crate::FieldReader<u8, u8>);
impl REC_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        REC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for REC_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `REC` writer - Receive Error Counter"]
pub struct REC_W<'a> {
    w: &'a mut W,
}
impl<'a> REC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | ((value as u32 & 0xff) << 8);
        self.w
    }
}
#[doc = "Field `TEC` reader - Transmit Error Counter"]
pub struct TEC_R(crate::FieldReader<u8, u8>);
impl TEC_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        TEC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TEC_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TEC` writer - Transmit Error Counter"]
pub struct TEC_W<'a> {
    w: &'a mut W,
}
impl<'a> TEC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 8:15 - Receive Error Counter"]
    #[inline(always)]
    pub fn rec(&self) -> REC_R {
        REC_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 0:7 - Transmit Error Counter"]
    #[inline(always)]
    pub fn tec(&self) -> TEC_R {
        TEC_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 8:15 - Receive Error Counter"]
    #[inline(always)]
    pub fn rec(&mut self) -> REC_W {
        REC_W { w: self }
    }
    #[doc = "Bits 0:7 - Transmit Error Counter"]
    #[inline(always)]
    pub fn tec(&mut self) -> TEC_W {
        TEC_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CAN Error Counter Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [canec](index.html) module"]
pub struct CANEC_SPEC;
impl crate::RegisterSpec for CANEC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [canec::R](R) reader structure"]
impl crate::Readable for CANEC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [canec::W](W) writer structure"]
impl crate::Writable for CANEC_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CANEC to value 0"]
impl crate::Resettable for CANEC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
