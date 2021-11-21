#[doc = "Register `CTRL1` reader"]
pub struct R(crate::R<CTRL1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTRL1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTRL1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTRL1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTRL1` writer"]
pub struct W(crate::W<CTRL1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTRL1_SPEC>;
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
impl From<crate::W<CTRL1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTRL1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADC_CLK_DIV_SEL` reader - Clock divider select for ADC"]
pub struct ADC_CLK_DIV_SEL_R(crate::FieldReader<u8, u8>);
impl ADC_CLK_DIV_SEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        ADC_CLK_DIV_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADC_CLK_DIV_SEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADC_CLK_DIV_SEL` writer - Clock divider select for ADC"]
pub struct ADC_CLK_DIV_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC_CLK_DIV_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 5)) | ((value as u32 & 0x03) << 5);
        self.w
    }
}
#[doc = "Field `XTAL_N_EN` reader - Enables XTAL_N output"]
pub struct XTAL_N_EN_R(crate::FieldReader<bool, bool>);
impl XTAL_N_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        XTAL_N_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for XTAL_N_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `XTAL_N_EN` writer - Enables XTAL_N output"]
pub struct XTAL_N_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> XTAL_N_EN_W<'a> {
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
#[doc = "Field `XTAL_EN` reader - Enables the crystal oscillator"]
pub struct XTAL_EN_R(crate::FieldReader<bool, bool>);
impl XTAL_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        XTAL_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for XTAL_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `XTAL_EN` writer - Enables the crystal oscillator"]
pub struct XTAL_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> XTAL_EN_W<'a> {
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
#[doc = "Field `PLL_LOST_LOCK_DET_EN` reader - Enables the PLL lock lost detection circuit"]
pub struct PLL_LOST_LOCK_DET_EN_R(crate::FieldReader<bool, bool>);
impl PLL_LOST_LOCK_DET_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PLL_LOST_LOCK_DET_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PLL_LOST_LOCK_DET_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PLL_LOST_LOCK_DET_EN` writer - Enables the PLL lock lost detection circuit"]
pub struct PLL_LOST_LOCK_DET_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> PLL_LOST_LOCK_DET_EN_W<'a> {
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
#[doc = "Field `PLL_LCK_DET_REARM` reader - Resets/Rearms the PLL lock detect circuit"]
pub struct PLL_LCK_DET_REARM_R(crate::FieldReader<bool, bool>);
impl PLL_LCK_DET_REARM_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PLL_LCK_DET_REARM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PLL_LCK_DET_REARM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PLL_LCK_DET_REARM` writer - Resets/Rearms the PLL lock detect circuit"]
pub struct PLL_LCK_DET_REARM_W<'a> {
    w: &'a mut W,
}
impl<'a> PLL_LCK_DET_REARM_W<'a> {
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
#[doc = "Field `SYS_CLK_LOST_DET_REARM` reader - Resets/Rearms the SYS_CLK lost detection feature"]
pub struct SYS_CLK_LOST_DET_REARM_R(crate::FieldReader<bool, bool>);
impl SYS_CLK_LOST_DET_REARM_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SYS_CLK_LOST_DET_REARM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SYS_CLK_LOST_DET_REARM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SYS_CLK_LOST_DET_REARM` writer - Resets/Rearms the SYS_CLK lost detection feature"]
pub struct SYS_CLK_LOST_DET_REARM_W<'a> {
    w: &'a mut W,
}
impl<'a> SYS_CLK_LOST_DET_REARM_W<'a> {
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
    #[doc = "Bits 5:6 - Clock divider select for ADC"]
    #[inline(always)]
    pub fn adc_clk_div_sel(&self) -> ADC_CLK_DIV_SEL_R {
        ADC_CLK_DIV_SEL_R::new(((self.bits >> 5) & 0x03) as u8)
    }
    #[doc = "Bit 4 - Enables XTAL_N output"]
    #[inline(always)]
    pub fn xtal_n_en(&self) -> XTAL_N_EN_R {
        XTAL_N_EN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Enables the crystal oscillator"]
    #[inline(always)]
    pub fn xtal_en(&self) -> XTAL_EN_R {
        XTAL_EN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Enables the PLL lock lost detection circuit"]
    #[inline(always)]
    pub fn pll_lost_lock_det_en(&self) -> PLL_LOST_LOCK_DET_EN_R {
        PLL_LOST_LOCK_DET_EN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - Resets/Rearms the PLL lock detect circuit"]
    #[inline(always)]
    pub fn pll_lck_det_rearm(&self) -> PLL_LCK_DET_REARM_R {
        PLL_LCK_DET_REARM_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Resets/Rearms the SYS_CLK lost detection feature"]
    #[inline(always)]
    pub fn sys_clk_lost_det_rearm(&self) -> SYS_CLK_LOST_DET_REARM_R {
        SYS_CLK_LOST_DET_REARM_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 5:6 - Clock divider select for ADC"]
    #[inline(always)]
    pub fn adc_clk_div_sel(&mut self) -> ADC_CLK_DIV_SEL_W {
        ADC_CLK_DIV_SEL_W { w: self }
    }
    #[doc = "Bit 4 - Enables XTAL_N output"]
    #[inline(always)]
    pub fn xtal_n_en(&mut self) -> XTAL_N_EN_W {
        XTAL_N_EN_W { w: self }
    }
    #[doc = "Bit 3 - Enables the crystal oscillator"]
    #[inline(always)]
    pub fn xtal_en(&mut self) -> XTAL_EN_W {
        XTAL_EN_W { w: self }
    }
    #[doc = "Bit 2 - Enables the PLL lock lost detection circuit"]
    #[inline(always)]
    pub fn pll_lost_lock_det_en(&mut self) -> PLL_LOST_LOCK_DET_EN_W {
        PLL_LOST_LOCK_DET_EN_W { w: self }
    }
    #[doc = "Bit 1 - Resets/Rearms the PLL lock detect circuit"]
    #[inline(always)]
    pub fn pll_lck_det_rearm(&mut self) -> PLL_LCK_DET_REARM_W {
        PLL_LCK_DET_REARM_W { w: self }
    }
    #[doc = "Bit 0 - Resets/Rearms the SYS_CLK lost detection feature"]
    #[inline(always)]
    pub fn sys_clk_lost_det_rearm(&mut self) -> SYS_CLK_LOST_DET_REARM_W {
        SYS_CLK_LOST_DET_REARM_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Clock Generation Module Control Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl1](index.html) module"]
pub struct CTRL1_SPEC;
impl crate::RegisterSpec for CTRL1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctrl1::R](R) reader structure"]
impl crate::Readable for CTRL1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctrl1::W](W) writer structure"]
impl crate::Writable for CTRL1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CTRL1 to value 0"]
impl crate::Resettable for CTRL1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
