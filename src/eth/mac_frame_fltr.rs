#[doc = "Register `MAC_FRAME_FLTR` reader"]
pub struct R(crate::R<MAC_FRAME_FLTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MAC_FRAME_FLTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MAC_FRAME_FLTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MAC_FRAME_FLTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MAC_FRAME_FLTR` writer"]
pub struct W(crate::W<MAC_FRAME_FLTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MAC_FRAME_FLTR_SPEC>;
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
impl From<crate::W<MAC_FRAME_FLTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MAC_FRAME_FLTR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RA` reader - Receive All"]
pub struct RA_R(crate::FieldReader<bool, bool>);
impl RA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RA` writer - Receive All"]
pub struct RA_W<'a> {
    w: &'a mut W,
}
impl<'a> RA_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | ((value as u32 & 0x01) << 31);
        self.w
    }
}
#[doc = "Field `DNTU` reader - Drop non TCP/UDP over IP Frames"]
pub struct DNTU_R(crate::FieldReader<bool, bool>);
impl DNTU_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DNTU_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DNTU_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DNTU` writer - Drop non TCP/UDP over IP Frames"]
pub struct DNTU_W<'a> {
    w: &'a mut W,
}
impl<'a> DNTU_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | ((value as u32 & 0x01) << 21);
        self.w
    }
}
#[doc = "Field `VFTE` reader - VLAN Tag Filter Enable"]
pub struct VFTE_R(crate::FieldReader<bool, bool>);
impl VFTE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        VFTE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VFTE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VFTE` writer - VLAN Tag Filter Enable"]
pub struct VFTE_W<'a> {
    w: &'a mut W,
}
impl<'a> VFTE_W<'a> {
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
#[doc = "Field `HDF` reader - Hash or Perfect Filter"]
pub struct HDF_R(crate::FieldReader<bool, bool>);
impl HDF_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        HDF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HDF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HDF` writer - Hash or Perfect Filter"]
pub struct HDF_W<'a> {
    w: &'a mut W,
}
impl<'a> HDF_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | ((value as u32 & 0x01) << 10);
        self.w
    }
}
#[doc = "Field `SAF` reader - Source Address Filter Enable"]
pub struct SAF_R(crate::FieldReader<bool, bool>);
impl SAF_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SAF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SAF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SAF` writer - Source Address Filter Enable"]
pub struct SAF_W<'a> {
    w: &'a mut W,
}
impl<'a> SAF_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | ((value as u32 & 0x01) << 9);
        self.w
    }
}
#[doc = "Field `SAIF` reader - SA Inverse Filtering"]
pub struct SAIF_R(crate::FieldReader<bool, bool>);
impl SAIF_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SAIF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SAIF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SAIF` writer - SA Inverse Filtering"]
pub struct SAIF_W<'a> {
    w: &'a mut W,
}
impl<'a> SAIF_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | ((value as u32 & 0x01) << 8);
        self.w
    }
}
#[doc = "Field `PCF` reader - Pass Control Frames"]
pub struct PCF_R(crate::FieldReader<u8, u8>);
impl PCF_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PCF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PCF_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PCF` writer - Pass Control Frames"]
pub struct PCF_W<'a> {
    w: &'a mut W,
}
impl<'a> PCF_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | ((value as u32 & 0x03) << 6);
        self.w
    }
}
#[doc = "Field `DBF` reader - Disable Broadcast Frames"]
pub struct DBF_R(crate::FieldReader<bool, bool>);
impl DBF_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DBF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DBF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DBF` writer - Disable Broadcast Frames"]
pub struct DBF_W<'a> {
    w: &'a mut W,
}
impl<'a> DBF_W<'a> {
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
#[doc = "Field `PM` reader - Pass All Multicast"]
pub struct PM_R(crate::FieldReader<bool, bool>);
impl PM_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PM` writer - Pass All Multicast"]
pub struct PM_W<'a> {
    w: &'a mut W,
}
impl<'a> PM_W<'a> {
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
#[doc = "Field `DAIF` reader - DA Inverse Filtering"]
pub struct DAIF_R(crate::FieldReader<bool, bool>);
impl DAIF_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DAIF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DAIF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DAIF` writer - DA Inverse Filtering"]
pub struct DAIF_W<'a> {
    w: &'a mut W,
}
impl<'a> DAIF_W<'a> {
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
#[doc = "Field `HMC` reader - Hash Multicast"]
pub struct HMC_R(crate::FieldReader<bool, bool>);
impl HMC_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        HMC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HMC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HMC` writer - Hash Multicast"]
pub struct HMC_W<'a> {
    w: &'a mut W,
}
impl<'a> HMC_W<'a> {
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
#[doc = "Field `HUC` reader - Hash Unicast"]
pub struct HUC_R(crate::FieldReader<bool, bool>);
impl HUC_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        HUC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HUC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HUC` writer - Hash Unicast"]
pub struct HUC_W<'a> {
    w: &'a mut W,
}
impl<'a> HUC_W<'a> {
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
#[doc = "Field `PR` reader - Promiscuous Mode"]
pub struct PR_R(crate::FieldReader<bool, bool>);
impl PR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PR` writer - Promiscuous Mode"]
pub struct PR_W<'a> {
    w: &'a mut W,
}
impl<'a> PR_W<'a> {
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
    #[doc = "Bit 31 - Receive All"]
    #[inline(always)]
    pub fn ra(&self) -> RA_R {
        RA_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Drop non TCP/UDP over IP Frames"]
    #[inline(always)]
    pub fn dntu(&self) -> DNTU_R {
        DNTU_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 16 - VLAN Tag Filter Enable"]
    #[inline(always)]
    pub fn vfte(&self) -> VFTE_R {
        VFTE_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Hash or Perfect Filter"]
    #[inline(always)]
    pub fn hdf(&self) -> HDF_R {
        HDF_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Source Address Filter Enable"]
    #[inline(always)]
    pub fn saf(&self) -> SAF_R {
        SAF_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - SA Inverse Filtering"]
    #[inline(always)]
    pub fn saif(&self) -> SAIF_R {
        SAIF_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bits 6:7 - Pass Control Frames"]
    #[inline(always)]
    pub fn pcf(&self) -> PCF_R {
        PCF_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bit 5 - Disable Broadcast Frames"]
    #[inline(always)]
    pub fn dbf(&self) -> DBF_R {
        DBF_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Pass All Multicast"]
    #[inline(always)]
    pub fn pm(&self) -> PM_R {
        PM_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - DA Inverse Filtering"]
    #[inline(always)]
    pub fn daif(&self) -> DAIF_R {
        DAIF_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Hash Multicast"]
    #[inline(always)]
    pub fn hmc(&self) -> HMC_R {
        HMC_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - Hash Unicast"]
    #[inline(always)]
    pub fn huc(&self) -> HUC_R {
        HUC_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Promiscuous Mode"]
    #[inline(always)]
    pub fn pr(&self) -> PR_R {
        PR_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 31 - Receive All"]
    #[inline(always)]
    pub fn ra(&mut self) -> RA_W {
        RA_W { w: self }
    }
    #[doc = "Bit 21 - Drop non TCP/UDP over IP Frames"]
    #[inline(always)]
    pub fn dntu(&mut self) -> DNTU_W {
        DNTU_W { w: self }
    }
    #[doc = "Bit 16 - VLAN Tag Filter Enable"]
    #[inline(always)]
    pub fn vfte(&mut self) -> VFTE_W {
        VFTE_W { w: self }
    }
    #[doc = "Bit 10 - Hash or Perfect Filter"]
    #[inline(always)]
    pub fn hdf(&mut self) -> HDF_W {
        HDF_W { w: self }
    }
    #[doc = "Bit 9 - Source Address Filter Enable"]
    #[inline(always)]
    pub fn saf(&mut self) -> SAF_W {
        SAF_W { w: self }
    }
    #[doc = "Bit 8 - SA Inverse Filtering"]
    #[inline(always)]
    pub fn saif(&mut self) -> SAIF_W {
        SAIF_W { w: self }
    }
    #[doc = "Bits 6:7 - Pass Control Frames"]
    #[inline(always)]
    pub fn pcf(&mut self) -> PCF_W {
        PCF_W { w: self }
    }
    #[doc = "Bit 5 - Disable Broadcast Frames"]
    #[inline(always)]
    pub fn dbf(&mut self) -> DBF_W {
        DBF_W { w: self }
    }
    #[doc = "Bit 4 - Pass All Multicast"]
    #[inline(always)]
    pub fn pm(&mut self) -> PM_W {
        PM_W { w: self }
    }
    #[doc = "Bit 3 - DA Inverse Filtering"]
    #[inline(always)]
    pub fn daif(&mut self) -> DAIF_W {
        DAIF_W { w: self }
    }
    #[doc = "Bit 2 - Hash Multicast"]
    #[inline(always)]
    pub fn hmc(&mut self) -> HMC_W {
        HMC_W { w: self }
    }
    #[doc = "Bit 1 - Hash Unicast"]
    #[inline(always)]
    pub fn huc(&mut self) -> HUC_W {
        HUC_W { w: self }
    }
    #[doc = "Bit 0 - Promiscuous Mode"]
    #[inline(always)]
    pub fn pr(&mut self) -> PR_W {
        PR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Contains the frame filtering controls\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mac_frame_fltr](index.html) module"]
pub struct MAC_FRAME_FLTR_SPEC;
impl crate::RegisterSpec for MAC_FRAME_FLTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mac_frame_fltr::R](R) reader structure"]
impl crate::Readable for MAC_FRAME_FLTR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mac_frame_fltr::W](W) writer structure"]
impl crate::Writable for MAC_FRAME_FLTR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MAC_FRAME_FLTR to value 0"]
impl crate::Resettable for MAC_FRAME_FLTR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
