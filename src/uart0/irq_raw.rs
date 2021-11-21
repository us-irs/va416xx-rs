#[doc = "Register `IRQ_RAW` reader"]
pub struct R(crate::R<IRQ_RAW_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IRQ_RAW_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IRQ_RAW_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IRQ_RAW_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `IRQ_RX` reader - RX Interrupt"]
pub struct IRQ_RX_R(crate::FieldReader<bool, bool>);
impl IRQ_RX_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        IRQ_RX_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IRQ_RX_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IRQ_RX_STATUS` reader - RX Status Interrupt"]
pub struct IRQ_RX_STATUS_R(crate::FieldReader<bool, bool>);
impl IRQ_RX_STATUS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        IRQ_RX_STATUS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IRQ_RX_STATUS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IRQ_RX_TO` reader - RX Timeout Interrupt"]
pub struct IRQ_RX_TO_R(crate::FieldReader<bool, bool>);
impl IRQ_RX_TO_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        IRQ_RX_TO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IRQ_RX_TO_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IRQ_TX` reader - TX Interrupt"]
pub struct IRQ_TX_R(crate::FieldReader<bool, bool>);
impl IRQ_TX_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        IRQ_TX_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IRQ_TX_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IRQ_TX_STATUS` reader - TX Status Interrupt"]
pub struct IRQ_TX_STATUS_R(crate::FieldReader<bool, bool>);
impl IRQ_TX_STATUS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        IRQ_TX_STATUS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IRQ_TX_STATUS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IRQ_TX_EMPTY` reader - TX Empty Interrupt"]
pub struct IRQ_TX_EMPTY_R(crate::FieldReader<bool, bool>);
impl IRQ_TX_EMPTY_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        IRQ_TX_EMPTY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IRQ_TX_EMPTY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IRQ_TX_CTS` reader - TX CTS Change Interrupt"]
pub struct IRQ_TX_CTS_R(crate::FieldReader<bool, bool>);
impl IRQ_TX_CTS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        IRQ_TX_CTS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IRQ_TX_CTS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - RX Interrupt"]
    #[inline(always)]
    pub fn irq_rx(&self) -> IRQ_RX_R {
        IRQ_RX_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - RX Status Interrupt"]
    #[inline(always)]
    pub fn irq_rx_status(&self) -> IRQ_RX_STATUS_R {
        IRQ_RX_STATUS_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - RX Timeout Interrupt"]
    #[inline(always)]
    pub fn irq_rx_to(&self) -> IRQ_RX_TO_R {
        IRQ_RX_TO_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 4 - TX Interrupt"]
    #[inline(always)]
    pub fn irq_tx(&self) -> IRQ_TX_R {
        IRQ_TX_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - TX Status Interrupt"]
    #[inline(always)]
    pub fn irq_tx_status(&self) -> IRQ_TX_STATUS_R {
        IRQ_TX_STATUS_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - TX Empty Interrupt"]
    #[inline(always)]
    pub fn irq_tx_empty(&self) -> IRQ_TX_EMPTY_R {
        IRQ_TX_EMPTY_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - TX CTS Change Interrupt"]
    #[inline(always)]
    pub fn irq_tx_cts(&self) -> IRQ_TX_CTS_R {
        IRQ_TX_CTS_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
#[doc = "IRQ Raw Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [irq_raw](index.html) module"]
pub struct IRQ_RAW_SPEC;
impl crate::RegisterSpec for IRQ_RAW_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [irq_raw::R](R) reader structure"]
impl crate::Readable for IRQ_RAW_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets IRQ_RAW to value 0"]
impl crate::Resettable for IRQ_RAW_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
