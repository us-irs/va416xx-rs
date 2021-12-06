#[doc = "Register `RAM_TRAP_ADDR0` reader"]
pub type R = crate::R<RamTrapAddr0Spec>;
#[doc = "Register `RAM_TRAP_ADDR0` writer"]
pub type W = crate::W<RamTrapAddr0Spec>;
#[doc = "Field `ADDR` reader - Address bits for trap match"]
pub type AddrR = crate::FieldReader<u32>;
#[doc = "Field `ADDR` writer - Address bits for trap match"]
pub type AddrW<'a, REG> = crate::FieldWriter<'a, REG, 29, u32>;
#[doc = "Field `ENABLE` reader - Enable Trap mode"]
pub type EnableR = crate::BitReader;
#[doc = "Field `ENABLE` writer - Enable Trap mode"]
pub type EnableW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 2:30 - Address bits for trap match"]
    #[inline(always)]
    pub fn addr(&self) -> AddrR {
        AddrR::new((self.bits >> 2) & 0x1fff_ffff)
    }
    #[doc = "Bit 31 - Enable Trap mode"]
    #[inline(always)]
    pub fn enable(&self) -> EnableR {
        EnableR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 2:30 - Address bits for trap match"]
    #[inline(always)]
    #[must_use]
    pub fn addr(&mut self) -> AddrW<RamTrapAddr0Spec> {
        AddrW::new(self, 2)
    }
    #[doc = "Bit 31 - Enable Trap mode"]
    #[inline(always)]
    #[must_use]
    pub fn enable(&mut self) -> EnableW<RamTrapAddr0Spec> {
        EnableW::new(self, 31)
    }
}
#[doc = "RAM0 EDAC Trap Address\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ram_trap_addr0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ram_trap_addr0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RamTrapAddr0Spec;
impl crate::RegisterSpec for RamTrapAddr0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ram_trap_addr0::R`](R) reader structure"]
impl crate::Readable for RamTrapAddr0Spec {}
#[doc = "`write(|w| ..)` method takes [`ram_trap_addr0::W`](W) writer structure"]
impl crate::Writable for RamTrapAddr0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RAM_TRAP_ADDR0 to value 0"]
impl crate::Resettable for RamTrapAddr0Spec {
    const RESET_VALUE: u32 = 0;
}
