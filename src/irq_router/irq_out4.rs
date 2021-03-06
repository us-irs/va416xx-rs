#[doc = "Register `IRQ_OUT4` reader"]
pub struct R(crate::R<IRQ_OUT4_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IRQ_OUT4_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IRQ_OUT4_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IRQ_OUT4_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `IRQ_OUT4` reader - IRQ_OUT\\[159:128\\]"]
pub struct IRQ_OUT4_R(crate::FieldReader<u32, u32>);
impl IRQ_OUT4_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        IRQ_OUT4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IRQ_OUT4_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:31 - IRQ_OUT\\[159:128\\]"]
    #[inline(always)]
    pub fn irq_out4(&self) -> IRQ_OUT4_R {
        IRQ_OUT4_R::new(self.bits as u32)
    }
}
#[doc = "DEBUG IRQ_OUT\\[159:128\\]\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [irq_out4](index.html) module"]
pub struct IRQ_OUT4_SPEC;
impl crate::RegisterSpec for IRQ_OUT4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [irq_out4::R](R) reader structure"]
impl crate::Readable for IRQ_OUT4_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets IRQ_OUT4 to value 0"]
impl crate::Resettable for IRQ_OUT4_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
