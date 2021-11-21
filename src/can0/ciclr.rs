#[doc = "Register `CICLR` reader"]
pub struct R(crate::R<CICLR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CICLR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CICLR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CICLR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CICLR` writer"]
pub struct W(crate::W<CICLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CICLR_SPEC>;
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
impl From<crate::W<CICLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CICLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EICLR` reader - Error Interrupt Clear"]
pub struct EICLR_R(crate::FieldReader<bool, bool>);
impl EICLR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EICLR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EICLR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EICLR` writer - Error Interrupt Clear"]
pub struct EICLR_W<'a> {
    w: &'a mut W,
}
impl<'a> EICLR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | ((value as u32 & 0x01) << 15);
        self.w
    }
}
#[doc = "Field `ICLR` reader - Buffer Interrupt Clear\\[14:0\\]"]
pub struct ICLR_R(crate::FieldReader<u16, u16>);
impl ICLR_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        ICLR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ICLR_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ICLR` writer - Buffer Interrupt Clear\\[14:0\\]"]
pub struct ICLR_W<'a> {
    w: &'a mut W,
}
impl<'a> ICLR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7fff) | (value as u32 & 0x7fff);
        self.w
    }
}
impl R {
    #[doc = "Bit 15 - Error Interrupt Clear"]
    #[inline(always)]
    pub fn eiclr(&self) -> EICLR_R {
        EICLR_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bits 0:14 - Buffer Interrupt Clear\\[14:0\\]"]
    #[inline(always)]
    pub fn iclr(&self) -> ICLR_R {
        ICLR_R::new((self.bits & 0x7fff) as u16)
    }
}
impl W {
    #[doc = "Bit 15 - Error Interrupt Clear"]
    #[inline(always)]
    pub fn eiclr(&mut self) -> EICLR_W {
        EICLR_W { w: self }
    }
    #[doc = "Bits 0:14 - Buffer Interrupt Clear\\[14:0\\]"]
    #[inline(always)]
    pub fn iclr(&mut self) -> ICLR_W {
        ICLR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CAN Interrupt Clear Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ciclr](index.html) module"]
pub struct CICLR_SPEC;
impl crate::RegisterSpec for CICLR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ciclr::R](R) reader structure"]
impl crate::Readable for CICLR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ciclr::W](W) writer structure"]
impl crate::Writable for CICLR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CICLR to value 0"]
impl crate::Resettable for CICLR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
