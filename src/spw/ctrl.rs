#[doc = "Register `CTRL` reader"]
pub struct R(crate::R<CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTRL` writer"]
pub struct W(crate::W<CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTRL_SPEC>;
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
impl From<crate::W<CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RA` reader - Reads as 1 if the RMAP command handler is available"]
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
#[doc = "Field `RX` reader - Reads as 1 if unaligned writes are available for the receiver"]
pub struct RX_R(crate::FieldReader<bool, bool>);
impl RX_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RX_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RX_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RC` reader - Reads as 1 if RMAP CRC is enabled in the core"]
pub struct RC_R(crate::FieldReader<bool, bool>);
impl RC_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NCH` reader - Number of DMA Channels minus one"]
pub struct NCH_R(crate::FieldReader<u8, u8>);
impl NCH_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        NCH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NCH_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PO` reader - The number of available SpaceWire ports minus one"]
pub struct PO_R(crate::FieldReader<bool, bool>);
impl PO_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PO_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CC` reader - CCSDS/CCITT CRC-16"]
pub struct CC_R(crate::FieldReader<bool, bool>);
impl CC_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ID` reader - Interrupt distribution available"]
pub struct ID_R(crate::FieldReader<bool, bool>);
impl ID_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ID_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ID_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LE` reader - Loop-back Enable"]
pub struct LE_R(crate::FieldReader<bool, bool>);
impl LE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LE` writer - Loop-back Enable"]
pub struct LE_W<'a> {
    w: &'a mut W,
}
impl<'a> LE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | ((value as u32 & 0x01) << 22);
        self.w
    }
}
#[doc = "Field `PS` reader - Selects the active port when the no port force bit is zero"]
pub struct PS_R(crate::FieldReader<bool, bool>);
impl PS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PS` writer - Selects the active port when the no port force bit is zero"]
pub struct PS_W<'a> {
    w: &'a mut W,
}
impl<'a> PS_W<'a> {
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
#[doc = "Field `NP` reader - Disable port force"]
pub struct NP_R(crate::FieldReader<bool, bool>);
impl NP_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        NP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NP` writer - Disable port force"]
pub struct NP_W<'a> {
    w: &'a mut W,
}
impl<'a> NP_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | ((value as u32 & 0x01) << 20);
        self.w
    }
}
#[doc = "Field `PNPA` reader - SpW Plug-and-Play Available"]
pub struct PNPA_R(crate::FieldReader<u8, u8>);
impl PNPA_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PNPA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PNPA_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RD` reader - If set only one RMAP buffer is used"]
pub struct RD_R(crate::FieldReader<bool, bool>);
impl RD_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RD` writer - If set only one RMAP buffer is used"]
pub struct RD_W<'a> {
    w: &'a mut W,
}
impl<'a> RD_W<'a> {
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
#[doc = "Field `RE` reader - Enable RMAP command handler"]
pub struct RE_R(crate::FieldReader<bool, bool>);
impl RE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RE` writer - Enable RMAP command handler"]
pub struct RE_W<'a> {
    w: &'a mut W,
}
impl<'a> RE_W<'a> {
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
#[doc = "Field `PE` reader - SpW Plug-and-Play Enable"]
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
#[doc = "Field `PE` writer - SpW Plug-and-Play Enable"]
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | ((value as u32 & 0x01) << 15);
        self.w
    }
}
#[doc = "Field `TL` reader - Transmitter Enable Lock Control"]
pub struct TL_R(crate::FieldReader<bool, bool>);
impl TL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TL` writer - Transmitter Enable Lock Control"]
pub struct TL_W<'a> {
    w: &'a mut W,
}
impl<'a> TL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | ((value as u32 & 0x01) << 13);
        self.w
    }
}
#[doc = "Field `TF` reader - Time-code Flag Filter"]
pub struct TF_R(crate::FieldReader<bool, bool>);
impl TF_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TF` writer - Time-code Flag Filter"]
pub struct TF_W<'a> {
    w: &'a mut W,
}
impl<'a> TF_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | ((value as u32 & 0x01) << 12);
        self.w
    }
}
#[doc = "Field `TR` reader - Enable time-code receptions"]
pub struct TR_R(crate::FieldReader<bool, bool>);
impl TR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TR` writer - Enable time-code receptions"]
pub struct TR_W<'a> {
    w: &'a mut W,
}
impl<'a> TR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | ((value as u32 & 0x01) << 11);
        self.w
    }
}
#[doc = "Field `TT` reader - Enable time-code transmissions"]
pub struct TT_R(crate::FieldReader<bool, bool>);
impl TT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TT` writer - Enable time-code transmissions"]
pub struct TT_W<'a> {
    w: &'a mut W,
}
impl<'a> TT_W<'a> {
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
#[doc = "Field `LI` reader - Generate interrupt when link error occurs"]
pub struct LI_R(crate::FieldReader<bool, bool>);
impl LI_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LI_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LI` writer - Generate interrupt when link error occurs"]
pub struct LI_W<'a> {
    w: &'a mut W,
}
impl<'a> LI_W<'a> {
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
#[doc = "Field `TQ` reader - Generate interrupt when a valid time-code is received"]
pub struct TQ_R(crate::FieldReader<bool, bool>);
impl TQ_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TQ_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TQ_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TQ` writer - Generate interrupt when a valid time-code is received"]
pub struct TQ_W<'a> {
    w: &'a mut W,
}
impl<'a> TQ_W<'a> {
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
#[doc = "Field `RS` reader - Make complete reset of the SpaceWire node. Self-clearing"]
pub struct RS_R(crate::FieldReader<bool, bool>);
impl RS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RS` writer - Make complete reset of the SpaceWire node. Self-clearing"]
pub struct RS_W<'a> {
    w: &'a mut W,
}
impl<'a> RS_W<'a> {
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
#[doc = "Field `PM` reader - Enable Promiscuous mode"]
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
#[doc = "Field `PM` writer - Enable Promiscuous mode"]
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u32 & 0x01) << 5);
        self.w
    }
}
#[doc = "Field `TI` reader - The host can generate a tick by writing a one to this field"]
pub struct TI_R(crate::FieldReader<bool, bool>);
impl TI_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TI_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TI` writer - The host can generate a tick by writing a one to this field"]
pub struct TI_W<'a> {
    w: &'a mut W,
}
impl<'a> TI_W<'a> {
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
#[doc = "Field `IE` reader - If set, an interrupt is generated when one or both of bit 8 to 9 is set and its corresponding event occurs"]
pub struct IE_R(crate::FieldReader<bool, bool>);
impl IE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        IE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IE` writer - If set, an interrupt is generated when one or both of bit 8 to 9 is set and its corresponding event occurs"]
pub struct IE_W<'a> {
    w: &'a mut W,
}
impl<'a> IE_W<'a> {
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
#[doc = "Field `AS` reader - Automatically start the link when a NULL has been received"]
pub struct AS_R(crate::FieldReader<bool, bool>);
impl AS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        AS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AS` writer - Automatically start the link when a NULL has been received"]
pub struct AS_W<'a> {
    w: &'a mut W,
}
impl<'a> AS_W<'a> {
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
#[doc = "Field `LS` reader - Start the link"]
pub struct LS_R(crate::FieldReader<bool, bool>);
impl LS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LS` writer - Start the link"]
pub struct LS_W<'a> {
    w: &'a mut W,
}
impl<'a> LS_W<'a> {
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
#[doc = "Field `LD` reader - Disable the SpaceWire CODEC"]
pub struct LD_R(crate::FieldReader<bool, bool>);
impl LD_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LD` writer - Disable the SpaceWire CODEC"]
pub struct LD_W<'a> {
    w: &'a mut W,
}
impl<'a> LD_W<'a> {
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
    #[doc = "Bit 31 - Reads as 1 if the RMAP command handler is available"]
    #[inline(always)]
    pub fn ra(&self) -> RA_R {
        RA_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bit 30 - Reads as 1 if unaligned writes are available for the receiver"]
    #[inline(always)]
    pub fn rx(&self) -> RX_R {
        RX_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 29 - Reads as 1 if RMAP CRC is enabled in the core"]
    #[inline(always)]
    pub fn rc(&self) -> RC_R {
        RC_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bits 27:28 - Number of DMA Channels minus one"]
    #[inline(always)]
    pub fn nch(&self) -> NCH_R {
        NCH_R::new(((self.bits >> 27) & 0x03) as u8)
    }
    #[doc = "Bit 26 - The number of available SpaceWire ports minus one"]
    #[inline(always)]
    pub fn po(&self) -> PO_R {
        PO_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 25 - CCSDS/CCITT CRC-16"]
    #[inline(always)]
    pub fn cc(&self) -> CC_R {
        CC_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Interrupt distribution available"]
    #[inline(always)]
    pub fn id(&self) -> ID_R {
        ID_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Loop-back Enable"]
    #[inline(always)]
    pub fn le(&self) -> LE_R {
        LE_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Selects the active port when the no port force bit is zero"]
    #[inline(always)]
    pub fn ps(&self) -> PS_R {
        PS_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Disable port force"]
    #[inline(always)]
    pub fn np(&self) -> NP_R {
        NP_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bits 18:19 - SpW Plug-and-Play Available"]
    #[inline(always)]
    pub fn pnpa(&self) -> PNPA_R {
        PNPA_R::new(((self.bits >> 18) & 0x03) as u8)
    }
    #[doc = "Bit 17 - If set only one RMAP buffer is used"]
    #[inline(always)]
    pub fn rd(&self) -> RD_R {
        RD_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Enable RMAP command handler"]
    #[inline(always)]
    pub fn re(&self) -> RE_R {
        RE_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 15 - SpW Plug-and-Play Enable"]
    #[inline(always)]
    pub fn pe(&self) -> PE_R {
        PE_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Transmitter Enable Lock Control"]
    #[inline(always)]
    pub fn tl(&self) -> TL_R {
        TL_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Time-code Flag Filter"]
    #[inline(always)]
    pub fn tf(&self) -> TF_R {
        TF_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Enable time-code receptions"]
    #[inline(always)]
    pub fn tr(&self) -> TR_R {
        TR_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Enable time-code transmissions"]
    #[inline(always)]
    pub fn tt(&self) -> TT_R {
        TT_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Generate interrupt when link error occurs"]
    #[inline(always)]
    pub fn li(&self) -> LI_R {
        LI_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Generate interrupt when a valid time-code is received"]
    #[inline(always)]
    pub fn tq(&self) -> TQ_R {
        TQ_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Make complete reset of the SpaceWire node. Self-clearing"]
    #[inline(always)]
    pub fn rs(&self) -> RS_R {
        RS_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Enable Promiscuous mode"]
    #[inline(always)]
    pub fn pm(&self) -> PM_R {
        PM_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - The host can generate a tick by writing a one to this field"]
    #[inline(always)]
    pub fn ti(&self) -> TI_R {
        TI_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - If set, an interrupt is generated when one or both of bit 8 to 9 is set and its corresponding event occurs"]
    #[inline(always)]
    pub fn ie(&self) -> IE_R {
        IE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Automatically start the link when a NULL has been received"]
    #[inline(always)]
    pub fn as_(&self) -> AS_R {
        AS_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - Start the link"]
    #[inline(always)]
    pub fn ls(&self) -> LS_R {
        LS_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Disable the SpaceWire CODEC"]
    #[inline(always)]
    pub fn ld(&self) -> LD_R {
        LD_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 22 - Loop-back Enable"]
    #[inline(always)]
    pub fn le(&mut self) -> LE_W {
        LE_W { w: self }
    }
    #[doc = "Bit 21 - Selects the active port when the no port force bit is zero"]
    #[inline(always)]
    pub fn ps(&mut self) -> PS_W {
        PS_W { w: self }
    }
    #[doc = "Bit 20 - Disable port force"]
    #[inline(always)]
    pub fn np(&mut self) -> NP_W {
        NP_W { w: self }
    }
    #[doc = "Bit 17 - If set only one RMAP buffer is used"]
    #[inline(always)]
    pub fn rd(&mut self) -> RD_W {
        RD_W { w: self }
    }
    #[doc = "Bit 16 - Enable RMAP command handler"]
    #[inline(always)]
    pub fn re(&mut self) -> RE_W {
        RE_W { w: self }
    }
    #[doc = "Bit 15 - SpW Plug-and-Play Enable"]
    #[inline(always)]
    pub fn pe(&mut self) -> PE_W {
        PE_W { w: self }
    }
    #[doc = "Bit 13 - Transmitter Enable Lock Control"]
    #[inline(always)]
    pub fn tl(&mut self) -> TL_W {
        TL_W { w: self }
    }
    #[doc = "Bit 12 - Time-code Flag Filter"]
    #[inline(always)]
    pub fn tf(&mut self) -> TF_W {
        TF_W { w: self }
    }
    #[doc = "Bit 11 - Enable time-code receptions"]
    #[inline(always)]
    pub fn tr(&mut self) -> TR_W {
        TR_W { w: self }
    }
    #[doc = "Bit 10 - Enable time-code transmissions"]
    #[inline(always)]
    pub fn tt(&mut self) -> TT_W {
        TT_W { w: self }
    }
    #[doc = "Bit 9 - Generate interrupt when link error occurs"]
    #[inline(always)]
    pub fn li(&mut self) -> LI_W {
        LI_W { w: self }
    }
    #[doc = "Bit 8 - Generate interrupt when a valid time-code is received"]
    #[inline(always)]
    pub fn tq(&mut self) -> TQ_W {
        TQ_W { w: self }
    }
    #[doc = "Bit 6 - Make complete reset of the SpaceWire node. Self-clearing"]
    #[inline(always)]
    pub fn rs(&mut self) -> RS_W {
        RS_W { w: self }
    }
    #[doc = "Bit 5 - Enable Promiscuous mode"]
    #[inline(always)]
    pub fn pm(&mut self) -> PM_W {
        PM_W { w: self }
    }
    #[doc = "Bit 4 - The host can generate a tick by writing a one to this field"]
    #[inline(always)]
    pub fn ti(&mut self) -> TI_W {
        TI_W { w: self }
    }
    #[doc = "Bit 3 - If set, an interrupt is generated when one or both of bit 8 to 9 is set and its corresponding event occurs"]
    #[inline(always)]
    pub fn ie(&mut self) -> IE_W {
        IE_W { w: self }
    }
    #[doc = "Bit 2 - Automatically start the link when a NULL has been received"]
    #[inline(always)]
    pub fn as_(&mut self) -> AS_W {
        AS_W { w: self }
    }
    #[doc = "Bit 1 - Start the link"]
    #[inline(always)]
    pub fn ls(&mut self) -> LS_W {
        LS_W { w: self }
    }
    #[doc = "Bit 0 - Disable the SpaceWire CODEC"]
    #[inline(always)]
    pub fn ld(&mut self) -> LD_W {
        LD_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl](index.html) module"]
pub struct CTRL_SPEC;
impl crate::RegisterSpec for CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctrl::R](R) reader structure"]
impl crate::Readable for CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctrl::W](W) writer structure"]
impl crate::Writable for CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CTRL to value 0xa201_0004"]
impl crate::Resettable for CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xa201_0004
    }
}
