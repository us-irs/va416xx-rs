#[doc = "Register `RAM_TRAP_SYND1` reader"]
pub struct R(crate::R<RAM_TRAP_SYND1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RAM_TRAP_SYND1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RAM_TRAP_SYND1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RAM_TRAP_SYND1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RAM_TRAP_SYND1` writer"]
pub struct W(crate::W<RAM_TRAP_SYND1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RAM_TRAP_SYND1_SPEC>;
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
impl From<crate::W<RAM_TRAP_SYND1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RAM_TRAP_SYND1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RAM_SYND_31_16` reader - 6-bit syndrome value for bits 31-16"]
pub struct RAM_SYND_31_16_R(crate::FieldReader<u8, u8>);
impl RAM_SYND_31_16_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        RAM_SYND_31_16_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RAM_SYND_31_16_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RAM_SYND_31_16` writer - 6-bit syndrome value for bits 31-16"]
pub struct RAM_SYND_31_16_W<'a> {
    w: &'a mut W,
}
impl<'a> RAM_SYND_31_16_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 6)) | ((value as u32 & 0x3f) << 6);
        self.w
    }
}
#[doc = "Field `RAM_SYND_7_0` reader - 6-bit syndrome value for bits 15-0"]
pub struct RAM_SYND_7_0_R(crate::FieldReader<u8, u8>);
impl RAM_SYND_7_0_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        RAM_SYND_7_0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RAM_SYND_7_0_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RAM_SYND_7_0` writer - 6-bit syndrome value for bits 15-0"]
pub struct RAM_SYND_7_0_W<'a> {
    w: &'a mut W,
}
impl<'a> RAM_SYND_7_0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | (value as u32 & 0x3f);
        self.w
    }
}
impl R {
    #[doc = "Bits 6:11 - 6-bit syndrome value for bits 31-16"]
    #[inline(always)]
    pub fn ram_synd_31_16(&self) -> RAM_SYND_31_16_R {
        RAM_SYND_31_16_R::new(((self.bits >> 6) & 0x3f) as u8)
    }
    #[doc = "Bits 0:5 - 6-bit syndrome value for bits 15-0"]
    #[inline(always)]
    pub fn ram_synd_7_0(&self) -> RAM_SYND_7_0_R {
        RAM_SYND_7_0_R::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 6:11 - 6-bit syndrome value for bits 31-16"]
    #[inline(always)]
    pub fn ram_synd_31_16(&mut self) -> RAM_SYND_31_16_W {
        RAM_SYND_31_16_W { w: self }
    }
    #[doc = "Bits 0:5 - 6-bit syndrome value for bits 15-0"]
    #[inline(always)]
    pub fn ram_synd_7_0(&mut self) -> RAM_SYND_7_0_W {
        RAM_SYND_7_0_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RAM1 EDAC Trap Syndrome\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ram_trap_synd1](index.html) module"]
pub struct RAM_TRAP_SYND1_SPEC;
impl crate::RegisterSpec for RAM_TRAP_SYND1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ram_trap_synd1::R](R) reader structure"]
impl crate::Readable for RAM_TRAP_SYND1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ram_trap_synd1::W](W) writer structure"]
impl crate::Writable for RAM_TRAP_SYND1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RAM_TRAP_SYND1 to value 0"]
impl crate::Resettable for RAM_TRAP_SYND1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
