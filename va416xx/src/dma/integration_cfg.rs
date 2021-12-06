#[doc = "Register `INTEGRATION_CFG` reader"]
pub type R = crate::R<IntegrationCfgSpec>;
#[doc = "Register `INTEGRATION_CFG` writer"]
pub type W = crate::W<IntegrationCfgSpec>;
#[doc = "Field `INT_TEST_EN` reader - Error Clear"]
pub type IntTestEnR = crate::BitReader;
#[doc = "Field `INT_TEST_EN` writer - Error Clear"]
pub type IntTestEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Error Clear"]
    #[inline(always)]
    pub fn int_test_en(&self) -> IntTestEnR {
        IntTestEnR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Error Clear"]
    #[inline(always)]
    #[must_use]
    pub fn int_test_en(&mut self) -> IntTestEnW<IntegrationCfgSpec> {
        IntTestEnW::new(self, 0)
    }
}
#[doc = "DMA integration configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`integration_cfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`integration_cfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntegrationCfgSpec;
impl crate::RegisterSpec for IntegrationCfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`integration_cfg::R`](R) reader structure"]
impl crate::Readable for IntegrationCfgSpec {}
#[doc = "`write(|w| ..)` method takes [`integration_cfg::W`](W) writer structure"]
impl crate::Writable for IntegrationCfgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INTEGRATION_CFG to value 0"]
impl crate::Resettable for IntegrationCfgSpec {
    const RESET_VALUE: u32 = 0;
}
