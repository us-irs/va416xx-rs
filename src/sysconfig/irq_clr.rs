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
#[doc = "Field `ROMMBE` writer - ROM Multi Bit Interrupt"]
pub struct ROMMBE_W<'a> {
    w: &'a mut W,
}
impl<'a> ROMMBE_W<'a> {
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
#[doc = "Field `ROMSBE` writer - ROM Single Bit Interrupt"]
pub struct ROMSBE_W<'a> {
    w: &'a mut W,
}
impl<'a> ROMSBE_W<'a> {
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
#[doc = "Field `RAM0MBE` writer - RAM0 Multi Bit Interrupt"]
pub struct RAM0MBE_W<'a> {
    w: &'a mut W,
}
impl<'a> RAM0MBE_W<'a> {
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
#[doc = "Field `RAM0SBE` writer - RAM0 Single Bit Interrupt"]
pub struct RAM0SBE_W<'a> {
    w: &'a mut W,
}
impl<'a> RAM0SBE_W<'a> {
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
#[doc = "Field `RAM1MBE` writer - RAM1 Multi Bit Interrupt"]
pub struct RAM1MBE_W<'a> {
    w: &'a mut W,
}
impl<'a> RAM1MBE_W<'a> {
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
#[doc = "Field `RAM1SBE` writer - RAM1 Single Bit Interrupt"]
pub struct RAM1SBE_W<'a> {
    w: &'a mut W,
}
impl<'a> RAM1SBE_W<'a> {
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
impl W {
    #[doc = "Bit 0 - ROM Multi Bit Interrupt"]
    #[inline(always)]
    pub fn rommbe(&mut self) -> ROMMBE_W {
        ROMMBE_W { w: self }
    }
    #[doc = "Bit 1 - ROM Single Bit Interrupt"]
    #[inline(always)]
    pub fn romsbe(&mut self) -> ROMSBE_W {
        ROMSBE_W { w: self }
    }
    #[doc = "Bit 2 - RAM0 Multi Bit Interrupt"]
    #[inline(always)]
    pub fn ram0mbe(&mut self) -> RAM0MBE_W {
        RAM0MBE_W { w: self }
    }
    #[doc = "Bit 3 - RAM0 Single Bit Interrupt"]
    #[inline(always)]
    pub fn ram0sbe(&mut self) -> RAM0SBE_W {
        RAM0SBE_W { w: self }
    }
    #[doc = "Bit 4 - RAM1 Multi Bit Interrupt"]
    #[inline(always)]
    pub fn ram1mbe(&mut self) -> RAM1MBE_W {
        RAM1MBE_W { w: self }
    }
    #[doc = "Bit 5 - RAM1 Single Bit Interrupt"]
    #[inline(always)]
    pub fn ram1sbe(&mut self) -> RAM1SBE_W {
        RAM1SBE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Clear EDAC Error Interrupt Status\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [irq_clr](index.html) module"]
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
