#[doc = "Register `CGCR` reader"]
pub struct R(crate::R<CGCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CGCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CGCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CGCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CGCR` writer"]
pub struct W(crate::W<CGCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CGCR_SPEC>;
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
impl From<crate::W<CGCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CGCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EIT` reader - Error Interrupt Type"]
pub struct EIT_R(crate::FieldReader<bool, bool>);
impl EIT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EIT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EIT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EIT` writer - Error Interrupt Type"]
pub struct EIT_W<'a> {
    w: &'a mut W,
}
impl<'a> EIT_W<'a> {
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
#[doc = "Field `DIAGEN` reader - Diagnostic Enable"]
pub struct DIAGEN_R(crate::FieldReader<bool, bool>);
impl DIAGEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DIAGEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DIAGEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DIAGEN` writer - Diagnostic Enable"]
pub struct DIAGEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DIAGEN_W<'a> {
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
#[doc = "Field `INTERNAL` reader - Internal"]
pub struct INTERNAL_R(crate::FieldReader<bool, bool>);
impl INTERNAL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        INTERNAL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INTERNAL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INTERNAL` writer - Internal"]
pub struct INTERNAL_W<'a> {
    w: &'a mut W,
}
impl<'a> INTERNAL_W<'a> {
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
#[doc = "Field `LOOPBACK` reader - Loopback"]
pub struct LOOPBACK_R(crate::FieldReader<bool, bool>);
impl LOOPBACK_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LOOPBACK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LOOPBACK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LOOPBACK` writer - Loopback"]
pub struct LOOPBACK_W<'a> {
    w: &'a mut W,
}
impl<'a> LOOPBACK_W<'a> {
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
#[doc = "Field `IGNACK` reader - Ignore Acknowledge"]
pub struct IGNACK_R(crate::FieldReader<bool, bool>);
impl IGNACK_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        IGNACK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IGNACK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IGNACK` writer - Ignore Acknowledge"]
pub struct IGNACK_W<'a> {
    w: &'a mut W,
}
impl<'a> IGNACK_W<'a> {
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
#[doc = "Field `LO` reader - Listen Only"]
pub struct LO_R(crate::FieldReader<bool, bool>);
impl LO_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LO_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LO` writer - Listen Only"]
pub struct LO_W<'a> {
    w: &'a mut W,
}
impl<'a> LO_W<'a> {
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
#[doc = "Field `DDIR` reader - Data Direction"]
pub struct DDIR_R(crate::FieldReader<bool, bool>);
impl DDIR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DDIR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DDIR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DDIR` writer - Data Direction"]
pub struct DDIR_W<'a> {
    w: &'a mut W,
}
impl<'a> DDIR_W<'a> {
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
#[doc = "Field `TSTPEN` reader - Time Sync Enable"]
pub struct TSTPEN_R(crate::FieldReader<bool, bool>);
impl TSTPEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TSTPEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TSTPEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TSTPEN` writer - Time Sync Enable"]
pub struct TSTPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TSTPEN_W<'a> {
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
#[doc = "Field `BUFFLOCK` reader - Buffer Lock"]
pub struct BUFFLOCK_R(crate::FieldReader<bool, bool>);
impl BUFFLOCK_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        BUFFLOCK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BUFFLOCK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BUFFLOCK` writer - Buffer Lock"]
pub struct BUFFLOCK_W<'a> {
    w: &'a mut W,
}
impl<'a> BUFFLOCK_W<'a> {
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
#[doc = "Field `CTX` reader - RW,Control Transmit"]
pub struct CTX_R(crate::FieldReader<bool, bool>);
impl CTX_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CTX_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CTX_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CTX` writer - RW,Control Transmit"]
pub struct CTX_W<'a> {
    w: &'a mut W,
}
impl<'a> CTX_W<'a> {
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
#[doc = "Field `CRX` reader - RW,Control Receive"]
pub struct CRX_R(crate::FieldReader<bool, bool>);
impl CRX_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CRX_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CRX_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CRX` writer - RW,Control Receive"]
pub struct CRX_W<'a> {
    w: &'a mut W,
}
impl<'a> CRX_W<'a> {
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
#[doc = "Field `CANEN` reader - CAN Enable"]
pub struct CANEN_R(crate::FieldReader<bool, bool>);
impl CANEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CANEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CANEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CANEN` writer - CAN Enable"]
pub struct CANEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CANEN_W<'a> {
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
    #[doc = "Bit 11 - Error Interrupt Type"]
    #[inline(always)]
    pub fn eit(&self) -> EIT_R {
        EIT_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Diagnostic Enable"]
    #[inline(always)]
    pub fn diagen(&self) -> DIAGEN_R {
        DIAGEN_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Internal"]
    #[inline(always)]
    pub fn internal(&self) -> INTERNAL_R {
        INTERNAL_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Loopback"]
    #[inline(always)]
    pub fn loopback(&self) -> LOOPBACK_R {
        LOOPBACK_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Ignore Acknowledge"]
    #[inline(always)]
    pub fn ignack(&self) -> IGNACK_R {
        IGNACK_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Listen Only"]
    #[inline(always)]
    pub fn lo(&self) -> LO_R {
        LO_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Data Direction"]
    #[inline(always)]
    pub fn ddir(&self) -> DDIR_R {
        DDIR_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Time Sync Enable"]
    #[inline(always)]
    pub fn tstpen(&self) -> TSTPEN_R {
        TSTPEN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Buffer Lock"]
    #[inline(always)]
    pub fn bufflock(&self) -> BUFFLOCK_R {
        BUFFLOCK_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - RW,Control Transmit"]
    #[inline(always)]
    pub fn ctx(&self) -> CTX_R {
        CTX_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - RW,Control Receive"]
    #[inline(always)]
    pub fn crx(&self) -> CRX_R {
        CRX_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - CAN Enable"]
    #[inline(always)]
    pub fn canen(&self) -> CANEN_R {
        CANEN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 11 - Error Interrupt Type"]
    #[inline(always)]
    pub fn eit(&mut self) -> EIT_W {
        EIT_W { w: self }
    }
    #[doc = "Bit 10 - Diagnostic Enable"]
    #[inline(always)]
    pub fn diagen(&mut self) -> DIAGEN_W {
        DIAGEN_W { w: self }
    }
    #[doc = "Bit 9 - Internal"]
    #[inline(always)]
    pub fn internal(&mut self) -> INTERNAL_W {
        INTERNAL_W { w: self }
    }
    #[doc = "Bit 8 - Loopback"]
    #[inline(always)]
    pub fn loopback(&mut self) -> LOOPBACK_W {
        LOOPBACK_W { w: self }
    }
    #[doc = "Bit 7 - Ignore Acknowledge"]
    #[inline(always)]
    pub fn ignack(&mut self) -> IGNACK_W {
        IGNACK_W { w: self }
    }
    #[doc = "Bit 6 - Listen Only"]
    #[inline(always)]
    pub fn lo(&mut self) -> LO_W {
        LO_W { w: self }
    }
    #[doc = "Bit 5 - Data Direction"]
    #[inline(always)]
    pub fn ddir(&mut self) -> DDIR_W {
        DDIR_W { w: self }
    }
    #[doc = "Bit 4 - Time Sync Enable"]
    #[inline(always)]
    pub fn tstpen(&mut self) -> TSTPEN_W {
        TSTPEN_W { w: self }
    }
    #[doc = "Bit 3 - Buffer Lock"]
    #[inline(always)]
    pub fn bufflock(&mut self) -> BUFFLOCK_W {
        BUFFLOCK_W { w: self }
    }
    #[doc = "Bit 2 - RW,Control Transmit"]
    #[inline(always)]
    pub fn ctx(&mut self) -> CTX_W {
        CTX_W { w: self }
    }
    #[doc = "Bit 1 - RW,Control Receive"]
    #[inline(always)]
    pub fn crx(&mut self) -> CRX_W {
        CRX_W { w: self }
    }
    #[doc = "Bit 0 - CAN Enable"]
    #[inline(always)]
    pub fn canen(&mut self) -> CANEN_W {
        CANEN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CAN Global Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cgcr](index.html) module"]
pub struct CGCR_SPEC;
impl crate::RegisterSpec for CGCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cgcr::R](R) reader structure"]
impl crate::Readable for CGCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cgcr::W](W) writer structure"]
impl crate::Writable for CGCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CGCR to value 0"]
impl crate::Resettable for CGCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
