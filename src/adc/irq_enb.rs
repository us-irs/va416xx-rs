#[doc = "Register `IRQ_ENB` reader"]
pub struct R(crate::R<IRQ_ENB_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IRQ_ENB_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IRQ_ENB_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IRQ_ENB_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IRQ_ENB` writer"]
pub struct W(crate::W<IRQ_ENB_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IRQ_ENB_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<IRQ_ENB_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IRQ_ENB_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FIFO_DEPTH_TRIG` reader - Enables the interrupt for the FIFO entry count meets or exceeds the trigger level"]
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
#[doc = "Field `FIFO_DEPTH_TRIG` writer - Enables the interrupt for the FIFO entry count meets or exceeds the trigger level"]
pub struct FIFO_DEPTH_TRIG_W<'a> {
    w: &'a mut W,
}
impl<'a> FIFO_DEPTH_TRIG_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 6)) | ((value as u32 & 0x01) << 6);
        self.w
    }
}
#[doc = "Field `TRIG_ERROR` reader - Enables the interrupt for a trigger error"]
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
#[doc = "Field `TRIG_ERROR` writer - Enables the interrupt for a trigger error"]
pub struct TRIG_ERROR_W<'a> {
    w: &'a mut W,
}
impl<'a> TRIG_ERROR_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u32 & 0x01) << 5);
        self.w
    }
}
#[doc = "Field `ADC_DONE` reader - Enables the interrupt for an ADC data acquisition completion"]
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
#[doc = "Field `ADC_DONE` writer - Enables the interrupt for an ADC data acquisition completion"]
pub struct ADC_DONE_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC_DONE_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u32 & 0x01) << 4);
        self.w
    }
}
#[doc = "Field `FIFO_UFLOW` reader - Enables the interrupt for a FIFO underflow"]
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
#[doc = "Field `FIFO_UFLOW` writer - Enables the interrupt for a FIFO underflow"]
pub struct FIFO_UFLOW_W<'a> {
    w: &'a mut W,
}
impl<'a> FIFO_UFLOW_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u32 & 0x01) << 3);
        self.w
    }
}
#[doc = "Field `FIFO_OFLOW` reader - Enables the interrupt for a FIFO overflow"]
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
#[doc = "Field `FIFO_OFLOW` writer - Enables the interrupt for a FIFO overflow"]
pub struct FIFO_OFLOW_W<'a> {
    w: &'a mut W,
}
impl<'a> FIFO_OFLOW_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
        self.w
    }
}
#[doc = "Field `FIFO_FULL` reader - Enables the interrupt for FIFO full"]
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
#[doc = "Field `FIFO_FULL` writer - Enables the interrupt for FIFO full"]
pub struct FIFO_FULL_W<'a> {
    w: &'a mut W,
}
impl<'a> FIFO_FULL_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
#[doc = "Field `FIFO_EMPTY` reader - Enables the interrupt for FIFO empty"]
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
#[doc = "Field `FIFO_EMPTY` writer - Enables the interrupt for FIFO empty"]
pub struct FIFO_EMPTY_W<'a> {
    w: &'a mut W,
}
impl<'a> FIFO_EMPTY_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
impl R {
    #[doc = "Bit 6 - Enables the interrupt for the FIFO entry count meets or exceeds the trigger level"]
    #[inline(always)]
    pub fn fifo_depth_trig(&self) -> FIFO_DEPTH_TRIG_R {
        FIFO_DEPTH_TRIG_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Enables the interrupt for a trigger error"]
    #[inline(always)]
    pub fn trig_error(&self) -> TRIG_ERROR_R {
        TRIG_ERROR_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Enables the interrupt for an ADC data acquisition completion"]
    #[inline(always)]
    pub fn adc_done(&self) -> ADC_DONE_R {
        ADC_DONE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Enables the interrupt for a FIFO underflow"]
    #[inline(always)]
    pub fn fifo_uflow(&self) -> FIFO_UFLOW_R {
        FIFO_UFLOW_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Enables the interrupt for a FIFO overflow"]
    #[inline(always)]
    pub fn fifo_oflow(&self) -> FIFO_OFLOW_R {
        FIFO_OFLOW_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - Enables the interrupt for FIFO full"]
    #[inline(always)]
    pub fn fifo_full(&self) -> FIFO_FULL_R {
        FIFO_FULL_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Enables the interrupt for FIFO empty"]
    #[inline(always)]
    pub fn fifo_empty(&self) -> FIFO_EMPTY_R {
        FIFO_EMPTY_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 6 - Enables the interrupt for the FIFO entry count meets or exceeds the trigger level"]
    #[inline(always)]
    pub fn fifo_depth_trig(&mut self) -> FIFO_DEPTH_TRIG_W {
        FIFO_DEPTH_TRIG_W { w: self }
    }
    #[doc = "Bit 5 - Enables the interrupt for a trigger error"]
    #[inline(always)]
    pub fn trig_error(&mut self) -> TRIG_ERROR_W {
        TRIG_ERROR_W { w: self }
    }
    #[doc = "Bit 4 - Enables the interrupt for an ADC data acquisition completion"]
    #[inline(always)]
    pub fn adc_done(&mut self) -> ADC_DONE_W {
        ADC_DONE_W { w: self }
    }
    #[doc = "Bit 3 - Enables the interrupt for a FIFO underflow"]
    #[inline(always)]
    pub fn fifo_uflow(&mut self) -> FIFO_UFLOW_W {
        FIFO_UFLOW_W { w: self }
    }
    #[doc = "Bit 2 - Enables the interrupt for a FIFO overflow"]
    #[inline(always)]
    pub fn fifo_oflow(&mut self) -> FIFO_OFLOW_W {
        FIFO_OFLOW_W { w: self }
    }
    #[doc = "Bit 1 - Enables the interrupt for FIFO full"]
    #[inline(always)]
    pub fn fifo_full(&mut self) -> FIFO_FULL_W {
        FIFO_FULL_W { w: self }
    }
    #[doc = "Bit 0 - Enables the interrupt for FIFO empty"]
    #[inline(always)]
    pub fn fifo_empty(&mut self) -> FIFO_EMPTY_W {
        FIFO_EMPTY_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Enable\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [irq_enb](index.html) module"]
pub struct IRQ_ENB_SPEC;
impl crate::RegisterSpec for IRQ_ENB_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [irq_enb::R](R) reader structure"]
impl crate::Readable for IRQ_ENB_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [irq_enb::W](W) writer structure"]
impl crate::Writable for IRQ_ENB_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets IRQ_ENB to value 0"]
impl crate::Resettable for IRQ_ENB_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
