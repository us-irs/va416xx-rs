#[doc = "Register `CNSTAT_CMB1` reader"]
pub type R = crate::R<CnstatCmb1Spec>;
#[doc = "Register `CNSTAT_CMB1` writer"]
pub type W = crate::W<CnstatCmb1Spec>;
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
    pub fn st(&mut self) -> StW<CnstatCmb1Spec> {
        StW::new(self, 0)
    }
    #[doc = "Bits 4:7 - Transmit Priority Code"]
    #[inline(always)]
    pub fn pri(&mut self) -> PriW<CnstatCmb1Spec> {
        PriW::new(self, 4)
    }
    #[doc = "Bits 12:15 - Data Length Code"]
    #[inline(always)]
    pub fn dlc(&mut self) -> DlcW<CnstatCmb1Spec> {
        DlcW::new(self, 12)
    }
}
#[doc = "Buffer Status / Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cnstat_cmb1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cnstat_cmb1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CnstatCmb1Spec;
impl crate::RegisterSpec for CnstatCmb1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cnstat_cmb1::R`](R) reader structure"]
impl crate::Readable for CnstatCmb1Spec {}
#[doc = "`write(|w| ..)` method takes [`cnstat_cmb1::W`](W) writer structure"]
impl crate::Writable for CnstatCmb1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CNSTAT_CMB1 to value 0"]
impl crate::Resettable for CnstatCmb1Spec {
    const RESET_VALUE: u32 = 0;
}
