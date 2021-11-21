#[doc = "Register `CTIM` reader"]
pub struct R(crate::R<CTIM_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTIM_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTIM_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTIM_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTIM` writer"]
pub struct W(crate::W<CTIM_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTIM_SPEC>;
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
impl From<crate::W<CTIM_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTIM_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PSC` reader - Prescaler Configuration"]
pub struct PSC_R(crate::FieldReader<u8, u8>);
impl PSC_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PSC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PSC_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PSC` writer - Prescaler Configuration"]
pub struct PSC_W<'a> {
    w: &'a mut W,
}
impl<'a> PSC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 9)) | ((value as u32 & 0x7f) << 9);
        self.w
    }
}
#[doc = "Field `SJW` reader - Synchronization Jump Width"]
pub struct SJW_R(crate::FieldReader<u8, u8>);
impl SJW_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SJW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SJW_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SJW` writer - Synchronization Jump Width"]
pub struct SJW_W<'a> {
    w: &'a mut W,
}
impl<'a> SJW_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 7)) | ((value as u32 & 0x03) << 7);
        self.w
    }
}
#[doc = "Field `TSEG1` reader - Time Segment 1"]
pub struct TSEG1_R(crate::FieldReader<u8, u8>);
impl TSEG1_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        TSEG1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TSEG1_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TSEG1` writer - Time Segment 1"]
pub struct TSEG1_W<'a> {
    w: &'a mut W,
}
impl<'a> TSEG1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 3)) | ((value as u32 & 0x0f) << 3);
        self.w
    }
}
#[doc = "Field `TSEG2` reader - Time Segment 2"]
pub struct TSEG2_R(crate::FieldReader<u8, u8>);
impl TSEG2_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        TSEG2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TSEG2_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TSEG2` writer - Time Segment 2"]
pub struct TSEG2_W<'a> {
    w: &'a mut W,
}
impl<'a> TSEG2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | (value as u32 & 0x07);
        self.w
    }
}
impl R {
    #[doc = "Bits 9:15 - Prescaler Configuration"]
    #[inline(always)]
    pub fn psc(&self) -> PSC_R {
        PSC_R::new(((self.bits >> 9) & 0x7f) as u8)
    }
    #[doc = "Bits 7:8 - Synchronization Jump Width"]
    #[inline(always)]
    pub fn sjw(&self) -> SJW_R {
        SJW_R::new(((self.bits >> 7) & 0x03) as u8)
    }
    #[doc = "Bits 3:6 - Time Segment 1"]
    #[inline(always)]
    pub fn tseg1(&self) -> TSEG1_R {
        TSEG1_R::new(((self.bits >> 3) & 0x0f) as u8)
    }
    #[doc = "Bits 0:2 - Time Segment 2"]
    #[inline(always)]
    pub fn tseg2(&self) -> TSEG2_R {
        TSEG2_R::new((self.bits & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 9:15 - Prescaler Configuration"]
    #[inline(always)]
    pub fn psc(&mut self) -> PSC_W {
        PSC_W { w: self }
    }
    #[doc = "Bits 7:8 - Synchronization Jump Width"]
    #[inline(always)]
    pub fn sjw(&mut self) -> SJW_W {
        SJW_W { w: self }
    }
    #[doc = "Bits 3:6 - Time Segment 1"]
    #[inline(always)]
    pub fn tseg1(&mut self) -> TSEG1_W {
        TSEG1_W { w: self }
    }
    #[doc = "Bits 0:2 - Time Segment 2"]
    #[inline(always)]
    pub fn tseg2(&mut self) -> TSEG2_W {
        TSEG2_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CAN Timing Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctim](index.html) module"]
pub struct CTIM_SPEC;
impl crate::RegisterSpec for CTIM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctim::R](R) reader structure"]
impl crate::Readable for CTIM_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctim::W](W) writer structure"]
impl crate::Writable for CTIM_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CTIM to value 0"]
impl crate::Resettable for CTIM_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
