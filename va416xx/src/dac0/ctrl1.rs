#[doc = "Register `CTRL1` reader"]
pub type R = crate::R<Ctrl1Spec>;
#[doc = "Register `CTRL1` writer"]
pub type W = crate::W<Ctrl1Spec>;
#[doc = "Field `DAC_SETTLING` reader - Sets the the amount of time in microseconds the control FSM waits for the DAC settling time"]
pub type DacSettlingR = crate::FieldReader;
#[doc = "Field `DAC_SETTLING` writer - Sets the the amount of time in microseconds the control FSM waits for the DAC settling time"]
pub type DacSettlingW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `DAC_EN` reader - Enables the DAC analog block"]
pub type DacEnR = crate::BitReader;
#[doc = "Field `DAC_EN` writer - Enables the DAC analog block"]
pub type DacEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 5:7 - Sets the the amount of time in microseconds the control FSM waits for the DAC settling time"]
    #[inline(always)]
    pub fn dac_settling(&self) -> DacSettlingR {
        DacSettlingR::new(((self.bits >> 5) & 7) as u8)
    }
    #[doc = "Bit 8 - Enables the DAC analog block"]
    #[inline(always)]
    pub fn dac_en(&self) -> DacEnR {
        DacEnR::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 5:7 - Sets the the amount of time in microseconds the control FSM waits for the DAC settling time"]
    #[inline(always)]
    #[must_use]
    pub fn dac_settling(&mut self) -> DacSettlingW<Ctrl1Spec> {
        DacSettlingW::new(self, 5)
    }
    #[doc = "Bit 8 - Enables the DAC analog block"]
    #[inline(always)]
    #[must_use]
    pub fn dac_en(&mut self) -> DacEnW<Ctrl1Spec> {
        DacEnW::new(self, 8)
    }
}
#[doc = "Control Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ctrl1Spec;
impl crate::RegisterSpec for Ctrl1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrl1::R`](R) reader structure"]
impl crate::Readable for Ctrl1Spec {}
#[doc = "`write(|w| ..)` method takes [`ctrl1::W`](W) writer structure"]
impl crate::Writable for Ctrl1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTRL1 to value 0"]
impl crate::Resettable for Ctrl1Spec {
    const RESET_VALUE: u32 = 0;
}
