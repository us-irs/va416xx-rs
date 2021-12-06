#[doc = "Register `CLKTOLIMIT` reader"]
pub type R = crate::R<ClktolimitSpec>;
#[doc = "Register `CLKTOLIMIT` writer"]
pub type W = crate::W<ClktolimitSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Clock Low Timeout Limit Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clktolimit::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clktolimit::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ClktolimitSpec;
impl crate::RegisterSpec for ClktolimitSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clktolimit::R`](R) reader structure"]
impl crate::Readable for ClktolimitSpec {}
#[doc = "`write(|w| ..)` method takes [`clktolimit::W`](W) writer structure"]
impl crate::Writable for ClktolimitSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CLKTOLIMIT to value 0"]
impl crate::Resettable for ClktolimitSpec {
    const RESET_VALUE: u32 = 0;
}
