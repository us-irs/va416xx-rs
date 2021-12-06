#[doc = "Register `RXFIFOIRQTRG` reader"]
pub type R = crate::R<RxfifoirqtrgSpec>;
#[doc = "Register `RXFIFOIRQTRG` writer"]
pub type W = crate::W<RxfifoirqtrgSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Rx FIFO IRQ Trigger Level\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxfifoirqtrg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rxfifoirqtrg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RxfifoirqtrgSpec;
impl crate::RegisterSpec for RxfifoirqtrgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rxfifoirqtrg::R`](R) reader structure"]
impl crate::Readable for RxfifoirqtrgSpec {}
#[doc = "`write(|w| ..)` method takes [`rxfifoirqtrg::W`](W) writer structure"]
impl crate::Writable for RxfifoirqtrgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RXFIFOIRQTRG to value 0"]
impl crate::Resettable for RxfifoirqtrgSpec {
    const RESET_VALUE: u32 = 0;
}
