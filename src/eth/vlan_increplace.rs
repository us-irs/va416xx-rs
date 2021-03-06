#[doc = "Register `VLAN_INCREPLACE` reader"]
pub struct R(crate::R<VLAN_INCREPLACE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<VLAN_INCREPLACE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<VLAN_INCREPLACE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<VLAN_INCREPLACE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `VLAN_INCREPLACE` writer"]
pub struct W(crate::W<VLAN_INCREPLACE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<VLAN_INCREPLACE_SPEC>;
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
impl From<crate::W<VLAN_INCREPLACE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<VLAN_INCREPLACE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CSVL` reader - C-VLAN or S-VLAN"]
pub struct CSVL_R(crate::FieldReader<bool, bool>);
impl CSVL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CSVL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CSVL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CSVL` writer - C-VLAN or S-VLAN"]
pub struct CSVL_W<'a> {
    w: &'a mut W,
}
impl<'a> CSVL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | ((value as u32 & 0x01) << 19);
        self.w
    }
}
#[doc = "Field `VLP` reader - VLAN Priority Control"]
pub struct VLP_R(crate::FieldReader<bool, bool>);
impl VLP_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        VLP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VLP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VLP` writer - VLAN Priority Control"]
pub struct VLP_W<'a> {
    w: &'a mut W,
}
impl<'a> VLP_W<'a> {
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
#[doc = "Field `VLC` reader - VLAN Tag Control in Transmit Frames"]
pub struct VLC_R(crate::FieldReader<u8, u8>);
impl VLC_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        VLC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VLC_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VLC` writer - VLAN Tag Control in Transmit Frames"]
pub struct VLC_W<'a> {
    w: &'a mut W,
}
impl<'a> VLC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | ((value as u32 & 0x03) << 16);
        self.w
    }
}
#[doc = "Field `VLT` reader - VLAN Tag for Transmit Frames"]
pub struct VLT_R(crate::FieldReader<u16, u16>);
impl VLT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        VLT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VLT_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VLT` writer - VLAN Tag for Transmit Frames"]
pub struct VLT_W<'a> {
    w: &'a mut W,
}
impl<'a> VLT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bit 19 - C-VLAN or S-VLAN"]
    #[inline(always)]
    pub fn csvl(&self) -> CSVL_R {
        CSVL_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 18 - VLAN Priority Control"]
    #[inline(always)]
    pub fn vlp(&self) -> VLP_R {
        VLP_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bits 16:17 - VLAN Tag Control in Transmit Frames"]
    #[inline(always)]
    pub fn vlc(&self) -> VLC_R {
        VLC_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bits 0:15 - VLAN Tag for Transmit Frames"]
    #[inline(always)]
    pub fn vlt(&self) -> VLT_R {
        VLT_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bit 19 - C-VLAN or S-VLAN"]
    #[inline(always)]
    pub fn csvl(&mut self) -> CSVL_W {
        CSVL_W { w: self }
    }
    #[doc = "Bit 18 - VLAN Priority Control"]
    #[inline(always)]
    pub fn vlp(&mut self) -> VLP_W {
        VLP_W { w: self }
    }
    #[doc = "Bits 16:17 - VLAN Tag Control in Transmit Frames"]
    #[inline(always)]
    pub fn vlc(&mut self) -> VLC_W {
        VLC_W { w: self }
    }
    #[doc = "Bits 0:15 - VLAN Tag for Transmit Frames"]
    #[inline(always)]
    pub fn vlt(&mut self) -> VLT_W {
        VLT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Holds the VLAN Tag for insertion into or replacement in the transmit frames\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [vlan_increplace](index.html) module"]
pub struct VLAN_INCREPLACE_SPEC;
impl crate::RegisterSpec for VLAN_INCREPLACE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [vlan_increplace::R](R) reader structure"]
impl crate::Readable for VLAN_INCREPLACE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [vlan_increplace::W](W) writer structure"]
impl crate::Writable for VLAN_INCREPLACE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets VLAN_INCREPLACE to value 0"]
impl crate::Resettable for VLAN_INCREPLACE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
