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
#[doc = "Field `PAREN` reader - Parity Enable"]
pub struct PAREN_R(crate::FieldReader<bool, bool>);
impl PAREN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PAREN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PAREN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PAREN` writer - Parity Enable"]
pub struct PAREN_W<'a> {
    w: &'a mut W,
}
impl<'a> PAREN_W<'a> {
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
#[doc = "Field `PAREVEN` reader - Parity Even/Odd(1/0)"]
pub struct PAREVEN_R(crate::FieldReader<bool, bool>);
impl PAREVEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PAREVEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PAREVEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PAREVEN` writer - Parity Even/Odd(1/0)"]
pub struct PAREVEN_W<'a> {
    w: &'a mut W,
}
impl<'a> PAREVEN_W<'a> {
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
#[doc = "Field `PARSTK` reader - Parity Sticky"]
pub struct PARSTK_R(crate::FieldReader<bool, bool>);
impl PARSTK_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PARSTK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PARSTK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PARSTK` writer - Parity Sticky"]
pub struct PARSTK_W<'a> {
    w: &'a mut W,
}
impl<'a> PARSTK_W<'a> {
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
#[doc = "Field `STOPBITS` reader - Stop Bits 1/2(0/1)"]
pub struct STOPBITS_R(crate::FieldReader<bool, bool>);
impl STOPBITS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        STOPBITS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for STOPBITS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `STOPBITS` writer - Stop Bits 1/2(0/1)"]
pub struct STOPBITS_W<'a> {
    w: &'a mut W,
}
impl<'a> STOPBITS_W<'a> {
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
#[doc = "Field `WORDSIZE` reader - Word Size in Bits 5/6/7/8(00/01/10/11)"]
pub struct WORDSIZE_R(crate::FieldReader<u8, u8>);
impl WORDSIZE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        WORDSIZE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WORDSIZE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WORDSIZE` writer - Word Size in Bits 5/6/7/8(00/01/10/11)"]
pub struct WORDSIZE_W<'a> {
    w: &'a mut W,
}
impl<'a> WORDSIZE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | ((value as u32 & 0x03) << 4);
        self.w
    }
}
#[doc = "Field `LOOPBACK` reader - Loopback Enable"]
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
#[doc = "Field `LOOPBACK` writer - Loopback Enable"]
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | ((value as u32 & 0x01) << 6);
        self.w
    }
}
#[doc = "Field `LOOPBACKBLK` reader - Loopback Block"]
pub struct LOOPBACKBLK_R(crate::FieldReader<bool, bool>);
impl LOOPBACKBLK_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LOOPBACKBLK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LOOPBACKBLK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LOOPBACKBLK` writer - Loopback Block"]
pub struct LOOPBACKBLK_W<'a> {
    w: &'a mut W,
}
impl<'a> LOOPBACKBLK_W<'a> {
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
#[doc = "Field `AUTOCTS` reader - Enable Auto CTS mode"]
pub struct AUTOCTS_R(crate::FieldReader<bool, bool>);
impl AUTOCTS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        AUTOCTS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AUTOCTS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AUTOCTS` writer - Enable Auto CTS mode"]
pub struct AUTOCTS_W<'a> {
    w: &'a mut W,
}
impl<'a> AUTOCTS_W<'a> {
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
#[doc = "Field `DEFRTS` reader - Default RTSn value"]
pub struct DEFRTS_R(crate::FieldReader<bool, bool>);
impl DEFRTS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DEFRTS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DEFRTS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DEFRTS` writer - Default RTSn value"]
pub struct DEFRTS_W<'a> {
    w: &'a mut W,
}
impl<'a> DEFRTS_W<'a> {
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
#[doc = "Field `AUTORTS` reader - Enable Auto RTS mode"]
pub struct AUTORTS_R(crate::FieldReader<bool, bool>);
impl AUTORTS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        AUTORTS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AUTORTS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AUTORTS` writer - Enable Auto RTS mode"]
pub struct AUTORTS_W<'a> {
    w: &'a mut W,
}
impl<'a> AUTORTS_W<'a> {
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
#[doc = "Field `BAUD8` reader - Enable BAUD8 mode"]
pub struct BAUD8_R(crate::FieldReader<bool, bool>);
impl BAUD8_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        BAUD8_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BAUD8_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BAUD8` writer - Enable BAUD8 mode"]
pub struct BAUD8_W<'a> {
    w: &'a mut W,
}
impl<'a> BAUD8_W<'a> {
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
impl R {
    #[doc = "Bit 0 - Parity Enable"]
    #[inline(always)]
    pub fn paren(&self) -> PAREN_R {
        PAREN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Parity Even/Odd(1/0)"]
    #[inline(always)]
    pub fn pareven(&self) -> PAREVEN_R {
        PAREVEN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Parity Sticky"]
    #[inline(always)]
    pub fn parstk(&self) -> PARSTK_R {
        PARSTK_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Stop Bits 1/2(0/1)"]
    #[inline(always)]
    pub fn stopbits(&self) -> STOPBITS_R {
        STOPBITS_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bits 4:5 - Word Size in Bits 5/6/7/8(00/01/10/11)"]
    #[inline(always)]
    pub fn wordsize(&self) -> WORDSIZE_R {
        WORDSIZE_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bit 6 - Loopback Enable"]
    #[inline(always)]
    pub fn loopback(&self) -> LOOPBACK_R {
        LOOPBACK_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Loopback Block"]
    #[inline(always)]
    pub fn loopbackblk(&self) -> LOOPBACKBLK_R {
        LOOPBACKBLK_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Enable Auto CTS mode"]
    #[inline(always)]
    pub fn autocts(&self) -> AUTOCTS_R {
        AUTOCTS_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Default RTSn value"]
    #[inline(always)]
    pub fn defrts(&self) -> DEFRTS_R {
        DEFRTS_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Enable Auto RTS mode"]
    #[inline(always)]
    pub fn autorts(&self) -> AUTORTS_R {
        AUTORTS_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Enable BAUD8 mode"]
    #[inline(always)]
    pub fn baud8(&self) -> BAUD8_R {
        BAUD8_R::new(((self.bits >> 11) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Parity Enable"]
    #[inline(always)]
    pub fn paren(&mut self) -> PAREN_W {
        PAREN_W { w: self }
    }
    #[doc = "Bit 1 - Parity Even/Odd(1/0)"]
    #[inline(always)]
    pub fn pareven(&mut self) -> PAREVEN_W {
        PAREVEN_W { w: self }
    }
    #[doc = "Bit 2 - Parity Sticky"]
    #[inline(always)]
    pub fn parstk(&mut self) -> PARSTK_W {
        PARSTK_W { w: self }
    }
    #[doc = "Bit 3 - Stop Bits 1/2(0/1)"]
    #[inline(always)]
    pub fn stopbits(&mut self) -> STOPBITS_W {
        STOPBITS_W { w: self }
    }
    #[doc = "Bits 4:5 - Word Size in Bits 5/6/7/8(00/01/10/11)"]
    #[inline(always)]
    pub fn wordsize(&mut self) -> WORDSIZE_W {
        WORDSIZE_W { w: self }
    }
    #[doc = "Bit 6 - Loopback Enable"]
    #[inline(always)]
    pub fn loopback(&mut self) -> LOOPBACK_W {
        LOOPBACK_W { w: self }
    }
    #[doc = "Bit 7 - Loopback Block"]
    #[inline(always)]
    pub fn loopbackblk(&mut self) -> LOOPBACKBLK_W {
        LOOPBACKBLK_W { w: self }
    }
    #[doc = "Bit 8 - Enable Auto CTS mode"]
    #[inline(always)]
    pub fn autocts(&mut self) -> AUTOCTS_W {
        AUTOCTS_W { w: self }
    }
    #[doc = "Bit 9 - Default RTSn value"]
    #[inline(always)]
    pub fn defrts(&mut self) -> DEFRTS_W {
        DEFRTS_W { w: self }
    }
    #[doc = "Bit 10 - Enable Auto RTS mode"]
    #[inline(always)]
    pub fn autorts(&mut self) -> AUTORTS_W {
        AUTORTS_W { w: self }
    }
    #[doc = "Bit 11 - Enable BAUD8 mode"]
    #[inline(always)]
    pub fn baud8(&mut self) -> BAUD8_W {
        BAUD8_W { w: self }
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
#[doc = "`reset()` method sets CTRL to value 0"]
impl crate::Resettable for CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
