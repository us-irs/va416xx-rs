#[doc = "Register `WDOGITOP` reader"]
pub struct R(crate::R<WDOGITOP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WDOGITOP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WDOGITOP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WDOGITOP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `WDOGITOP` writer"]
pub struct W(crate::W<WDOGITOP_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WDOGITOP_SPEC>;
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
impl From<crate::W<WDOGITOP_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WDOGITOP_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WDOGINT` reader - Set output value"]
pub struct WDOGINT_R(crate::FieldReader<bool, bool>);
impl WDOGINT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        WDOGINT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WDOGINT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WDOGINT` writer - Set output value"]
pub struct WDOGINT_W<'a> {
    w: &'a mut W,
}
impl<'a> WDOGINT_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
#[doc = "Field `WDOGRES` reader - Set output value"]
pub struct WDOGRES_R(crate::FieldReader<bool, bool>);
impl WDOGRES_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        WDOGRES_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WDOGRES_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WDOGRES` writer - Set output value"]
pub struct WDOGRES_W<'a> {
    w: &'a mut W,
}
impl<'a> WDOGRES_W<'a> {
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
    #[doc = "Bit 1 - Set output value"]
    #[inline(always)]
    pub fn wdogint(&self) -> WDOGINT_R {
        WDOGINT_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Set output value"]
    #[inline(always)]
    pub fn wdogres(&self) -> WDOGRES_R {
        WDOGRES_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Set output value"]
    #[inline(always)]
    pub fn wdogint(&mut self) -> WDOGINT_W {
        WDOGINT_W { w: self }
    }
    #[doc = "Bit 0 - Set output value"]
    #[inline(always)]
    pub fn wdogres(&mut self) -> WDOGRES_W {
        WDOGRES_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Integration test output set\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wdogitop](index.html) module"]
pub struct WDOGITOP_SPEC;
impl crate::RegisterSpec for WDOGITOP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [wdogitop::R](R) reader structure"]
impl crate::Readable for WDOGITOP_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [wdogitop::W](W) writer structure"]
impl crate::Writable for WDOGITOP_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets WDOGITOP to value 0"]
impl crate::Resettable for WDOGITOP_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
