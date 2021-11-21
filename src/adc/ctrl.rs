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
#[doc = "Field `CONV_CNT` reader - Conversion count describes the number of conversions to be applied for triggers/sweeps. (N+1 conversions)"]
pub struct CONV_CNT_R(crate::FieldReader<u8, u8>);
impl CONV_CNT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CONV_CNT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CONV_CNT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CONV_CNT` writer - Conversion count describes the number of conversions to be applied for triggers/sweeps. (N+1 conversions)"]
pub struct CONV_CNT_W<'a> {
    w: &'a mut W,
}
impl<'a> CONV_CNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 20)) | ((value as u32 & 0x0f) << 20);
        self.w
    }
}
#[doc = "Field `MANUAL_TRIG` reader - Starts analog acquisition"]
pub struct MANUAL_TRIG_R(crate::FieldReader<bool, bool>);
impl MANUAL_TRIG_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MANUAL_TRIG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MANUAL_TRIG_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MANUAL_TRIG` writer - Starts analog acquisition"]
pub struct MANUAL_TRIG_W<'a> {
    w: &'a mut W,
}
impl<'a> MANUAL_TRIG_W<'a> {
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
#[doc = "Field `EXT_TRIG_EN` reader - Allows the external trigger to start analog acquisition"]
pub struct EXT_TRIG_EN_R(crate::FieldReader<bool, bool>);
impl EXT_TRIG_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EXT_TRIG_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EXT_TRIG_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EXT_TRIG_EN` writer - Allows the external trigger to start analog acquisition"]
pub struct EXT_TRIG_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> EXT_TRIG_EN_W<'a> {
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
#[doc = "Field `SWEEP_EN` reader - ADC data acquisition for all enabled channel"]
pub struct SWEEP_EN_R(crate::FieldReader<bool, bool>);
impl SWEEP_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SWEEP_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SWEEP_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SWEEP_EN` writer - ADC data acquisition for all enabled channel"]
pub struct SWEEP_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SWEEP_EN_W<'a> {
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
#[doc = "Field `CHAN_TAG_EN` reader - Enables the channel tag to be saved with the ADC data"]
pub struct CHAN_TAG_EN_R(crate::FieldReader<bool, bool>);
impl CHAN_TAG_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CHAN_TAG_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CHAN_TAG_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CHAN_TAG_EN` writer - Enables the channel tag to be saved with the ADC data"]
pub struct CHAN_TAG_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> CHAN_TAG_EN_W<'a> {
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
#[doc = "Field `CHAN_EN` reader - Enables the channel for data collection"]
pub struct CHAN_EN_R(crate::FieldReader<u16, u16>);
impl CHAN_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        CHAN_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CHAN_EN_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CHAN_EN` writer - Enables the channel for data collection"]
pub struct CHAN_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> CHAN_EN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 20:23 - Conversion count describes the number of conversions to be applied for triggers/sweeps. (N+1 conversions)"]
    #[inline(always)]
    pub fn conv_cnt(&self) -> CONV_CNT_R {
        CONV_CNT_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bit 19 - Starts analog acquisition"]
    #[inline(always)]
    pub fn manual_trig(&self) -> MANUAL_TRIG_R {
        MANUAL_TRIG_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Allows the external trigger to start analog acquisition"]
    #[inline(always)]
    pub fn ext_trig_en(&self) -> EXT_TRIG_EN_R {
        EXT_TRIG_EN_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 17 - ADC data acquisition for all enabled channel"]
    #[inline(always)]
    pub fn sweep_en(&self) -> SWEEP_EN_R {
        SWEEP_EN_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Enables the channel tag to be saved with the ADC data"]
    #[inline(always)]
    pub fn chan_tag_en(&self) -> CHAN_TAG_EN_R {
        CHAN_TAG_EN_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bits 0:15 - Enables the channel for data collection"]
    #[inline(always)]
    pub fn chan_en(&self) -> CHAN_EN_R {
        CHAN_EN_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 20:23 - Conversion count describes the number of conversions to be applied for triggers/sweeps. (N+1 conversions)"]
    #[inline(always)]
    pub fn conv_cnt(&mut self) -> CONV_CNT_W {
        CONV_CNT_W { w: self }
    }
    #[doc = "Bit 19 - Starts analog acquisition"]
    #[inline(always)]
    pub fn manual_trig(&mut self) -> MANUAL_TRIG_W {
        MANUAL_TRIG_W { w: self }
    }
    #[doc = "Bit 18 - Allows the external trigger to start analog acquisition"]
    #[inline(always)]
    pub fn ext_trig_en(&mut self) -> EXT_TRIG_EN_W {
        EXT_TRIG_EN_W { w: self }
    }
    #[doc = "Bit 17 - ADC data acquisition for all enabled channel"]
    #[inline(always)]
    pub fn sweep_en(&mut self) -> SWEEP_EN_W {
        SWEEP_EN_W { w: self }
    }
    #[doc = "Bit 16 - Enables the channel tag to be saved with the ADC data"]
    #[inline(always)]
    pub fn chan_tag_en(&mut self) -> CHAN_TAG_EN_W {
        CHAN_TAG_EN_W { w: self }
    }
    #[doc = "Bits 0:15 - Enables the channel for data collection"]
    #[inline(always)]
    pub fn chan_en(&mut self) -> CHAN_EN_W {
        CHAN_EN_W { w: self }
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
