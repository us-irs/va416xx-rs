#[doc = "Register `ADC_CAL` reader"]
pub struct R(crate::R<ADC_CAL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADC_CAL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADC_CAL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADC_CAL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `ADC_CAL` reader - ADC Calibration bits"]
pub struct ADC_CAL_R(crate::FieldReader<u8, u8>);
impl ADC_CAL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        ADC_CAL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADC_CAL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:4 - ADC Calibration bits"]
    #[inline(always)]
    pub fn adc_cal(&self) -> ADC_CAL_R {
        ADC_CAL_R::new((self.bits & 0x1f) as u8)
    }
}
#[doc = "ADC Calibration Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc_cal](index.html) module"]
pub struct ADC_CAL_SPEC;
impl crate::RegisterSpec for ADC_CAL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [adc_cal::R](R) reader structure"]
impl crate::Readable for ADC_CAL_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets ADC_CAL to value 0"]
impl crate::Resettable for ADC_CAL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
