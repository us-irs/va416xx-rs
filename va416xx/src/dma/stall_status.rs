#[doc = "Register `STALL_STATUS` reader"]
pub type R = crate::R<StallStatusSpec>;
#[doc = "Register `STALL_STATUS` writer"]
pub type W = crate::W<StallStatusSpec>;
#[doc = "Field `STALL_STATUS` reader - DMA is stalled"]
pub type StallStatusR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - DMA is stalled"]
    #[inline(always)]
    pub fn stall_status(&self) -> StallStatusR {
        StallStatusR::new((self.bits & 1) != 0)
    }
}
impl W {}
#[doc = "DMA stall status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stall_status::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`stall_status::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StallStatusSpec;
impl crate::RegisterSpec for StallStatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stall_status::R`](R) reader structure"]
impl crate::Readable for StallStatusSpec {}
#[doc = "`write(|w| ..)` method takes [`stall_status::W`](W) writer structure"]
impl crate::Writable for StallStatusSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets STALL_STATUS to value 0"]
impl crate::Resettable for StallStatusSpec {
    const RESET_VALUE: u32 = 0;
}
