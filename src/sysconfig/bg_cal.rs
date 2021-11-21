#[doc = "Register `BG_CAL` reader"]
pub struct R(crate::R<BG_CAL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BG_CAL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BG_CAL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BG_CAL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `BG_CAL` reader - Bandgap Calibration bits"]
pub struct BG_CAL_R(crate::FieldReader<u8, u8>);
impl BG_CAL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        BG_CAL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BG_CAL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:2 - Bandgap Calibration bits"]
    #[inline(always)]
    pub fn bg_cal(&self) -> BG_CAL_R {
        BG_CAL_R::new((self.bits & 0x07) as u8)
    }
}
#[doc = "Bandgap Calibration Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bg_cal](index.html) module"]
pub struct BG_CAL_SPEC;
impl crate::RegisterSpec for BG_CAL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [bg_cal::R](R) reader structure"]
impl crate::Readable for BG_CAL_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets BG_CAL to value 0"]
impl crate::Resettable for BG_CAL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
