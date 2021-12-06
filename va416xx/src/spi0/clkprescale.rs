#[doc = "Register `CLKPRESCALE` reader"]
pub type R = crate::R<ClkprescaleSpec>;
#[doc = "Register `CLKPRESCALE` writer"]
pub type W = crate::W<ClkprescaleSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Clock Pre Scale divide value\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clkprescale::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clkprescale::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ClkprescaleSpec;
impl crate::RegisterSpec for ClkprescaleSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clkprescale::R`](R) reader structure"]
impl crate::Readable for ClkprescaleSpec {}
#[doc = "`write(|w| ..)` method takes [`clkprescale::W`](W) writer structure"]
impl crate::Writable for ClkprescaleSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CLKPRESCALE to value 0"]
impl crate::Resettable for ClkprescaleSpec {
    const RESET_VALUE: u32 = 0;
}
