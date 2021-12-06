#[doc = "Register `CHNL_REQ_MASK_CLR` reader"]
pub type R = crate::R<ChnlReqMaskClrSpec>;
#[doc = "Register `CHNL_REQ_MASK_CLR` writer"]
pub type W = crate::W<ChnlReqMaskClrSpec>;
#[doc = "Field `CH0` reader - Channel Request Mask clear"]
pub type Ch0R = crate::BitReader;
#[doc = "Field `CH0` writer - Channel Request Mask clear"]
pub type Ch0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH1` reader - Channel Request Mask clear"]
pub type Ch1R = crate::BitReader;
#[doc = "Field `CH1` writer - Channel Request Mask clear"]
pub type Ch1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH2` reader - Channel Request Mask clear"]
pub type Ch2R = crate::BitReader;
#[doc = "Field `CH2` writer - Channel Request Mask clear"]
pub type Ch2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH3` reader - Channel Request Mask clear"]
pub type Ch3R = crate::BitReader;
#[doc = "Field `CH3` writer - Channel Request Mask clear"]
pub type Ch3W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Channel Request Mask clear"]
    #[inline(always)]
    pub fn ch0(&self) -> Ch0R {
        Ch0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Channel Request Mask clear"]
    #[inline(always)]
    pub fn ch1(&self) -> Ch1R {
        Ch1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Channel Request Mask clear"]
    #[inline(always)]
    pub fn ch2(&self) -> Ch2R {
        Ch2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Channel Request Mask clear"]
    #[inline(always)]
    pub fn ch3(&self) -> Ch3R {
        Ch3R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Channel Request Mask clear"]
    #[inline(always)]
    #[must_use]
    pub fn ch0(&mut self) -> Ch0W<ChnlReqMaskClrSpec> {
        Ch0W::new(self, 0)
    }
    #[doc = "Bit 1 - Channel Request Mask clear"]
    #[inline(always)]
    #[must_use]
    pub fn ch1(&mut self) -> Ch1W<ChnlReqMaskClrSpec> {
        Ch1W::new(self, 1)
    }
    #[doc = "Bit 2 - Channel Request Mask clear"]
    #[inline(always)]
    #[must_use]
    pub fn ch2(&mut self) -> Ch2W<ChnlReqMaskClrSpec> {
        Ch2W::new(self, 2)
    }
    #[doc = "Bit 3 - Channel Request Mask clear"]
    #[inline(always)]
    #[must_use]
    pub fn ch3(&mut self) -> Ch3W<ChnlReqMaskClrSpec> {
        Ch3W::new(self, 3)
    }
}
#[doc = "DMA channel request mask clear\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`chnl_req_mask_clr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`chnl_req_mask_clr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ChnlReqMaskClrSpec;
impl crate::RegisterSpec for ChnlReqMaskClrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`chnl_req_mask_clr::R`](R) reader structure"]
impl crate::Readable for ChnlReqMaskClrSpec {}
#[doc = "`write(|w| ..)` method takes [`chnl_req_mask_clr::W`](W) writer structure"]
impl crate::Writable for ChnlReqMaskClrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CHNL_REQ_MASK_CLR to value 0"]
impl crate::Resettable for ChnlReqMaskClrSpec {
    const RESET_VALUE: u32 = 0;
}
