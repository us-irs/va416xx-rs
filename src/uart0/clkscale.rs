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
#[doc = "Field `FRAC` reader - Fractional Divide (64ths)"]
pub struct FRAC_R(crate::FieldReader<u8, u8>);
impl FRAC_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        FRAC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FRAC_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FRAC` writer - Fractional Divide (64ths)"]
pub struct FRAC_W<'a> {
    w: &'a mut W,
}
impl<'a> FRAC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | (value as u32 & 0x3f);
        self.w
    }
}
#[doc = "Field `INT` reader - Integer Divide"]
pub struct INT_R(crate::FieldReader<u32, u32>);
impl INT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        INT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INT_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INT` writer - Integer Divide"]
pub struct INT_W<'a> {
    w: &'a mut W,
}
impl<'a> INT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0003_ffff << 6)) | ((value as u32 & 0x0003_ffff) << 6);
        self.w
    }
}
#[doc = "Field `RESET` writer - Reset Baud Counter"]
pub struct RESET_W<'a> {
    w: &'a mut W,
}
impl<'a> RESET_W<'a> {
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
    #[doc = "Bits 0:5 - Fractional Divide (64ths)"]
    #[inline(always)]
    pub fn frac(&self) -> FRAC_R {
        FRAC_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 6:23 - Integer Divide"]
    #[inline(always)]
    pub fn int(&self) -> INT_R {
        INT_R::new(((self.bits >> 6) & 0x0003_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:5 - Fractional Divide (64ths)"]
    #[inline(always)]
    pub fn frac(&mut self) -> FRAC_W {
        FRAC_W { w: self }
    }
    #[doc = "Bits 6:23 - Integer Divide"]
    #[inline(always)]
    pub fn int(&mut self) -> INT_W {
        INT_W { w: self }
    }
    #[doc = "Bit 31 - Reset Baud Counter"]
    #[inline(always)]
    pub fn reset(&mut self) -> RESET_W {
        RESET_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Clock Scale Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clkscale](index.html) module"]
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
