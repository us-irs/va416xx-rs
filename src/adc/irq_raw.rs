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
#[doc = "Field `FIFO_DEPTH_TRIG` reader - Indicates the interrupt for the FIFO entry count meets or exceeds the trigger level"]
pub struct FIFO_DEPTH_TRIG_R(crate::FieldReader<bool, bool>);
impl FIFO_DEPTH_TRIG_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FIFO_DEPTH_TRIG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FIFO_DEPTH_TRIG_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TRIG_ERROR` reader - Indicates a manual or external trigger occurred when the ADC was BUSY doing a conversion"]
pub struct TRIG_ERROR_R(crate::FieldReader<bool, bool>);
impl TRIG_ERROR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TRIG_ERROR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TRIG_ERROR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADC_DONE` reader - Indicates that a ADC conversion is done"]
pub struct ADC_DONE_R(crate::FieldReader<bool, bool>);
impl ADC_DONE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ADC_DONE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADC_DONE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FIFO_UFLOW` reader - Indicates data was unavailable when a new trigger for ADC update is received"]
pub struct FIFO_UFLOW_R(crate::FieldReader<bool, bool>);
impl FIFO_UFLOW_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FIFO_UFLOW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FIFO_UFLOW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FIFO_OFLOW` reader - Indicates a FIFO overflow occurred (FIFO was full when new data was written)"]
pub struct FIFO_OFLOW_R(crate::FieldReader<bool, bool>);
impl FIFO_OFLOW_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FIFO_OFLOW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FIFO_OFLOW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FIFO_FULL` reader - Indicates the FIFO is full"]
pub struct FIFO_FULL_R(crate::FieldReader<bool, bool>);
impl FIFO_FULL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FIFO_FULL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FIFO_FULL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FIFO_EMPTY` reader - Indicates the FIFO is empty"]
pub struct FIFO_EMPTY_R(crate::FieldReader<bool, bool>);
impl FIFO_EMPTY_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FIFO_EMPTY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FIFO_EMPTY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 6 - Indicates the interrupt for the FIFO entry count meets or exceeds the trigger level"]
    #[inline(always)]
    pub fn fifo_depth_trig(&self) -> FIFO_DEPTH_TRIG_R {
        FIFO_DEPTH_TRIG_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Indicates a manual or external trigger occurred when the ADC was BUSY doing a conversion"]
    #[inline(always)]
    pub fn trig_error(&self) -> TRIG_ERROR_R {
        TRIG_ERROR_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Indicates that a ADC conversion is done"]
    #[inline(always)]
    pub fn adc_done(&self) -> ADC_DONE_R {
        ADC_DONE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Indicates data was unavailable when a new trigger for ADC update is received"]
    #[inline(always)]
    pub fn fifo_uflow(&self) -> FIFO_UFLOW_R {
        FIFO_UFLOW_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Indicates a FIFO overflow occurred (FIFO was full when new data was written)"]
    #[inline(always)]
    pub fn fifo_oflow(&self) -> FIFO_OFLOW_R {
        FIFO_OFLOW_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - Indicates the FIFO is full"]
    #[inline(always)]
    pub fn fifo_full(&self) -> FIFO_FULL_R {
        FIFO_FULL_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Indicates the FIFO is empty"]
    #[inline(always)]
    pub fn fifo_empty(&self) -> FIFO_EMPTY_R {
        FIFO_EMPTY_R::new((self.bits & 0x01) != 0)
    }
}
#[doc = "Raw Interrupt Status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [irq_raw](index.html) module"]
pub struct IRQ_RAW_SPEC;
impl crate::RegisterSpec for IRQ_RAW_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [irq_raw::R](R) reader structure"]
impl crate::Readable for IRQ_RAW_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets IRQ_RAW to value 0x01"]
impl crate::Resettable for IRQ_RAW_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x01
    }
}
