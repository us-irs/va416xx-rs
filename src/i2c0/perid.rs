#[doc = "Register `PERID` reader"]
pub struct R(crate::R<PERID_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PERID_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PERID_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PERID_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Peripheral ID Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [perid](index.html) module"]
pub struct PERID_SPEC;
impl crate::RegisterSpec for PERID_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [perid::R](R) reader structure"]
impl crate::Readable for PERID_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets PERID to value 0x0214_07e9"]
impl crate::Resettable for PERID_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0214_07e9
    }
}
