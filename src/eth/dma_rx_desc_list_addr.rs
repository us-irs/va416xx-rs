#[doc = "Register `DMA_RX_DESC_LIST_ADDR` reader"]
pub struct R(crate::R<DMA_RX_DESC_LIST_ADDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMA_RX_DESC_LIST_ADDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMA_RX_DESC_LIST_ADDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMA_RX_DESC_LIST_ADDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DMA_RX_DESC_LIST_ADDR` writer"]
pub struct W(crate::W<DMA_RX_DESC_LIST_ADDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMA_RX_DESC_LIST_ADDR_SPEC>;
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
impl From<crate::W<DMA_RX_DESC_LIST_ADDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DMA_RX_DESC_LIST_ADDR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RDESLA` reader - Start of Receive List"]
pub struct RDESLA_R(crate::FieldReader<u32, u32>);
impl RDESLA_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        RDESLA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RDESLA_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RDESLA` writer - Start of Receive List"]
pub struct RDESLA_W<'a> {
    w: &'a mut W,
}
impl<'a> RDESLA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = value as u32;
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Start of Receive List"]
    #[inline(always)]
    pub fn rdesla(&self) -> RDESLA_R {
        RDESLA_R::new(self.bits as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Start of Receive List"]
    #[inline(always)]
    pub fn rdesla(&mut self) -> RDESLA_W {
        RDESLA_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Points the DMA to the start of the Receive Descriptor list\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_rx_desc_list_addr](index.html) module"]
pub struct DMA_RX_DESC_LIST_ADDR_SPEC;
impl crate::RegisterSpec for DMA_RX_DESC_LIST_ADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dma_rx_desc_list_addr::R](R) reader structure"]
impl crate::Readable for DMA_RX_DESC_LIST_ADDR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dma_rx_desc_list_addr::W](W) writer structure"]
impl crate::Writable for DMA_RX_DESC_LIST_ADDR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DMA_RX_DESC_LIST_ADDR to value 0"]
impl crate::Resettable for DMA_RX_DESC_LIST_ADDR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
