#[doc = "Register `MAC_GMII_ADDR` reader"]
pub struct R(crate::R<MAC_GMII_ADDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MAC_GMII_ADDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MAC_GMII_ADDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MAC_GMII_ADDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MAC_GMII_ADDR` writer"]
pub struct W(crate::W<MAC_GMII_ADDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MAC_GMII_ADDR_SPEC>;
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
impl From<crate::W<MAC_GMII_ADDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MAC_GMII_ADDR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PA` reader - Physical Layer Address"]
pub struct PA_R(crate::FieldReader<u8, u8>);
impl PA_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PA_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PA` writer - Physical Layer Address"]
pub struct PA_W<'a> {
    w: &'a mut W,
}
impl<'a> PA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 11)) | ((value as u32 & 0x1f) << 11);
        self.w
    }
}
#[doc = "Field `GR` reader - GMII Register"]
pub struct GR_R(crate::FieldReader<u8, u8>);
impl GR_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        GR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GR_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GR` writer - GMII Register"]
pub struct GR_W<'a> {
    w: &'a mut W,
}
impl<'a> GR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 6)) | ((value as u32 & 0x1f) << 6);
        self.w
    }
}
#[doc = "Field `CR` reader - CSR Clock Range"]
pub struct CR_R(crate::FieldReader<u8, u8>);
impl CR_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CR_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CR` writer - CSR Clock Range"]
pub struct CR_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 2)) | ((value as u32 & 0x0f) << 2);
        self.w
    }
}
#[doc = "Field `GW` reader - GMII Write/Read"]
pub struct GW_R(crate::FieldReader<bool, bool>);
impl GW_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        GW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GW` writer - GMII Write/Read"]
pub struct GW_W<'a> {
    w: &'a mut W,
}
impl<'a> GW_W<'a> {
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
#[doc = "Field `GB` reader - GMII Busy"]
pub struct GB_R(crate::FieldReader<bool, bool>);
impl GB_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        GB_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GB_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GB` writer - GMII Busy"]
pub struct GB_W<'a> {
    w: &'a mut W,
}
impl<'a> GB_W<'a> {
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
    #[doc = "Bits 11:15 - Physical Layer Address"]
    #[inline(always)]
    pub fn pa(&self) -> PA_R {
        PA_R::new(((self.bits >> 11) & 0x1f) as u8)
    }
    #[doc = "Bits 6:10 - GMII Register"]
    #[inline(always)]
    pub fn gr(&self) -> GR_R {
        GR_R::new(((self.bits >> 6) & 0x1f) as u8)
    }
    #[doc = "Bits 2:5 - CSR Clock Range"]
    #[inline(always)]
    pub fn cr(&self) -> CR_R {
        CR_R::new(((self.bits >> 2) & 0x0f) as u8)
    }
    #[doc = "Bit 1 - GMII Write/Read"]
    #[inline(always)]
    pub fn gw(&self) -> GW_R {
        GW_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - GMII Busy"]
    #[inline(always)]
    pub fn gb(&self) -> GB_R {
        GB_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 11:15 - Physical Layer Address"]
    #[inline(always)]
    pub fn pa(&mut self) -> PA_W {
        PA_W { w: self }
    }
    #[doc = "Bits 6:10 - GMII Register"]
    #[inline(always)]
    pub fn gr(&mut self) -> GR_W {
        GR_W { w: self }
    }
    #[doc = "Bits 2:5 - CSR Clock Range"]
    #[inline(always)]
    pub fn cr(&mut self) -> CR_W {
        CR_W { w: self }
    }
    #[doc = "Bit 1 - GMII Write/Read"]
    #[inline(always)]
    pub fn gw(&mut self) -> GW_W {
        GW_W { w: self }
    }
    #[doc = "Bit 0 - GMII Busy"]
    #[inline(always)]
    pub fn gb(&mut self) -> GB_W {
        GB_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Controls the management cycles to an external PHY\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mac_gmii_addr](index.html) module"]
pub struct MAC_GMII_ADDR_SPEC;
impl crate::RegisterSpec for MAC_GMII_ADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mac_gmii_addr::R](R) reader structure"]
impl crate::Readable for MAC_GMII_ADDR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mac_gmii_addr::W](W) writer structure"]
impl crate::Writable for MAC_GMII_ADDR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MAC_GMII_ADDR to value 0"]
impl crate::Resettable for MAC_GMII_ADDR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
