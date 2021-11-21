#[doc = "Register `DREG_CAL` reader"]
pub struct R(crate::R<DREG_CAL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DREG_CAL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DREG_CAL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DREG_CAL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `DREG_CAL` reader - Digital LDO Regulator Calibration bits"]
pub struct DREG_CAL_R(crate::FieldReader<u16, u16>);
impl DREG_CAL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        DREG_CAL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DREG_CAL_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:8 - Digital LDO Regulator Calibration bits"]
    #[inline(always)]
    pub fn dreg_cal(&self) -> DREG_CAL_R {
        DREG_CAL_R::new((self.bits & 0x01ff) as u16)
    }
}
#[doc = "Digital LDO Regulator Calibration Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dreg_cal](index.html) module"]
pub struct DREG_CAL_SPEC;
impl crate::RegisterSpec for DREG_CAL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dreg_cal::R](R) reader structure"]
impl crate::Readable for DREG_CAL_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DREG_CAL to value 0"]
impl crate::Resettable for DREG_CAL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
