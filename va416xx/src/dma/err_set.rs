#[doc = "Register `ERR_SET` reader"]
pub type R = crate::R<ErrSetSpec>;
#[doc = "Register `ERR_SET` writer"]
pub type W = crate::W<ErrSetSpec>;
#[doc = "Field `ERR_SET` reader - Set Error"]
pub type ErrSetR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Set Error"]
    #[inline(always)]
    pub fn err_set(&self) -> ErrSetR {
        ErrSetR::new((self.bits & 1) != 0)
    }
}
impl W {}
#[doc = "DMA bus error set\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`err_set::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`err_set::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ErrSetSpec;
impl crate::RegisterSpec for ErrSetSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`err_set::R`](R) reader structure"]
impl crate::Readable for ErrSetSpec {}
#[doc = "`write(|w| ..)` method takes [`err_set::W`](W) writer structure"]
impl crate::Writable for ErrSetSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ERR_SET to value 0"]
impl crate::Resettable for ErrSetSpec {
    const RESET_VALUE: u32 = 0;
}
