#[doc = "Register `IRQ_END` reader"]
pub struct R(crate::R<IRQ_END_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IRQ_END_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IRQ_END_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IRQ_END_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RORIM` reader - RX Overrun"]
pub struct RORIM_R(crate::FieldReader<bool, bool>);
impl RORIM_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RORIM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RORIM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RTIM` reader - RX Timeout"]
pub struct RTIM_R(crate::FieldReader<bool, bool>);
impl RTIM_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RTIM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RTIM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXIM` reader - RX Fifo is at least half full"]
pub struct RXIM_R(crate::FieldReader<bool, bool>);
impl RXIM_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RXIM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXIM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXIM` reader - TX Fifo is at least half empty"]
pub struct TXIM_R(crate::FieldReader<bool, bool>);
impl TXIM_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TXIM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXIM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - RX Overrun"]
    #[inline(always)]
    pub fn rorim(&self) -> RORIM_R {
        RORIM_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - RX Timeout"]
    #[inline(always)]
    pub fn rtim(&self) -> RTIM_R {
        RTIM_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - RX Fifo is at least half full"]
    #[inline(always)]
    pub fn rxim(&self) -> RXIM_R {
        RXIM_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - TX Fifo is at least half empty"]
    #[inline(always)]
    pub fn txim(&self) -> TXIM_R {
        TXIM_R::new(((self.bits >> 3) & 0x01) != 0)
    }
}
#[doc = "Enabled Interrupt Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [irq_end](index.html) module"]
pub struct IRQ_END_SPEC;
impl crate::RegisterSpec for IRQ_END_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [irq_end::R](R) reader structure"]
impl crate::Readable for IRQ_END_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets IRQ_END to value 0"]
impl crate::Resettable for IRQ_END_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
