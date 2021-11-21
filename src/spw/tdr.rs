#[doc = "Register `TDR` reader"]
pub struct R(crate::R<TDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `DISCONNECT` reader - Used to generate the 850 ns disconnect time period"]
pub struct DISCONNECT_R(crate::FieldReader<u16, u16>);
impl DISCONNECT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        DISCONNECT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DISCONNECT_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIMER64` reader - Used to generate the 6.4 and 12.8 us time periods"]
pub struct TIMER64_R(crate::FieldReader<u16, u16>);
impl TIMER64_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        TIMER64_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIMER64_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 12:21 - Used to generate the 850 ns disconnect time period"]
    #[inline(always)]
    pub fn disconnect(&self) -> DISCONNECT_R {
        DISCONNECT_R::new(((self.bits >> 12) & 0x03ff) as u16)
    }
    #[doc = "Bits 0:11 - Used to generate the 6.4 and 12.8 us time periods"]
    #[inline(always)]
    pub fn timer64(&self) -> TIMER64_R {
        TIMER64_R::new((self.bits & 0x0fff) as u16)
    }
}
#[doc = "Timer and Disconnect Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tdr](index.html) module"]
pub struct TDR_SPEC;
impl crate::RegisterSpec for TDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tdr::R](R) reader structure"]
impl crate::Readable for TDR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets TDR to value 0"]
impl crate::Resettable for TDR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
