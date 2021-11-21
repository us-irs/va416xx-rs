#[doc = "Register `S0_FIFO_CLR` writer"]
pub struct W(crate::W<S0_FIFO_CLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<S0_FIFO_CLR_SPEC>;
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
impl From<crate::W<S0_FIFO_CLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<S0_FIFO_CLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RXFIFO` writer - Clear Rx FIFO"]
pub struct RXFIFO_W<'a> {
    w: &'a mut W,
}
impl<'a> RXFIFO_W<'a> {
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
#[doc = "Field `TXFIFO` writer - Clear Tx FIFO"]
pub struct TXFIFO_W<'a> {
    w: &'a mut W,
}
impl<'a> TXFIFO_W<'a> {
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
impl W {
    #[doc = "Bit 0 - Clear Rx FIFO"]
    #[inline(always)]
    pub fn rxfifo(&mut self) -> RXFIFO_W {
        RXFIFO_W { w: self }
    }
    #[doc = "Bit 1 - Clear Tx FIFO"]
    #[inline(always)]
    pub fn txfifo(&mut self) -> TXFIFO_W {
        TXFIFO_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Slave Clear FIFO Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [s0_fifo_clr](index.html) module"]
pub struct S0_FIFO_CLR_SPEC;
impl crate::RegisterSpec for S0_FIFO_CLR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [s0_fifo_clr::W](W) writer structure"]
impl crate::Writable for S0_FIFO_CLR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets S0_FIFO_CLR to value 0"]
impl crate::Resettable for S0_FIFO_CLR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
