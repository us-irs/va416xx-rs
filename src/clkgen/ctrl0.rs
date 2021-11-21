#[doc = "Register `CTRL0` reader"]
pub struct R(crate::R<CTRL0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTRL0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTRL0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTRL0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTRL0` writer"]
pub struct W(crate::W<CTRL0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTRL0_SPEC>;
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
impl From<crate::W<CTRL0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTRL0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SYS_CLK_LOST_DET_EN` reader - Enable the circuit that detects loss of SYS_CLK"]
pub struct SYS_CLK_LOST_DET_EN_R(crate::FieldReader<bool, bool>);
impl SYS_CLK_LOST_DET_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SYS_CLK_LOST_DET_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SYS_CLK_LOST_DET_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SYS_CLK_LOST_DET_EN` writer - Enable the circuit that detects loss of SYS_CLK"]
pub struct SYS_CLK_LOST_DET_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SYS_CLK_LOST_DET_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | ((value as u32 & 0x01) << 31);
        self.w
    }
}
#[doc = "Field `PLL_RESET` reader - Writing this bit to 1 puts the PLL into reset"]
pub struct PLL_RESET_R(crate::FieldReader<bool, bool>);
impl PLL_RESET_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PLL_RESET_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PLL_RESET_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PLL_RESET` writer - Writing this bit to 1 puts the PLL into reset"]
pub struct PLL_RESET_W<'a> {
    w: &'a mut W,
}
impl<'a> PLL_RESET_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | ((value as u32 & 0x01) << 30);
        self.w
    }
}
#[doc = "Field `CLK_DIV_SEL` reader - Selects the PLL out divider to divide by 1/2/4/8"]
pub struct CLK_DIV_SEL_R(crate::FieldReader<u8, u8>);
impl CLK_DIV_SEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CLK_DIV_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CLK_DIV_SEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CLK_DIV_SEL` writer - Selects the PLL out divider to divide by 1/2/4/8"]
pub struct CLK_DIV_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> CLK_DIV_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 28)) | ((value as u32 & 0x03) << 28);
        self.w
    }
}
#[doc = "Field `PLL_CLKR` reader - PLL Symbol; selects the values 1-16 for the reference divider"]
pub struct PLL_CLKR_R(crate::FieldReader<u8, u8>);
impl PLL_CLKR_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PLL_CLKR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PLL_CLKR_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PLL_CLKR` writer - PLL Symbol; selects the values 1-16 for the reference divider"]
pub struct PLL_CLKR_W<'a> {
    w: &'a mut W,
}
impl<'a> PLL_CLKR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 24)) | ((value as u32 & 0x0f) << 24);
        self.w
    }
}
#[doc = "Field `PLL_CLKF` reader - PLL Symbol; selects the values 1-64 for the multiplication factor"]
pub struct PLL_CLKF_R(crate::FieldReader<u8, u8>);
impl PLL_CLKF_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PLL_CLKF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PLL_CLKF_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PLL_CLKF` writer - PLL Symbol; selects the values 1-64 for the multiplication factor"]
pub struct PLL_CLKF_W<'a> {
    w: &'a mut W,
}
impl<'a> PLL_CLKF_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 18)) | ((value as u32 & 0x3f) << 18);
        self.w
    }
}
#[doc = "Field `PLL_CLKOD` reader - PLL Symbol; selects the values 1-16 for the post VCO divider"]
pub struct PLL_CLKOD_R(crate::FieldReader<u8, u8>);
impl PLL_CLKOD_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PLL_CLKOD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PLL_CLKOD_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PLL_CLKOD` writer - PLL Symbol; selects the values 1-16 for the post VCO divider"]
pub struct PLL_CLKOD_W<'a> {
    w: &'a mut W,
}
impl<'a> PLL_CLKOD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 14)) | ((value as u32 & 0x0f) << 14);
        self.w
    }
}
#[doc = "Field `PLL_BWADJ` reader - PLL Symbol; selects the values 1-64 for the bandwidth divider"]
pub struct PLL_BWADJ_R(crate::FieldReader<u8, u8>);
impl PLL_BWADJ_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PLL_BWADJ_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PLL_BWADJ_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PLL_BWADJ` writer - PLL Symbol; selects the values 1-64 for the bandwidth divider"]
pub struct PLL_BWADJ_W<'a> {
    w: &'a mut W,
}
impl<'a> PLL_BWADJ_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 8)) | ((value as u32 & 0x3f) << 8);
        self.w
    }
}
#[doc = "Field `PLL_TEST` reader - PLL Symbol; Reference-to-counters-to-output bypass when high"]
pub struct PLL_TEST_R(crate::FieldReader<bool, bool>);
impl PLL_TEST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PLL_TEST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PLL_TEST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PLL_TEST` writer - PLL Symbol; Reference-to-counters-to-output bypass when high"]
pub struct PLL_TEST_W<'a> {
    w: &'a mut W,
}
impl<'a> PLL_TEST_W<'a> {
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
#[doc = "Field `PLL_BYPASS` reader - PLL Symbol; reference-to-output bypass when high"]
pub struct PLL_BYPASS_R(crate::FieldReader<bool, bool>);
impl PLL_BYPASS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PLL_BYPASS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PLL_BYPASS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PLL_BYPASS` writer - PLL Symbol; reference-to-output bypass when high"]
pub struct PLL_BYPASS_W<'a> {
    w: &'a mut W,
}
impl<'a> PLL_BYPASS_W<'a> {
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
#[doc = "Field `PLL_PWDN` reader - PLL Symbol; power down when high"]
pub struct PLL_PWDN_R(crate::FieldReader<bool, bool>);
impl PLL_PWDN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PLL_PWDN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PLL_PWDN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PLL_PWDN` writer - PLL Symbol; power down when high"]
pub struct PLL_PWDN_W<'a> {
    w: &'a mut W,
}
impl<'a> PLL_PWDN_W<'a> {
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
#[doc = "Field `PLL_INTFB` reader - PLL Symbol; select internal feedback path when high rather than FCLK"]
pub struct PLL_INTFB_R(crate::FieldReader<bool, bool>);
impl PLL_INTFB_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PLL_INTFB_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PLL_INTFB_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PLL_INTFB` writer - PLL Symbol; select internal feedback path when high rather than FCLK"]
pub struct PLL_INTFB_W<'a> {
    w: &'a mut W,
}
impl<'a> PLL_INTFB_W<'a> {
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
#[doc = "Field `CLKSEL_SYS` reader - Input clock select to PLL"]
pub struct CLKSEL_SYS_R(crate::FieldReader<u8, u8>);
impl CLKSEL_SYS_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CLKSEL_SYS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CLKSEL_SYS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CLKSEL_SYS` writer - Input clock select to PLL"]
pub struct CLKSEL_SYS_W<'a> {
    w: &'a mut W,
}
impl<'a> CLKSEL_SYS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | ((value as u32 & 0x03) << 2);
        self.w
    }
}
#[doc = "Field `REF_CLK_SEL` reader - PLL Reference Clock Select"]
pub struct REF_CLK_SEL_R(crate::FieldReader<u8, u8>);
impl REF_CLK_SEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        REF_CLK_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for REF_CLK_SEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `REF_CLK_SEL` writer - PLL Reference Clock Select"]
pub struct REF_CLK_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> REF_CLK_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bit 31 - Enable the circuit that detects loss of SYS_CLK"]
    #[inline(always)]
    pub fn sys_clk_lost_det_en(&self) -> SYS_CLK_LOST_DET_EN_R {
        SYS_CLK_LOST_DET_EN_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bit 30 - Writing this bit to 1 puts the PLL into reset"]
    #[inline(always)]
    pub fn pll_reset(&self) -> PLL_RESET_R {
        PLL_RESET_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bits 28:29 - Selects the PLL out divider to divide by 1/2/4/8"]
    #[inline(always)]
    pub fn clk_div_sel(&self) -> CLK_DIV_SEL_R {
        CLK_DIV_SEL_R::new(((self.bits >> 28) & 0x03) as u8)
    }
    #[doc = "Bits 24:27 - PLL Symbol; selects the values 1-16 for the reference divider"]
    #[inline(always)]
    pub fn pll_clkr(&self) -> PLL_CLKR_R {
        PLL_CLKR_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 18:23 - PLL Symbol; selects the values 1-64 for the multiplication factor"]
    #[inline(always)]
    pub fn pll_clkf(&self) -> PLL_CLKF_R {
        PLL_CLKF_R::new(((self.bits >> 18) & 0x3f) as u8)
    }
    #[doc = "Bits 14:17 - PLL Symbol; selects the values 1-16 for the post VCO divider"]
    #[inline(always)]
    pub fn pll_clkod(&self) -> PLL_CLKOD_R {
        PLL_CLKOD_R::new(((self.bits >> 14) & 0x0f) as u8)
    }
    #[doc = "Bits 8:13 - PLL Symbol; selects the values 1-64 for the bandwidth divider"]
    #[inline(always)]
    pub fn pll_bwadj(&self) -> PLL_BWADJ_R {
        PLL_BWADJ_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bit 7 - PLL Symbol; Reference-to-counters-to-output bypass when high"]
    #[inline(always)]
    pub fn pll_test(&self) -> PLL_TEST_R {
        PLL_TEST_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - PLL Symbol; reference-to-output bypass when high"]
    #[inline(always)]
    pub fn pll_bypass(&self) -> PLL_BYPASS_R {
        PLL_BYPASS_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - PLL Symbol; power down when high"]
    #[inline(always)]
    pub fn pll_pwdn(&self) -> PLL_PWDN_R {
        PLL_PWDN_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - PLL Symbol; select internal feedback path when high rather than FCLK"]
    #[inline(always)]
    pub fn pll_intfb(&self) -> PLL_INTFB_R {
        PLL_INTFB_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bits 2:3 - Input clock select to PLL"]
    #[inline(always)]
    pub fn clksel_sys(&self) -> CLKSEL_SYS_R {
        CLKSEL_SYS_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bits 0:1 - PLL Reference Clock Select"]
    #[inline(always)]
    pub fn ref_clk_sel(&self) -> REF_CLK_SEL_R {
        REF_CLK_SEL_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bit 31 - Enable the circuit that detects loss of SYS_CLK"]
    #[inline(always)]
    pub fn sys_clk_lost_det_en(&mut self) -> SYS_CLK_LOST_DET_EN_W {
        SYS_CLK_LOST_DET_EN_W { w: self }
    }
    #[doc = "Bit 30 - Writing this bit to 1 puts the PLL into reset"]
    #[inline(always)]
    pub fn pll_reset(&mut self) -> PLL_RESET_W {
        PLL_RESET_W { w: self }
    }
    #[doc = "Bits 28:29 - Selects the PLL out divider to divide by 1/2/4/8"]
    #[inline(always)]
    pub fn clk_div_sel(&mut self) -> CLK_DIV_SEL_W {
        CLK_DIV_SEL_W { w: self }
    }
    #[doc = "Bits 24:27 - PLL Symbol; selects the values 1-16 for the reference divider"]
    #[inline(always)]
    pub fn pll_clkr(&mut self) -> PLL_CLKR_W {
        PLL_CLKR_W { w: self }
    }
    #[doc = "Bits 18:23 - PLL Symbol; selects the values 1-64 for the multiplication factor"]
    #[inline(always)]
    pub fn pll_clkf(&mut self) -> PLL_CLKF_W {
        PLL_CLKF_W { w: self }
    }
    #[doc = "Bits 14:17 - PLL Symbol; selects the values 1-16 for the post VCO divider"]
    #[inline(always)]
    pub fn pll_clkod(&mut self) -> PLL_CLKOD_W {
        PLL_CLKOD_W { w: self }
    }
    #[doc = "Bits 8:13 - PLL Symbol; selects the values 1-64 for the bandwidth divider"]
    #[inline(always)]
    pub fn pll_bwadj(&mut self) -> PLL_BWADJ_W {
        PLL_BWADJ_W { w: self }
    }
    #[doc = "Bit 7 - PLL Symbol; Reference-to-counters-to-output bypass when high"]
    #[inline(always)]
    pub fn pll_test(&mut self) -> PLL_TEST_W {
        PLL_TEST_W { w: self }
    }
    #[doc = "Bit 6 - PLL Symbol; reference-to-output bypass when high"]
    #[inline(always)]
    pub fn pll_bypass(&mut self) -> PLL_BYPASS_W {
        PLL_BYPASS_W { w: self }
    }
    #[doc = "Bit 5 - PLL Symbol; power down when high"]
    #[inline(always)]
    pub fn pll_pwdn(&mut self) -> PLL_PWDN_W {
        PLL_PWDN_W { w: self }
    }
    #[doc = "Bit 4 - PLL Symbol; select internal feedback path when high rather than FCLK"]
    #[inline(always)]
    pub fn pll_intfb(&mut self) -> PLL_INTFB_W {
        PLL_INTFB_W { w: self }
    }
    #[doc = "Bits 2:3 - Input clock select to PLL"]
    #[inline(always)]
    pub fn clksel_sys(&mut self) -> CLKSEL_SYS_W {
        CLKSEL_SYS_W { w: self }
    }
    #[doc = "Bits 0:1 - PLL Reference Clock Select"]
    #[inline(always)]
    pub fn ref_clk_sel(&mut self) -> REF_CLK_SEL_W {
        REF_CLK_SEL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Clock Generation Module Control Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl0](index.html) module"]
pub struct CTRL0_SPEC;
impl crate::RegisterSpec for CTRL0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctrl0::R](R) reader structure"]
impl crate::Readable for CTRL0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctrl0::W](W) writer structure"]
impl crate::Writable for CTRL0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CTRL0 to value 0x30"]
impl crate::Resettable for CTRL0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x30
    }
}
