#[doc = "Register `CLKDIV2` reader"]
pub type R = crate::R<Clkdiv2Spec>;
#[doc = "Register `CLKDIV2` writer"]
pub type W = crate::W<Clkdiv2Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Clock divide value. 0 will disable the clock\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clkdiv2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clkdiv2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Clkdiv2Spec;
impl crate::RegisterSpec for Clkdiv2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clkdiv2::R`](R) reader structure"]
impl crate::Readable for Clkdiv2Spec {}
#[doc = "`write(|w| ..)` method takes [`clkdiv2::W`](W) writer structure"]
impl crate::Writable for Clkdiv2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CLKDIV2 to value 0"]
impl crate::Resettable for Clkdiv2Spec {
    const RESET_VALUE: u32 = 0;
}
