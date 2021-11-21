#[doc = "Register `RST_CNTL_RAM1` reader"]
pub struct R(crate::R<RST_CNTL_RAM1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RST_CNTL_RAM1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RST_CNTL_RAM1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RST_CNTL_RAM1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RST_CNTL_RAM1` writer"]
pub struct W(crate::W<RST_CNTL_RAM1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RST_CNTL_RAM1_SPEC>;
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
impl From<crate::W<RST_CNTL_RAM1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RST_CNTL_RAM1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `POR` reader - Power On Reset Status"]
pub struct POR_R(crate::FieldReader<bool, bool>);
impl POR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        POR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for POR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `POR` writer - Power On Reset Status"]
pub struct POR_W<'a> {
    w: &'a mut W,
}
impl<'a> POR_W<'a> {
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
#[doc = "Field `EXTRST` reader - External Reset Status"]
pub struct EXTRST_R(crate::FieldReader<bool, bool>);
impl EXTRST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EXTRST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EXTRST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EXTRST` writer - External Reset Status"]
pub struct EXTRST_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTRST_W<'a> {
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
#[doc = "Field `SYSRSTREQ` reader - SYSRESETREQ Reset Status"]
pub struct SYSRSTREQ_R(crate::FieldReader<bool, bool>);
impl SYSRSTREQ_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SYSRSTREQ_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SYSRSTREQ_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SYSRSTREQ` writer - SYSRESETREQ Reset Status"]
pub struct SYSRSTREQ_W<'a> {
    w: &'a mut W,
}
impl<'a> SYSRSTREQ_W<'a> {
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
#[doc = "Field `LOOKUP` reader - LOOKUP Reset Status"]
pub struct LOOKUP_R(crate::FieldReader<bool, bool>);
impl LOOKUP_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LOOKUP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LOOKUP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LOOKUP` writer - LOOKUP Reset Status"]
pub struct LOOKUP_W<'a> {
    w: &'a mut W,
}
impl<'a> LOOKUP_W<'a> {
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
#[doc = "Field `WATCHDOG` reader - WATCHDOG Reset Status"]
pub struct WATCHDOG_R(crate::FieldReader<bool, bool>);
impl WATCHDOG_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        WATCHDOG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WATCHDOG_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WATCHDOG` writer - WATCHDOG Reset Status"]
pub struct WATCHDOG_W<'a> {
    w: &'a mut W,
}
impl<'a> WATCHDOG_W<'a> {
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
#[doc = "Field `MEMERR` reader - Memory Error Reset Status"]
pub struct MEMERR_R(crate::FieldReader<bool, bool>);
impl MEMERR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MEMERR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MEMERR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - Power On Reset Status"]
    #[inline(always)]
    pub fn por(&self) -> POR_R {
        POR_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - External Reset Status"]
    #[inline(always)]
    pub fn extrst(&self) -> EXTRST_R {
        EXTRST_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - SYSRESETREQ Reset Status"]
    #[inline(always)]
    pub fn sysrstreq(&self) -> SYSRSTREQ_R {
        SYSRSTREQ_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - LOOKUP Reset Status"]
    #[inline(always)]
    pub fn lookup(&self) -> LOOKUP_R {
        LOOKUP_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - WATCHDOG Reset Status"]
    #[inline(always)]
    pub fn watchdog(&self) -> WATCHDOG_R {
        WATCHDOG_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Memory Error Reset Status"]
    #[inline(always)]
    pub fn memerr(&self) -> MEMERR_R {
        MEMERR_R::new(((self.bits >> 5) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Power On Reset Status"]
    #[inline(always)]
    pub fn por(&mut self) -> POR_W {
        POR_W { w: self }
    }
    #[doc = "Bit 1 - External Reset Status"]
    #[inline(always)]
    pub fn extrst(&mut self) -> EXTRST_W {
        EXTRST_W { w: self }
    }
    #[doc = "Bit 2 - SYSRESETREQ Reset Status"]
    #[inline(always)]
    pub fn sysrstreq(&mut self) -> SYSRSTREQ_W {
        SYSRSTREQ_W { w: self }
    }
    #[doc = "Bit 3 - LOOKUP Reset Status"]
    #[inline(always)]
    pub fn lookup(&mut self) -> LOOKUP_W {
        LOOKUP_W { w: self }
    }
    #[doc = "Bit 4 - WATCHDOG Reset Status"]
    #[inline(always)]
    pub fn watchdog(&mut self) -> WATCHDOG_W {
        WATCHDOG_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RAM Reset Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rst_cntl_ram1](index.html) module"]
pub struct RST_CNTL_RAM1_SPEC;
impl crate::RegisterSpec for RST_CNTL_RAM1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rst_cntl_ram1::R](R) reader structure"]
impl crate::Readable for RST_CNTL_RAM1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rst_cntl_ram1::W](W) writer structure"]
impl crate::Writable for RST_CNTL_RAM1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RST_CNTL_RAM1 to value 0x3f"]
impl crate::Resettable for RST_CNTL_RAM1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x3f
    }
}
