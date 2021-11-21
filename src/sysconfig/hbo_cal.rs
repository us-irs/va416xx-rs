#[doc = "Register `HBO_CAL` reader"]
pub struct R(crate::R<HBO_CAL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HBO_CAL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HBO_CAL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HBO_CAL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `OSC_CAL` reader - 1MHz OSC Calibration bit"]
pub struct OSC_CAL_R(crate::FieldReader<bool, bool>);
impl OSC_CAL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        OSC_CAL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OSC_CAL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HBO_CAL` reader - Heart Beat OSC Calibration bits"]
pub struct HBO_CAL_R(crate::FieldReader<u8, u8>);
impl HBO_CAL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        HBO_CAL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HBO_CAL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 3 - 1MHz OSC Calibration bit"]
    #[inline(always)]
    pub fn osc_cal(&self) -> OSC_CAL_R {
        OSC_CAL_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bits 0:2 - Heart Beat OSC Calibration bits"]
    #[inline(always)]
    pub fn hbo_cal(&self) -> HBO_CAL_R {
        HBO_CAL_R::new((self.bits & 0x07) as u8)
    }
}
#[doc = "Heart Beat OSC Calibration Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hbo_cal](index.html) module"]
pub struct HBO_CAL_SPEC;
impl crate::RegisterSpec for HBO_CAL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hbo_cal::R](R) reader structure"]
impl crate::Readable for HBO_CAL_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets HBO_CAL to value 0"]
impl crate::Resettable for HBO_CAL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
