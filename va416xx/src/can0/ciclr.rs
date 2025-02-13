#[doc = "Register `CICLR` reader"]
pub type R = crate::R<CiclrSpec>;
#[doc = "Register `CICLR` writer"]
pub type W = crate::W<CiclrSpec>;
#[doc = "Field `ICLR` reader - Buffer Interrupt Clear\\[14:0\\]"]
pub type IclrR = crate::FieldReader<u16>;
#[doc = "Field `ICLR` writer - Buffer Interrupt Clear\\[14:0\\]"]
pub type IclrW<'a, REG> = crate::FieldWriter<'a, REG, 15, u16>;
#[doc = "Field `EICLR` reader - Error Interrupt Clear"]
pub type EiclrR = crate::BitReader;
#[doc = "Field `EICLR` writer - Error Interrupt Clear"]
pub type EiclrW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:14 - Buffer Interrupt Clear\\[14:0\\]"]
    #[inline(always)]
    pub fn iclr(&self) -> IclrR {
        IclrR::new((self.bits & 0x7fff) as u16)
    }
    #[doc = "Bit 15 - Error Interrupt Clear"]
    #[inline(always)]
    pub fn eiclr(&self) -> EiclrR {
        EiclrR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:14 - Buffer Interrupt Clear\\[14:0\\]"]
    #[inline(always)]
    pub fn iclr(&mut self) -> IclrW<CiclrSpec> {
        IclrW::new(self, 0)
    }
    #[doc = "Bit 15 - Error Interrupt Clear"]
    #[inline(always)]
    pub fn eiclr(&mut self) -> EiclrW<CiclrSpec> {
        EiclrW::new(self, 15)
    }
}
#[doc = "CAN Interrupt Clear Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ciclr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ciclr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CiclrSpec;
impl crate::RegisterSpec for CiclrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ciclr::R`](R) reader structure"]
impl crate::Readable for CiclrSpec {}
#[doc = "`write(|w| ..)` method takes [`ciclr::W`](W) writer structure"]
impl crate::Writable for CiclrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CICLR to value 0"]
impl crate::Resettable for CiclrSpec {
    const RESET_VALUE: u32 = 0;
}
