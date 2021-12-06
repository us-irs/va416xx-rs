#[doc = "Register `SYSTIME_SECSUPDAT` reader"]
pub type R = crate::R<SystimeSecsupdatSpec>;
#[doc = "Register `SYSTIME_SECSUPDAT` writer"]
pub type W = crate::W<SystimeSecsupdatSpec>;
#[doc = "Field `TSS` reader - Timestamp Second"]
pub type TssR = crate::FieldReader<u32>;
#[doc = "Field `TSS` writer - Timestamp Second"]
pub type TssW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Timestamp Second"]
    #[inline(always)]
    pub fn tss(&self) -> TssR {
        TssR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Timestamp Second"]
    #[inline(always)]
    #[must_use]
    pub fn tss(&mut self) -> TssW<SystimeSecsupdatSpec> {
        TssW::new(self, 0)
    }
}
#[doc = "Holds the lower 32 bits of the second field to be written to, added to, or subtracted from the system time value\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`systime_secsupdat::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`systime_secsupdat::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SystimeSecsupdatSpec;
impl crate::RegisterSpec for SystimeSecsupdatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`systime_secsupdat::R`](R) reader structure"]
impl crate::Readable for SystimeSecsupdatSpec {}
#[doc = "`write(|w| ..)` method takes [`systime_secsupdat::W`](W) writer structure"]
impl crate::Writable for SystimeSecsupdatSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SYSTIME_SECSUPDAT to value 0"]
impl crate::Resettable for SystimeSecsupdatSpec {
    const RESET_VALUE: u32 = 0;
}
