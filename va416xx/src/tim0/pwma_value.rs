#[doc = "Register `PWMA_VALUE` reader"]
pub type R = crate::R<PwmaValueSpec>;
#[doc = "Register `PWMA_VALUE` writer"]
pub type W = crate::W<PwmaValueSpec>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "The Pulse Width Modulation ValueA\n\nYou can [`read`](crate::Reg::read) this register and get [`pwma_value::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwma_value::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PwmaValueSpec;
impl crate::RegisterSpec for PwmaValueSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pwma_value::R`](R) reader structure"]
impl crate::Readable for PwmaValueSpec {}
#[doc = "`write(|w| ..)` method takes [`pwma_value::W`](W) writer structure"]
impl crate::Writable for PwmaValueSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PWMA_VALUE to value 0"]
impl crate::Resettable for PwmaValueSpec {
    const RESET_VALUE: u32 = 0;
}
