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
#[doc = "Field `TFE` reader - Transmit FIFO empty"]
pub struct TFE_R(crate::FieldReader<bool, bool>);
impl TFE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TFE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TFE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TNF` reader - Transmit FIFO not full"]
pub struct TNF_R(crate::FieldReader<bool, bool>);
impl TNF_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TNF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TNF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RNE` reader - Receive FIFO not empty"]
pub struct RNE_R(crate::FieldReader<bool, bool>);
impl RNE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RNE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RNE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RFF` reader - Receive FIFO Full"]
pub struct RFF_R(crate::FieldReader<bool, bool>);
impl RFF_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RFF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RFF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BUSY` reader - Busy"]
pub struct BUSY_R(crate::FieldReader<bool, bool>);
impl BUSY_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        BUSY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BUSY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXDATAFIRST` reader - Pending Data is first Byte in BLOCKMODE"]
pub struct RXDATAFIRST_R(crate::FieldReader<bool, bool>);
impl RXDATAFIRST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RXDATAFIRST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXDATAFIRST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXTRIGGER` reader - RX FIFO Above Trigger Level"]
pub struct RXTRIGGER_R(crate::FieldReader<bool, bool>);
impl RXTRIGGER_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RXTRIGGER_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXTRIGGER_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXTRIGGER` reader - TX FIFO Below Trigger Level"]
pub struct TXTRIGGER_R(crate::FieldReader<bool, bool>);
impl TXTRIGGER_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TXTRIGGER_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXTRIGGER_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - Transmit FIFO empty"]
    #[inline(always)]
    pub fn tfe(&self) -> TFE_R {
        TFE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Transmit FIFO not full"]
    #[inline(always)]
    pub fn tnf(&self) -> TNF_R {
        TNF_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Receive FIFO not empty"]
    #[inline(always)]
    pub fn rne(&self) -> RNE_R {
        RNE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Receive FIFO Full"]
    #[inline(always)]
    pub fn rff(&self) -> RFF_R {
        RFF_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Busy"]
    #[inline(always)]
    pub fn busy(&self) -> BUSY_R {
        BUSY_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Pending Data is first Byte in BLOCKMODE"]
    #[inline(always)]
    pub fn rxdatafirst(&self) -> RXDATAFIRST_R {
        RXDATAFIRST_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - RX FIFO Above Trigger Level"]
    #[inline(always)]
    pub fn rxtrigger(&self) -> RXTRIGGER_R {
        RXTRIGGER_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - TX FIFO Below Trigger Level"]
    #[inline(always)]
    pub fn txtrigger(&self) -> TXTRIGGER_R {
        TXTRIGGER_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
#[doc = "Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [status](index.html) module"]
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
