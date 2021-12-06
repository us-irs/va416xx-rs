#[doc = "Register `ANALOG_CNTL` reader"]
pub type R = crate::R<AnalogCntlSpec>;
#[doc = "Register `ANALOG_CNTL` writer"]
pub type W = crate::W<AnalogCntlSpec>;
#[doc = "Field `TMOSC` reader - Test Mode"]
pub type TmoscR = crate::BitReader;
#[doc = "Field `TMOSC` writer - Test Mode"]
pub type TmoscW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TMPOKDIS` reader - Test Mode"]
pub type TmpokdisR = crate::BitReader;
#[doc = "Field `TMPOKDIS` writer - Test Mode"]
pub type TmpokdisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TM_ADCMUX_N` reader - Test Mode"]
pub type TmAdcmuxNR = crate::BitReader;
#[doc = "Field `TM_ADCMUX_N` writer - Test Mode"]
pub type TmAdcmuxNW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TM_ADCMUX_P` reader - Test Mode"]
pub type TmAdcmuxPR = crate::BitReader;
#[doc = "Field `TM_ADCMUX_P` writer - Test Mode"]
pub type TmAdcmuxPW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TMRATIO` reader - Test Mode"]
pub type TmratioR = crate::BitReader;
#[doc = "Field `TMRATIO` writer - Test Mode"]
pub type TmratioW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TMATOMUX` reader - Test Mode"]
pub type TmatomuxR = crate::FieldReader;
#[doc = "Field `TMATOMUX` writer - Test Mode"]
pub type TmatomuxW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `ADC_STEST` reader - Number of clocks for sample time"]
pub type AdcStestR = crate::FieldReader;
#[doc = "Field `ADC_STEST` writer - Number of clocks for sample time"]
pub type AdcStestW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `RCLK_POS_EN` reader - Enable normal test clock"]
pub type RclkPosEnR = crate::BitReader;
#[doc = "Field `RCLK_POS_EN` writer - Enable normal test clock"]
pub type RclkPosEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RCLK_NEG_EN` reader - Enable inverted test clock"]
pub type RclkNegEnR = crate::BitReader;
#[doc = "Field `RCLK_NEG_EN` writer - Enable inverted test clock"]
pub type RclkNegEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `APB2CLK_POS_EN` reader - Enable normal APB2CLK for test output"]
pub type Apb2clkPosEnR = crate::BitReader;
#[doc = "Field `APB2CLK_POS_EN` writer - Enable normal APB2CLK for test output"]
pub type Apb2clkPosEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `APB2CLK_NEG_EN` reader - Enable inverted APB2CLK for test output"]
pub type Apb2clkNegEnR = crate::BitReader;
#[doc = "Field `APB2CLK_NEG_EN` writer - Enable inverted APB2CLK for test output"]
pub type Apb2clkNegEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TM_ANALOG_PD_EN` reader - Enables pull down on analog pads"]
pub type TmAnalogPdEnR = crate::BitReader;
#[doc = "Field `TM_ANALOG_PD_EN` writer - Enables pull down on analog pads"]
pub type TmAnalogPdEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `JMP2BOOT` reader - Enables a skip of all delay counters and eFuse read"]
pub type Jmp2bootR = crate::BitReader;
#[doc = "Field `JMP2BOOT` writer - Enables a skip of all delay counters and eFuse read"]
pub type Jmp2bootW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SKIPBOOT` reader - Enables a skip of all delay counters, eFuse read, and boot"]
pub type SkipbootR = crate::BitReader;
#[doc = "Field `SKIPBOOT` writer - Enables a skip of all delay counters, eFuse read, and boot"]
pub type SkipbootW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Test Mode"]
    #[inline(always)]
    pub fn tmosc(&self) -> TmoscR {
        TmoscR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Test Mode"]
    #[inline(always)]
    pub fn tmpokdis(&self) -> TmpokdisR {
        TmpokdisR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Test Mode"]
    #[inline(always)]
    pub fn tm_adcmux_n(&self) -> TmAdcmuxNR {
        TmAdcmuxNR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Test Mode"]
    #[inline(always)]
    pub fn tm_adcmux_p(&self) -> TmAdcmuxPR {
        TmAdcmuxPR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Test Mode"]
    #[inline(always)]
    pub fn tmratio(&self) -> TmratioR {
        TmratioR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:6 - Test Mode"]
    #[inline(always)]
    pub fn tmatomux(&self) -> TmatomuxR {
        TmatomuxR::new(((self.bits >> 5) & 3) as u8)
    }
    #[doc = "Bits 9:12 - Number of clocks for sample time"]
    #[inline(always)]
    pub fn adc_stest(&self) -> AdcStestR {
        AdcStestR::new(((self.bits >> 9) & 0x0f) as u8)
    }
    #[doc = "Bit 14 - Enable normal test clock"]
    #[inline(always)]
    pub fn rclk_pos_en(&self) -> RclkPosEnR {
        RclkPosEnR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Enable inverted test clock"]
    #[inline(always)]
    pub fn rclk_neg_en(&self) -> RclkNegEnR {
        RclkNegEnR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Enable normal APB2CLK for test output"]
    #[inline(always)]
    pub fn apb2clk_pos_en(&self) -> Apb2clkPosEnR {
        Apb2clkPosEnR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Enable inverted APB2CLK for test output"]
    #[inline(always)]
    pub fn apb2clk_neg_en(&self) -> Apb2clkNegEnR {
        Apb2clkNegEnR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Enables pull down on analog pads"]
    #[inline(always)]
    pub fn tm_analog_pd_en(&self) -> TmAnalogPdEnR {
        TmAnalogPdEnR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Enables a skip of all delay counters and eFuse read"]
    #[inline(always)]
    pub fn jmp2boot(&self) -> Jmp2bootR {
        Jmp2bootR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Enables a skip of all delay counters, eFuse read, and boot"]
    #[inline(always)]
    pub fn skipboot(&self) -> SkipbootR {
        SkipbootR::new(((self.bits >> 20) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Test Mode"]
    #[inline(always)]
    #[must_use]
    pub fn tmosc(&mut self) -> TmoscW<AnalogCntlSpec> {
        TmoscW::new(self, 0)
    }
    #[doc = "Bit 1 - Test Mode"]
    #[inline(always)]
    #[must_use]
    pub fn tmpokdis(&mut self) -> TmpokdisW<AnalogCntlSpec> {
        TmpokdisW::new(self, 1)
    }
    #[doc = "Bit 2 - Test Mode"]
    #[inline(always)]
    #[must_use]
    pub fn tm_adcmux_n(&mut self) -> TmAdcmuxNW<AnalogCntlSpec> {
        TmAdcmuxNW::new(self, 2)
    }
    #[doc = "Bit 3 - Test Mode"]
    #[inline(always)]
    #[must_use]
    pub fn tm_adcmux_p(&mut self) -> TmAdcmuxPW<AnalogCntlSpec> {
        TmAdcmuxPW::new(self, 3)
    }
    #[doc = "Bit 4 - Test Mode"]
    #[inline(always)]
    #[must_use]
    pub fn tmratio(&mut self) -> TmratioW<AnalogCntlSpec> {
        TmratioW::new(self, 4)
    }
    #[doc = "Bits 5:6 - Test Mode"]
    #[inline(always)]
    #[must_use]
    pub fn tmatomux(&mut self) -> TmatomuxW<AnalogCntlSpec> {
        TmatomuxW::new(self, 5)
    }
    #[doc = "Bits 9:12 - Number of clocks for sample time"]
    #[inline(always)]
    #[must_use]
    pub fn adc_stest(&mut self) -> AdcStestW<AnalogCntlSpec> {
        AdcStestW::new(self, 9)
    }
    #[doc = "Bit 14 - Enable normal test clock"]
    #[inline(always)]
    #[must_use]
    pub fn rclk_pos_en(&mut self) -> RclkPosEnW<AnalogCntlSpec> {
        RclkPosEnW::new(self, 14)
    }
    #[doc = "Bit 15 - Enable inverted test clock"]
    #[inline(always)]
    #[must_use]
    pub fn rclk_neg_en(&mut self) -> RclkNegEnW<AnalogCntlSpec> {
        RclkNegEnW::new(self, 15)
    }
    #[doc = "Bit 16 - Enable normal APB2CLK for test output"]
    #[inline(always)]
    #[must_use]
    pub fn apb2clk_pos_en(&mut self) -> Apb2clkPosEnW<AnalogCntlSpec> {
        Apb2clkPosEnW::new(self, 16)
    }
    #[doc = "Bit 17 - Enable inverted APB2CLK for test output"]
    #[inline(always)]
    #[must_use]
    pub fn apb2clk_neg_en(&mut self) -> Apb2clkNegEnW<AnalogCntlSpec> {
        Apb2clkNegEnW::new(self, 17)
    }
    #[doc = "Bit 18 - Enables pull down on analog pads"]
    #[inline(always)]
    #[must_use]
    pub fn tm_analog_pd_en(&mut self) -> TmAnalogPdEnW<AnalogCntlSpec> {
        TmAnalogPdEnW::new(self, 18)
    }
    #[doc = "Bit 19 - Enables a skip of all delay counters and eFuse read"]
    #[inline(always)]
    #[must_use]
    pub fn jmp2boot(&mut self) -> Jmp2bootW<AnalogCntlSpec> {
        Jmp2bootW::new(self, 19)
    }
    #[doc = "Bit 20 - Enables a skip of all delay counters, eFuse read, and boot"]
    #[inline(always)]
    #[must_use]
    pub fn skipboot(&mut self) -> SkipbootW<AnalogCntlSpec> {
        SkipbootW::new(self, 20)
    }
}
#[doc = "Analog Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`analog_cntl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`analog_cntl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AnalogCntlSpec;
impl crate::RegisterSpec for AnalogCntlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`analog_cntl::R`](R) reader structure"]
impl crate::Readable for AnalogCntlSpec {}
#[doc = "`write(|w| ..)` method takes [`analog_cntl::W`](W) writer structure"]
impl crate::Writable for AnalogCntlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ANALOG_CNTL to value 0"]
impl crate::Resettable for AnalogCntlSpec {
    const RESET_VALUE: u32 = 0;
}
