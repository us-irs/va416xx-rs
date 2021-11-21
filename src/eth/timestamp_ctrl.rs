#[doc = "Register `TIMESTAMP_CTRL` reader"]
pub struct R(crate::R<TIMESTAMP_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TIMESTAMP_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TIMESTAMP_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TIMESTAMP_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TIMESTAMP_CTRL` writer"]
pub struct W(crate::W<TIMESTAMP_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TIMESTAMP_CTRL_SPEC>;
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
impl From<crate::W<TIMESTAMP_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TIMESTAMP_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ATSEN3` reader - Auxiliary Snapshot 3 Enable"]
pub struct ATSEN3_R(crate::FieldReader<bool, bool>);
impl ATSEN3_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ATSEN3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ATSEN3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ATSEN3` writer - Auxiliary Snapshot 3 Enable"]
pub struct ATSEN3_W<'a> {
    w: &'a mut W,
}
impl<'a> ATSEN3_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 28)) | ((value as u32 & 0x01) << 28);
        self.w
    }
}
#[doc = "Field `ATSEN2` reader - Auxiliary Snapshot 2 Enable"]
pub struct ATSEN2_R(crate::FieldReader<bool, bool>);
impl ATSEN2_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ATSEN2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ATSEN2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ATSEN2` writer - Auxiliary Snapshot 2 Enable"]
pub struct ATSEN2_W<'a> {
    w: &'a mut W,
}
impl<'a> ATSEN2_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 27)) | ((value as u32 & 0x01) << 27);
        self.w
    }
}
#[doc = "Field `ATSEN1` reader - Auxiliary Snapshot 1 Enable"]
pub struct ATSEN1_R(crate::FieldReader<bool, bool>);
impl ATSEN1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ATSEN1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ATSEN1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ATSEN1` writer - Auxiliary Snapshot 1 Enable"]
pub struct ATSEN1_W<'a> {
    w: &'a mut W,
}
impl<'a> ATSEN1_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 26)) | ((value as u32 & 0x01) << 26);
        self.w
    }
}
#[doc = "Field `ATSEN0` reader - Auxiliary Snapshot 0 Enable"]
pub struct ATSEN0_R(crate::FieldReader<bool, bool>);
impl ATSEN0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ATSEN0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ATSEN0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ATSEN0` writer - Auxiliary Snapshot 0 Enable"]
pub struct ATSEN0_W<'a> {
    w: &'a mut W,
}
impl<'a> ATSEN0_W<'a> {
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
#[doc = "Field `ATSFC` reader - Auxiliary Snapshot FIFO Clear"]
pub struct ATSFC_R(crate::FieldReader<bool, bool>);
impl ATSFC_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ATSFC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ATSFC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ATSFC` writer - Auxiliary Snapshot FIFO Clear"]
pub struct ATSFC_W<'a> {
    w: &'a mut W,
}
impl<'a> ATSFC_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | ((value as u32 & 0x01) << 24);
        self.w
    }
}
#[doc = "Field `TSENMACADDR` reader - Enable MAC address for PTP Frame Filtering"]
pub struct TSENMACADDR_R(crate::FieldReader<bool, bool>);
impl TSENMACADDR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TSENMACADDR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TSENMACADDR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TSENMACADDR` writer - Enable MAC address for PTP Frame Filtering"]
pub struct TSENMACADDR_W<'a> {
    w: &'a mut W,
}
impl<'a> TSENMACADDR_W<'a> {
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
#[doc = "Field `SNAPTYPSEL` reader - Select PTP packets for Taking Snapshots"]
pub struct SNAPTYPSEL_R(crate::FieldReader<u8, u8>);
impl SNAPTYPSEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SNAPTYPSEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SNAPTYPSEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SNAPTYPSEL` writer - Select PTP packets for Taking Snapshots"]
pub struct SNAPTYPSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> SNAPTYPSEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | ((value as u32 & 0x03) << 16);
        self.w
    }
}
#[doc = "Field `TSMSTRENA` reader - Enable Snapshot for Messages Relevant to Master"]
pub struct TSMSTRENA_R(crate::FieldReader<bool, bool>);
impl TSMSTRENA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TSMSTRENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TSMSTRENA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TSMSTRENA` writer - Enable Snapshot for Messages Relevant to Master"]
pub struct TSMSTRENA_W<'a> {
    w: &'a mut W,
}
impl<'a> TSMSTRENA_W<'a> {
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
#[doc = "Field `TSEVNTENA` reader - Enable Timestamp Snapshot for Event Messages"]
pub struct TSEVNTENA_R(crate::FieldReader<bool, bool>);
impl TSEVNTENA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TSEVNTENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TSEVNTENA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TSEVNTENA` writer - Enable Timestamp Snapshot for Event Messages"]
pub struct TSEVNTENA_W<'a> {
    w: &'a mut W,
}
impl<'a> TSEVNTENA_W<'a> {
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
#[doc = "Field `TSIPV4ENA` reader - Enable Processing of PTP Frames Sent over IPv4-UDP"]
pub struct TSIPV4ENA_R(crate::FieldReader<bool, bool>);
impl TSIPV4ENA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TSIPV4ENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TSIPV4ENA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TSIPV4ENA` writer - Enable Processing of PTP Frames Sent over IPv4-UDP"]
pub struct TSIPV4ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> TSIPV4ENA_W<'a> {
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
#[doc = "Field `TSIPV6ENA` reader - Enable Processing of PTP Frames Sent over IPv6-UDP"]
pub struct TSIPV6ENA_R(crate::FieldReader<bool, bool>);
impl TSIPV6ENA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TSIPV6ENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TSIPV6ENA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TSIPV6ENA` writer - Enable Processing of PTP Frames Sent over IPv6-UDP"]
pub struct TSIPV6ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> TSIPV6ENA_W<'a> {
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
#[doc = "Field `TSIPENA` reader - Enable Processing of PTP over Ethernet Frames"]
pub struct TSIPENA_R(crate::FieldReader<bool, bool>);
impl TSIPENA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TSIPENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TSIPENA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TSIPENA` writer - Enable Processing of PTP over Ethernet Frames"]
pub struct TSIPENA_W<'a> {
    w: &'a mut W,
}
impl<'a> TSIPENA_W<'a> {
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
#[doc = "Field `TSVER2ENA` reader - Enable PTP packet Processing for Version 2 Format"]
pub struct TSVER2ENA_R(crate::FieldReader<bool, bool>);
impl TSVER2ENA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TSVER2ENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TSVER2ENA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TSVER2ENA` writer - Enable PTP packet Processing for Version 2 Format"]
pub struct TSVER2ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> TSVER2ENA_W<'a> {
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
#[doc = "Field `TSCTRLSSR` reader - Timestamp Digital or Binary Rollover Control"]
pub struct TSCTRLSSR_R(crate::FieldReader<bool, bool>);
impl TSCTRLSSR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TSCTRLSSR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TSCTRLSSR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TSCTRLSSR` writer - Timestamp Digital or Binary Rollover Control"]
pub struct TSCTRLSSR_W<'a> {
    w: &'a mut W,
}
impl<'a> TSCTRLSSR_W<'a> {
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
#[doc = "Field `TSENALL` reader - Enable Timestamp for All Frames"]
pub struct TSENALL_R(crate::FieldReader<bool, bool>);
impl TSENALL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TSENALL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TSENALL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TSENALL` writer - Enable Timestamp for All Frames"]
pub struct TSENALL_W<'a> {
    w: &'a mut W,
}
impl<'a> TSENALL_W<'a> {
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
#[doc = "Field `TSADDRREG` reader - Addend Reg Update"]
pub struct TSADDRREG_R(crate::FieldReader<bool, bool>);
impl TSADDRREG_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TSADDRREG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TSADDRREG_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TSADDRREG` writer - Addend Reg Update"]
pub struct TSADDRREG_W<'a> {
    w: &'a mut W,
}
impl<'a> TSADDRREG_W<'a> {
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
#[doc = "Field `TSTRIG` reader - Timestamp Interrupt Trigger Enable"]
pub struct TSTRIG_R(crate::FieldReader<bool, bool>);
impl TSTRIG_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TSTRIG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TSTRIG_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TSTRIG` writer - Timestamp Interrupt Trigger Enable"]
pub struct TSTRIG_W<'a> {
    w: &'a mut W,
}
impl<'a> TSTRIG_W<'a> {
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
#[doc = "Field `TSUPDT` reader - Timestamp Update"]
pub struct TSUPDT_R(crate::FieldReader<bool, bool>);
impl TSUPDT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TSUPDT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TSUPDT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TSUPDT` writer - Timestamp Update"]
pub struct TSUPDT_W<'a> {
    w: &'a mut W,
}
impl<'a> TSUPDT_W<'a> {
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
#[doc = "Field `TSINIT` reader - Timestamp Initialize"]
pub struct TSINIT_R(crate::FieldReader<bool, bool>);
impl TSINIT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TSINIT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TSINIT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TSINIT` writer - Timestamp Initialize"]
pub struct TSINIT_W<'a> {
    w: &'a mut W,
}
impl<'a> TSINIT_W<'a> {
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
#[doc = "Field `TSCFUPDT` reader - Timestamp Fine or Coarse Update"]
pub struct TSCFUPDT_R(crate::FieldReader<bool, bool>);
impl TSCFUPDT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TSCFUPDT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TSCFUPDT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TSCFUPDT` writer - Timestamp Fine or Coarse Update"]
pub struct TSCFUPDT_W<'a> {
    w: &'a mut W,
}
impl<'a> TSCFUPDT_W<'a> {
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
#[doc = "Field `TSENA` reader - Timestamp Enable"]
pub struct TSENA_R(crate::FieldReader<bool, bool>);
impl TSENA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TSENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TSENA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TSENA` writer - Timestamp Enable"]
pub struct TSENA_W<'a> {
    w: &'a mut W,
}
impl<'a> TSENA_W<'a> {
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
    #[doc = "Bit 28 - Auxiliary Snapshot 3 Enable"]
    #[inline(always)]
    pub fn atsen3(&self) -> ATSEN3_R {
        ATSEN3_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 27 - Auxiliary Snapshot 2 Enable"]
    #[inline(always)]
    pub fn atsen2(&self) -> ATSEN2_R {
        ATSEN2_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Auxiliary Snapshot 1 Enable"]
    #[inline(always)]
    pub fn atsen1(&self) -> ATSEN1_R {
        ATSEN1_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Auxiliary Snapshot 0 Enable"]
    #[inline(always)]
    pub fn atsen0(&self) -> ATSEN0_R {
        ATSEN0_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Auxiliary Snapshot FIFO Clear"]
    #[inline(always)]
    pub fn atsfc(&self) -> ATSFC_R {
        ATSFC_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Enable MAC address for PTP Frame Filtering"]
    #[inline(always)]
    pub fn tsenmacaddr(&self) -> TSENMACADDR_R {
        TSENMACADDR_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bits 16:17 - Select PTP packets for Taking Snapshots"]
    #[inline(always)]
    pub fn snaptypsel(&self) -> SNAPTYPSEL_R {
        SNAPTYPSEL_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bit 15 - Enable Snapshot for Messages Relevant to Master"]
    #[inline(always)]
    pub fn tsmstrena(&self) -> TSMSTRENA_R {
        TSMSTRENA_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Enable Timestamp Snapshot for Event Messages"]
    #[inline(always)]
    pub fn tsevntena(&self) -> TSEVNTENA_R {
        TSEVNTENA_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Enable Processing of PTP Frames Sent over IPv4-UDP"]
    #[inline(always)]
    pub fn tsipv4ena(&self) -> TSIPV4ENA_R {
        TSIPV4ENA_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Enable Processing of PTP Frames Sent over IPv6-UDP"]
    #[inline(always)]
    pub fn tsipv6ena(&self) -> TSIPV6ENA_R {
        TSIPV6ENA_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Enable Processing of PTP over Ethernet Frames"]
    #[inline(always)]
    pub fn tsipena(&self) -> TSIPENA_R {
        TSIPENA_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Enable PTP packet Processing for Version 2 Format"]
    #[inline(always)]
    pub fn tsver2ena(&self) -> TSVER2ENA_R {
        TSVER2ENA_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Timestamp Digital or Binary Rollover Control"]
    #[inline(always)]
    pub fn tsctrlssr(&self) -> TSCTRLSSR_R {
        TSCTRLSSR_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Enable Timestamp for All Frames"]
    #[inline(always)]
    pub fn tsenall(&self) -> TSENALL_R {
        TSENALL_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Addend Reg Update"]
    #[inline(always)]
    pub fn tsaddrreg(&self) -> TSADDRREG_R {
        TSADDRREG_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Timestamp Interrupt Trigger Enable"]
    #[inline(always)]
    pub fn tstrig(&self) -> TSTRIG_R {
        TSTRIG_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Timestamp Update"]
    #[inline(always)]
    pub fn tsupdt(&self) -> TSUPDT_R {
        TSUPDT_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Timestamp Initialize"]
    #[inline(always)]
    pub fn tsinit(&self) -> TSINIT_R {
        TSINIT_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - Timestamp Fine or Coarse Update"]
    #[inline(always)]
    pub fn tscfupdt(&self) -> TSCFUPDT_R {
        TSCFUPDT_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Timestamp Enable"]
    #[inline(always)]
    pub fn tsena(&self) -> TSENA_R {
        TSENA_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 28 - Auxiliary Snapshot 3 Enable"]
    #[inline(always)]
    pub fn atsen3(&mut self) -> ATSEN3_W {
        ATSEN3_W { w: self }
    }
    #[doc = "Bit 27 - Auxiliary Snapshot 2 Enable"]
    #[inline(always)]
    pub fn atsen2(&mut self) -> ATSEN2_W {
        ATSEN2_W { w: self }
    }
    #[doc = "Bit 26 - Auxiliary Snapshot 1 Enable"]
    #[inline(always)]
    pub fn atsen1(&mut self) -> ATSEN1_W {
        ATSEN1_W { w: self }
    }
    #[doc = "Bit 25 - Auxiliary Snapshot 0 Enable"]
    #[inline(always)]
    pub fn atsen0(&mut self) -> ATSEN0_W {
        ATSEN0_W { w: self }
    }
    #[doc = "Bit 24 - Auxiliary Snapshot FIFO Clear"]
    #[inline(always)]
    pub fn atsfc(&mut self) -> ATSFC_W {
        ATSFC_W { w: self }
    }
    #[doc = "Bit 18 - Enable MAC address for PTP Frame Filtering"]
    #[inline(always)]
    pub fn tsenmacaddr(&mut self) -> TSENMACADDR_W {
        TSENMACADDR_W { w: self }
    }
    #[doc = "Bits 16:17 - Select PTP packets for Taking Snapshots"]
    #[inline(always)]
    pub fn snaptypsel(&mut self) -> SNAPTYPSEL_W {
        SNAPTYPSEL_W { w: self }
    }
    #[doc = "Bit 15 - Enable Snapshot for Messages Relevant to Master"]
    #[inline(always)]
    pub fn tsmstrena(&mut self) -> TSMSTRENA_W {
        TSMSTRENA_W { w: self }
    }
    #[doc = "Bit 14 - Enable Timestamp Snapshot for Event Messages"]
    #[inline(always)]
    pub fn tsevntena(&mut self) -> TSEVNTENA_W {
        TSEVNTENA_W { w: self }
    }
    #[doc = "Bit 13 - Enable Processing of PTP Frames Sent over IPv4-UDP"]
    #[inline(always)]
    pub fn tsipv4ena(&mut self) -> TSIPV4ENA_W {
        TSIPV4ENA_W { w: self }
    }
    #[doc = "Bit 12 - Enable Processing of PTP Frames Sent over IPv6-UDP"]
    #[inline(always)]
    pub fn tsipv6ena(&mut self) -> TSIPV6ENA_W {
        TSIPV6ENA_W { w: self }
    }
    #[doc = "Bit 11 - Enable Processing of PTP over Ethernet Frames"]
    #[inline(always)]
    pub fn tsipena(&mut self) -> TSIPENA_W {
        TSIPENA_W { w: self }
    }
    #[doc = "Bit 10 - Enable PTP packet Processing for Version 2 Format"]
    #[inline(always)]
    pub fn tsver2ena(&mut self) -> TSVER2ENA_W {
        TSVER2ENA_W { w: self }
    }
    #[doc = "Bit 9 - Timestamp Digital or Binary Rollover Control"]
    #[inline(always)]
    pub fn tsctrlssr(&mut self) -> TSCTRLSSR_W {
        TSCTRLSSR_W { w: self }
    }
    #[doc = "Bit 8 - Enable Timestamp for All Frames"]
    #[inline(always)]
    pub fn tsenall(&mut self) -> TSENALL_W {
        TSENALL_W { w: self }
    }
    #[doc = "Bit 5 - Addend Reg Update"]
    #[inline(always)]
    pub fn tsaddrreg(&mut self) -> TSADDRREG_W {
        TSADDRREG_W { w: self }
    }
    #[doc = "Bit 4 - Timestamp Interrupt Trigger Enable"]
    #[inline(always)]
    pub fn tstrig(&mut self) -> TSTRIG_W {
        TSTRIG_W { w: self }
    }
    #[doc = "Bit 3 - Timestamp Update"]
    #[inline(always)]
    pub fn tsupdt(&mut self) -> TSUPDT_W {
        TSUPDT_W { w: self }
    }
    #[doc = "Bit 2 - Timestamp Initialize"]
    #[inline(always)]
    pub fn tsinit(&mut self) -> TSINIT_W {
        TSINIT_W { w: self }
    }
    #[doc = "Bit 1 - Timestamp Fine or Coarse Update"]
    #[inline(always)]
    pub fn tscfupdt(&mut self) -> TSCFUPDT_W {
        TSCFUPDT_W { w: self }
    }
    #[doc = "Bit 0 - Timestamp Enable"]
    #[inline(always)]
    pub fn tsena(&mut self) -> TSENA_W {
        TSENA_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Controls the IEEE 1588 timestamp generation and update logic\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [timestamp_ctrl](index.html) module"]
pub struct TIMESTAMP_CTRL_SPEC;
impl crate::RegisterSpec for TIMESTAMP_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [timestamp_ctrl::R](R) reader structure"]
impl crate::Readable for TIMESTAMP_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [timestamp_ctrl::W](W) writer structure"]
impl crate::Writable for TIMESTAMP_CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TIMESTAMP_CTRL to value 0"]
impl crate::Resettable for TIMESTAMP_CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
