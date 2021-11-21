#[doc = "Register `MMC_INTR_RX` reader"]
pub struct R(crate::R<MMC_INTR_RX_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MMC_INTR_RX_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MMC_INTR_RX_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MMC_INTR_RX_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MMC_INTR_RX` writer"]
pub struct W(crate::W<MMC_INTR_RX_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MMC_INTR_RX_SPEC>;
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
impl From<crate::W<MMC_INTR_RX_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MMC_INTR_RX_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RXCTRLFIS` reader - MMC Receive Control Frame Counter Interrupt Status"]
pub struct RXCTRLFIS_R(crate::FieldReader<bool, bool>);
impl RXCTRLFIS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RXCTRLFIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXCTRLFIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXCTRLFIS` writer - MMC Receive Control Frame Counter Interrupt Status"]
pub struct RXCTRLFIS_W<'a> {
    w: &'a mut W,
}
impl<'a> RXCTRLFIS_W<'a> {
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
#[doc = "Field `RXRCVERRFIS` reader - MMC Receive Error Frame Counter Interrupt Status"]
pub struct RXRCVERRFIS_R(crate::FieldReader<bool, bool>);
impl RXRCVERRFIS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RXRCVERRFIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXRCVERRFIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXRCVERRFIS` writer - MMC Receive Error Frame Counter Interrupt Status"]
pub struct RXRCVERRFIS_W<'a> {
    w: &'a mut W,
}
impl<'a> RXRCVERRFIS_W<'a> {
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
#[doc = "Field `RXWDOGFIS` reader - MMC Receive Watchdog Error Frame Counter Interrupt Status"]
pub struct RXWDOGFIS_R(crate::FieldReader<bool, bool>);
impl RXWDOGFIS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RXWDOGFIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXWDOGFIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXWDOGFIS` writer - MMC Receive Watchdog Error Frame Counter Interrupt Status"]
pub struct RXWDOGFIS_W<'a> {
    w: &'a mut W,
}
impl<'a> RXWDOGFIS_W<'a> {
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
#[doc = "Field `RXVLANGBFIS` reader - MMC Receive VLAN Good Bad Frame Counter Interrupt Status"]
pub struct RXVLANGBFIS_R(crate::FieldReader<bool, bool>);
impl RXVLANGBFIS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RXVLANGBFIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXVLANGBFIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXVLANGBFIS` writer - MMC Receive VLAN Good Bad Frame Counter Interrupt Status"]
pub struct RXVLANGBFIS_W<'a> {
    w: &'a mut W,
}
impl<'a> RXVLANGBFIS_W<'a> {
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
#[doc = "Field `RXFOVFIS` reader - MMC Receive FIFO Overflow Frame Counter Interrupt Status"]
pub struct RXFOVFIS_R(crate::FieldReader<bool, bool>);
impl RXFOVFIS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RXFOVFIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXFOVFIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXFOVFIS` writer - MMC Receive FIFO Overflow Frame Counter Interrupt Status"]
pub struct RXFOVFIS_W<'a> {
    w: &'a mut W,
}
impl<'a> RXFOVFIS_W<'a> {
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
#[doc = "Field `RXPAUSFIS` reader - MMC Receive Pause Frame Counter Interrupt Status"]
pub struct RXPAUSFIS_R(crate::FieldReader<bool, bool>);
impl RXPAUSFIS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RXPAUSFIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXPAUSFIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXPAUSFIS` writer - MMC Receive Pause Frame Counter Interrupt Status"]
pub struct RXPAUSFIS_W<'a> {
    w: &'a mut W,
}
impl<'a> RXPAUSFIS_W<'a> {
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
#[doc = "Field `RXORANGEFIS` reader - MMC Receive Out Of Range Error Frame Counter Interrupt Status."]
pub struct RXORANGEFIS_R(crate::FieldReader<bool, bool>);
impl RXORANGEFIS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RXORANGEFIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXORANGEFIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXORANGEFIS` writer - MMC Receive Out Of Range Error Frame Counter Interrupt Status."]
pub struct RXORANGEFIS_W<'a> {
    w: &'a mut W,
}
impl<'a> RXORANGEFIS_W<'a> {
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
#[doc = "Field `RXLENERFIS` reader - MMC Receive Length Error Frame Counter Interrupt Status"]
pub struct RXLENERFIS_R(crate::FieldReader<bool, bool>);
impl RXLENERFIS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RXLENERFIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXLENERFIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXLENERFIS` writer - MMC Receive Length Error Frame Counter Interrupt Status"]
pub struct RXLENERFIS_W<'a> {
    w: &'a mut W,
}
impl<'a> RXLENERFIS_W<'a> {
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
#[doc = "Field `RXUCGFIS` reader - MMC Receive Unicast Good Frame Counter Interrupt Status"]
pub struct RXUCGFIS_R(crate::FieldReader<bool, bool>);
impl RXUCGFIS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RXUCGFIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXUCGFIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXUCGFIS` writer - MMC Receive Unicast Good Frame Counter Interrupt Status"]
pub struct RXUCGFIS_W<'a> {
    w: &'a mut W,
}
impl<'a> RXUCGFIS_W<'a> {
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
#[doc = "Field `RX1024TMAXOCTGBFIS` reader - MMC Receive 1024 to Maximum Octet Good Bad Frame Counter Interrupt Status."]
pub struct RX1024TMAXOCTGBFIS_R(crate::FieldReader<bool, bool>);
impl RX1024TMAXOCTGBFIS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RX1024TMAXOCTGBFIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RX1024TMAXOCTGBFIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RX1024TMAXOCTGBFIS` writer - MMC Receive 1024 to Maximum Octet Good Bad Frame Counter Interrupt Status."]
pub struct RX1024TMAXOCTGBFIS_W<'a> {
    w: &'a mut W,
}
impl<'a> RX1024TMAXOCTGBFIS_W<'a> {
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
#[doc = "Field `RX512T1023OCTGBFIS` reader - MMC Receive 512 to 1023 Octet Good Bad Frame Counter Interrupt Status"]
pub struct RX512T1023OCTGBFIS_R(crate::FieldReader<bool, bool>);
impl RX512T1023OCTGBFIS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RX512T1023OCTGBFIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RX512T1023OCTGBFIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RX512T1023OCTGBFIS` writer - MMC Receive 512 to 1023 Octet Good Bad Frame Counter Interrupt Status"]
pub struct RX512T1023OCTGBFIS_W<'a> {
    w: &'a mut W,
}
impl<'a> RX512T1023OCTGBFIS_W<'a> {
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
#[doc = "Field `RX256T511OCTGBFIS` reader - MMC Receive 256 to 511 Octet Good Bad Frame Counter Interrupt Status"]
pub struct RX256T511OCTGBFIS_R(crate::FieldReader<bool, bool>);
impl RX256T511OCTGBFIS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RX256T511OCTGBFIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RX256T511OCTGBFIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RX256T511OCTGBFIS` writer - MMC Receive 256 to 511 Octet Good Bad Frame Counter Interrupt Status"]
pub struct RX256T511OCTGBFIS_W<'a> {
    w: &'a mut W,
}
impl<'a> RX256T511OCTGBFIS_W<'a> {
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
#[doc = "Field `RX128T255OCTGBFIS` reader - MMC Receive 128 to 255 Octet Good Bad Frame Counter Interrupt Status"]
pub struct RX128T255OCTGBFIS_R(crate::FieldReader<bool, bool>);
impl RX128T255OCTGBFIS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RX128T255OCTGBFIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RX128T255OCTGBFIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RX128T255OCTGBFIS` writer - MMC Receive 128 to 255 Octet Good Bad Frame Counter Interrupt Status"]
pub struct RX128T255OCTGBFIS_W<'a> {
    w: &'a mut W,
}
impl<'a> RX128T255OCTGBFIS_W<'a> {
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
#[doc = "Field `RX65T127OCTGBFIS` reader - MMC Receive 65 to 127 Octet Good Bad Frame Counter Interrupt Status"]
pub struct RX65T127OCTGBFIS_R(crate::FieldReader<bool, bool>);
impl RX65T127OCTGBFIS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RX65T127OCTGBFIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RX65T127OCTGBFIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RX65T127OCTGBFIS` writer - MMC Receive 65 to 127 Octet Good Bad Frame Counter Interrupt Status"]
pub struct RX65T127OCTGBFIS_W<'a> {
    w: &'a mut W,
}
impl<'a> RX65T127OCTGBFIS_W<'a> {
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
#[doc = "Field `RX64OCTGBFIS` reader - MMC Receive 64 Octet Good Bad Frame Counter Interrupt Status"]
pub struct RX64OCTGBFIS_R(crate::FieldReader<bool, bool>);
impl RX64OCTGBFIS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RX64OCTGBFIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RX64OCTGBFIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RX64OCTGBFIS` writer - MMC Receive 64 Octet Good Bad Frame Counter Interrupt Status"]
pub struct RX64OCTGBFIS_W<'a> {
    w: &'a mut W,
}
impl<'a> RX64OCTGBFIS_W<'a> {
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
#[doc = "Field `RXOSIZEGFIS` reader - MMC Receive Oversize Good Frame Counter Interrupt Status"]
pub struct RXOSIZEGFIS_R(crate::FieldReader<bool, bool>);
impl RXOSIZEGFIS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RXOSIZEGFIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXOSIZEGFIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXOSIZEGFIS` writer - MMC Receive Oversize Good Frame Counter Interrupt Status"]
pub struct RXOSIZEGFIS_W<'a> {
    w: &'a mut W,
}
impl<'a> RXOSIZEGFIS_W<'a> {
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
#[doc = "Field `RXUSIZEGFIS` reader - MMC Receive Undersize Good Frame Counter Interrupt Status"]
pub struct RXUSIZEGFIS_R(crate::FieldReader<bool, bool>);
impl RXUSIZEGFIS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RXUSIZEGFIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXUSIZEGFIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXUSIZEGFIS` writer - MMC Receive Undersize Good Frame Counter Interrupt Status"]
pub struct RXUSIZEGFIS_W<'a> {
    w: &'a mut W,
}
impl<'a> RXUSIZEGFIS_W<'a> {
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
#[doc = "Field `RXJABERFIS` reader - MMC Receive Jabber Error Frame Counter Interrupt Status"]
pub struct RXJABERFIS_R(crate::FieldReader<bool, bool>);
impl RXJABERFIS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RXJABERFIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXJABERFIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXJABERFIS` writer - MMC Receive Jabber Error Frame Counter Interrupt Status"]
pub struct RXJABERFIS_W<'a> {
    w: &'a mut W,
}
impl<'a> RXJABERFIS_W<'a> {
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
#[doc = "Field `RXRUNTFIS` reader - MMC Receive Runt Frame Counter Interrupt Status"]
pub struct RXRUNTFIS_R(crate::FieldReader<bool, bool>);
impl RXRUNTFIS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RXRUNTFIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXRUNTFIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXRUNTFIS` writer - MMC Receive Runt Frame Counter Interrupt Status"]
pub struct RXRUNTFIS_W<'a> {
    w: &'a mut W,
}
impl<'a> RXRUNTFIS_W<'a> {
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
#[doc = "Field `RXALGNERFIS` reader - MMC Receive Alignment Error Frame Counter Interrupt Status"]
pub struct RXALGNERFIS_R(crate::FieldReader<bool, bool>);
impl RXALGNERFIS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RXALGNERFIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXALGNERFIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXALGNERFIS` writer - MMC Receive Alignment Error Frame Counter Interrupt Status"]
pub struct RXALGNERFIS_W<'a> {
    w: &'a mut W,
}
impl<'a> RXALGNERFIS_W<'a> {
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
#[doc = "Field `RXCRCERFIS` reader - MMC Receive CRC Error Frame Counter Interrupt Status"]
pub struct RXCRCERFIS_R(crate::FieldReader<bool, bool>);
impl RXCRCERFIS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RXCRCERFIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXCRCERFIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXCRCERFIS` writer - MMC Receive CRC Error Frame Counter Interrupt Status"]
pub struct RXCRCERFIS_W<'a> {
    w: &'a mut W,
}
impl<'a> RXCRCERFIS_W<'a> {
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
#[doc = "Field `RXMCGFIS` reader - MMC Receive Multicast Good Frame Counter Interrupt Status"]
pub struct RXMCGFIS_R(crate::FieldReader<bool, bool>);
impl RXMCGFIS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RXMCGFIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXMCGFIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXMCGFIS` writer - MMC Receive Multicast Good Frame Counter Interrupt Status"]
pub struct RXMCGFIS_W<'a> {
    w: &'a mut W,
}
impl<'a> RXMCGFIS_W<'a> {
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
#[doc = "Field `RXBCGFIS` reader - MMC Receive Broadcast Good Frame Counter Interrupt Status"]
pub struct RXBCGFIS_R(crate::FieldReader<bool, bool>);
impl RXBCGFIS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RXBCGFIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXBCGFIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXBCGFIS` writer - MMC Receive Broadcast Good Frame Counter Interrupt Status"]
pub struct RXBCGFIS_W<'a> {
    w: &'a mut W,
}
impl<'a> RXBCGFIS_W<'a> {
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
#[doc = "Field `RXGOCTIS` reader - MMC Receive Good Octet Counter Interrupt Status"]
pub struct RXGOCTIS_R(crate::FieldReader<bool, bool>);
impl RXGOCTIS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RXGOCTIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXGOCTIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXGOCTIS` writer - MMC Receive Good Octet Counter Interrupt Status"]
pub struct RXGOCTIS_W<'a> {
    w: &'a mut W,
}
impl<'a> RXGOCTIS_W<'a> {
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
#[doc = "Field `RXGBOCTIS` reader - MMC Receive Good Bad Octet Counter Interrupt Status"]
pub struct RXGBOCTIS_R(crate::FieldReader<bool, bool>);
impl RXGBOCTIS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RXGBOCTIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXGBOCTIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXGBOCTIS` writer - MMC Receive Good Bad Octet Counter Interrupt Status"]
pub struct RXGBOCTIS_W<'a> {
    w: &'a mut W,
}
impl<'a> RXGBOCTIS_W<'a> {
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
#[doc = "Field `RXGBFRMIS` reader - MMC Receive Good Bad Frame Counter Interrupt Status"]
pub struct RXGBFRMIS_R(crate::FieldReader<bool, bool>);
impl RXGBFRMIS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RXGBFRMIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXGBFRMIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXGBFRMIS` writer - MMC Receive Good Bad Frame Counter Interrupt Status"]
pub struct RXGBFRMIS_W<'a> {
    w: &'a mut W,
}
impl<'a> RXGBFRMIS_W<'a> {
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
    #[doc = "Bit 25 - MMC Receive Control Frame Counter Interrupt Status"]
    #[inline(always)]
    pub fn rxctrlfis(&self) -> RXCTRLFIS_R {
        RXCTRLFIS_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 24 - MMC Receive Error Frame Counter Interrupt Status"]
    #[inline(always)]
    pub fn rxrcverrfis(&self) -> RXRCVERRFIS_R {
        RXRCVERRFIS_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 23 - MMC Receive Watchdog Error Frame Counter Interrupt Status"]
    #[inline(always)]
    pub fn rxwdogfis(&self) -> RXWDOGFIS_R {
        RXWDOGFIS_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 22 - MMC Receive VLAN Good Bad Frame Counter Interrupt Status"]
    #[inline(always)]
    pub fn rxvlangbfis(&self) -> RXVLANGBFIS_R {
        RXVLANGBFIS_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 21 - MMC Receive FIFO Overflow Frame Counter Interrupt Status"]
    #[inline(always)]
    pub fn rxfovfis(&self) -> RXFOVFIS_R {
        RXFOVFIS_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 20 - MMC Receive Pause Frame Counter Interrupt Status"]
    #[inline(always)]
    pub fn rxpausfis(&self) -> RXPAUSFIS_R {
        RXPAUSFIS_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 19 - MMC Receive Out Of Range Error Frame Counter Interrupt Status."]
    #[inline(always)]
    pub fn rxorangefis(&self) -> RXORANGEFIS_R {
        RXORANGEFIS_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 18 - MMC Receive Length Error Frame Counter Interrupt Status"]
    #[inline(always)]
    pub fn rxlenerfis(&self) -> RXLENERFIS_R {
        RXLENERFIS_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 17 - MMC Receive Unicast Good Frame Counter Interrupt Status"]
    #[inline(always)]
    pub fn rxucgfis(&self) -> RXUCGFIS_R {
        RXUCGFIS_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 16 - MMC Receive 1024 to Maximum Octet Good Bad Frame Counter Interrupt Status."]
    #[inline(always)]
    pub fn rx1024tmaxoctgbfis(&self) -> RX1024TMAXOCTGBFIS_R {
        RX1024TMAXOCTGBFIS_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 15 - MMC Receive 512 to 1023 Octet Good Bad Frame Counter Interrupt Status"]
    #[inline(always)]
    pub fn rx512t1023octgbfis(&self) -> RX512T1023OCTGBFIS_R {
        RX512T1023OCTGBFIS_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 14 - MMC Receive 256 to 511 Octet Good Bad Frame Counter Interrupt Status"]
    #[inline(always)]
    pub fn rx256t511octgbfis(&self) -> RX256T511OCTGBFIS_R {
        RX256T511OCTGBFIS_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 13 - MMC Receive 128 to 255 Octet Good Bad Frame Counter Interrupt Status"]
    #[inline(always)]
    pub fn rx128t255octgbfis(&self) -> RX128T255OCTGBFIS_R {
        RX128T255OCTGBFIS_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12 - MMC Receive 65 to 127 Octet Good Bad Frame Counter Interrupt Status"]
    #[inline(always)]
    pub fn rx65t127octgbfis(&self) -> RX65T127OCTGBFIS_R {
        RX65T127OCTGBFIS_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11 - MMC Receive 64 Octet Good Bad Frame Counter Interrupt Status"]
    #[inline(always)]
    pub fn rx64octgbfis(&self) -> RX64OCTGBFIS_R {
        RX64OCTGBFIS_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10 - MMC Receive Oversize Good Frame Counter Interrupt Status"]
    #[inline(always)]
    pub fn rxosizegfis(&self) -> RXOSIZEGFIS_R {
        RXOSIZEGFIS_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - MMC Receive Undersize Good Frame Counter Interrupt Status"]
    #[inline(always)]
    pub fn rxusizegfis(&self) -> RXUSIZEGFIS_R {
        RXUSIZEGFIS_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - MMC Receive Jabber Error Frame Counter Interrupt Status"]
    #[inline(always)]
    pub fn rxjaberfis(&self) -> RXJABERFIS_R {
        RXJABERFIS_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7 - MMC Receive Runt Frame Counter Interrupt Status"]
    #[inline(always)]
    pub fn rxruntfis(&self) -> RXRUNTFIS_R {
        RXRUNTFIS_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - MMC Receive Alignment Error Frame Counter Interrupt Status"]
    #[inline(always)]
    pub fn rxalgnerfis(&self) -> RXALGNERFIS_R {
        RXALGNERFIS_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - MMC Receive CRC Error Frame Counter Interrupt Status"]
    #[inline(always)]
    pub fn rxcrcerfis(&self) -> RXCRCERFIS_R {
        RXCRCERFIS_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - MMC Receive Multicast Good Frame Counter Interrupt Status"]
    #[inline(always)]
    pub fn rxmcgfis(&self) -> RXMCGFIS_R {
        RXMCGFIS_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - MMC Receive Broadcast Good Frame Counter Interrupt Status"]
    #[inline(always)]
    pub fn rxbcgfis(&self) -> RXBCGFIS_R {
        RXBCGFIS_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - MMC Receive Good Octet Counter Interrupt Status"]
    #[inline(always)]
    pub fn rxgoctis(&self) -> RXGOCTIS_R {
        RXGOCTIS_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - MMC Receive Good Bad Octet Counter Interrupt Status"]
    #[inline(always)]
    pub fn rxgboctis(&self) -> RXGBOCTIS_R {
        RXGBOCTIS_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - MMC Receive Good Bad Frame Counter Interrupt Status"]
    #[inline(always)]
    pub fn rxgbfrmis(&self) -> RXGBFRMIS_R {
        RXGBFRMIS_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 25 - MMC Receive Control Frame Counter Interrupt Status"]
    #[inline(always)]
    pub fn rxctrlfis(&mut self) -> RXCTRLFIS_W {
        RXCTRLFIS_W { w: self }
    }
    #[doc = "Bit 24 - MMC Receive Error Frame Counter Interrupt Status"]
    #[inline(always)]
    pub fn rxrcverrfis(&mut self) -> RXRCVERRFIS_W {
        RXRCVERRFIS_W { w: self }
    }
    #[doc = "Bit 23 - MMC Receive Watchdog Error Frame Counter Interrupt Status"]
    #[inline(always)]
    pub fn rxwdogfis(&mut self) -> RXWDOGFIS_W {
        RXWDOGFIS_W { w: self }
    }
    #[doc = "Bit 22 - MMC Receive VLAN Good Bad Frame Counter Interrupt Status"]
    #[inline(always)]
    pub fn rxvlangbfis(&mut self) -> RXVLANGBFIS_W {
        RXVLANGBFIS_W { w: self }
    }
    #[doc = "Bit 21 - MMC Receive FIFO Overflow Frame Counter Interrupt Status"]
    #[inline(always)]
    pub fn rxfovfis(&mut self) -> RXFOVFIS_W {
        RXFOVFIS_W { w: self }
    }
    #[doc = "Bit 20 - MMC Receive Pause Frame Counter Interrupt Status"]
    #[inline(always)]
    pub fn rxpausfis(&mut self) -> RXPAUSFIS_W {
        RXPAUSFIS_W { w: self }
    }
    #[doc = "Bit 19 - MMC Receive Out Of Range Error Frame Counter Interrupt Status."]
    #[inline(always)]
    pub fn rxorangefis(&mut self) -> RXORANGEFIS_W {
        RXORANGEFIS_W { w: self }
    }
    #[doc = "Bit 18 - MMC Receive Length Error Frame Counter Interrupt Status"]
    #[inline(always)]
    pub fn rxlenerfis(&mut self) -> RXLENERFIS_W {
        RXLENERFIS_W { w: self }
    }
    #[doc = "Bit 17 - MMC Receive Unicast Good Frame Counter Interrupt Status"]
    #[inline(always)]
    pub fn rxucgfis(&mut self) -> RXUCGFIS_W {
        RXUCGFIS_W { w: self }
    }
    #[doc = "Bit 16 - MMC Receive 1024 to Maximum Octet Good Bad Frame Counter Interrupt Status."]
    #[inline(always)]
    pub fn rx1024tmaxoctgbfis(&mut self) -> RX1024TMAXOCTGBFIS_W {
        RX1024TMAXOCTGBFIS_W { w: self }
    }
    #[doc = "Bit 15 - MMC Receive 512 to 1023 Octet Good Bad Frame Counter Interrupt Status"]
    #[inline(always)]
    pub fn rx512t1023octgbfis(&mut self) -> RX512T1023OCTGBFIS_W {
        RX512T1023OCTGBFIS_W { w: self }
    }
    #[doc = "Bit 14 - MMC Receive 256 to 511 Octet Good Bad Frame Counter Interrupt Status"]
    #[inline(always)]
    pub fn rx256t511octgbfis(&mut self) -> RX256T511OCTGBFIS_W {
        RX256T511OCTGBFIS_W { w: self }
    }
    #[doc = "Bit 13 - MMC Receive 128 to 255 Octet Good Bad Frame Counter Interrupt Status"]
    #[inline(always)]
    pub fn rx128t255octgbfis(&mut self) -> RX128T255OCTGBFIS_W {
        RX128T255OCTGBFIS_W { w: self }
    }
    #[doc = "Bit 12 - MMC Receive 65 to 127 Octet Good Bad Frame Counter Interrupt Status"]
    #[inline(always)]
    pub fn rx65t127octgbfis(&mut self) -> RX65T127OCTGBFIS_W {
        RX65T127OCTGBFIS_W { w: self }
    }
    #[doc = "Bit 11 - MMC Receive 64 Octet Good Bad Frame Counter Interrupt Status"]
    #[inline(always)]
    pub fn rx64octgbfis(&mut self) -> RX64OCTGBFIS_W {
        RX64OCTGBFIS_W { w: self }
    }
    #[doc = "Bit 10 - MMC Receive Oversize Good Frame Counter Interrupt Status"]
    #[inline(always)]
    pub fn rxosizegfis(&mut self) -> RXOSIZEGFIS_W {
        RXOSIZEGFIS_W { w: self }
    }
    #[doc = "Bit 9 - MMC Receive Undersize Good Frame Counter Interrupt Status"]
    #[inline(always)]
    pub fn rxusizegfis(&mut self) -> RXUSIZEGFIS_W {
        RXUSIZEGFIS_W { w: self }
    }
    #[doc = "Bit 8 - MMC Receive Jabber Error Frame Counter Interrupt Status"]
    #[inline(always)]
    pub fn rxjaberfis(&mut self) -> RXJABERFIS_W {
        RXJABERFIS_W { w: self }
    }
    #[doc = "Bit 7 - MMC Receive Runt Frame Counter Interrupt Status"]
    #[inline(always)]
    pub fn rxruntfis(&mut self) -> RXRUNTFIS_W {
        RXRUNTFIS_W { w: self }
    }
    #[doc = "Bit 6 - MMC Receive Alignment Error Frame Counter Interrupt Status"]
    #[inline(always)]
    pub fn rxalgnerfis(&mut self) -> RXALGNERFIS_W {
        RXALGNERFIS_W { w: self }
    }
    #[doc = "Bit 5 - MMC Receive CRC Error Frame Counter Interrupt Status"]
    #[inline(always)]
    pub fn rxcrcerfis(&mut self) -> RXCRCERFIS_W {
        RXCRCERFIS_W { w: self }
    }
    #[doc = "Bit 4 - MMC Receive Multicast Good Frame Counter Interrupt Status"]
    #[inline(always)]
    pub fn rxmcgfis(&mut self) -> RXMCGFIS_W {
        RXMCGFIS_W { w: self }
    }
    #[doc = "Bit 3 - MMC Receive Broadcast Good Frame Counter Interrupt Status"]
    #[inline(always)]
    pub fn rxbcgfis(&mut self) -> RXBCGFIS_W {
        RXBCGFIS_W { w: self }
    }
    #[doc = "Bit 2 - MMC Receive Good Octet Counter Interrupt Status"]
    #[inline(always)]
    pub fn rxgoctis(&mut self) -> RXGOCTIS_W {
        RXGOCTIS_W { w: self }
    }
    #[doc = "Bit 1 - MMC Receive Good Bad Octet Counter Interrupt Status"]
    #[inline(always)]
    pub fn rxgboctis(&mut self) -> RXGBOCTIS_W {
        RXGBOCTIS_W { w: self }
    }
    #[doc = "Bit 0 - MMC Receive Good Bad Frame Counter Interrupt Status"]
    #[inline(always)]
    pub fn rxgbfrmis(&mut self) -> RXGBFRMIS_W {
        RXGBFRMIS_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "MMC Receive Interrupt Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mmc_intr_rx](index.html) module"]
pub struct MMC_INTR_RX_SPEC;
impl crate::RegisterSpec for MMC_INTR_RX_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mmc_intr_rx::R](R) reader structure"]
impl crate::Readable for MMC_INTR_RX_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mmc_intr_rx::W](W) writer structure"]
impl crate::Writable for MMC_INTR_RX_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MMC_INTR_RX to value 0"]
impl crate::Resettable for MMC_INTR_RX_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
