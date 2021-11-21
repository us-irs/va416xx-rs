#[doc = "Register `S0_ADDRESSB` reader"]
pub struct R(crate::R<S0_ADDRESSB_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<S0_ADDRESSB_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<S0_ADDRESSB_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<S0_ADDRESSB_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `S0_ADDRESSB` writer"]
pub struct W(crate::W<S0_ADDRESSB_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<S0_ADDRESSB_SPEC>;
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
impl From<crate::W<S0_ADDRESSB_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<S0_ADDRESSB_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RW` reader - Read write value"]
pub struct RW_R(crate::FieldReader<bool, bool>);
impl RW_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RW` writer - Read write value"]
pub struct RW_W<'a> {
    w: &'a mut W,
}
impl<'a> RW_W<'a> {
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
#[doc = "Field `ADDRESS` reader - Address value"]
pub struct ADDRESS_R(crate::FieldReader<u16, u16>);
impl ADDRESS_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        ADDRESS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADDRESS_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADDRESS` writer - Address value"]
pub struct ADDRESS_W<'a> {
    w: &'a mut W,
}
impl<'a> ADDRESS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03ff << 1)) | ((value as u32 & 0x03ff) << 1);
        self.w
    }
}
#[doc = "Field `ADDRESSBEN` reader - Enable Address B"]
pub struct ADDRESSBEN_R(crate::FieldReader<bool, bool>);
impl ADDRESSBEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ADDRESSBEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADDRESSBEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADDRESSBEN` writer - Enable Address B"]
pub struct ADDRESSBEN_W<'a> {
    w: &'a mut W,
}
impl<'a> ADDRESSBEN_W<'a> {
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
impl R {
    #[doc = "Bit 0 - Read write value"]
    #[inline(always)]
    pub fn rw(&self) -> RW_R {
        RW_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 1:10 - Address value"]
    #[inline(always)]
    pub fn address(&self) -> ADDRESS_R {
        ADDRESS_R::new(((self.bits >> 1) & 0x03ff) as u16)
    }
    #[doc = "Bit 15 - Enable Address B"]
    #[inline(always)]
    pub fn addressben(&self) -> ADDRESSBEN_R {
        ADDRESSBEN_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Read write value"]
    #[inline(always)]
    pub fn rw(&mut self) -> RW_W {
        RW_W { w: self }
    }
    #[doc = "Bits 1:10 - Address value"]
    #[inline(always)]
    pub fn address(&mut self) -> ADDRESS_W {
        ADDRESS_W { w: self }
    }
    #[doc = "Bit 15 - Enable Address B"]
    #[inline(always)]
    pub fn addressben(&mut self) -> ADDRESSBEN_W {
        ADDRESSBEN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Slave I2C Address B Value\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [s0_addressb](index.html) module"]
pub struct S0_ADDRESSB_SPEC;
impl crate::RegisterSpec for S0_ADDRESSB_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [s0_addressb::R](R) reader structure"]
impl crate::Readable for S0_ADDRESSB_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [s0_addressb::W](W) writer structure"]
impl crate::Writable for S0_ADDRESSB_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets S0_ADDRESSB to value 0"]
impl crate::Resettable for S0_ADDRESSB_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
