#[doc = "Register `S0_MAXWORDS` reader"]
pub type R = crate::R<S0MaxwordsSpec>;
#[doc = "Register `S0_MAXWORDS` writer"]
pub type W = crate::W<S0MaxwordsSpec>;
#[doc = "Field `MAXWORD` reader - Max Word Count"]
pub type MaxwordR = crate::FieldReader<u16>;
#[doc = "Field `MAXWORD` writer - Max Word Count"]
pub type MaxwordW<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
#[doc = "Field `ENABLE` reader - Enables the max word count"]
pub type EnableR = crate::BitReader;
#[doc = "Field `ENABLE` writer - Enables the max word count"]
pub type EnableW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:10 - Max Word Count"]
    #[inline(always)]
    pub fn maxword(&self) -> MaxwordR {
        MaxwordR::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bit 31 - Enables the max word count"]
    #[inline(always)]
    pub fn enable(&self) -> EnableR {
        EnableR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:10 - Max Word Count"]
    #[inline(always)]
    #[must_use]
    pub fn maxword(&mut self) -> MaxwordW<S0MaxwordsSpec> {
        MaxwordW::new(self, 0)
    }
    #[doc = "Bit 31 - Enables the max word count"]
    #[inline(always)]
    #[must_use]
    pub fn enable(&mut self) -> EnableW<S0MaxwordsSpec> {
        EnableW::new(self, 31)
    }
}
#[doc = "Slave MaxWords Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`s0_maxwords::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`s0_maxwords::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct S0MaxwordsSpec;
impl crate::RegisterSpec for S0MaxwordsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`s0_maxwords::R`](R) reader structure"]
impl crate::Readable for S0MaxwordsSpec {}
#[doc = "`write(|w| ..)` method takes [`s0_maxwords::W`](W) writer structure"]
impl crate::Writable for S0MaxwordsSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets S0_MAXWORDS to value 0"]
impl crate::Resettable for S0MaxwordsSpec {
    const RESET_VALUE: u32 = 0;
}
