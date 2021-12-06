#[doc = "Register `DMA_RX_POLL_DEMAND` reader"]
pub type R = crate::R<DmaRxPollDemandSpec>;
#[doc = "Register `DMA_RX_POLL_DEMAND` writer"]
pub type W = crate::W<DmaRxPollDemandSpec>;
#[doc = "Field `RPD` reader - Receive Poll Demand (Read Only and Write Trigger)"]
pub type RpdR = crate::FieldReader<u32>;
#[doc = "Field `RPD` writer - Receive Poll Demand (Read Only and Write Trigger)"]
pub type RpdW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Receive Poll Demand (Read Only and Write Trigger)"]
    #[inline(always)]
    pub fn rpd(&self) -> RpdR {
        RpdR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Receive Poll Demand (Read Only and Write Trigger)"]
    #[inline(always)]
    #[must_use]
    pub fn rpd(&mut self) -> RpdW<DmaRxPollDemandSpec> {
        RpdW::new(self, 0)
    }
}
#[doc = "Used by the host to instruct the DMA to poll the Receive Descriptor list\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dma_rx_poll_demand::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dma_rx_poll_demand::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DmaRxPollDemandSpec;
impl crate::RegisterSpec for DmaRxPollDemandSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dma_rx_poll_demand::R`](R) reader structure"]
impl crate::Readable for DmaRxPollDemandSpec {}
#[doc = "`write(|w| ..)` method takes [`dma_rx_poll_demand::W`](W) writer structure"]
impl crate::Writable for DmaRxPollDemandSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DMA_RX_POLL_DEMAND to value 0"]
impl crate::Resettable for DmaRxPollDemandSpec {
    const RESET_VALUE: u32 = 0;
}
