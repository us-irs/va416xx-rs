#[doc = "Register `CLKDIV3` reader"]
pub type R = crate::R<Clkdiv3Spec>;
#[doc = "Register `CLKDIV3` writer"]
pub type W = crate::W<Clkdiv3Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Clock divide value. 0 will disable the clock\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clkdiv3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clkdiv3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Clkdiv3Spec;
impl crate::RegisterSpec for Clkdiv3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clkdiv3::R`](R) reader structure"]
impl crate::Readable for Clkdiv3Spec {}
#[doc = "`write(|w| ..)` method takes [`clkdiv3::W`](W) writer structure"]
impl crate::Writable for Clkdiv3Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CLKDIV3 to value 0"]
impl crate::Resettable for Clkdiv3Spec {
    const RESET_VALUE: u32 = 0;
}
