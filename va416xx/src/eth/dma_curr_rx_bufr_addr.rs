#[doc = "Register `DMA_CURR_RX_BUFR_ADDR` reader"]
pub type R = crate::R<DmaCurrRxBufrAddrSpec>;
#[doc = "Register `DMA_CURR_RX_BUFR_ADDR` writer"]
pub type W = crate::W<DmaCurrRxBufrAddrSpec>;
#[doc = "Field `CURTBUFAPTR` reader - Cleared on Reset. Pointer updated by the DMA during operation."]
pub type CurtbufaptrR = crate::FieldReader<u32>;
#[doc = "Field `CURTBUFAPTR` writer - Cleared on Reset. Pointer updated by the DMA during operation."]
pub type CurtbufaptrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Cleared on Reset. Pointer updated by the DMA during operation."]
    #[inline(always)]
    pub fn curtbufaptr(&self) -> CurtbufaptrR {
        CurtbufaptrR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Cleared on Reset. Pointer updated by the DMA during operation."]
    #[inline(always)]
    pub fn curtbufaptr(&mut self) -> CurtbufaptrW<DmaCurrRxBufrAddrSpec> {
        CurtbufaptrW::new(self, 0)
    }
}
#[doc = "Contains the current Receive Buffer address read by the DMA\n\nYou can [`read`](crate::Reg::read) this register and get [`dma_curr_rx_bufr_addr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_curr_rx_bufr_addr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DmaCurrRxBufrAddrSpec;
impl crate::RegisterSpec for DmaCurrRxBufrAddrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dma_curr_rx_bufr_addr::R`](R) reader structure"]
impl crate::Readable for DmaCurrRxBufrAddrSpec {}
#[doc = "`write(|w| ..)` method takes [`dma_curr_rx_bufr_addr::W`](W) writer structure"]
impl crate::Writable for DmaCurrRxBufrAddrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DMA_CURR_RX_BUFR_ADDR to value 0"]
impl crate::Resettable for DmaCurrRxBufrAddrSpec {
    const RESET_VALUE: u32 = 0;
}
