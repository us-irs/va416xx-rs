#[doc = "Register `ADCSEL` reader"]
pub struct R(crate::R<ADCSEL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADCSEL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADCSEL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADCSEL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADCSEL` writer"]
pub struct W(crate::W<ADCSEL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADCSEL_SPEC>;
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
impl From<crate::W<ADCSEL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADCSEL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADCSEL` reader - ADC trigger source selection value"]
pub struct ADCSEL_R(crate::FieldReader<u8, u8>);
impl ADCSEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        ADCSEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADCSEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADCSEL` writer - ADC trigger source selection value"]
pub struct ADCSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> ADCSEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | (value as u32 & 0x1f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:4 - ADC trigger source selection value"]
    #[inline(always)]
    pub fn adcsel(&self) -> ADCSEL_R {
        ADCSEL_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - ADC trigger source selection value"]
    #[inline(always)]
    pub fn adcsel(&mut self) -> ADCSEL_W {
        ADCSEL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt select for ADC\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adcsel](index.html) module"]
pub struct ADCSEL_SPEC;
impl crate::RegisterSpec for ADCSEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [adcsel::R](R) reader structure"]
impl crate::Readable for ADCSEL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adcsel::W](W) writer structure"]
impl crate::Writable for ADCSEL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ADCSEL to value 0x1f"]
impl crate::Resettable for ADCSEL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x1f
    }
}
