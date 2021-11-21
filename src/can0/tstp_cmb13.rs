#[doc = "Register `TSTP_CMB13` reader"]
pub struct R(crate::R<TSTP_CMB13_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TSTP_CMB13_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TSTP_CMB13_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TSTP_CMB13_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TSTP_CMB13` writer"]
pub struct W(crate::W<TSTP_CMB13_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TSTP_CMB13_SPEC>;
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
impl From<crate::W<TSTP_CMB13_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TSTP_CMB13_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TIMESTAMP` reader - Timestamp"]
pub struct TIMESTAMP_R(crate::FieldReader<u16, u16>);
impl TIMESTAMP_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        TIMESTAMP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIMESTAMP_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIMESTAMP` writer - Timestamp"]
pub struct TIMESTAMP_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMESTAMP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Timestamp"]
    #[inline(always)]
    pub fn timestamp(&self) -> TIMESTAMP_R {
        TIMESTAMP_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Timestamp"]
    #[inline(always)]
    pub fn timestamp(&mut self) -> TIMESTAMP_W {
        TIMESTAMP_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CAN Frame Timestamp\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tstp_cmb13](index.html) module"]
pub struct TSTP_CMB13_SPEC;
impl crate::RegisterSpec for TSTP_CMB13_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tstp_cmb13::R](R) reader structure"]
impl crate::Readable for TSTP_CMB13_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tstp_cmb13::W](W) writer structure"]
impl crate::Writable for TSTP_CMB13_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TSTP_CMB13 to value 0"]
impl crate::Resettable for TSTP_CMB13_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
