#[doc = "Register `ROM_SBE` reader"]
pub struct R(crate::R<ROM_SBE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ROM_SBE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ROM_SBE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ROM_SBE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ROM_SBE` writer"]
pub struct W(crate::W<ROM_SBE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ROM_SBE_SPEC>;
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
impl From<crate::W<ROM_SBE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ROM_SBE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `COUNT` reader - RAM0 EDAC Single Bit Errors"]
pub struct COUNT_R(crate::FieldReader<u16, u16>);
impl COUNT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        COUNT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for COUNT_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `COUNT` writer - RAM0 EDAC Single Bit Errors"]
pub struct COUNT_W<'a> {
    w: &'a mut W,
}
impl<'a> COUNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - RAM0 EDAC Single Bit Errors"]
    #[inline(always)]
    pub fn count(&self) -> COUNT_R {
        COUNT_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - RAM0 EDAC Single Bit Errors"]
    #[inline(always)]
    pub fn count(&mut self) -> COUNT_W {
        COUNT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Count of ROM EDAC Single Bit Errors\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rom_sbe](index.html) module"]
pub struct ROM_SBE_SPEC;
impl crate::RegisterSpec for ROM_SBE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rom_sbe::R](R) reader structure"]
impl crate::Readable for ROM_SBE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rom_sbe::W](W) writer structure"]
impl crate::Writable for ROM_SBE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ROM_SBE to value 0"]
impl crate::Resettable for ROM_SBE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
