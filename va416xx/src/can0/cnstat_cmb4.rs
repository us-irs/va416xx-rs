#[doc = "Register `CNSTAT_CMB4` reader"]
pub type R = crate::R<CnstatCmb4Spec>;
#[doc = "Register `CNSTAT_CMB4` writer"]
pub type W = crate::W<CnstatCmb4Spec>;
#[doc = "Field `ST` reader - Buffer Status"]
pub type StR = crate::FieldReader;
#[doc = "Field `ST` writer - Buffer Status"]
pub type StW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PRI` reader - Transmit Priority Code"]
pub type PriR = crate::FieldReader;
#[doc = "Field `PRI` writer - Transmit Priority Code"]
pub type PriW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `DLC` reader - Data Length Code"]
pub type DlcR = crate::FieldReader;
#[doc = "Field `DLC` writer - Data Length Code"]
pub type DlcW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - Buffer Status"]
    #[inline(always)]
    pub fn st(&self) -> StR {
        StR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Transmit Priority Code"]
    #[inline(always)]
    pub fn pri(&self) -> PriR {
        PriR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - Data Length Code"]
    #[inline(always)]
    pub fn dlc(&self) -> DlcR {
        DlcR::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Buffer Status"]
    #[inline(always)]
    #[must_use]
    pub fn st(&mut self) -> StW<CnstatCmb4Spec> {
        StW::new(self, 0)
    }
    #[doc = "Bits 4:7 - Transmit Priority Code"]
    #[inline(always)]
    #[must_use]
    pub fn pri(&mut self) -> PriW<CnstatCmb4Spec> {
        PriW::new(self, 4)
    }
    #[doc = "Bits 12:15 - Data Length Code"]
    #[inline(always)]
    #[must_use]
    pub fn dlc(&mut self) -> DlcW<CnstatCmb4Spec> {
        DlcW::new(self, 12)
    }
}
#[doc = "Buffer Status / Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cnstat_cmb4::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cnstat_cmb4::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CnstatCmb4Spec;
impl crate::RegisterSpec for CnstatCmb4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cnstat_cmb4::R`](R) reader structure"]
impl crate::Readable for CnstatCmb4Spec {}
#[doc = "`write(|w| ..)` method takes [`cnstat_cmb4::W`](W) writer structure"]
impl crate::Writable for CnstatCmb4Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CNSTAT_CMB4 to value 0"]
impl crate::Resettable for CnstatCmb4Spec {
    const RESET_VALUE: u32 = 0;
}
