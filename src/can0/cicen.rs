#[doc = "Register `CICEN` reader"]
pub struct R(crate::R<CICEN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CICEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CICEN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CICEN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CICEN` writer"]
pub struct W(crate::W<CICEN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CICEN_SPEC>;
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
impl From<crate::W<CICEN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CICEN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EICEN` reader - Error Interrupt Code Enable"]
pub struct EICEN_R(crate::FieldReader<bool, bool>);
impl EICEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EICEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EICEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EICEN` writer - Error Interrupt Code Enable"]
pub struct EICEN_W<'a> {
    w: &'a mut W,
}
impl<'a> EICEN_W<'a> {
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
#[doc = "Field `ICEN` reader - Buffer Interrupt Code Enable\\[14:0\\]"]
pub struct ICEN_R(crate::FieldReader<u16, u16>);
impl ICEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        ICEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ICEN_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ICEN` writer - Buffer Interrupt Code Enable\\[14:0\\]"]
pub struct ICEN_W<'a> {
    w: &'a mut W,
}
impl<'a> ICEN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7fff) | (value as u32 & 0x7fff);
        self.w
    }
}
impl R {
    #[doc = "Bit 15 - Error Interrupt Code Enable"]
    #[inline(always)]
    pub fn eicen(&self) -> EICEN_R {
        EICEN_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bits 0:14 - Buffer Interrupt Code Enable\\[14:0\\]"]
    #[inline(always)]
    pub fn icen(&self) -> ICEN_R {
        ICEN_R::new((self.bits & 0x7fff) as u16)
    }
}
impl W {
    #[doc = "Bit 15 - Error Interrupt Code Enable"]
    #[inline(always)]
    pub fn eicen(&mut self) -> EICEN_W {
        EICEN_W { w: self }
    }
    #[doc = "Bits 0:14 - Buffer Interrupt Code Enable\\[14:0\\]"]
    #[inline(always)]
    pub fn icen(&mut self) -> ICEN_W {
        ICEN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CAN Interrupt Code Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cicen](index.html) module"]
pub struct CICEN_SPEC;
impl crate::RegisterSpec for CICEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cicen::R](R) reader structure"]
impl crate::Readable for CICEN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cicen::W](W) writer structure"]
impl crate::Writable for CICEN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CICEN to value 0"]
impl crate::Resettable for CICEN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
