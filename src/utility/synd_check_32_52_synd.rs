#[doc = "Register `SYND_CHECK_32_52_SYND` reader"]
pub struct R(crate::R<SYND_CHECK_32_52_SYND_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SYND_CHECK_32_52_SYND_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SYND_CHECK_32_52_SYND_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SYND_CHECK_32_52_SYND_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `MBE` reader - Multiple bit error detect status"]
pub struct MBE_R(crate::FieldReader<u8, u8>);
impl MBE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        MBE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MBE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SBE` reader - Single bit error detect status"]
pub struct SBE_R(crate::FieldReader<u8, u8>);
impl SBE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SBE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SBE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SYND_CHECK_32_52_SYND` reader - Corrected syndrome value"]
pub struct SYND_CHECK_32_52_SYND_R(crate::FieldReader<u32, u32>);
impl SYND_CHECK_32_52_SYND_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        SYND_CHECK_32_52_SYND_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SYND_CHECK_32_52_SYND_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 28:31 - Multiple bit error detect status"]
    #[inline(always)]
    pub fn mbe(&self) -> MBE_R {
        MBE_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - Single bit error detect status"]
    #[inline(always)]
    pub fn sbe(&self) -> SBE_R {
        SBE_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 0:19 - Corrected syndrome value"]
    #[inline(always)]
    pub fn synd_check_32_52_synd(&self) -> SYND_CHECK_32_52_SYND_R {
        SYND_CHECK_32_52_SYND_R::new((self.bits & 0x000f_ffff) as u32)
    }
}
#[doc = "EDAC Decode Syndrome\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [synd_check_32_52_synd](index.html) module"]
pub struct SYND_CHECK_32_52_SYND_SPEC;
impl crate::RegisterSpec for SYND_CHECK_32_52_SYND_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [synd_check_32_52_synd::R](R) reader structure"]
impl crate::Readable for SYND_CHECK_32_52_SYND_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SYND_CHECK_32_52_SYND to value 0"]
impl crate::Resettable for SYND_CHECK_32_52_SYND_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
