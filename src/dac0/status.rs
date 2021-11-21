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
#[doc = "Field `DAC_BUSY` reader - Indicates a DAC data acquisition is in process"]
pub struct DAC_BUSY_R(crate::FieldReader<bool, bool>);
impl DAC_BUSY_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DAC_BUSY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DAC_BUSY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FIFO_ENTRY_CNT` reader - Indicates the number of entries in the FIFO"]
pub struct FIFO_ENTRY_CNT_R(crate::FieldReader<u8, u8>);
impl FIFO_ENTRY_CNT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        FIFO_ENTRY_CNT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FIFO_ENTRY_CNT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 7 - Indicates a DAC data acquisition is in process"]
    #[inline(always)]
    pub fn dac_busy(&self) -> DAC_BUSY_R {
        DAC_BUSY_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 0:5 - Indicates the number of entries in the FIFO"]
    #[inline(always)]
    pub fn fifo_entry_cnt(&self) -> FIFO_ENTRY_CNT_R {
        FIFO_ENTRY_CNT_R::new((self.bits & 0x3f) as u8)
    }
}
#[doc = "Status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [status](index.html) module"]
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
