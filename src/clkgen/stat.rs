#[doc = "Register `STAT` reader"]
pub struct R(crate::R<STAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STAT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STAT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `SYSCLKLOST` reader - Set when SYS_CLK has dropped to less than 1MHz"]
pub struct SYSCLKLOST_R(crate::FieldReader<bool, bool>);
impl SYSCLKLOST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SYSCLKLOST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SYSCLKLOST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LOCKLOST` reader - LOCK high Symbol indicates that RFLSIP or FBSLIP have occurred for 64 consecutive cycles"]
pub struct LOCKLOST_R(crate::FieldReader<bool, bool>);
impl LOCKLOST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LOCKLOST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LOCKLOST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RFSLIP` reader - Reference cycle slip output (CLKOUT frequency high)"]
pub struct RFSLIP_R(crate::FieldReader<bool, bool>);
impl RFSLIP_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RFSLIP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RFSLIP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FBSLIP` reader - Feedback cycle slip output (CLKOUT frequency low)"]
pub struct FBSLIP_R(crate::FieldReader<bool, bool>);
impl FBSLIP_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FBSLIP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FBSLIP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 3 - Set when SYS_CLK has dropped to less than 1MHz"]
    #[inline(always)]
    pub fn sysclklost(&self) -> SYSCLKLOST_R {
        SYSCLKLOST_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - LOCK high Symbol indicates that RFLSIP or FBSLIP have occurred for 64 consecutive cycles"]
    #[inline(always)]
    pub fn locklost(&self) -> LOCKLOST_R {
        LOCKLOST_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - Reference cycle slip output (CLKOUT frequency high)"]
    #[inline(always)]
    pub fn rfslip(&self) -> RFSLIP_R {
        RFSLIP_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Feedback cycle slip output (CLKOUT frequency low)"]
    #[inline(always)]
    pub fn fbslip(&self) -> FBSLIP_R {
        FBSLIP_R::new((self.bits & 0x01) != 0)
    }
}
#[doc = "Clock Generation Module Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [stat](index.html) module"]
pub struct STAT_SPEC;
impl crate::RegisterSpec for STAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [stat::R](R) reader structure"]
impl crate::Readable for STAT_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets STAT to value 0"]
impl crate::Resettable for STAT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
