#[doc = "Register `DMA_ACTIVE_CLR` reader"]
pub type R = crate::R<DmaActiveClrSpec>;
#[doc = "Register `DMA_ACTIVE_CLR` writer"]
pub type W = crate::W<DmaActiveClrSpec>;
#[doc = "Field `CH0` reader - DMA Active clear"]
pub type Ch0R = crate::BitReader;
#[doc = "Field `CH0` writer - DMA Active clear"]
pub type Ch0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH1` reader - DMA Active clear"]
pub type Ch1R = crate::BitReader;
#[doc = "Field `CH1` writer - DMA Active clear"]
pub type Ch1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH2` reader - DMA Active clear"]
pub type Ch2R = crate::BitReader;
#[doc = "Field `CH2` writer - DMA Active clear"]
pub type Ch2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH3` reader - DMA Active clear"]
pub type Ch3R = crate::BitReader;
#[doc = "Field `CH3` writer - DMA Active clear"]
pub type Ch3W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - DMA Active clear"]
    #[inline(always)]
    pub fn ch0(&self) -> Ch0R {
        Ch0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DMA Active clear"]
    #[inline(always)]
    pub fn ch1(&self) -> Ch1R {
        Ch1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - DMA Active clear"]
    #[inline(always)]
    pub fn ch2(&self) -> Ch2R {
        Ch2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - DMA Active clear"]
    #[inline(always)]
    pub fn ch3(&self) -> Ch3R {
        Ch3R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DMA Active clear"]
    #[inline(always)]
    #[must_use]
    pub fn ch0(&mut self) -> Ch0W<DmaActiveClrSpec> {
        Ch0W::new(self, 0)
    }
    #[doc = "Bit 1 - DMA Active clear"]
    #[inline(always)]
    #[must_use]
    pub fn ch1(&mut self) -> Ch1W<DmaActiveClrSpec> {
        Ch1W::new(self, 1)
    }
    #[doc = "Bit 2 - DMA Active clear"]
    #[inline(always)]
    #[must_use]
    pub fn ch2(&mut self) -> Ch2W<DmaActiveClrSpec> {
        Ch2W::new(self, 2)
    }
    #[doc = "Bit 3 - DMA Active clear"]
    #[inline(always)]
    #[must_use]
    pub fn ch3(&mut self) -> Ch3W<DmaActiveClrSpec> {
        Ch3W::new(self, 3)
    }
}
#[doc = "DMA active clear\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dma_active_clr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dma_active_clr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DmaActiveClrSpec;
impl crate::RegisterSpec for DmaActiveClrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dma_active_clr::R`](R) reader structure"]
impl crate::Readable for DmaActiveClrSpec {}
#[doc = "`write(|w| ..)` method takes [`dma_active_clr::W`](W) writer structure"]
impl crate::Writable for DmaActiveClrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DMA_ACTIVE_CLR to value 0"]
impl crate::Resettable for DmaActiveClrSpec {
    const RESET_VALUE: u32 = 0;
}
