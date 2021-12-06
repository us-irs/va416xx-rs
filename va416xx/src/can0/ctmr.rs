#[doc = "Register `CTMR` reader"]
pub type R = crate::R<CtmrSpec>;
#[doc = "Register `CTMR` writer"]
pub type W = crate::W<CtmrSpec>;
#[doc = "Field `CTMR` reader - Time Stamp Counter"]
pub type CtmrR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Time Stamp Counter"]
    #[inline(always)]
    pub fn ctmr(&self) -> CtmrR {
        CtmrR::new((self.bits & 0xffff) as u16)
    }
}
impl W {}
#[doc = "CAN Timer Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctmr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctmr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtmrSpec;
impl crate::RegisterSpec for CtmrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctmr::R`](R) reader structure"]
impl crate::Readable for CtmrSpec {}
#[doc = "`write(|w| ..)` method takes [`ctmr::W`](W) writer structure"]
impl crate::Writable for CtmrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTMR to value 0"]
impl crate::Resettable for CtmrSpec {
    const RESET_VALUE: u32 = 0;
}
