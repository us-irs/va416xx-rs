#[doc = "Register `S0_TXCOUNT` reader"]
pub struct R(crate::R<S0_TXCOUNT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<S0_TXCOUNT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<S0_TXCOUNT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<S0_TXCOUNT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `VALUE` reader - Count value"]
pub struct VALUE_R(crate::FieldReader<u16, u16>);
impl VALUE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        VALUE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VALUE_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:10 - Count value"]
    #[inline(always)]
    pub fn value(&self) -> VALUE_R {
        VALUE_R::new((self.bits & 0x07ff) as u16)
    }
}
#[doc = "Slave TX Count Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [s0_txcount](index.html) module"]
pub struct S0_TXCOUNT_SPEC;
impl crate::RegisterSpec for S0_TXCOUNT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [s0_txcount::R](R) reader structure"]
impl crate::Readable for S0_TXCOUNT_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets S0_TXCOUNT to value 0"]
impl crate::Resettable for S0_TXCOUNT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
