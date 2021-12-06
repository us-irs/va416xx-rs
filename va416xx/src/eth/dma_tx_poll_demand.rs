#[doc = "Register `DMA_TX_POLL_DEMAND` reader"]
pub type R = crate::R<DmaTxPollDemandSpec>;
#[doc = "Register `DMA_TX_POLL_DEMAND` writer"]
pub type W = crate::W<DmaTxPollDemandSpec>;
#[doc = "Field `TPD` reader - Transmit Poll Demand (Read Only and Write Trigger)"]
pub type TpdR = crate::FieldReader<u32>;
#[doc = "Field `TPD` writer - Transmit Poll Demand (Read Only and Write Trigger)"]
pub type TpdW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Transmit Poll Demand (Read Only and Write Trigger)"]
    #[inline(always)]
    pub fn tpd(&self) -> TpdR {
        TpdR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Transmit Poll Demand (Read Only and Write Trigger)"]
    #[inline(always)]
    #[must_use]
    pub fn tpd(&mut self) -> TpdW<DmaTxPollDemandSpec> {
        TpdW::new(self, 0)
    }
}
#[doc = "Used by the host to instruct the DMA to poll the transmit Descriptor list\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dma_tx_poll_demand::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dma_tx_poll_demand::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DmaTxPollDemandSpec;
impl crate::RegisterSpec for DmaTxPollDemandSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dma_tx_poll_demand::R`](R) reader structure"]
impl crate::Readable for DmaTxPollDemandSpec {}
#[doc = "`write(|w| ..)` method takes [`dma_tx_poll_demand::W`](W) writer structure"]
impl crate::Writable for DmaTxPollDemandSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DMA_TX_POLL_DEMAND to value 0"]
impl crate::Resettable for DmaTxPollDemandSpec {
    const RESET_VALUE: u32 = 0;
}
