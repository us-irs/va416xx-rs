#[doc = "Register `AREG_CAL` reader"]
pub struct R(crate::R<AREG_CAL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AREG_CAL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AREG_CAL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AREG_CAL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `AREG_CAL` reader - Analog LDO Regulator Calibration bits"]
pub struct AREG_CAL_R(crate::FieldReader<u16, u16>);
impl AREG_CAL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        AREG_CAL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AREG_CAL_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:8 - Analog LDO Regulator Calibration bits"]
    #[inline(always)]
    pub fn areg_cal(&self) -> AREG_CAL_R {
        AREG_CAL_R::new((self.bits & 0x01ff) as u16)
    }
}
#[doc = "Analog LDO Regulator Calibration Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [areg_cal](index.html) module"]
pub struct AREG_CAL_SPEC;
impl crate::RegisterSpec for AREG_CAL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [areg_cal::R](R) reader structure"]
impl crate::Readable for AREG_CAL_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets AREG_CAL to value 0"]
impl crate::Resettable for AREG_CAL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
