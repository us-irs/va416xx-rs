#[doc = "Register `CHNL_PRI_ALT_SET` reader"]
pub type R = crate::R<ChnlPriAltSetSpec>;
#[doc = "Register `CHNL_PRI_ALT_SET` writer"]
pub type W = crate::W<ChnlPriAltSetSpec>;
#[doc = "Field `CH0` reader - Channel PRI_ALT set"]
pub type Ch0R = crate::BitReader;
#[doc = "Field `CH0` writer - Channel PRI_ALT set"]
pub type Ch0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH1` reader - Channel PRI_ALT set"]
pub type Ch1R = crate::BitReader;
#[doc = "Field `CH1` writer - Channel PRI_ALT set"]
pub type Ch1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH2` reader - Channel PRI_ALT set"]
pub type Ch2R = crate::BitReader;
#[doc = "Field `CH2` writer - Channel PRI_ALT set"]
pub type Ch2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH3` reader - Channel PRI_ALT set"]
pub type Ch3R = crate::BitReader;
#[doc = "Field `CH3` writer - Channel PRI_ALT set"]
pub type Ch3W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Channel PRI_ALT set"]
    #[inline(always)]
    pub fn ch0(&self) -> Ch0R {
        Ch0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Channel PRI_ALT set"]
    #[inline(always)]
    pub fn ch1(&self) -> Ch1R {
        Ch1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Channel PRI_ALT set"]
    #[inline(always)]
    pub fn ch2(&self) -> Ch2R {
        Ch2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Channel PRI_ALT set"]
    #[inline(always)]
    pub fn ch3(&self) -> Ch3R {
        Ch3R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Channel PRI_ALT set"]
    #[inline(always)]
    pub fn ch0(&mut self) -> Ch0W<ChnlPriAltSetSpec> {
        Ch0W::new(self, 0)
    }
    #[doc = "Bit 1 - Channel PRI_ALT set"]
    #[inline(always)]
    pub fn ch1(&mut self) -> Ch1W<ChnlPriAltSetSpec> {
        Ch1W::new(self, 1)
    }
    #[doc = "Bit 2 - Channel PRI_ALT set"]
    #[inline(always)]
    pub fn ch2(&mut self) -> Ch2W<ChnlPriAltSetSpec> {
        Ch2W::new(self, 2)
    }
    #[doc = "Bit 3 - Channel PRI_ALT set"]
    #[inline(always)]
    pub fn ch3(&mut self) -> Ch3W<ChnlPriAltSetSpec> {
        Ch3W::new(self, 3)
    }
}
#[doc = "DMA channel primary alternate set\n\nYou can [`read`](crate::Reg::read) this register and get [`chnl_pri_alt_set::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chnl_pri_alt_set::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ChnlPriAltSetSpec;
impl crate::RegisterSpec for ChnlPriAltSetSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`chnl_pri_alt_set::R`](R) reader structure"]
impl crate::Readable for ChnlPriAltSetSpec {}
#[doc = "`write(|w| ..)` method takes [`chnl_pri_alt_set::W`](W) writer structure"]
impl crate::Writable for ChnlPriAltSetSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CHNL_PRI_ALT_SET to value 0"]
impl crate::Resettable for ChnlPriAltSetSpec {
    const RESET_VALUE: u32 = 0;
}
