#[doc = "Register `CEDIAG` reader"]
pub struct R(crate::R<CEDIAG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CEDIAG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CEDIAG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CEDIAG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CEDIAG` writer"]
pub struct W(crate::W<CEDIAG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CEDIAG_SPEC>;
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
impl From<crate::W<CEDIAG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CEDIAG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DRIVE` reader - Drive"]
pub struct DRIVE_R(crate::FieldReader<bool, bool>);
impl DRIVE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DRIVE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DRIVE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DRIVE` writer - Drive"]
pub struct DRIVE_W<'a> {
    w: &'a mut W,
}
impl<'a> DRIVE_W<'a> {
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
#[doc = "Field `MON` reader - Monitor"]
pub struct MON_R(crate::FieldReader<bool, bool>);
impl MON_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MON_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MON_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MON` writer - Monitor"]
pub struct MON_W<'a> {
    w: &'a mut W,
}
impl<'a> MON_W<'a> {
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
#[doc = "Field `CRC` reader - CRC"]
pub struct CRC_R(crate::FieldReader<bool, bool>);
impl CRC_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CRC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CRC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CRC` writer - CRC"]
pub struct CRC_W<'a> {
    w: &'a mut W,
}
impl<'a> CRC_W<'a> {
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
#[doc = "Field `STUFF` reader - Stuff Error"]
pub struct STUFF_R(crate::FieldReader<bool, bool>);
impl STUFF_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        STUFF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for STUFF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `STUFF` writer - Stuff Error"]
pub struct STUFF_W<'a> {
    w: &'a mut W,
}
impl<'a> STUFF_W<'a> {
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
#[doc = "Field `TXE` reader - Transmit Error"]
pub struct TXE_R(crate::FieldReader<bool, bool>);
impl TXE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TXE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXE` writer - Transmit Error"]
pub struct TXE_W<'a> {
    w: &'a mut W,
}
impl<'a> TXE_W<'a> {
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
#[doc = "Field `EBID` reader - Error Bit Identifier"]
pub struct EBID_R(crate::FieldReader<u8, u8>);
impl EBID_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        EBID_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EBID_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EBID` writer - Error Bit Identifier"]
pub struct EBID_W<'a> {
    w: &'a mut W,
}
impl<'a> EBID_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 4)) | ((value as u32 & 0x3f) << 4);
        self.w
    }
}
#[doc = "Field `EFID` reader - Error Field Identifier"]
pub struct EFID_R(crate::FieldReader<u8, u8>);
impl EFID_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        EFID_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EFID_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EFID` writer - Error Field Identifier"]
pub struct EFID_W<'a> {
    w: &'a mut W,
}
impl<'a> EFID_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | (value as u32 & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bit 14 - Drive"]
    #[inline(always)]
    pub fn drive(&self) -> DRIVE_R {
        DRIVE_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Monitor"]
    #[inline(always)]
    pub fn mon(&self) -> MON_R {
        MON_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12 - CRC"]
    #[inline(always)]
    pub fn crc(&self) -> CRC_R {
        CRC_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Stuff Error"]
    #[inline(always)]
    pub fn stuff(&self) -> STUFF_R {
        STUFF_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Transmit Error"]
    #[inline(always)]
    pub fn txe(&self) -> TXE_R {
        TXE_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bits 4:9 - Error Bit Identifier"]
    #[inline(always)]
    pub fn ebid(&self) -> EBID_R {
        EBID_R::new(((self.bits >> 4) & 0x3f) as u8)
    }
    #[doc = "Bits 0:3 - Error Field Identifier"]
    #[inline(always)]
    pub fn efid(&self) -> EFID_R {
        EFID_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 14 - Drive"]
    #[inline(always)]
    pub fn drive(&mut self) -> DRIVE_W {
        DRIVE_W { w: self }
    }
    #[doc = "Bit 13 - Monitor"]
    #[inline(always)]
    pub fn mon(&mut self) -> MON_W {
        MON_W { w: self }
    }
    #[doc = "Bit 12 - CRC"]
    #[inline(always)]
    pub fn crc(&mut self) -> CRC_W {
        CRC_W { w: self }
    }
    #[doc = "Bit 11 - Stuff Error"]
    #[inline(always)]
    pub fn stuff(&mut self) -> STUFF_W {
        STUFF_W { w: self }
    }
    #[doc = "Bit 10 - Transmit Error"]
    #[inline(always)]
    pub fn txe(&mut self) -> TXE_W {
        TXE_W { w: self }
    }
    #[doc = "Bits 4:9 - Error Bit Identifier"]
    #[inline(always)]
    pub fn ebid(&mut self) -> EBID_W {
        EBID_W { w: self }
    }
    #[doc = "Bits 0:3 - Error Field Identifier"]
    #[inline(always)]
    pub fn efid(&mut self) -> EFID_W {
        EFID_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CAN Error Diagnostic Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cediag](index.html) module"]
pub struct CEDIAG_SPEC;
impl crate::RegisterSpec for CEDIAG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cediag::R](R) reader structure"]
impl crate::Readable for CEDIAG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cediag::W](W) writer structure"]
impl crate::Writable for CEDIAG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CEDIAG to value 0"]
impl crate::Resettable for CEDIAG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
