#[doc = "Register `PROCID` reader"]
pub struct R(crate::R<PROCID_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PROCID_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PROCID_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PROCID_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Processor ID Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [procid](index.html) module"]
pub struct PROCID_SPEC;
impl crate::RegisterSpec for PROCID_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [procid::R](R) reader structure"]
impl crate::Readable for PROCID_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets PROCID to value 0x0400_57e3"]
impl crate::Resettable for PROCID_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0400_57e3
    }
}
