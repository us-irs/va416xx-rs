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
#[doc = "Field `ROMMBE` reader - ROM Multi Bit Interrupt"]
pub struct ROMMBE_R(crate::FieldReader<bool, bool>);
impl ROMMBE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ROMMBE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ROMMBE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
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
#[doc = "Field `ROMSBE` reader - ROM Single Bit Interrupt"]
pub struct ROMSBE_R(crate::FieldReader<bool, bool>);
impl ROMSBE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ROMSBE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ROMSBE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
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
#[doc = "Field `RAM0MBE` reader - RAM0 Multi Bit Interrupt"]
pub struct RAM0MBE_R(crate::FieldReader<bool, bool>);
impl RAM0MBE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RAM0MBE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RAM0MBE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
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
#[doc = "Field `RAM0SBE` reader - RAM0 Single Bit Interrupt"]
pub struct RAM0SBE_R(crate::FieldReader<bool, bool>);
impl RAM0SBE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RAM0SBE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RAM0SBE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
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
#[doc = "Field `RAM1MBE` reader - RAM1 Multi Bit Interrupt"]
pub struct RAM1MBE_R(crate::FieldReader<bool, bool>);
impl RAM1MBE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RAM1MBE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RAM1MBE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
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
#[doc = "Field `RAM1SBE` reader - RAM1 Single Bit Interrupt"]
pub struct RAM1SBE_R(crate::FieldReader<bool, bool>);
impl RAM1SBE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RAM1SBE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RAM1SBE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
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
impl R {
    #[doc = "Bit 0 - ROM Multi Bit Interrupt"]
    #[inline(always)]
    pub fn rommbe(&self) -> ROMMBE_R {
        ROMMBE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - ROM Single Bit Interrupt"]
    #[inline(always)]
    pub fn romsbe(&self) -> ROMSBE_R {
        ROMSBE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - RAM0 Multi Bit Interrupt"]
    #[inline(always)]
    pub fn ram0mbe(&self) -> RAM0MBE_R {
        RAM0MBE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - RAM0 Single Bit Interrupt"]
    #[inline(always)]
    pub fn ram0sbe(&self) -> RAM0SBE_R {
        RAM0SBE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - RAM1 Multi Bit Interrupt"]
    #[inline(always)]
    pub fn ram1mbe(&self) -> RAM1MBE_R {
        RAM1MBE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - RAM1 Single Bit Interrupt"]
    #[inline(always)]
    pub fn ram1sbe(&self) -> RAM1SBE_R {
        RAM1SBE_R::new(((self.bits >> 5) & 0x01) != 0)
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
#[doc = "Enable EDAC Error Interrupt Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [irq_enb](index.html) module"]
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
