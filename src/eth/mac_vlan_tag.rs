#[doc = "Register `MAC_VLAN_TAG` reader"]
pub struct R(crate::R<MAC_VLAN_TAG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MAC_VLAN_TAG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MAC_VLAN_TAG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MAC_VLAN_TAG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MAC_VLAN_TAG` writer"]
pub struct W(crate::W<MAC_VLAN_TAG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MAC_VLAN_TAG_SPEC>;
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
impl From<crate::W<MAC_VLAN_TAG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MAC_VLAN_TAG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ESVL` reader - Enable S-VLAN"]
pub struct ESVL_R(crate::FieldReader<bool, bool>);
impl ESVL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ESVL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ESVL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ESVL` writer - Enable S-VLAN"]
pub struct ESVL_W<'a> {
    w: &'a mut W,
}
impl<'a> ESVL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | ((value as u32 & 0x01) << 18);
        self.w
    }
}
#[doc = "Field `VTIM` reader - VLAN Tag Inverse Match Enable"]
pub struct VTIM_R(crate::FieldReader<bool, bool>);
impl VTIM_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        VTIM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VTIM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VTIM` writer - VLAN Tag Inverse Match Enable"]
pub struct VTIM_W<'a> {
    w: &'a mut W,
}
impl<'a> VTIM_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | ((value as u32 & 0x01) << 17);
        self.w
    }
}
#[doc = "Field `ETV` reader - Enable 12-Bit VLAN Tag Comparison"]
pub struct ETV_R(crate::FieldReader<bool, bool>);
impl ETV_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ETV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ETV_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ETV` writer - Enable 12-Bit VLAN Tag Comparison"]
pub struct ETV_W<'a> {
    w: &'a mut W,
}
impl<'a> ETV_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | ((value as u32 & 0x01) << 16);
        self.w
    }
}
#[doc = "Field `VL` reader - VLAN Tag identifier for Receive Frames"]
pub struct VL_R(crate::FieldReader<u16, u16>);
impl VL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        VL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VL_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VL` writer - VLAN Tag identifier for Receive Frames"]
pub struct VL_W<'a> {
    w: &'a mut W,
}
impl<'a> VL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bit 18 - Enable S-VLAN"]
    #[inline(always)]
    pub fn esvl(&self) -> ESVL_R {
        ESVL_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 17 - VLAN Tag Inverse Match Enable"]
    #[inline(always)]
    pub fn vtim(&self) -> VTIM_R {
        VTIM_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Enable 12-Bit VLAN Tag Comparison"]
    #[inline(always)]
    pub fn etv(&self) -> ETV_R {
        ETV_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bits 0:15 - VLAN Tag identifier for Receive Frames"]
    #[inline(always)]
    pub fn vl(&self) -> VL_R {
        VL_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bit 18 - Enable S-VLAN"]
    #[inline(always)]
    pub fn esvl(&mut self) -> ESVL_W {
        ESVL_W { w: self }
    }
    #[doc = "Bit 17 - VLAN Tag Inverse Match Enable"]
    #[inline(always)]
    pub fn vtim(&mut self) -> VTIM_W {
        VTIM_W { w: self }
    }
    #[doc = "Bit 16 - Enable 12-Bit VLAN Tag Comparison"]
    #[inline(always)]
    pub fn etv(&mut self) -> ETV_W {
        ETV_W { w: self }
    }
    #[doc = "Bits 0:15 - VLAN Tag identifier for Receive Frames"]
    #[inline(always)]
    pub fn vl(&mut self) -> VL_W {
        VL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Identifies IEEE 802.1Q VLAN type frames\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mac_vlan_tag](index.html) module"]
pub struct MAC_VLAN_TAG_SPEC;
impl crate::RegisterSpec for MAC_VLAN_TAG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mac_vlan_tag::R](R) reader structure"]
impl crate::Readable for MAC_VLAN_TAG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mac_vlan_tag::W](W) writer structure"]
impl crate::Writable for MAC_VLAN_TAG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MAC_VLAN_TAG to value 0"]
impl crate::Resettable for MAC_VLAN_TAG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
