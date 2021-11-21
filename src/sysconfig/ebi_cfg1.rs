#[doc = "Register `EBI_CFG1` reader"]
pub struct R(crate::R<EBI_CFG1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EBI_CFG1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EBI_CFG1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EBI_CFG1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EBI_CFG1` writer"]
pub struct W(crate::W<EBI_CFG1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EBI_CFG1_SPEC>;
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
impl From<crate::W<EBI_CFG1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EBI_CFG1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADDRLOW0` reader - Lower bound address for CEN0"]
pub struct ADDRLOW0_R(crate::FieldReader<u8, u8>);
impl ADDRLOW0_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        ADDRLOW0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADDRLOW0_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADDRLOW0` writer - Lower bound address for CEN0"]
pub struct ADDRLOW0_W<'a> {
    w: &'a mut W,
}
impl<'a> ADDRLOW0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
#[doc = "Field `ADDRHIGH0` reader - Upper bound address for CEN0"]
pub struct ADDRHIGH0_R(crate::FieldReader<u8, u8>);
impl ADDRHIGH0_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        ADDRHIGH0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADDRHIGH0_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADDRHIGH0` writer - Upper bound address for CEN0"]
pub struct ADDRHIGH0_W<'a> {
    w: &'a mut W,
}
impl<'a> ADDRHIGH0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | ((value as u32 & 0xff) << 8);
        self.w
    }
}
#[doc = "Field `CFGREADCYCLE` reader - Number of cycles for a read - N plus 1"]
pub struct CFGREADCYCLE_R(crate::FieldReader<u8, u8>);
impl CFGREADCYCLE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CFGREADCYCLE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CFGREADCYCLE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CFGREADCYCLE` writer - Number of cycles for a read - N plus 1"]
pub struct CFGREADCYCLE_W<'a> {
    w: &'a mut W,
}
impl<'a> CFGREADCYCLE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 16)) | ((value as u32 & 0x07) << 16);
        self.w
    }
}
#[doc = "Field `CFGWRITECYCLE` reader - Number of cycles for a write - N plus 1"]
pub struct CFGWRITECYCLE_R(crate::FieldReader<u8, u8>);
impl CFGWRITECYCLE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CFGWRITECYCLE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CFGWRITECYCLE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CFGWRITECYCLE` writer - Number of cycles for a write - N plus 1"]
pub struct CFGWRITECYCLE_W<'a> {
    w: &'a mut W,
}
impl<'a> CFGWRITECYCLE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 19)) | ((value as u32 & 0x07) << 19);
        self.w
    }
}
#[doc = "Field `CFGTURNAROUNDCYCLE` reader - Number of cycles for turnaround - N plus 1"]
pub struct CFGTURNAROUNDCYCLE_R(crate::FieldReader<u8, u8>);
impl CFGTURNAROUNDCYCLE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CFGTURNAROUNDCYCLE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CFGTURNAROUNDCYCLE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CFGTURNAROUNDCYCLE` writer - Number of cycles for turnaround - N plus 1"]
pub struct CFGTURNAROUNDCYCLE_W<'a> {
    w: &'a mut W,
}
impl<'a> CFGTURNAROUNDCYCLE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 22)) | ((value as u32 & 0x07) << 22);
        self.w
    }
}
#[doc = "Field `CFGSIZE` reader - 8 bit (0) or 16 bit (1) port size"]
pub struct CFGSIZE_R(crate::FieldReader<bool, bool>);
impl CFGSIZE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CFGSIZE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CFGSIZE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CFGSIZE` writer - 8 bit (0) or 16 bit (1) port size"]
pub struct CFGSIZE_W<'a> {
    w: &'a mut W,
}
impl<'a> CFGSIZE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | ((value as u32 & 0x01) << 25);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Lower bound address for CEN0"]
    #[inline(always)]
    pub fn addrlow0(&self) -> ADDRLOW0_R {
        ADDRLOW0_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Upper bound address for CEN0"]
    #[inline(always)]
    pub fn addrhigh0(&self) -> ADDRHIGH0_R {
        ADDRHIGH0_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:18 - Number of cycles for a read - N plus 1"]
    #[inline(always)]
    pub fn cfgreadcycle(&self) -> CFGREADCYCLE_R {
        CFGREADCYCLE_R::new(((self.bits >> 16) & 0x07) as u8)
    }
    #[doc = "Bits 19:21 - Number of cycles for a write - N plus 1"]
    #[inline(always)]
    pub fn cfgwritecycle(&self) -> CFGWRITECYCLE_R {
        CFGWRITECYCLE_R::new(((self.bits >> 19) & 0x07) as u8)
    }
    #[doc = "Bits 22:24 - Number of cycles for turnaround - N plus 1"]
    #[inline(always)]
    pub fn cfgturnaroundcycle(&self) -> CFGTURNAROUNDCYCLE_R {
        CFGTURNAROUNDCYCLE_R::new(((self.bits >> 22) & 0x07) as u8)
    }
    #[doc = "Bit 25 - 8 bit (0) or 16 bit (1) port size"]
    #[inline(always)]
    pub fn cfgsize(&self) -> CFGSIZE_R {
        CFGSIZE_R::new(((self.bits >> 25) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - Lower bound address for CEN0"]
    #[inline(always)]
    pub fn addrlow0(&mut self) -> ADDRLOW0_W {
        ADDRLOW0_W { w: self }
    }
    #[doc = "Bits 8:15 - Upper bound address for CEN0"]
    #[inline(always)]
    pub fn addrhigh0(&mut self) -> ADDRHIGH0_W {
        ADDRHIGH0_W { w: self }
    }
    #[doc = "Bits 16:18 - Number of cycles for a read - N plus 1"]
    #[inline(always)]
    pub fn cfgreadcycle(&mut self) -> CFGREADCYCLE_W {
        CFGREADCYCLE_W { w: self }
    }
    #[doc = "Bits 19:21 - Number of cycles for a write - N plus 1"]
    #[inline(always)]
    pub fn cfgwritecycle(&mut self) -> CFGWRITECYCLE_W {
        CFGWRITECYCLE_W { w: self }
    }
    #[doc = "Bits 22:24 - Number of cycles for turnaround - N plus 1"]
    #[inline(always)]
    pub fn cfgturnaroundcycle(&mut self) -> CFGTURNAROUNDCYCLE_W {
        CFGTURNAROUNDCYCLE_W { w: self }
    }
    #[doc = "Bit 25 - 8 bit (0) or 16 bit (1) port size"]
    #[inline(always)]
    pub fn cfgsize(&mut self) -> CFGSIZE_W {
        CFGSIZE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "EBI Config Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ebi_cfg1](index.html) module"]
pub struct EBI_CFG1_SPEC;
impl crate::RegisterSpec for EBI_CFG1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ebi_cfg1::R](R) reader structure"]
impl crate::Readable for EBI_CFG1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ebi_cfg1::W](W) writer structure"]
impl crate::Writable for EBI_CFG1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets EBI_CFG1 to value 0"]
impl crate::Resettable for EBI_CFG1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
