#[doc = "Register `DMAADDR0` reader"]
pub struct R(crate::R<DMAADDR0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMAADDR0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMAADDR0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMAADDR0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DMAADDR0` writer"]
pub struct W(crate::W<DMAADDR0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMAADDR0_SPEC>;
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
impl From<crate::W<DMAADDR0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DMAADDR0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MASK` reader - Mask"]
pub struct MASK_R(crate::FieldReader<u8, u8>);
impl MASK_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        MASK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MASK_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MASK` writer - Mask"]
pub struct MASK_W<'a> {
    w: &'a mut W,
}
impl<'a> MASK_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | ((value as u32 & 0xff) << 8);
        self.w
    }
}
#[doc = "Field `ADDR` reader - Address"]
pub struct ADDR_R(crate::FieldReader<u8, u8>);
impl ADDR_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        ADDR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADDR_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADDR` writer - Address"]
pub struct ADDR_W<'a> {
    w: &'a mut W,
}
impl<'a> ADDR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 8:15 - Mask"]
    #[inline(always)]
    pub fn mask(&self) -> MASK_R {
        MASK_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 0:7 - Address"]
    #[inline(always)]
    pub fn addr(&self) -> ADDR_R {
        ADDR_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 8:15 - Mask"]
    #[inline(always)]
    pub fn mask(&mut self) -> MASK_W {
        MASK_W { w: self }
    }
    #[doc = "Bits 0:7 - Address"]
    #[inline(always)]
    pub fn addr(&mut self) -> ADDR_W {
        ADDR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DMA Receiver Table Address Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmaaddr0](index.html) module"]
pub struct DMAADDR0_SPEC;
impl crate::RegisterSpec for DMAADDR0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dmaaddr0::R](R) reader structure"]
impl crate::Readable for DMAADDR0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dmaaddr0::W](W) writer structure"]
impl crate::Writable for DMAADDR0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DMAADDR0 to value 0"]
impl crate::Resettable for DMAADDR0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
