#[doc = "Register `S0_ADDRESSMASK` reader"]
pub type R = crate::R<S0AddressmaskSpec>;
#[doc = "Register `S0_ADDRESSMASK` writer"]
pub type W = crate::W<S0AddressmaskSpec>;
#[doc = "Field `RWMASK` reader - Read/Write mask"]
pub type RwmaskR = crate::BitReader;
#[doc = "Field `RWMASK` writer - Read/Write mask"]
pub type RwmaskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MASK` reader - Address mask value"]
pub type MaskR = crate::FieldReader<u16>;
#[doc = "Field `MASK` writer - Address mask value"]
pub type MaskW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bit 0 - Read/Write mask"]
    #[inline(always)]
    pub fn rwmask(&self) -> RwmaskR {
        RwmaskR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:10 - Address mask value"]
    #[inline(always)]
    pub fn mask(&self) -> MaskR {
        MaskR::new(((self.bits >> 1) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - Read/Write mask"]
    #[inline(always)]
    #[must_use]
    pub fn rwmask(&mut self) -> RwmaskW<S0AddressmaskSpec> {
        RwmaskW::new(self, 0)
    }
    #[doc = "Bits 1:10 - Address mask value"]
    #[inline(always)]
    #[must_use]
    pub fn mask(&mut self) -> MaskW<S0AddressmaskSpec> {
        MaskW::new(self, 1)
    }
}
#[doc = "Slave I2C Address Mask value\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`s0_addressmask::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`s0_addressmask::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct S0AddressmaskSpec;
impl crate::RegisterSpec for S0AddressmaskSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`s0_addressmask::R`](R) reader structure"]
impl crate::Readable for S0AddressmaskSpec {}
#[doc = "`write(|w| ..)` method takes [`s0_addressmask::W`](W) writer structure"]
impl crate::Writable for S0AddressmaskSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets S0_ADDRESSMASK to value 0"]
impl crate::Resettable for S0AddressmaskSpec {
    const RESET_VALUE: u32 = 0;
}
