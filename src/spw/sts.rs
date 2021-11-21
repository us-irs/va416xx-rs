#[doc = "Register `STS` reader"]
pub struct R(crate::R<STS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `STS` writer"]
pub struct W(crate::W<STS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<STS_SPEC>;
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
impl From<crate::W<STS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<STS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `NRXD` reader - Number of Receive Descriptors"]
pub struct NRXD_R(crate::FieldReader<u8, u8>);
impl NRXD_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        NRXD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NRXD_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NRXD` writer - Number of Receive Descriptors"]
pub struct NRXD_W<'a> {
    w: &'a mut W,
}
impl<'a> NRXD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 26)) | ((value as u32 & 0x03) << 26);
        self.w
    }
}
#[doc = "Field `NTXD` reader - Number of Transmit Descriptors"]
pub struct NTXD_R(crate::FieldReader<u8, u8>);
impl NTXD_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        NTXD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NTXD_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NTXD` writer - Number of Transmit Descriptors"]
pub struct NTXD_W<'a> {
    w: &'a mut W,
}
impl<'a> NTXD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 24)) | ((value as u32 & 0x03) << 24);
        self.w
    }
}
#[doc = "Field `LS` reader - Link State"]
pub struct LS_R(crate::FieldReader<u8, u8>);
impl LS_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        LS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LS` writer - Link State"]
pub struct LS_W<'a> {
    w: &'a mut W,
}
impl<'a> LS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 21)) | ((value as u32 & 0x07) << 21);
        self.w
    }
}
#[doc = "Field `AP` reader - Active port"]
pub struct AP_R(crate::FieldReader<bool, bool>);
impl AP_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        AP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AP` writer - Active port"]
pub struct AP_W<'a> {
    w: &'a mut W,
}
impl<'a> AP_W<'a> {
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
#[doc = "Field `EE` reader - Set to one when a packet is received with an EOP after the first byte for a non-RMAP packet and after the second byte for a RMAP packet"]
pub struct EE_R(crate::FieldReader<bool, bool>);
impl EE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EE` writer - Set to one when a packet is received with an EOP after the first byte for a non-RMAP packet and after the second byte for a RMAP packet"]
pub struct EE_W<'a> {
    w: &'a mut W,
}
impl<'a> EE_W<'a> {
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
#[doc = "Field `IA` reader - Packet is received with an invalid destination address field"]
pub struct IA_R(crate::FieldReader<bool, bool>);
impl IA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        IA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IA` writer - Packet is received with an invalid destination address field"]
pub struct IA_W<'a> {
    w: &'a mut W,
}
impl<'a> IA_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | ((value as u32 & 0x01) << 7);
        self.w
    }
}
#[doc = "Field `WE` reader - A synchronization problem has occurred when receiving NChars"]
pub struct WE_R(crate::FieldReader<bool, bool>);
impl WE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        WE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WE` writer - A synchronization problem has occurred when receiving NChars"]
pub struct WE_W<'a> {
    w: &'a mut W,
}
impl<'a> WE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | ((value as u32 & 0x01) << 6);
        self.w
    }
}
#[doc = "Field `PE` reader - Parity error has occurred"]
pub struct PE_R(crate::FieldReader<bool, bool>);
impl PE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PE` writer - Parity error has occurred"]
pub struct PE_W<'a> {
    w: &'a mut W,
}
impl<'a> PE_W<'a> {
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
#[doc = "Field `DE` reader - Disconnection error has occurred"]
pub struct DE_R(crate::FieldReader<bool, bool>);
impl DE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DE` writer - Disconnection error has occurred"]
pub struct DE_W<'a> {
    w: &'a mut W,
}
impl<'a> DE_W<'a> {
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
#[doc = "Field `ER` reader - Escape error has occurred"]
pub struct ER_R(crate::FieldReader<bool, bool>);
impl ER_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ER_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ER_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ER` writer - Escape error has occurred"]
pub struct ER_W<'a> {
    w: &'a mut W,
}
impl<'a> ER_W<'a> {
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
#[doc = "Field `CE` reader - Credit has occurred"]
pub struct CE_R(crate::FieldReader<bool, bool>);
impl CE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CE` writer - Credit has occurred"]
pub struct CE_W<'a> {
    w: &'a mut W,
}
impl<'a> CE_W<'a> {
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
#[doc = "Field `TO` reader - A new time count value was received"]
pub struct TO_R(crate::FieldReader<bool, bool>);
impl TO_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TO_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TO` writer - A new time count value was received"]
pub struct TO_W<'a> {
    w: &'a mut W,
}
impl<'a> TO_W<'a> {
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
    #[doc = "Bits 26:27 - Number of Receive Descriptors"]
    #[inline(always)]
    pub fn nrxd(&self) -> NRXD_R {
        NRXD_R::new(((self.bits >> 26) & 0x03) as u8)
    }
    #[doc = "Bits 24:25 - Number of Transmit Descriptors"]
    #[inline(always)]
    pub fn ntxd(&self) -> NTXD_R {
        NTXD_R::new(((self.bits >> 24) & 0x03) as u8)
    }
    #[doc = "Bits 21:23 - Link State"]
    #[inline(always)]
    pub fn ls(&self) -> LS_R {
        LS_R::new(((self.bits >> 21) & 0x07) as u8)
    }
    #[doc = "Bit 9 - Active port"]
    #[inline(always)]
    pub fn ap(&self) -> AP_R {
        AP_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Set to one when a packet is received with an EOP after the first byte for a non-RMAP packet and after the second byte for a RMAP packet"]
    #[inline(always)]
    pub fn ee(&self) -> EE_R {
        EE_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Packet is received with an invalid destination address field"]
    #[inline(always)]
    pub fn ia(&self) -> IA_R {
        IA_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - A synchronization problem has occurred when receiving NChars"]
    #[inline(always)]
    pub fn we(&self) -> WE_R {
        WE_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Parity error has occurred"]
    #[inline(always)]
    pub fn pe(&self) -> PE_R {
        PE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Disconnection error has occurred"]
    #[inline(always)]
    pub fn de(&self) -> DE_R {
        DE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Escape error has occurred"]
    #[inline(always)]
    pub fn er(&self) -> ER_R {
        ER_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - Credit has occurred"]
    #[inline(always)]
    pub fn ce(&self) -> CE_R {
        CE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - A new time count value was received"]
    #[inline(always)]
    pub fn to(&self) -> TO_R {
        TO_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 26:27 - Number of Receive Descriptors"]
    #[inline(always)]
    pub fn nrxd(&mut self) -> NRXD_W {
        NRXD_W { w: self }
    }
    #[doc = "Bits 24:25 - Number of Transmit Descriptors"]
    #[inline(always)]
    pub fn ntxd(&mut self) -> NTXD_W {
        NTXD_W { w: self }
    }
    #[doc = "Bits 21:23 - Link State"]
    #[inline(always)]
    pub fn ls(&mut self) -> LS_W {
        LS_W { w: self }
    }
    #[doc = "Bit 9 - Active port"]
    #[inline(always)]
    pub fn ap(&mut self) -> AP_W {
        AP_W { w: self }
    }
    #[doc = "Bit 8 - Set to one when a packet is received with an EOP after the first byte for a non-RMAP packet and after the second byte for a RMAP packet"]
    #[inline(always)]
    pub fn ee(&mut self) -> EE_W {
        EE_W { w: self }
    }
    #[doc = "Bit 7 - Packet is received with an invalid destination address field"]
    #[inline(always)]
    pub fn ia(&mut self) -> IA_W {
        IA_W { w: self }
    }
    #[doc = "Bit 6 - A synchronization problem has occurred when receiving NChars"]
    #[inline(always)]
    pub fn we(&mut self) -> WE_W {
        WE_W { w: self }
    }
    #[doc = "Bit 4 - Parity error has occurred"]
    #[inline(always)]
    pub fn pe(&mut self) -> PE_W {
        PE_W { w: self }
    }
    #[doc = "Bit 3 - Disconnection error has occurred"]
    #[inline(always)]
    pub fn de(&mut self) -> DE_W {
        DE_W { w: self }
    }
    #[doc = "Bit 2 - Escape error has occurred"]
    #[inline(always)]
    pub fn er(&mut self) -> ER_W {
        ER_W { w: self }
    }
    #[doc = "Bit 1 - Credit has occurred"]
    #[inline(always)]
    pub fn ce(&mut self) -> CE_W {
        CE_W { w: self }
    }
    #[doc = "Bit 0 - A new time count value was received"]
    #[inline(always)]
    pub fn to(&mut self) -> TO_W {
        TO_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Status/Interrupt Source Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sts](index.html) module"]
pub struct STS_SPEC;
impl crate::RegisterSpec for STS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sts::R](R) reader structure"]
impl crate::Readable for STS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sts::W](W) writer structure"]
impl crate::Writable for STS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets STS to value 0x0640_0000"]
impl crate::Resettable for STS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0640_0000
    }
}
