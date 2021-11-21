#[doc = "Register `ANALOG_CNTL` reader"]
pub struct R(crate::R<ANALOG_CNTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ANALOG_CNTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ANALOG_CNTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ANALOG_CNTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ANALOG_CNTL` writer"]
pub struct W(crate::W<ANALOG_CNTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ANALOG_CNTL_SPEC>;
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
impl From<crate::W<ANALOG_CNTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ANALOG_CNTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TMOSC` reader - Test Mode"]
pub struct TMOSC_R(crate::FieldReader<bool, bool>);
impl TMOSC_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TMOSC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TMOSC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TMOSC` writer - Test Mode"]
pub struct TMOSC_W<'a> {
    w: &'a mut W,
}
impl<'a> TMOSC_W<'a> {
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
#[doc = "Field `TMPOKDIS` reader - Test Mode"]
pub struct TMPOKDIS_R(crate::FieldReader<bool, bool>);
impl TMPOKDIS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TMPOKDIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TMPOKDIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TMPOKDIS` writer - Test Mode"]
pub struct TMPOKDIS_W<'a> {
    w: &'a mut W,
}
impl<'a> TMPOKDIS_W<'a> {
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
#[doc = "Field `TM_ADCMUX_N` reader - Test Mode"]
pub struct TM_ADCMUX_N_R(crate::FieldReader<bool, bool>);
impl TM_ADCMUX_N_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TM_ADCMUX_N_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TM_ADCMUX_N_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TM_ADCMUX_N` writer - Test Mode"]
pub struct TM_ADCMUX_N_W<'a> {
    w: &'a mut W,
}
impl<'a> TM_ADCMUX_N_W<'a> {
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
#[doc = "Field `TM_ADCMUX_P` reader - Test Mode"]
pub struct TM_ADCMUX_P_R(crate::FieldReader<bool, bool>);
impl TM_ADCMUX_P_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TM_ADCMUX_P_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TM_ADCMUX_P_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TM_ADCMUX_P` writer - Test Mode"]
pub struct TM_ADCMUX_P_W<'a> {
    w: &'a mut W,
}
impl<'a> TM_ADCMUX_P_W<'a> {
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
#[doc = "Field `TMRATIO` reader - Test Mode"]
pub struct TMRATIO_R(crate::FieldReader<bool, bool>);
impl TMRATIO_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TMRATIO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TMRATIO_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TMRATIO` writer - Test Mode"]
pub struct TMRATIO_W<'a> {
    w: &'a mut W,
}
impl<'a> TMRATIO_W<'a> {
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
#[doc = "Field `TMATOMUX` reader - Test Mode"]
pub struct TMATOMUX_R(crate::FieldReader<u8, u8>);
impl TMATOMUX_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        TMATOMUX_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TMATOMUX_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TMATOMUX` writer - Test Mode"]
pub struct TMATOMUX_W<'a> {
    w: &'a mut W,
}
impl<'a> TMATOMUX_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 5)) | ((value as u32 & 0x03) << 5);
        self.w
    }
}
#[doc = "Field `ADC_STEST` reader - Number of clocks for sample time"]
pub struct ADC_STEST_R(crate::FieldReader<u8, u8>);
impl ADC_STEST_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        ADC_STEST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADC_STEST_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADC_STEST` writer - Number of clocks for sample time"]
pub struct ADC_STEST_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC_STEST_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 9)) | ((value as u32 & 0x0f) << 9);
        self.w
    }
}
#[doc = "Field `RCLK_POS_EN` reader - Enable normal test clock"]
pub struct RCLK_POS_EN_R(crate::FieldReader<bool, bool>);
impl RCLK_POS_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RCLK_POS_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RCLK_POS_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RCLK_POS_EN` writer - Enable normal test clock"]
pub struct RCLK_POS_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> RCLK_POS_EN_W<'a> {
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
#[doc = "Field `RCLK_NEG_EN` reader - Enable inverted test clock"]
pub struct RCLK_NEG_EN_R(crate::FieldReader<bool, bool>);
impl RCLK_NEG_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RCLK_NEG_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RCLK_NEG_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RCLK_NEG_EN` writer - Enable inverted test clock"]
pub struct RCLK_NEG_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> RCLK_NEG_EN_W<'a> {
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
#[doc = "Field `APB2CLK_POS_EN` reader - Enable normal APB2CLK for test output"]
pub struct APB2CLK_POS_EN_R(crate::FieldReader<bool, bool>);
impl APB2CLK_POS_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        APB2CLK_POS_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for APB2CLK_POS_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `APB2CLK_POS_EN` writer - Enable normal APB2CLK for test output"]
pub struct APB2CLK_POS_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> APB2CLK_POS_EN_W<'a> {
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
#[doc = "Field `APB2CLK_NEG_EN` reader - Enable inverted APB2CLK for test output"]
pub struct APB2CLK_NEG_EN_R(crate::FieldReader<bool, bool>);
impl APB2CLK_NEG_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        APB2CLK_NEG_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for APB2CLK_NEG_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `APB2CLK_NEG_EN` writer - Enable inverted APB2CLK for test output"]
pub struct APB2CLK_NEG_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> APB2CLK_NEG_EN_W<'a> {
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
#[doc = "Field `TM_ANALOG_PD_EN` reader - Enables pull down on analog pads"]
pub struct TM_ANALOG_PD_EN_R(crate::FieldReader<bool, bool>);
impl TM_ANALOG_PD_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TM_ANALOG_PD_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TM_ANALOG_PD_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TM_ANALOG_PD_EN` writer - Enables pull down on analog pads"]
pub struct TM_ANALOG_PD_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> TM_ANALOG_PD_EN_W<'a> {
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
#[doc = "Field `JMP2BOOT` reader - Enables a skip of all delay counters and eFuse read"]
pub struct JMP2BOOT_R(crate::FieldReader<bool, bool>);
impl JMP2BOOT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        JMP2BOOT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for JMP2BOOT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `JMP2BOOT` writer - Enables a skip of all delay counters and eFuse read"]
pub struct JMP2BOOT_W<'a> {
    w: &'a mut W,
}
impl<'a> JMP2BOOT_W<'a> {
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
#[doc = "Field `SKIPBOOT` reader - Enables a skip of all delay counters, eFuse read, and boot"]
pub struct SKIPBOOT_R(crate::FieldReader<bool, bool>);
impl SKIPBOOT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SKIPBOOT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SKIPBOOT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SKIPBOOT` writer - Enables a skip of all delay counters, eFuse read, and boot"]
pub struct SKIPBOOT_W<'a> {
    w: &'a mut W,
}
impl<'a> SKIPBOOT_W<'a> {
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
impl R {
    #[doc = "Bit 0 - Test Mode"]
    #[inline(always)]
    pub fn tmosc(&self) -> TMOSC_R {
        TMOSC_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Test Mode"]
    #[inline(always)]
    pub fn tmpokdis(&self) -> TMPOKDIS_R {
        TMPOKDIS_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Test Mode"]
    #[inline(always)]
    pub fn tm_adcmux_n(&self) -> TM_ADCMUX_N_R {
        TM_ADCMUX_N_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Test Mode"]
    #[inline(always)]
    pub fn tm_adcmux_p(&self) -> TM_ADCMUX_P_R {
        TM_ADCMUX_P_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Test Mode"]
    #[inline(always)]
    pub fn tmratio(&self) -> TMRATIO_R {
        TMRATIO_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bits 5:6 - Test Mode"]
    #[inline(always)]
    pub fn tmatomux(&self) -> TMATOMUX_R {
        TMATOMUX_R::new(((self.bits >> 5) & 0x03) as u8)
    }
    #[doc = "Bits 9:12 - Number of clocks for sample time"]
    #[inline(always)]
    pub fn adc_stest(&self) -> ADC_STEST_R {
        ADC_STEST_R::new(((self.bits >> 9) & 0x0f) as u8)
    }
    #[doc = "Bit 14 - Enable normal test clock"]
    #[inline(always)]
    pub fn rclk_pos_en(&self) -> RCLK_POS_EN_R {
        RCLK_POS_EN_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Enable inverted test clock"]
    #[inline(always)]
    pub fn rclk_neg_en(&self) -> RCLK_NEG_EN_R {
        RCLK_NEG_EN_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Enable normal APB2CLK for test output"]
    #[inline(always)]
    pub fn apb2clk_pos_en(&self) -> APB2CLK_POS_EN_R {
        APB2CLK_POS_EN_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Enable inverted APB2CLK for test output"]
    #[inline(always)]
    pub fn apb2clk_neg_en(&self) -> APB2CLK_NEG_EN_R {
        APB2CLK_NEG_EN_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Enables pull down on analog pads"]
    #[inline(always)]
    pub fn tm_analog_pd_en(&self) -> TM_ANALOG_PD_EN_R {
        TM_ANALOG_PD_EN_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Enables a skip of all delay counters and eFuse read"]
    #[inline(always)]
    pub fn jmp2boot(&self) -> JMP2BOOT_R {
        JMP2BOOT_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Enables a skip of all delay counters, eFuse read, and boot"]
    #[inline(always)]
    pub fn skipboot(&self) -> SKIPBOOT_R {
        SKIPBOOT_R::new(((self.bits >> 20) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Test Mode"]
    #[inline(always)]
    pub fn tmosc(&mut self) -> TMOSC_W {
        TMOSC_W { w: self }
    }
    #[doc = "Bit 1 - Test Mode"]
    #[inline(always)]
    pub fn tmpokdis(&mut self) -> TMPOKDIS_W {
        TMPOKDIS_W { w: self }
    }
    #[doc = "Bit 2 - Test Mode"]
    #[inline(always)]
    pub fn tm_adcmux_n(&mut self) -> TM_ADCMUX_N_W {
        TM_ADCMUX_N_W { w: self }
    }
    #[doc = "Bit 3 - Test Mode"]
    #[inline(always)]
    pub fn tm_adcmux_p(&mut self) -> TM_ADCMUX_P_W {
        TM_ADCMUX_P_W { w: self }
    }
    #[doc = "Bit 4 - Test Mode"]
    #[inline(always)]
    pub fn tmratio(&mut self) -> TMRATIO_W {
        TMRATIO_W { w: self }
    }
    #[doc = "Bits 5:6 - Test Mode"]
    #[inline(always)]
    pub fn tmatomux(&mut self) -> TMATOMUX_W {
        TMATOMUX_W { w: self }
    }
    #[doc = "Bits 9:12 - Number of clocks for sample time"]
    #[inline(always)]
    pub fn adc_stest(&mut self) -> ADC_STEST_W {
        ADC_STEST_W { w: self }
    }
    #[doc = "Bit 14 - Enable normal test clock"]
    #[inline(always)]
    pub fn rclk_pos_en(&mut self) -> RCLK_POS_EN_W {
        RCLK_POS_EN_W { w: self }
    }
    #[doc = "Bit 15 - Enable inverted test clock"]
    #[inline(always)]
    pub fn rclk_neg_en(&mut self) -> RCLK_NEG_EN_W {
        RCLK_NEG_EN_W { w: self }
    }
    #[doc = "Bit 16 - Enable normal APB2CLK for test output"]
    #[inline(always)]
    pub fn apb2clk_pos_en(&mut self) -> APB2CLK_POS_EN_W {
        APB2CLK_POS_EN_W { w: self }
    }
    #[doc = "Bit 17 - Enable inverted APB2CLK for test output"]
    #[inline(always)]
    pub fn apb2clk_neg_en(&mut self) -> APB2CLK_NEG_EN_W {
        APB2CLK_NEG_EN_W { w: self }
    }
    #[doc = "Bit 18 - Enables pull down on analog pads"]
    #[inline(always)]
    pub fn tm_analog_pd_en(&mut self) -> TM_ANALOG_PD_EN_W {
        TM_ANALOG_PD_EN_W { w: self }
    }
    #[doc = "Bit 19 - Enables a skip of all delay counters and eFuse read"]
    #[inline(always)]
    pub fn jmp2boot(&mut self) -> JMP2BOOT_W {
        JMP2BOOT_W { w: self }
    }
    #[doc = "Bit 20 - Enables a skip of all delay counters, eFuse read, and boot"]
    #[inline(always)]
    pub fn skipboot(&mut self) -> SKIPBOOT_W {
        SKIPBOOT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Analog Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [analog_cntl](index.html) module"]
pub struct ANALOG_CNTL_SPEC;
impl crate::RegisterSpec for ANALOG_CNTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [analog_cntl::R](R) reader structure"]
impl crate::Readable for ANALOG_CNTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [analog_cntl::W](W) writer structure"]
impl crate::Writable for ANALOG_CNTL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ANALOG_CNTL to value 0"]
impl crate::Resettable for ANALOG_CNTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
