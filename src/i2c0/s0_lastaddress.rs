#[doc = "Register `S0_LASTADDRESS` reader"]
pub struct R(crate::R<S0_LASTADDRESS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<S0_LASTADDRESS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<S0_LASTADDRESS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<S0_LASTADDRESS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `ADDRESS` reader - Address value"]
pub struct ADDRESS_R(crate::FieldReader<u16, u16>);
impl ADDRESS_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        ADDRESS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADDRESS_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DIRECTION` reader - Transaction direction 0=master send, 1=master receive"]
pub struct DIRECTION_R(crate::FieldReader<bool, bool>);
impl DIRECTION_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DIRECTION_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DIRECTION_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 1:10 - Address value"]
    #[inline(always)]
    pub fn address(&self) -> ADDRESS_R {
        ADDRESS_R::new(((self.bits >> 1) & 0x03ff) as u16)
    }
    #[doc = "Bit 0 - Transaction direction 0=master send, 1=master receive"]
    #[inline(always)]
    pub fn direction(&self) -> DIRECTION_R {
        DIRECTION_R::new((self.bits & 0x01) != 0)
    }
}
#[doc = "Slave I2C Last Address value\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [s0_lastaddress](index.html) module"]
pub struct S0_LASTADDRESS_SPEC;
impl crate::RegisterSpec for S0_LASTADDRESS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [s0_lastaddress::R](R) reader structure"]
impl crate::Readable for S0_LASTADDRESS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets S0_LASTADDRESS to value 0"]
impl crate::Resettable for S0_LASTADDRESS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
