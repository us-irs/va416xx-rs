#[doc = "Register `MAC_WDOG_TO` reader"]
pub type R = crate::R<MacWdogToSpec>;
#[doc = "Register `MAC_WDOG_TO` writer"]
pub type W = crate::W<MacWdogToSpec>;
#[doc = "Field `WTO` reader - Watchdog Timeout"]
pub type WtoR = crate::FieldReader<u16>;
#[doc = "Field `WTO` writer - Watchdog Timeout"]
pub type WtoW<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
#[doc = "Field `PWE` reader - Programmable Watchdog Enable"]
pub type PweR = crate::BitReader;
#[doc = "Field `PWE` writer - Programmable Watchdog Enable"]
pub type PweW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:13 - Watchdog Timeout"]
    #[inline(always)]
    pub fn wto(&self) -> WtoR {
        WtoR::new((self.bits & 0x3fff) as u16)
    }
    #[doc = "Bit 16 - Programmable Watchdog Enable"]
    #[inline(always)]
    pub fn pwe(&self) -> PweR {
        PweR::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:13 - Watchdog Timeout"]
    #[inline(always)]
    #[must_use]
    pub fn wto(&mut self) -> WtoW<MacWdogToSpec> {
        WtoW::new(self, 0)
    }
    #[doc = "Bit 16 - Programmable Watchdog Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pwe(&mut self) -> PweW<MacWdogToSpec> {
        PweW::new(self, 16)
    }
}
#[doc = "Controls the watchdog time-out for received frames\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mac_wdog_to::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mac_wdog_to::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MacWdogToSpec;
impl crate::RegisterSpec for MacWdogToSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mac_wdog_to::R`](R) reader structure"]
impl crate::Readable for MacWdogToSpec {}
#[doc = "`write(|w| ..)` method takes [`mac_wdog_to::W`](W) writer structure"]
impl crate::Writable for MacWdogToSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MAC_WDOG_TO to value 0"]
impl crate::Resettable for MacWdogToSpec {
    const RESET_VALUE: u32 = 0;
}
