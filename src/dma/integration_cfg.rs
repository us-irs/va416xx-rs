#[doc = "Register `INTEGRATION_CFG` reader"]
pub struct R(crate::R<INTEGRATION_CFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTEGRATION_CFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTEGRATION_CFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTEGRATION_CFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INTEGRATION_CFG` writer"]
pub struct W(crate::W<INTEGRATION_CFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INTEGRATION_CFG_SPEC>;
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
impl From<crate::W<INTEGRATION_CFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INTEGRATION_CFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `INT_TEST_EN` reader - Error Clear"]
pub struct INT_TEST_EN_R(crate::FieldReader<bool, bool>);
impl INT_TEST_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        INT_TEST_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INT_TEST_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INT_TEST_EN` writer - Error Clear"]
pub struct INT_TEST_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> INT_TEST_EN_W<'a> {
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
    #[doc = "Bit 0 - Error Clear"]
    #[inline(always)]
    pub fn int_test_en(&self) -> INT_TEST_EN_R {
        INT_TEST_EN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Error Clear"]
    #[inline(always)]
    pub fn int_test_en(&mut self) -> INT_TEST_EN_W {
        INT_TEST_EN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DMA integration configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [integration_cfg](index.html) module"]
pub struct INTEGRATION_CFG_SPEC;
impl crate::RegisterSpec for INTEGRATION_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [integration_cfg::R](R) reader structure"]
impl crate::Readable for INTEGRATION_CFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [integration_cfg::W](W) writer structure"]
impl crate::Writable for INTEGRATION_CFG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets INTEGRATION_CFG to value 0"]
impl crate::Resettable for INTEGRATION_CFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
