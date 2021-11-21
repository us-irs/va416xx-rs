#[doc = "Register `DMACTRL0` reader"]
pub struct R(crate::R<DMACTRL0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMACTRL0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMACTRL0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMACTRL0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DMACTRL0` writer"]
pub struct W(crate::W<DMACTRL0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMACTRL0_SPEC>;
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
impl From<crate::W<DMACTRL0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DMACTRL0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `INTNUM` reader - Interrupt number used for this channel"]
pub struct INTNUM_R(crate::FieldReader<u8, u8>);
impl INTNUM_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        INTNUM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INTNUM_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INTNUM` writer - Interrupt number used for this channel"]
pub struct INTNUM_W<'a> {
    w: &'a mut W,
}
impl<'a> INTNUM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 26)) | ((value as u32 & 0x3f) << 26);
        self.w
    }
}
#[doc = "Field `EP` reader - EEP Termination"]
pub struct EP_R(crate::FieldReader<bool, bool>);
impl EP_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EP` writer - EEP Termination"]
pub struct EP_W<'a> {
    w: &'a mut W,
}
impl<'a> EP_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | ((value as u32 & 0x01) << 23);
        self.w
    }
}
#[doc = "Field `TR` reader - Truncated"]
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
#[doc = "Field `TR` writer - Truncated"]
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | ((value as u32 & 0x01) << 22);
        self.w
    }
}
#[doc = "Field `IE` reader - Interrupt code transmit enable on EEP"]
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
#[doc = "Field `IE` writer - Interrupt code transmit enable on EEP"]
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | ((value as u32 & 0x01) << 21);
        self.w
    }
}
#[doc = "Field `IT` reader - Interrupt code transmit enable on truncation"]
pub struct IT_R(crate::FieldReader<bool, bool>);
impl IT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        IT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IT` writer - Interrupt code transmit enable on truncation"]
pub struct IT_W<'a> {
    w: &'a mut W,
}
impl<'a> IT_W<'a> {
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
#[doc = "Field `RP` reader - Receive Packet IRQ"]
pub struct RP_R(crate::FieldReader<bool, bool>);
impl RP_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RP` writer - Receive Packet IRQ"]
pub struct RP_W<'a> {
    w: &'a mut W,
}
impl<'a> RP_W<'a> {
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
#[doc = "Field `TP` reader - Transmit Packet IRQ"]
pub struct TP_R(crate::FieldReader<bool, bool>);
impl TP_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TP` writer - Transmit Packet IRQ"]
pub struct TP_W<'a> {
    w: &'a mut W,
}
impl<'a> TP_W<'a> {
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
#[doc = "Field `TL` reader - Transmit Enable Lock"]
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
#[doc = "Field `TL` writer - Transmit Enable Lock"]
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | ((value as u32 & 0x01) << 17);
        self.w
    }
}
#[doc = "Field `LE` reader - Disable transmitter when a link error occurs"]
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
#[doc = "Field `LE` writer - Disable transmitter when a link error occurs"]
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | ((value as u32 & 0x01) << 16);
        self.w
    }
}
#[doc = "Field `SP` reader - Strip PID"]
pub struct SP_R(crate::FieldReader<bool, bool>);
impl SP_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SP` writer - Strip PID"]
pub struct SP_W<'a> {
    w: &'a mut W,
}
impl<'a> SP_W<'a> {
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
#[doc = "Field `SA` reader - Strip Address"]
pub struct SA_R(crate::FieldReader<bool, bool>);
impl SA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SA` writer - Strip Address"]
pub struct SA_W<'a> {
    w: &'a mut W,
}
impl<'a> SA_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | ((value as u32 & 0x01) << 14);
        self.w
    }
}
#[doc = "Field `EN` reader - Enable Address"]
pub struct EN_R(crate::FieldReader<bool, bool>);
impl EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EN` writer - Enable Address"]
pub struct EN_W<'a> {
    w: &'a mut W,
}
impl<'a> EN_W<'a> {
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
#[doc = "Field `NS` reader - If cleared, packets will be discarded when a packet is arriving and there are no active descriptors. If set, the GRSPW will wait for a descriptor to be activated"]
pub struct NS_R(crate::FieldReader<bool, bool>);
impl NS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        NS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NS` writer - If cleared, packets will be discarded when a packet is arriving and there are no active descriptors. If set, the GRSPW will wait for a descriptor to be activated"]
pub struct NS_W<'a> {
    w: &'a mut W,
}
impl<'a> NS_W<'a> {
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
#[doc = "Field `RD` reader - Indicates to the GRSPW that there are enabled descriptors in the descriptor table"]
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
#[doc = "Field `RD` writer - Indicates to the GRSPW that there are enabled descriptors in the descriptor table"]
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | ((value as u32 & 0x01) << 11);
        self.w
    }
}
#[doc = "Field `RX` reader - Reception to the DMA channel is currently active"]
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
#[doc = "Field `AT` reader - Abort the currently transmitting packet and disable transmissions"]
pub struct AT_R(crate::FieldReader<bool, bool>);
impl AT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        AT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RA` reader - An error response was detected on the AHB bus - DMA receive"]
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
#[doc = "Field `RA` writer - An error response was detected on the AHB bus - DMA receive"]
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | ((value as u32 & 0x01) << 8);
        self.w
    }
}
#[doc = "Field `TA` reader - An error response was detected on the AHB bus - DMA transmit"]
pub struct TA_R(crate::FieldReader<bool, bool>);
impl TA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TA` writer - An error response was detected on the AHB bus - DMA transmit"]
pub struct TA_W<'a> {
    w: &'a mut W,
}
impl<'a> TA_W<'a> {
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
#[doc = "Field `PR` reader - Set each time a packet has been received"]
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
#[doc = "Field `PR` writer - Set each time a packet has been received"]
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | ((value as u32 & 0x01) << 6);
        self.w
    }
}
#[doc = "Field `PS` reader - Set each time a packet has been sent"]
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
#[doc = "Field `PS` writer - Set each time a packet has been sent"]
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u32 & 0x01) << 5);
        self.w
    }
}
#[doc = "Field `AI` reader - An interrupt will be generated each time an AHB error occurs when this DMA channel is accessing the bus"]
pub struct AI_R(crate::FieldReader<bool, bool>);
impl AI_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        AI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AI_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AI` writer - An interrupt will be generated each time an AHB error occurs when this DMA channel is accessing the bus"]
pub struct AI_W<'a> {
    w: &'a mut W,
}
impl<'a> AI_W<'a> {
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
#[doc = "Field `RI` reader - An interrupt will be generated each time a packet has been received"]
pub struct RI_R(crate::FieldReader<bool, bool>);
impl RI_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RI_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RI` writer - An interrupt will be generated each time a packet has been received"]
pub struct RI_W<'a> {
    w: &'a mut W,
}
impl<'a> RI_W<'a> {
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
#[doc = "Field `TI` reader - An interrupt will be generated each time a packet is transmitted"]
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
#[doc = "Field `TI` writer - An interrupt will be generated each time a packet is transmitted"]
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
        self.w
    }
}
#[doc = "Field `RE` reader - Packets are allowed to be received to this channel"]
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
#[doc = "Field `RE` writer - Packets are allowed to be received to this channel"]
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
#[doc = "Field `TE` reader - Write a one to this bit each time new descriptors are activated in the table"]
pub struct TE_R(crate::FieldReader<bool, bool>);
impl TE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TE` writer - Write a one to this bit each time new descriptors are activated in the table"]
pub struct TE_W<'a> {
    w: &'a mut W,
}
impl<'a> TE_W<'a> {
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
    #[doc = "Bits 26:31 - Interrupt number used for this channel"]
    #[inline(always)]
    pub fn intnum(&self) -> INTNUM_R {
        INTNUM_R::new(((self.bits >> 26) & 0x3f) as u8)
    }
    #[doc = "Bit 23 - EEP Termination"]
    #[inline(always)]
    pub fn ep(&self) -> EP_R {
        EP_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Truncated"]
    #[inline(always)]
    pub fn tr(&self) -> TR_R {
        TR_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Interrupt code transmit enable on EEP"]
    #[inline(always)]
    pub fn ie(&self) -> IE_R {
        IE_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Interrupt code transmit enable on truncation"]
    #[inline(always)]
    pub fn it(&self) -> IT_R {
        IT_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Receive Packet IRQ"]
    #[inline(always)]
    pub fn rp(&self) -> RP_R {
        RP_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Transmit Packet IRQ"]
    #[inline(always)]
    pub fn tp(&self) -> TP_R {
        TP_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Transmit Enable Lock"]
    #[inline(always)]
    pub fn tl(&self) -> TL_R {
        TL_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Disable transmitter when a link error occurs"]
    #[inline(always)]
    pub fn le(&self) -> LE_R {
        LE_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Strip PID"]
    #[inline(always)]
    pub fn sp(&self) -> SP_R {
        SP_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Strip Address"]
    #[inline(always)]
    pub fn sa(&self) -> SA_R {
        SA_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Enable Address"]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12 - If cleared, packets will be discarded when a packet is arriving and there are no active descriptors. If set, the GRSPW will wait for a descriptor to be activated"]
    #[inline(always)]
    pub fn ns(&self) -> NS_R {
        NS_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Indicates to the GRSPW that there are enabled descriptors in the descriptor table"]
    #[inline(always)]
    pub fn rd(&self) -> RD_R {
        RD_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Reception to the DMA channel is currently active"]
    #[inline(always)]
    pub fn rx(&self) -> RX_R {
        RX_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Abort the currently transmitting packet and disable transmissions"]
    #[inline(always)]
    pub fn at(&self) -> AT_R {
        AT_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - An error response was detected on the AHB bus - DMA receive"]
    #[inline(always)]
    pub fn ra(&self) -> RA_R {
        RA_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7 - An error response was detected on the AHB bus - DMA transmit"]
    #[inline(always)]
    pub fn ta(&self) -> TA_R {
        TA_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Set each time a packet has been received"]
    #[inline(always)]
    pub fn pr(&self) -> PR_R {
        PR_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Set each time a packet has been sent"]
    #[inline(always)]
    pub fn ps(&self) -> PS_R {
        PS_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - An interrupt will be generated each time an AHB error occurs when this DMA channel is accessing the bus"]
    #[inline(always)]
    pub fn ai(&self) -> AI_R {
        AI_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - An interrupt will be generated each time a packet has been received"]
    #[inline(always)]
    pub fn ri(&self) -> RI_R {
        RI_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - An interrupt will be generated each time a packet is transmitted"]
    #[inline(always)]
    pub fn ti(&self) -> TI_R {
        TI_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - Packets are allowed to be received to this channel"]
    #[inline(always)]
    pub fn re(&self) -> RE_R {
        RE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Write a one to this bit each time new descriptors are activated in the table"]
    #[inline(always)]
    pub fn te(&self) -> TE_R {
        TE_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 26:31 - Interrupt number used for this channel"]
    #[inline(always)]
    pub fn intnum(&mut self) -> INTNUM_W {
        INTNUM_W { w: self }
    }
    #[doc = "Bit 23 - EEP Termination"]
    #[inline(always)]
    pub fn ep(&mut self) -> EP_W {
        EP_W { w: self }
    }
    #[doc = "Bit 22 - Truncated"]
    #[inline(always)]
    pub fn tr(&mut self) -> TR_W {
        TR_W { w: self }
    }
    #[doc = "Bit 21 - Interrupt code transmit enable on EEP"]
    #[inline(always)]
    pub fn ie(&mut self) -> IE_W {
        IE_W { w: self }
    }
    #[doc = "Bit 20 - Interrupt code transmit enable on truncation"]
    #[inline(always)]
    pub fn it(&mut self) -> IT_W {
        IT_W { w: self }
    }
    #[doc = "Bit 19 - Receive Packet IRQ"]
    #[inline(always)]
    pub fn rp(&mut self) -> RP_W {
        RP_W { w: self }
    }
    #[doc = "Bit 18 - Transmit Packet IRQ"]
    #[inline(always)]
    pub fn tp(&mut self) -> TP_W {
        TP_W { w: self }
    }
    #[doc = "Bit 17 - Transmit Enable Lock"]
    #[inline(always)]
    pub fn tl(&mut self) -> TL_W {
        TL_W { w: self }
    }
    #[doc = "Bit 16 - Disable transmitter when a link error occurs"]
    #[inline(always)]
    pub fn le(&mut self) -> LE_W {
        LE_W { w: self }
    }
    #[doc = "Bit 15 - Strip PID"]
    #[inline(always)]
    pub fn sp(&mut self) -> SP_W {
        SP_W { w: self }
    }
    #[doc = "Bit 14 - Strip Address"]
    #[inline(always)]
    pub fn sa(&mut self) -> SA_W {
        SA_W { w: self }
    }
    #[doc = "Bit 13 - Enable Address"]
    #[inline(always)]
    pub fn en(&mut self) -> EN_W {
        EN_W { w: self }
    }
    #[doc = "Bit 12 - If cleared, packets will be discarded when a packet is arriving and there are no active descriptors. If set, the GRSPW will wait for a descriptor to be activated"]
    #[inline(always)]
    pub fn ns(&mut self) -> NS_W {
        NS_W { w: self }
    }
    #[doc = "Bit 11 - Indicates to the GRSPW that there are enabled descriptors in the descriptor table"]
    #[inline(always)]
    pub fn rd(&mut self) -> RD_W {
        RD_W { w: self }
    }
    #[doc = "Bit 8 - An error response was detected on the AHB bus - DMA receive"]
    #[inline(always)]
    pub fn ra(&mut self) -> RA_W {
        RA_W { w: self }
    }
    #[doc = "Bit 7 - An error response was detected on the AHB bus - DMA transmit"]
    #[inline(always)]
    pub fn ta(&mut self) -> TA_W {
        TA_W { w: self }
    }
    #[doc = "Bit 6 - Set each time a packet has been received"]
    #[inline(always)]
    pub fn pr(&mut self) -> PR_W {
        PR_W { w: self }
    }
    #[doc = "Bit 5 - Set each time a packet has been sent"]
    #[inline(always)]
    pub fn ps(&mut self) -> PS_W {
        PS_W { w: self }
    }
    #[doc = "Bit 4 - An interrupt will be generated each time an AHB error occurs when this DMA channel is accessing the bus"]
    #[inline(always)]
    pub fn ai(&mut self) -> AI_W {
        AI_W { w: self }
    }
    #[doc = "Bit 3 - An interrupt will be generated each time a packet has been received"]
    #[inline(always)]
    pub fn ri(&mut self) -> RI_W {
        RI_W { w: self }
    }
    #[doc = "Bit 2 - An interrupt will be generated each time a packet is transmitted"]
    #[inline(always)]
    pub fn ti(&mut self) -> TI_W {
        TI_W { w: self }
    }
    #[doc = "Bit 1 - Packets are allowed to be received to this channel"]
    #[inline(always)]
    pub fn re(&mut self) -> RE_W {
        RE_W { w: self }
    }
    #[doc = "Bit 0 - Write a one to this bit each time new descriptors are activated in the table"]
    #[inline(always)]
    pub fn te(&mut self) -> TE_W {
        TE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DMA Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmactrl0](index.html) module"]
pub struct DMACTRL0_SPEC;
impl crate::RegisterSpec for DMACTRL0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dmactrl0::R](R) reader structure"]
impl crate::Readable for DMACTRL0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dmactrl0::W](W) writer structure"]
impl crate::Writable for DMACTRL0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DMACTRL0 to value 0"]
impl crate::Resettable for DMACTRL0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
