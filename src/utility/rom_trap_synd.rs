#[doc = "Register `ROM_TRAP_SYND` reader"]
pub struct R(crate::R<ROM_TRAP_SYND_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ROM_TRAP_SYND_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ROM_TRAP_SYND_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ROM_TRAP_SYND_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ROM_TRAP_SYND` writer"]
pub struct W(crate::W<ROM_TRAP_SYND_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ROM_TRAP_SYND_SPEC>;
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
impl From<crate::W<ROM_TRAP_SYND_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ROM_TRAP_SYND_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `R0M_SYND_31_16` reader - 6-bit syndrome value for bits 31-16"]
pub struct R0M_SYND_31_16_R(crate::FieldReader<u8, u8>);
impl R0M_SYND_31_16_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        R0M_SYND_31_16_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for R0M_SYND_31_16_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `R0M_SYND_31_16` writer - 6-bit syndrome value for bits 31-16"]
pub struct R0M_SYND_31_16_W<'a> {
    w: &'a mut W,
}
impl<'a> R0M_SYND_31_16_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 6)) | ((value as u32 & 0x3f) << 6);
        self.w
    }
}
#[doc = "Field `ROM_SYND_7_0` reader - 6-bit syndrome value for bits 15-0"]
pub struct ROM_SYND_7_0_R(crate::FieldReader<u8, u8>);
impl ROM_SYND_7_0_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        ROM_SYND_7_0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ROM_SYND_7_0_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ROM_SYND_7_0` writer - 6-bit syndrome value for bits 15-0"]
pub struct ROM_SYND_7_0_W<'a> {
    w: &'a mut W,
}
impl<'a> ROM_SYND_7_0_W<'a> {
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
    pub fn r0m_synd_31_16(&self) -> R0M_SYND_31_16_R {
        R0M_SYND_31_16_R::new(((self.bits >> 6) & 0x3f) as u8)
    }
    #[doc = "Bits 0:5 - 6-bit syndrome value for bits 15-0"]
    #[inline(always)]
    pub fn rom_synd_7_0(&self) -> ROM_SYND_7_0_R {
        ROM_SYND_7_0_R::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 6:11 - 6-bit syndrome value for bits 31-16"]
    #[inline(always)]
    pub fn r0m_synd_31_16(&mut self) -> R0M_SYND_31_16_W {
        R0M_SYND_31_16_W { w: self }
    }
    #[doc = "Bits 0:5 - 6-bit syndrome value for bits 15-0"]
    #[inline(always)]
    pub fn rom_synd_7_0(&mut self) -> ROM_SYND_7_0_W {
        ROM_SYND_7_0_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ROM EDAC Trap Syndrome\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rom_trap_synd](index.html) module"]
pub struct ROM_TRAP_SYND_SPEC;
impl crate::RegisterSpec for ROM_TRAP_SYND_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rom_trap_synd::R](R) reader structure"]
impl crate::Readable for ROM_TRAP_SYND_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rom_trap_synd::W](W) writer structure"]
impl crate::Writable for ROM_TRAP_SYND_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ROM_TRAP_SYND to value 0"]
impl crate::Resettable for ROM_TRAP_SYND_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
