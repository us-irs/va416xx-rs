#[doc = "Register `TXSTATUS` reader"]
pub struct R(crate::R<TXSTATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TXSTATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TXSTATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TXSTATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `WRRDY` reader - Write Fifo NOT Full"]
pub struct WRRDY_R(crate::FieldReader<bool, bool>);
impl WRRDY_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        WRRDY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WRRDY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WRBUSY` reader - Write Fifo Full"]
pub struct WRBUSY_R(crate::FieldReader<bool, bool>);
impl WRBUSY_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        WRBUSY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WRBUSY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXBUSY` reader - TX Busy Transmitting"]
pub struct TXBUSY_R(crate::FieldReader<bool, bool>);
impl TXBUSY_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TXBUSY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXBUSY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WRLOST` reader - Write Data Lost (Fifo Overflow)"]
pub struct WRLOST_R(crate::FieldReader<bool, bool>);
impl WRLOST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        WRLOST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WRLOST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXCTSN` reader - TX CTSn Input Value"]
pub struct TXCTSN_R(crate::FieldReader<bool, bool>);
impl TXCTSN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TXCTSN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXCTSN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - Write Fifo NOT Full"]
    #[inline(always)]
    pub fn wrrdy(&self) -> WRRDY_R {
        WRRDY_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Write Fifo Full"]
    #[inline(always)]
    pub fn wrbusy(&self) -> WRBUSY_R {
        WRBUSY_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - TX Busy Transmitting"]
    #[inline(always)]
    pub fn txbusy(&self) -> TXBUSY_R {
        TXBUSY_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Write Data Lost (Fifo Overflow)"]
    #[inline(always)]
    pub fn wrlost(&self) -> WRLOST_R {
        WRLOST_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 15 - TX CTSn Input Value"]
    #[inline(always)]
    pub fn txctsn(&self) -> TXCTSN_R {
        TXCTSN_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
#[doc = "Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [txstatus](index.html) module"]
pub struct TXSTATUS_SPEC;
impl crate::RegisterSpec for TXSTATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [txstatus::R](R) reader structure"]
impl crate::Readable for TXSTATUS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets TXSTATUS to value 0"]
impl crate::Resettable for TXSTATUS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
