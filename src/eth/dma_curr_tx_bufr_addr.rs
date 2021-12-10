#[doc = "Register `DMA_CURR_TX_BUFR_ADDR` reader"]
pub struct R(crate::R<DMA_CURR_TX_BUFR_ADDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMA_CURR_TX_BUFR_ADDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMA_CURR_TX_BUFR_ADDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMA_CURR_TX_BUFR_ADDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DMA_CURR_TX_BUFR_ADDR` writer"]
pub struct W(crate::W<DMA_CURR_TX_BUFR_ADDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMA_CURR_TX_BUFR_ADDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<DMA_CURR_TX_BUFR_ADDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DMA_CURR_TX_BUFR_ADDR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CURTBUFAPTR` reader - Cleared on Reset. Pointer updated by the DMA during operation."]
pub struct CURTBUFAPTR_R(crate::FieldReader<u32, u32>);
impl CURTBUFAPTR_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        CURTBUFAPTR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CURTBUFAPTR_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CURTBUFAPTR` writer - Cleared on Reset. Pointer updated by the DMA during operation."]
pub struct CURTBUFAPTR_W<'a> {
    w: &'a mut W,
}
impl<'a> CURTBUFAPTR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = value as u32;
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Cleared on Reset. Pointer updated by the DMA during operation."]
    #[inline(always)]
    pub fn curtbufaptr(&self) -> CURTBUFAPTR_R {
        CURTBUFAPTR_R::new(self.bits as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Cleared on Reset. Pointer updated by the DMA during operation."]
    #[inline(always)]
    pub fn curtbufaptr(&mut self) -> CURTBUFAPTR_W {
        CURTBUFAPTR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Contains the start address of the current Receive Descriptor read by the DMA\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_curr_tx_bufr_addr](index.html) module"]
pub struct DMA_CURR_TX_BUFR_ADDR_SPEC;
impl crate::RegisterSpec for DMA_CURR_TX_BUFR_ADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dma_curr_tx_bufr_addr::R](R) reader structure"]
impl crate::Readable for DMA_CURR_TX_BUFR_ADDR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dma_curr_tx_bufr_addr::W](W) writer structure"]
impl crate::Writable for DMA_CURR_TX_BUFR_ADDR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DMA_CURR_TX_BUFR_ADDR to value 0"]
impl crate::Resettable for DMA_CURR_TX_BUFR_ADDR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
