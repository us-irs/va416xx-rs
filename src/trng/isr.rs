#[doc = "Register `ISR` reader"]
pub struct R(crate::R<ISR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ISR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ISR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ISR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `VN_ERR` reader - Indicates a Von Neumann error"]
pub struct VN_ERR_R(crate::FieldReader<bool, bool>);
impl VN_ERR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        VN_ERR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VN_ERR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CRNGT_ERR` reader - Indicates a Continuous Random Number Generation Testing (CRNGT) error"]
pub struct CRNGT_ERR_R(crate::FieldReader<bool, bool>);
impl CRNGT_ERR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CRNGT_ERR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CRNGT_ERR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AUTOCORR_ERR` reader - Indicates that the Autocorrelation test failed four times in a row"]
pub struct AUTOCORR_ERR_R(crate::FieldReader<bool, bool>);
impl AUTOCORR_ERR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        AUTOCORR_ERR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AUTOCORR_ERR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EHR_VALID` reader - 192 bits have been collected in the TRNG"]
pub struct EHR_VALID_R(crate::FieldReader<bool, bool>);
impl EHR_VALID_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EHR_VALID_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EHR_VALID_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 3 - Indicates a Von Neumann error"]
    #[inline(always)]
    pub fn vn_err(&self) -> VN_ERR_R {
        VN_ERR_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Indicates a Continuous Random Number Generation Testing (CRNGT) error"]
    #[inline(always)]
    pub fn crngt_err(&self) -> CRNGT_ERR_R {
        CRNGT_ERR_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - Indicates that the Autocorrelation test failed four times in a row"]
    #[inline(always)]
    pub fn autocorr_err(&self) -> AUTOCORR_ERR_R {
        AUTOCORR_ERR_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - 192 bits have been collected in the TRNG"]
    #[inline(always)]
    pub fn ehr_valid(&self) -> EHR_VALID_R {
        EHR_VALID_R::new((self.bits & 0x01) != 0)
    }
}
#[doc = "Interrupt Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [isr](index.html) module"]
pub struct ISR_SPEC;
impl crate::RegisterSpec for ISR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [isr::R](R) reader structure"]
impl crate::Readable for ISR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets ISR to value 0"]
impl crate::Resettable for ISR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
