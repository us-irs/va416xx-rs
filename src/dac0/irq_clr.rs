#[doc = "Register `IRQ_CLR` writer"]
pub struct W(crate::W<IRQ_CLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IRQ_CLR_SPEC>;
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
impl From<crate::W<IRQ_CLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IRQ_CLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TRIG_ERROR` writer - Clears the trigger error interrupt status. Always reads 0"]
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u32 & 0x01) << 3);
        self.w
    }
}
#[doc = "Field `DAC_DONE` writer - Clears the DAC done interrupt status. Always reads 0"]
pub struct DAC_DONE_W<'a> {
    w: &'a mut W,
}
impl<'a> DAC_DONE_W<'a> {
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
#[doc = "Field `FIFO_UFLOW` writer - Clears the FIFO underflow interrupt status. Always reads 0"]
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
#[doc = "Field `FIFO_OFLOW` writer - Clears the FIFO overflow interrupt status. Always reads 0"]
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
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
impl W {
    #[doc = "Bit 3 - Clears the trigger error interrupt status. Always reads 0"]
    #[inline(always)]
    pub fn trig_error(&mut self) -> TRIG_ERROR_W {
        TRIG_ERROR_W { w: self }
    }
    #[doc = "Bit 2 - Clears the DAC done interrupt status. Always reads 0"]
    #[inline(always)]
    pub fn dac_done(&mut self) -> DAC_DONE_W {
        DAC_DONE_W { w: self }
    }
    #[doc = "Bit 1 - Clears the FIFO underflow interrupt status. Always reads 0"]
    #[inline(always)]
    pub fn fifo_uflow(&mut self) -> FIFO_UFLOW_W {
        FIFO_UFLOW_W { w: self }
    }
    #[doc = "Bit 0 - Clears the FIFO overflow interrupt status. Always reads 0"]
    #[inline(always)]
    pub fn fifo_oflow(&mut self) -> FIFO_OFLOW_W {
        FIFO_OFLOW_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Clear Interrupt\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [irq_clr](index.html) module"]
pub struct IRQ_CLR_SPEC;
impl crate::RegisterSpec for IRQ_CLR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [irq_clr::W](W) writer structure"]
impl crate::Writable for IRQ_CLR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets IRQ_CLR to value 0"]
impl crate::Resettable for IRQ_CLR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
