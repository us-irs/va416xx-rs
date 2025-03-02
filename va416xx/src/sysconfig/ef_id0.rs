#[doc = "Register `EF_ID0` reader"]
pub type R = crate::R<EfId0Spec>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[doc = "EFuse ID0 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ef_id0::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EfId0Spec;
impl crate::RegisterSpec for EfId0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ef_id0::R`](R) reader structure"]
impl crate::Readable for EfId0Spec {}
#[doc = "`reset()` method sets EF_ID0 to value 0"]
impl crate::Resettable for EfId0Spec {
    const RESET_VALUE: u32 = 0;
}
