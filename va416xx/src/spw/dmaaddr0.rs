#[doc = "Register `DMAADDR0` reader"]
pub type R = crate::R<Dmaaddr0Spec>;
#[doc = "Register `DMAADDR0` writer"]
pub type W = crate::W<Dmaaddr0Spec>;
#[doc = "Field `ADDR` reader - Address"]
pub type AddrR = crate::FieldReader;
#[doc = "Field `ADDR` writer - Address"]
pub type AddrW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `MASK` reader - Mask"]
pub type MaskR = crate::FieldReader;
#[doc = "Field `MASK` writer - Mask"]
pub type MaskW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Address"]
    #[inline(always)]
    pub fn addr(&self) -> AddrR {
        AddrR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Mask"]
    #[inline(always)]
    pub fn mask(&self) -> MaskR {
        MaskR::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Address"]
    #[inline(always)]
    #[must_use]
    pub fn addr(&mut self) -> AddrW<Dmaaddr0Spec> {
        AddrW::new(self, 0)
    }
    #[doc = "Bits 8:15 - Mask"]
    #[inline(always)]
    #[must_use]
    pub fn mask(&mut self) -> MaskW<Dmaaddr0Spec> {
        MaskW::new(self, 8)
    }
}
#[doc = "DMA Receiver Table Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmaaddr0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmaaddr0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dmaaddr0Spec;
impl crate::RegisterSpec for Dmaaddr0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmaaddr0::R`](R) reader structure"]
impl crate::Readable for Dmaaddr0Spec {}
#[doc = "`write(|w| ..)` method takes [`dmaaddr0::W`](W) writer structure"]
impl crate::Writable for Dmaaddr0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DMAADDR0 to value 0"]
impl crate::Resettable for Dmaaddr0Spec {
    const RESET_VALUE: u32 = 0;
}
