#[doc = "Register `DMAMAXLEN0` reader"]
pub type R = crate::R<Dmamaxlen0Spec>;
#[doc = "Register `DMAMAXLEN0` writer"]
pub type W = crate::W<Dmamaxlen0Spec>;
#[doc = "Field `RXMAXLEN` reader - Receiver packet maximum length in bytes"]
pub type RxmaxlenR = crate::FieldReader<u32>;
#[doc = "Field `RXMAXLEN` writer - Receiver packet maximum length in bytes"]
pub type RxmaxlenW<'a, REG> = crate::FieldWriter<'a, REG, 23, u32>;
impl R {
    #[doc = "Bits 2:24 - Receiver packet maximum length in bytes"]
    #[inline(always)]
    pub fn rxmaxlen(&self) -> RxmaxlenR {
        RxmaxlenR::new((self.bits >> 2) & 0x007f_ffff)
    }
}
impl W {
    #[doc = "Bits 2:24 - Receiver packet maximum length in bytes"]
    #[inline(always)]
    #[must_use]
    pub fn rxmaxlen(&mut self) -> RxmaxlenW<Dmamaxlen0Spec> {
        RxmaxlenW::new(self, 2)
    }
}
#[doc = "DMA RX Maximum Length Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmamaxlen0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmamaxlen0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dmamaxlen0Spec;
impl crate::RegisterSpec for Dmamaxlen0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmamaxlen0::R`](R) reader structure"]
impl crate::Readable for Dmamaxlen0Spec {}
#[doc = "`write(|w| ..)` method takes [`dmamaxlen0::W`](W) writer structure"]
impl crate::Writable for Dmamaxlen0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DMAMAXLEN0 to value 0"]
impl crate::Resettable for Dmamaxlen0Spec {
    const RESET_VALUE: u32 = 0;
}
