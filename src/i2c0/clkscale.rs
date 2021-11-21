#[doc = "Register `CLKSCALE` reader"]
pub struct R(crate::R<CLKSCALE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CLKSCALE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CLKSCALE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CLKSCALE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CLKSCALE` writer"]
pub struct W(crate::W<CLKSCALE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CLKSCALE_SPEC>;
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
impl From<crate::W<CLKSCALE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CLKSCALE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `VALUE` reader - Enable FastMode"]
pub struct VALUE_R(crate::FieldReader<u32, u32>);
impl VALUE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        VALUE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VALUE_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VALUE` writer - Enable FastMode"]
pub struct VALUE_W<'a> {
    w: &'a mut W,
}
impl<'a> VALUE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7fff_ffff) | (value as u32 & 0x7fff_ffff);
        self.w
    }
}
#[doc = "Field `FASTMODE` reader - Enable FastMode"]
pub struct FASTMODE_R(crate::FieldReader<bool, bool>);
impl FASTMODE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FASTMODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FASTMODE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FASTMODE` writer - Enable FastMode"]
pub struct FASTMODE_W<'a> {
    w: &'a mut W,
}
impl<'a> FASTMODE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | ((value as u32 & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:30 - Enable FastMode"]
    #[inline(always)]
    pub fn value(&self) -> VALUE_R {
        VALUE_R::new((self.bits & 0x7fff_ffff) as u32)
    }
    #[doc = "Bit 31 - Enable FastMode"]
    #[inline(always)]
    pub fn fastmode(&self) -> FASTMODE_R {
        FASTMODE_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:30 - Enable FastMode"]
    #[inline(always)]
    pub fn value(&mut self) -> VALUE_W {
        VALUE_W { w: self }
    }
    #[doc = "Bit 31 - Enable FastMode"]
    #[inline(always)]
    pub fn fastmode(&mut self) -> FASTMODE_W {
        FASTMODE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Clock Scale divide value\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clkscale](index.html) module"]
pub struct CLKSCALE_SPEC;
impl crate::RegisterSpec for CLKSCALE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [clkscale::R](R) reader structure"]
impl crate::Readable for CLKSCALE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [clkscale::W](W) writer structure"]
impl crate::Writable for CLKSCALE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CLKSCALE to value 0"]
impl crate::Resettable for CLKSCALE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
