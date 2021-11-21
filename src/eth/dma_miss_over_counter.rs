#[doc = "Register `DMA_MISS_OVER_COUNTER` reader"]
pub struct R(crate::R<DMA_MISS_OVER_COUNTER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMA_MISS_OVER_COUNTER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMA_MISS_OVER_COUNTER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMA_MISS_OVER_COUNTER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DMA_MISS_OVER_COUNTER` writer"]
pub struct W(crate::W<DMA_MISS_OVER_COUNTER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMA_MISS_OVER_COUNTER_SPEC>;
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
impl From<crate::W<DMA_MISS_OVER_COUNTER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DMA_MISS_OVER_COUNTER_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OVFCNTOVF` reader - This bit is set every time the Overflow Frame Counter (Bits\\[27:17\\])overflows"]
pub struct OVFCNTOVF_R(crate::FieldReader<bool, bool>);
impl OVFCNTOVF_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        OVFCNTOVF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OVFCNTOVF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OVFCNTOVF` writer - This bit is set every time the Overflow Frame Counter (Bits\\[27:17\\])overflows"]
pub struct OVFCNTOVF_W<'a> {
    w: &'a mut W,
}
impl<'a> OVFCNTOVF_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 28)) | ((value as u32 & 0x01) << 28);
        self.w
    }
}
#[doc = "Field `OVFFRMCNT` reader - This field indicates the number of frames missed by the application"]
pub struct OVFFRMCNT_R(crate::FieldReader<u16, u16>);
impl OVFFRMCNT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        OVFFRMCNT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OVFFRMCNT_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OVFFRMCNT` writer - This field indicates the number of frames missed by the application"]
pub struct OVFFRMCNT_W<'a> {
    w: &'a mut W,
}
impl<'a> OVFFRMCNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07ff << 17)) | ((value as u32 & 0x07ff) << 17);
        self.w
    }
}
#[doc = "Field `MISCNTOVF` reader - This bit is set every time Missed Frame Counter (Bits\\[15:0\\]) overflows"]
pub struct MISCNTOVF_R(crate::FieldReader<bool, bool>);
impl MISCNTOVF_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MISCNTOVF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MISCNTOVF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MISCNTOVF` writer - This bit is set every time Missed Frame Counter (Bits\\[15:0\\]) overflows"]
pub struct MISCNTOVF_W<'a> {
    w: &'a mut W,
}
impl<'a> MISCNTOVF_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 16)) | ((value as u32 & 0x01) << 16);
        self.w
    }
}
#[doc = "Field `MISFRMCNT` reader - This field indicates the number of frames missed by the controller because of the Host Receive Buffer being unavailable."]
pub struct MISFRMCNT_R(crate::FieldReader<u16, u16>);
impl MISFRMCNT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        MISFRMCNT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MISFRMCNT_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MISFRMCNT` writer - This field indicates the number of frames missed by the controller because of the Host Receive Buffer being unavailable."]
pub struct MISFRMCNT_W<'a> {
    w: &'a mut W,
}
impl<'a> MISFRMCNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bit 28 - This bit is set every time the Overflow Frame Counter (Bits\\[27:17\\])overflows"]
    #[inline(always)]
    pub fn ovfcntovf(&self) -> OVFCNTOVF_R {
        OVFCNTOVF_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bits 17:27 - This field indicates the number of frames missed by the application"]
    #[inline(always)]
    pub fn ovffrmcnt(&self) -> OVFFRMCNT_R {
        OVFFRMCNT_R::new(((self.bits >> 17) & 0x07ff) as u16)
    }
    #[doc = "Bit 16 - This bit is set every time Missed Frame Counter (Bits\\[15:0\\]) overflows"]
    #[inline(always)]
    pub fn miscntovf(&self) -> MISCNTOVF_R {
        MISCNTOVF_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bits 0:15 - This field indicates the number of frames missed by the controller because of the Host Receive Buffer being unavailable."]
    #[inline(always)]
    pub fn misfrmcnt(&self) -> MISFRMCNT_R {
        MISFRMCNT_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bit 28 - This bit is set every time the Overflow Frame Counter (Bits\\[27:17\\])overflows"]
    #[inline(always)]
    pub fn ovfcntovf(&mut self) -> OVFCNTOVF_W {
        OVFCNTOVF_W { w: self }
    }
    #[doc = "Bits 17:27 - This field indicates the number of frames missed by the application"]
    #[inline(always)]
    pub fn ovffrmcnt(&mut self) -> OVFFRMCNT_W {
        OVFFRMCNT_W { w: self }
    }
    #[doc = "Bit 16 - This bit is set every time Missed Frame Counter (Bits\\[15:0\\]) overflows"]
    #[inline(always)]
    pub fn miscntovf(&mut self) -> MISCNTOVF_W {
        MISCNTOVF_W { w: self }
    }
    #[doc = "Bits 0:15 - This field indicates the number of frames missed by the controller because of the Host Receive Buffer being unavailable."]
    #[inline(always)]
    pub fn misfrmcnt(&mut self) -> MISFRMCNT_W {
        MISFRMCNT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Contains the counters for discarded frames because no Receive Descriptor is available\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_miss_over_counter](index.html) module"]
pub struct DMA_MISS_OVER_COUNTER_SPEC;
impl crate::RegisterSpec for DMA_MISS_OVER_COUNTER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dma_miss_over_counter::R](R) reader structure"]
impl crate::Readable for DMA_MISS_OVER_COUNTER_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dma_miss_over_counter::W](W) writer structure"]
impl crate::Writable for DMA_MISS_OVER_COUNTER_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DMA_MISS_OVER_COUNTER to value 0"]
impl crate::Resettable for DMA_MISS_OVER_COUNTER_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
