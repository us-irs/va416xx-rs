#[doc = "Register `DAC1_CAL` reader"]
pub struct R(crate::R<DAC1_CAL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DAC1_CAL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DAC1_CAL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DAC1_CAL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `DAC1_CAL` reader - DAC1 Calibration bits"]
pub struct DAC1_CAL_R(crate::FieldReader<u8, u8>);
impl DAC1_CAL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DAC1_CAL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DAC1_CAL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:4 - DAC1 Calibration bits"]
    #[inline(always)]
    pub fn dac1_cal(&self) -> DAC1_CAL_R {
        DAC1_CAL_R::new((self.bits & 0x1f) as u8)
    }
}
#[doc = "DAC1 Calibration Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dac1_cal](index.html) module"]
pub struct DAC1_CAL_SPEC;
impl crate::RegisterSpec for DAC1_CAL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dac1_cal::R](R) reader structure"]
impl crate::Readable for DAC1_CAL_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DAC1_CAL to value 0"]
impl crate::Resettable for DAC1_CAL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
