#[doc = "Register `ROM_PROT` reader"]
pub struct R(crate::R<ROM_PROT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ROM_PROT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ROM_PROT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ROM_PROT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ROM_PROT` writer"]
pub struct W(crate::W<ROM_PROT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ROM_PROT_SPEC>;
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
impl From<crate::W<ROM_PROT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ROM_PROT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WREN` reader - ROM Write Enable Bit"]
pub struct WREN_R(crate::FieldReader<bool, bool>);
impl WREN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        WREN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WREN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WREN` writer - ROM Write Enable Bit"]
pub struct WREN_W<'a> {
    w: &'a mut W,
}
impl<'a> WREN_W<'a> {
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
impl R {
    #[doc = "Bit 0 - ROM Write Enable Bit"]
    #[inline(always)]
    pub fn wren(&self) -> WREN_R {
        WREN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - ROM Write Enable Bit"]
    #[inline(always)]
    pub fn wren(&mut self) -> WREN_W {
        WREN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ROM Protection Configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rom_prot](index.html) module"]
pub struct ROM_PROT_SPEC;
impl crate::RegisterSpec for ROM_PROT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rom_prot::R](R) reader structure"]
impl crate::Readable for ROM_PROT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rom_prot::W](W) writer structure"]
impl crate::Writable for ROM_PROT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ROM_PROT to value 0"]
impl crate::Resettable for ROM_PROT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
