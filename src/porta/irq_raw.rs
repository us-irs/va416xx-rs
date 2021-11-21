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
#[doc = "Raw Interrupt Status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [irq_raw](index.html) module"]
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
