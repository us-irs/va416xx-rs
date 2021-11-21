#[doc = "Register `DMATXDESC0` reader"]
pub struct R(crate::R<DMATXDESC0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMATXDESC0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMATXDESC0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMATXDESC0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DMATXDESC0` writer"]
pub struct W(crate::W<DMATXDESC0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMATXDESC0_SPEC>;
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
impl From<crate::W<DMATXDESC0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DMATXDESC0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DESCBASEADDR` reader - Sets the base address of the descriptor table"]
pub struct DESCBASEADDR_R(crate::FieldReader<u32, u32>);
impl DESCBASEADDR_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        DESCBASEADDR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DESCBASEADDR_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DESCBASEADDR` writer - Sets the base address of the descriptor table"]
pub struct DESCBASEADDR_W<'a> {
    w: &'a mut W,
}
impl<'a> DESCBASEADDR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x003f_ffff << 10)) | ((value as u32 & 0x003f_ffff) << 10);
        self.w
    }
}
#[doc = "Field `DESCSEL` reader - Offset into the descriptor table"]
pub struct DESCSEL_R(crate::FieldReader<u8, u8>);
impl DESCSEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DESCSEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DESCSEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DESCSEL` writer - Offset into the descriptor table"]
pub struct DESCSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> DESCSEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 4)) | ((value as u32 & 0x3f) << 4);
        self.w
    }
}
impl R {
    #[doc = "Bits 10:31 - Sets the base address of the descriptor table"]
    #[inline(always)]
    pub fn descbaseaddr(&self) -> DESCBASEADDR_R {
        DESCBASEADDR_R::new(((self.bits >> 10) & 0x003f_ffff) as u32)
    }
    #[doc = "Bits 4:9 - Offset into the descriptor table"]
    #[inline(always)]
    pub fn descsel(&self) -> DESCSEL_R {
        DESCSEL_R::new(((self.bits >> 4) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 10:31 - Sets the base address of the descriptor table"]
    #[inline(always)]
    pub fn descbaseaddr(&mut self) -> DESCBASEADDR_W {
        DESCBASEADDR_W { w: self }
    }
    #[doc = "Bits 4:9 - Offset into the descriptor table"]
    #[inline(always)]
    pub fn descsel(&mut self) -> DESCSEL_W {
        DESCSEL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DMA Transmitter Descriptor Table Address Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmatxdesc0](index.html) module"]
pub struct DMATXDESC0_SPEC;
impl crate::RegisterSpec for DMATXDESC0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dmatxdesc0::R](R) reader structure"]
impl crate::Readable for DMATXDESC0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dmatxdesc0::W](W) writer structure"]
impl crate::Writable for DMATXDESC0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DMATXDESC0 to value 0"]
impl crate::Resettable for DMATXDESC0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
