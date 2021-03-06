#[doc = "Register `DMA_RX_POLL_DEMAND` reader"]
pub struct R(crate::R<DMA_RX_POLL_DEMAND_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMA_RX_POLL_DEMAND_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMA_RX_POLL_DEMAND_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMA_RX_POLL_DEMAND_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DMA_RX_POLL_DEMAND` writer"]
pub struct W(crate::W<DMA_RX_POLL_DEMAND_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMA_RX_POLL_DEMAND_SPEC>;
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
impl From<crate::W<DMA_RX_POLL_DEMAND_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DMA_RX_POLL_DEMAND_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RPD` reader - Receive Poll Demand (Read Only and Write Trigger)"]
pub struct RPD_R(crate::FieldReader<u32, u32>);
impl RPD_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        RPD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RPD_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RPD` writer - Receive Poll Demand (Read Only and Write Trigger)"]
pub struct RPD_W<'a> {
    w: &'a mut W,
}
impl<'a> RPD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = value as u32;
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Receive Poll Demand (Read Only and Write Trigger)"]
    #[inline(always)]
    pub fn rpd(&self) -> RPD_R {
        RPD_R::new(self.bits as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Receive Poll Demand (Read Only and Write Trigger)"]
    #[inline(always)]
    pub fn rpd(&mut self) -> RPD_W {
        RPD_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Used by the host to instruct the DMA to poll the Receive Descriptor list\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_rx_poll_demand](index.html) module"]
pub struct DMA_RX_POLL_DEMAND_SPEC;
impl crate::RegisterSpec for DMA_RX_POLL_DEMAND_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dma_rx_poll_demand::R](R) reader structure"]
impl crate::Readable for DMA_RX_POLL_DEMAND_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dma_rx_poll_demand::W](W) writer structure"]
impl crate::Writable for DMA_RX_POLL_DEMAND_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DMA_RX_POLL_DEMAND to value 0"]
impl crate::Resettable for DMA_RX_POLL_DEMAND_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
