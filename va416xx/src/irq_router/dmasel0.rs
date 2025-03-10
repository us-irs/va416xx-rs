#[doc = "Register `DMASEL0` reader"]
pub type R = crate::R<Dmasel0Spec>;
#[doc = "Register `DMASEL0` writer"]
pub type W = crate::W<Dmasel0Spec>;
#[doc = "Field `DMASEL` reader - DMA trigger source selection value"]
pub type DmaselR = crate::FieldReader;
#[doc = "Field `DMASEL` writer - DMA trigger source selection value"]
pub type DmaselW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bits 0:6 - DMA trigger source selection value"]
    #[inline(always)]
    pub fn dmasel(&self) -> DmaselR {
        DmaselR::new((self.bits & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - DMA trigger source selection value"]
    #[inline(always)]
    pub fn dmasel(&mut self) -> DmaselW<Dmasel0Spec> {
        DmaselW::new(self, 0)
    }
}
#[doc = "Interrupt select for DMA channel 0\n\nYou can [`read`](crate::Reg::read) this register and get [`dmasel0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmasel0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dmasel0Spec;
impl crate::RegisterSpec for Dmasel0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmasel0::R`](R) reader structure"]
impl crate::Readable for Dmasel0Spec {}
#[doc = "`write(|w| ..)` method takes [`dmasel0::W`](W) writer structure"]
impl crate::Writable for Dmasel0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DMASEL0 to value 0x7f"]
impl crate::Resettable for Dmasel0Spec {
    const RESET_VALUE: u32 = 0x7f;
}
