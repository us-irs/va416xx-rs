#[doc = "Register `DMA_RX_INTR_WDOG_TIMER` reader"]
pub struct R(crate::R<DMA_RX_INTR_WDOG_TIMER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMA_RX_INTR_WDOG_TIMER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMA_RX_INTR_WDOG_TIMER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMA_RX_INTR_WDOG_TIMER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DMA_RX_INTR_WDOG_TIMER` writer"]
pub struct W(crate::W<DMA_RX_INTR_WDOG_TIMER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMA_RX_INTR_WDOG_TIMER_SPEC>;
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
impl From<crate::W<DMA_RX_INTR_WDOG_TIMER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DMA_RX_INTR_WDOG_TIMER_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RIWT` reader - These bits indicate the number of system clock cycles x 256 for which the watchdog timer is set."]
pub struct RIWT_R(crate::FieldReader<u8, u8>);
impl RIWT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        RIWT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RIWT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RIWT` writer - These bits indicate the number of system clock cycles x 256 for which the watchdog timer is set."]
pub struct RIWT_W<'a> {
    w: &'a mut W,
}
impl<'a> RIWT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - These bits indicate the number of system clock cycles x 256 for which the watchdog timer is set."]
    #[inline(always)]
    pub fn riwt(&self) -> RIWT_R {
        RIWT_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - These bits indicate the number of system clock cycles x 256 for which the watchdog timer is set."]
    #[inline(always)]
    pub fn riwt(&mut self) -> RIWT_W {
        RIWT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Watchdog timeout for Receive Interrupt from DMA\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_rx_intr_wdog_timer](index.html) module"]
pub struct DMA_RX_INTR_WDOG_TIMER_SPEC;
impl crate::RegisterSpec for DMA_RX_INTR_WDOG_TIMER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dma_rx_intr_wdog_timer::R](R) reader structure"]
impl crate::Readable for DMA_RX_INTR_WDOG_TIMER_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dma_rx_intr_wdog_timer::W](W) writer structure"]
impl crate::Writable for DMA_RX_INTR_WDOG_TIMER_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DMA_RX_INTR_WDOG_TIMER to value 0"]
impl crate::Resettable for DMA_RX_INTR_WDOG_TIMER_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
