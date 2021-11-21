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
#[doc = "Field `CLKENABLED` reader - I2C CLK Enabled"]
pub struct CLKENABLED_R(crate::FieldReader<bool, bool>);
impl CLKENABLED_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CLKENABLED_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CLKENABLED_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CLKENABLED` writer - I2C CLK Enabled"]
pub struct CLKENABLED_W<'a> {
    w: &'a mut W,
}
impl<'a> CLKENABLED_W<'a> {
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
#[doc = "Field `ENABLED` reader - I2C Activated"]
pub struct ENABLED_R(crate::FieldReader<bool, bool>);
impl ENABLED_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ENABLED_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ENABLED_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ENABLED` writer - I2C Activated"]
pub struct ENABLED_W<'a> {
    w: &'a mut W,
}
impl<'a> ENABLED_W<'a> {
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
#[doc = "Field `ENABLE` reader - I2C Active"]
pub struct ENABLE_R(crate::FieldReader<bool, bool>);
impl ENABLE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ENABLE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ENABLE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ENABLE` writer - I2C Active"]
pub struct ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> ENABLE_W<'a> {
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
#[doc = "Field `TXFEMD` reader - TX FIFIO Empty Mode"]
pub struct TXFEMD_R(crate::FieldReader<bool, bool>);
impl TXFEMD_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TXFEMD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXFEMD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXFEMD` writer - TX FIFIO Empty Mode"]
pub struct TXFEMD_W<'a> {
    w: &'a mut W,
}
impl<'a> TXFEMD_W<'a> {
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
#[doc = "Field `RXFFMD` reader - RX FIFO Full Mode"]
pub struct RXFFMD_R(crate::FieldReader<bool, bool>);
impl RXFFMD_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RXFFMD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXFFMD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXFFMD` writer - RX FIFO Full Mode"]
pub struct RXFFMD_W<'a> {
    w: &'a mut W,
}
impl<'a> RXFFMD_W<'a> {
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
#[doc = "Field `ALGFILTER` reader - Enable Input Analog Glitch Filter"]
pub struct ALGFILTER_R(crate::FieldReader<bool, bool>);
impl ALGFILTER_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ALGFILTER_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ALGFILTER_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ALGFILTER` writer - Enable Input Analog Glitch Filter"]
pub struct ALGFILTER_W<'a> {
    w: &'a mut W,
}
impl<'a> ALGFILTER_W<'a> {
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
#[doc = "Field `DLGFILTER` reader - Enable Input Digital Glitch Filter"]
pub struct DLGFILTER_R(crate::FieldReader<bool, bool>);
impl DLGFILTER_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DLGFILTER_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DLGFILTER_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DLGFILTER` writer - Enable Input Digital Glitch Filter"]
pub struct DLGFILTER_W<'a> {
    w: &'a mut W,
}
impl<'a> DLGFILTER_W<'a> {
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
#[doc = "Field `LOOPBACK` reader - Enable LoopBack Mode"]
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
#[doc = "Field `LOOPBACK` writer - Enable LoopBack Mode"]
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
#[doc = "Field `TMCONFIGENB` reader - Enable Timing Config Register"]
pub struct TMCONFIGENB_R(crate::FieldReader<bool, bool>);
impl TMCONFIGENB_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TMCONFIGENB_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TMCONFIGENB_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TMCONFIGENB` writer - Enable Timing Config Register"]
pub struct TMCONFIGENB_W<'a> {
    w: &'a mut W,
}
impl<'a> TMCONFIGENB_W<'a> {
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
impl R {
    #[doc = "Bit 0 - I2C CLK Enabled"]
    #[inline(always)]
    pub fn clkenabled(&self) -> CLKENABLED_R {
        CLKENABLED_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - I2C Activated"]
    #[inline(always)]
    pub fn enabled(&self) -> ENABLED_R {
        ENABLED_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - I2C Active"]
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - TX FIFIO Empty Mode"]
    #[inline(always)]
    pub fn txfemd(&self) -> TXFEMD_R {
        TXFEMD_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - RX FIFO Full Mode"]
    #[inline(always)]
    pub fn rxffmd(&self) -> RXFFMD_R {
        RXFFMD_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Enable Input Analog Glitch Filter"]
    #[inline(always)]
    pub fn algfilter(&self) -> ALGFILTER_R {
        ALGFILTER_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Enable Input Digital Glitch Filter"]
    #[inline(always)]
    pub fn dlgfilter(&self) -> DLGFILTER_R {
        DLGFILTER_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Enable LoopBack Mode"]
    #[inline(always)]
    pub fn loopback(&self) -> LOOPBACK_R {
        LOOPBACK_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Enable Timing Config Register"]
    #[inline(always)]
    pub fn tmconfigenb(&self) -> TMCONFIGENB_R {
        TMCONFIGENB_R::new(((self.bits >> 9) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - I2C CLK Enabled"]
    #[inline(always)]
    pub fn clkenabled(&mut self) -> CLKENABLED_W {
        CLKENABLED_W { w: self }
    }
    #[doc = "Bit 1 - I2C Activated"]
    #[inline(always)]
    pub fn enabled(&mut self) -> ENABLED_W {
        ENABLED_W { w: self }
    }
    #[doc = "Bit 2 - I2C Active"]
    #[inline(always)]
    pub fn enable(&mut self) -> ENABLE_W {
        ENABLE_W { w: self }
    }
    #[doc = "Bit 3 - TX FIFIO Empty Mode"]
    #[inline(always)]
    pub fn txfemd(&mut self) -> TXFEMD_W {
        TXFEMD_W { w: self }
    }
    #[doc = "Bit 4 - RX FIFO Full Mode"]
    #[inline(always)]
    pub fn rxffmd(&mut self) -> RXFFMD_W {
        RXFFMD_W { w: self }
    }
    #[doc = "Bit 5 - Enable Input Analog Glitch Filter"]
    #[inline(always)]
    pub fn algfilter(&mut self) -> ALGFILTER_W {
        ALGFILTER_W { w: self }
    }
    #[doc = "Bit 6 - Enable Input Digital Glitch Filter"]
    #[inline(always)]
    pub fn dlgfilter(&mut self) -> DLGFILTER_W {
        DLGFILTER_W { w: self }
    }
    #[doc = "Bit 8 - Enable LoopBack Mode"]
    #[inline(always)]
    pub fn loopback(&mut self) -> LOOPBACK_W {
        LOOPBACK_W { w: self }
    }
    #[doc = "Bit 9 - Enable Timing Config Register"]
    #[inline(always)]
    pub fn tmconfigenb(&mut self) -> TMCONFIGENB_W {
        TMCONFIGENB_W { w: self }
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
