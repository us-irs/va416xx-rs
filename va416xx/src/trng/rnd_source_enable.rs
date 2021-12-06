#[doc = "Register `RND_SOURCE_ENABLE` reader"]
pub type R = crate::R<RndSourceEnableSpec>;
#[doc = "Register `RND_SOURCE_ENABLE` writer"]
pub type W = crate::W<RndSourceEnableSpec>;
#[doc = "Field `RND_SRC_EN` reader - The entropy source, ring oscillator, is enabled"]
pub type RndSrcEnR = crate::BitReader;
#[doc = "Field `RND_SRC_EN` writer - The entropy source, ring oscillator, is enabled"]
pub type RndSrcEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - The entropy source, ring oscillator, is enabled"]
    #[inline(always)]
    pub fn rnd_src_en(&self) -> RndSrcEnR {
        RndSrcEnR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - The entropy source, ring oscillator, is enabled"]
    #[inline(always)]
    #[must_use]
    pub fn rnd_src_en(&mut self) -> RndSrcEnW<RndSourceEnableSpec> {
        RndSrcEnW::new(self, 0)
    }
}
#[doc = "Random Source Enable Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rnd_source_enable::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rnd_source_enable::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RndSourceEnableSpec;
impl crate::RegisterSpec for RndSourceEnableSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rnd_source_enable::R`](R) reader structure"]
impl crate::Readable for RndSourceEnableSpec {}
#[doc = "`write(|w| ..)` method takes [`rnd_source_enable::W`](W) writer structure"]
impl crate::Writable for RndSourceEnableSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RND_SOURCE_ENABLE to value 0"]
impl crate::Resettable for RndSourceEnableSpec {
    const RESET_VALUE: u32 = 0;
}
