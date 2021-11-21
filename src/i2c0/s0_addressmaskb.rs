#[doc = "Register `S0_ADDRESSMASKB` reader"]
pub struct R(crate::R<S0_ADDRESSMASKB_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<S0_ADDRESSMASKB_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<S0_ADDRESSMASKB_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<S0_ADDRESSMASKB_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `S0_ADDRESSMASKB` writer"]
pub struct W(crate::W<S0_ADDRESSMASKB_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<S0_ADDRESSMASKB_SPEC>;
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
impl From<crate::W<S0_ADDRESSMASKB_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<S0_ADDRESSMASKB_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RWMASK` reader - Read write mask"]
pub struct RWMASK_R(crate::FieldReader<bool, bool>);
impl RWMASK_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RWMASK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RWMASK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RWMASK` writer - Read write mask"]
pub struct RWMASK_W<'a> {
    w: &'a mut W,
}
impl<'a> RWMASK_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
#[doc = "Field `MASK` reader - Address mask value"]
pub struct MASK_R(crate::FieldReader<u16, u16>);
impl MASK_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        MASK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MASK_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MASK` writer - Address mask value"]
pub struct MASK_W<'a> {
    w: &'a mut W,
}
impl<'a> MASK_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03ff << 1)) | ((value as u32 & 0x03ff) << 1);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Read write mask"]
    #[inline(always)]
    pub fn rwmask(&self) -> RWMASK_R {
        RWMASK_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 1:10 - Address mask value"]
    #[inline(always)]
    pub fn mask(&self) -> MASK_R {
        MASK_R::new(((self.bits >> 1) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - Read write mask"]
    #[inline(always)]
    pub fn rwmask(&mut self) -> RWMASK_W {
        RWMASK_W { w: self }
    }
    #[doc = "Bits 1:10 - Address mask value"]
    #[inline(always)]
    pub fn mask(&mut self) -> MASK_W {
        MASK_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Slave I2C Address B Mask value\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [s0_addressmaskb](index.html) module"]
pub struct S0_ADDRESSMASKB_SPEC;
impl crate::RegisterSpec for S0_ADDRESSMASKB_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [s0_addressmaskb::R](R) reader structure"]
impl crate::Readable for S0_ADDRESSMASKB_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [s0_addressmaskb::W](W) writer structure"]
impl crate::Writable for S0_ADDRESSMASKB_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets S0_ADDRESSMASKB to value 0x07fe"]
impl crate::Resettable for S0_ADDRESSMASKB_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x07fe
    }
}
