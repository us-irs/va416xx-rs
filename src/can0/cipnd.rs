#[doc = "Register `CIPND` reader"]
pub struct R(crate::R<CIPND_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CIPND_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CIPND_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CIPND_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CIPND` writer"]
pub struct W(crate::W<CIPND_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CIPND_SPEC>;
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
impl From<crate::W<CIPND_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CIPND_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EIPND` reader - Error Interrupt Pending"]
pub struct EIPND_R(crate::FieldReader<bool, bool>);
impl EIPND_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EIPND_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EIPND_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EIPND` writer - Error Interrupt Pending"]
pub struct EIPND_W<'a> {
    w: &'a mut W,
}
impl<'a> EIPND_W<'a> {
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
#[doc = "Field `IPND` reader - Buffer Interrupt Pending\\[14:0\\]"]
pub struct IPND_R(crate::FieldReader<u16, u16>);
impl IPND_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        IPND_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IPND_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IPND` writer - Buffer Interrupt Pending\\[14:0\\]"]
pub struct IPND_W<'a> {
    w: &'a mut W,
}
impl<'a> IPND_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7fff) | (value as u32 & 0x7fff);
        self.w
    }
}
impl R {
    #[doc = "Bit 15 - Error Interrupt Pending"]
    #[inline(always)]
    pub fn eipnd(&self) -> EIPND_R {
        EIPND_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bits 0:14 - Buffer Interrupt Pending\\[14:0\\]"]
    #[inline(always)]
    pub fn ipnd(&self) -> IPND_R {
        IPND_R::new((self.bits & 0x7fff) as u16)
    }
}
impl W {
    #[doc = "Bit 15 - Error Interrupt Pending"]
    #[inline(always)]
    pub fn eipnd(&mut self) -> EIPND_W {
        EIPND_W { w: self }
    }
    #[doc = "Bits 0:14 - Buffer Interrupt Pending\\[14:0\\]"]
    #[inline(always)]
    pub fn ipnd(&mut self) -> IPND_W {
        IPND_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CAN Interrupt Pending Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cipnd](index.html) module"]
pub struct CIPND_SPEC;
impl crate::RegisterSpec for CIPND_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cipnd::R](R) reader structure"]
impl crate::Readable for CIPND_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cipnd::W](W) writer structure"]
impl crate::Writable for CIPND_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CIPND to value 0"]
impl crate::Resettable for CIPND_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
