#[doc = "Register `CTRL1` reader"]
pub struct R(crate::R<CTRL1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTRL1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTRL1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTRL1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTRL1` writer"]
pub struct W(crate::W<CTRL1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTRL1_SPEC>;
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
impl From<crate::W<CTRL1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTRL1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DAC_EN` reader - Enables the DAC analog block"]
pub struct DAC_EN_R(crate::FieldReader<bool, bool>);
impl DAC_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DAC_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DAC_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DAC_EN` writer - Enables the DAC analog block"]
pub struct DAC_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> DAC_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | ((value as u32 & 0x01) << 8);
        self.w
    }
}
#[doc = "Field `DAC_SETTLING` reader - Sets the the amount of time in microseconds the control FSM waits for the DAC settling time"]
pub struct DAC_SETTLING_R(crate::FieldReader<u8, u8>);
impl DAC_SETTLING_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DAC_SETTLING_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DAC_SETTLING_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DAC_SETTLING` writer - Sets the the amount of time in microseconds the control FSM waits for the DAC settling time"]
pub struct DAC_SETTLING_W<'a> {
    w: &'a mut W,
}
impl<'a> DAC_SETTLING_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 5)) | ((value as u32 & 0x07) << 5);
        self.w
    }
}
impl R {
    #[doc = "Bit 8 - Enables the DAC analog block"]
    #[inline(always)]
    pub fn dac_en(&self) -> DAC_EN_R {
        DAC_EN_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bits 5:7 - Sets the the amount of time in microseconds the control FSM waits for the DAC settling time"]
    #[inline(always)]
    pub fn dac_settling(&self) -> DAC_SETTLING_R {
        DAC_SETTLING_R::new(((self.bits >> 5) & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bit 8 - Enables the DAC analog block"]
    #[inline(always)]
    pub fn dac_en(&mut self) -> DAC_EN_W {
        DAC_EN_W { w: self }
    }
    #[doc = "Bits 5:7 - Sets the the amount of time in microseconds the control FSM waits for the DAC settling time"]
    #[inline(always)]
    pub fn dac_settling(&mut self) -> DAC_SETTLING_W {
        DAC_SETTLING_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Control Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl1](index.html) module"]
pub struct CTRL1_SPEC;
impl crate::RegisterSpec for CTRL1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctrl1::R](R) reader structure"]
impl crate::Readable for CTRL1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctrl1::W](W) writer structure"]
impl crate::Writable for CTRL1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CTRL1 to value 0"]
impl crate::Resettable for CTRL1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
