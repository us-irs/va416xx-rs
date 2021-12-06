#[doc = "Register `DMA_CURR_RX_DESC` reader"]
pub type R = crate::R<DmaCurrRxDescSpec>;
#[doc = "Register `DMA_CURR_RX_DESC` writer"]
pub type W = crate::W<DmaCurrRxDescSpec>;
#[doc = "Field `CURRDESAPTR` reader - Cleared on Reset. Pointer updated by the DMA during operation."]
pub type CurrdesaptrR = crate::FieldReader<u32>;
#[doc = "Field `CURRDESAPTR` writer - Cleared on Reset. Pointer updated by the DMA during operation."]
pub type CurrdesaptrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Cleared on Reset. Pointer updated by the DMA during operation."]
    #[inline(always)]
    pub fn currdesaptr(&self) -> CurrdesaptrR {
        CurrdesaptrR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Cleared on Reset. Pointer updated by the DMA during operation."]
    #[inline(always)]
    #[must_use]
    pub fn currdesaptr(&mut self) -> CurrdesaptrW<DmaCurrRxDescSpec> {
        CurrdesaptrW::new(self, 0)
    }
}
#[doc = "Contains the start address of the current Receive Descriptor read by the DMA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dma_curr_rx_desc::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dma_curr_rx_desc::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DmaCurrRxDescSpec;
impl crate::RegisterSpec for DmaCurrRxDescSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dma_curr_rx_desc::R`](R) reader structure"]
impl crate::Readable for DmaCurrRxDescSpec {}
#[doc = "`write(|w| ..)` method takes [`dma_curr_rx_desc::W`](W) writer structure"]
impl crate::Writable for DmaCurrRxDescSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DMA_CURR_RX_DESC to value 0"]
impl crate::Resettable for DmaCurrRxDescSpec {
    const RESET_VALUE: u32 = 0;
}
