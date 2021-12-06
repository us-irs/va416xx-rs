#[doc = "Register `FIFO_CLR` reader"]
pub type R = crate::R<FifoClrSpec>;
#[doc = "Register `FIFO_CLR` writer"]
pub type W = crate::W<FifoClrSpec>;
#[doc = "Field `FIFO_CLR` writer - Clears the DAC FIFO. Always reads 0"]
pub type FifoClrW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Clears the DAC FIFO. Always reads 0"]
    #[inline(always)]
    #[must_use]
    pub fn fifo_clr(&mut self) -> FifoClrW<FifoClrSpec> {
        FifoClrW::new(self, 0)
    }
}
#[doc = "FIFO Clear\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fifo_clr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fifo_clr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FifoClrSpec;
impl crate::RegisterSpec for FifoClrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fifo_clr::R`](R) reader structure"]
impl crate::Readable for FifoClrSpec {}
#[doc = "`write(|w| ..)` method takes [`fifo_clr::W`](W) writer structure"]
impl crate::Writable for FifoClrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FIFO_CLR to value 0"]
impl crate::Resettable for FifoClrSpec {
    const RESET_VALUE: u32 = 0;
}
