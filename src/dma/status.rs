#[doc = "Register `STATUS` reader"]
pub struct R(crate::R<STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `TEST_STATUS` reader - Test Status Logic Included"]
pub struct TEST_STATUS_R(crate::FieldReader<u8, u8>);
impl TEST_STATUS_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        TEST_STATUS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TEST_STATUS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CHNLS_MINUS1` reader - Number of Available Channels Minus 1"]
pub struct CHNLS_MINUS1_R(crate::FieldReader<u8, u8>);
impl CHNLS_MINUS1_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CHNLS_MINUS1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CHNLS_MINUS1_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `STATE` reader - Current State of the control state machine"]
pub struct STATE_R(crate::FieldReader<u8, u8>);
impl STATE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        STATE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for STATE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MASTER_ENABLE` reader - Enable status of the controller"]
pub struct MASTER_ENABLE_R(crate::FieldReader<bool, bool>);
impl MASTER_ENABLE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MASTER_ENABLE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MASTER_ENABLE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 28:31 - Test Status Logic Included"]
    #[inline(always)]
    pub fn test_status(&self) -> TEST_STATUS_R {
        TEST_STATUS_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
    #[doc = "Bits 16:20 - Number of Available Channels Minus 1"]
    #[inline(always)]
    pub fn chnls_minus1(&self) -> CHNLS_MINUS1_R {
        CHNLS_MINUS1_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 4:7 - Current State of the control state machine"]
    #[inline(always)]
    pub fn state(&self) -> STATE_R {
        STATE_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 0 - Enable status of the controller"]
    #[inline(always)]
    pub fn master_enable(&self) -> MASTER_ENABLE_R {
        MASTER_ENABLE_R::new((self.bits & 0x01) != 0)
    }
}
#[doc = "DMA Status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [status](index.html) module"]
pub struct STATUS_SPEC;
impl crate::RegisterSpec for STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [status::R](R) reader structure"]
impl crate::Readable for STATUS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets STATUS to value 0"]
impl crate::Resettable for STATUS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
