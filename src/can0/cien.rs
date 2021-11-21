#[doc = "Register `CIEN` reader"]
pub struct R(crate::R<CIEN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CIEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CIEN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CIEN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CIEN` writer"]
pub struct W(crate::W<CIEN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CIEN_SPEC>;
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
impl From<crate::W<CIEN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CIEN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EIEN` reader - Error Interrupt Enable"]
pub struct EIEN_R(crate::FieldReader<bool, bool>);
impl EIEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EIEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EIEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EIEN` writer - Error Interrupt Enable"]
pub struct EIEN_W<'a> {
    w: &'a mut W,
}
impl<'a> EIEN_W<'a> {
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
#[doc = "Field `IEN` reader - Buffer Interrupt Enable\\[14:0\\]"]
pub struct IEN_R(crate::FieldReader<u16, u16>);
impl IEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        IEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IEN_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IEN` writer - Buffer Interrupt Enable\\[14:0\\]"]
pub struct IEN_W<'a> {
    w: &'a mut W,
}
impl<'a> IEN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7fff) | (value as u32 & 0x7fff);
        self.w
    }
}
impl R {
    #[doc = "Bit 15 - Error Interrupt Enable"]
    #[inline(always)]
    pub fn eien(&self) -> EIEN_R {
        EIEN_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bits 0:14 - Buffer Interrupt Enable\\[14:0\\]"]
    #[inline(always)]
    pub fn ien(&self) -> IEN_R {
        IEN_R::new((self.bits & 0x7fff) as u16)
    }
}
impl W {
    #[doc = "Bit 15 - Error Interrupt Enable"]
    #[inline(always)]
    pub fn eien(&mut self) -> EIEN_W {
        EIEN_W { w: self }
    }
    #[doc = "Bits 0:14 - Buffer Interrupt Enable\\[14:0\\]"]
    #[inline(always)]
    pub fn ien(&mut self) -> IEN_W {
        IEN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CAN Interrupt Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cien](index.html) module"]
pub struct CIEN_SPEC;
impl crate::RegisterSpec for CIEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cien::R](R) reader structure"]
impl crate::Readable for CIEN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cien::W](W) writer structure"]
impl crate::Writable for CIEN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CIEN to value 0"]
impl crate::Resettable for CIEN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
