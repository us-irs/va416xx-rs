#[doc = "Register `ROM_SCRUB` reader"]
pub struct R(crate::R<ROM_SCRUB_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ROM_SCRUB_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ROM_SCRUB_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ROM_SCRUB_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ROM_SCRUB` writer"]
pub struct W(crate::W<ROM_SCRUB_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ROM_SCRUB_SPEC>;
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
impl From<crate::W<ROM_SCRUB_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ROM_SCRUB_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `VALUE` reader - Counter divide value"]
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
#[doc = "Field `VALUE` writer - Counter divide value"]
pub struct VALUE_W<'a> {
    w: &'a mut W,
}
impl<'a> VALUE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x00ff_ffff) | (value as u32 & 0x00ff_ffff);
        self.w
    }
}
#[doc = "Field `RESET` writer - Reset Counter"]
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
    #[doc = "Bits 0:23 - Counter divide value"]
    #[inline(always)]
    pub fn value(&self) -> VALUE_R {
        VALUE_R::new((self.bits & 0x00ff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:23 - Counter divide value"]
    #[inline(always)]
    pub fn value(&mut self) -> VALUE_W {
        VALUE_W { w: self }
    }
    #[doc = "Bit 31 - Reset Counter"]
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
#[doc = "ROM Scrub Period Configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rom_scrub](index.html) module"]
pub struct ROM_SCRUB_SPEC;
impl crate::RegisterSpec for ROM_SCRUB_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rom_scrub::R](R) reader structure"]
impl crate::Readable for ROM_SCRUB_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rom_scrub::W](W) writer structure"]
impl crate::Writable for ROM_SCRUB_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ROM_SCRUB to value 0"]
impl crate::Resettable for ROM_SCRUB_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
