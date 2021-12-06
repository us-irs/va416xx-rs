#[doc = "Register `ERR_CLR` reader"]
pub type R = crate::R<ErrClrSpec>;
#[doc = "Register `ERR_CLR` writer"]
pub type W = crate::W<ErrClrSpec>;
#[doc = "Field `ERR_CLR` reader - Error Clear"]
pub type ErrClrR = crate::BitReader;
#[doc = "Field `ERR_CLR` writer - Error Clear"]
pub type ErrClrW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Error Clear"]
    #[inline(always)]
    pub fn err_clr(&self) -> ErrClrR {
        ErrClrR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Error Clear"]
    #[inline(always)]
    #[must_use]
    pub fn err_clr(&mut self) -> ErrClrW<ErrClrSpec> {
        ErrClrW::new(self, 0)
    }
}
#[doc = "DMA bus error clear\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`err_clr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`err_clr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ErrClrSpec;
impl crate::RegisterSpec for ErrClrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`err_clr::R`](R) reader structure"]
impl crate::Readable for ErrClrSpec {}
#[doc = "`write(|w| ..)` method takes [`err_clr::W`](W) writer structure"]
impl crate::Writable for ErrClrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ERR_CLR to value 0"]
impl crate::Resettable for ErrClrSpec {
    const RESET_VALUE: u32 = 0;
}
