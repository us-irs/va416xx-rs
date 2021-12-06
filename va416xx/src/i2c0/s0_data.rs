#[doc = "Register `S0_DATA` reader"]
pub type R = crate::R<S0DataSpec>;
#[doc = "Register `S0_DATA` writer"]
pub type W = crate::W<S0DataSpec>;
#[doc = "Field `VALUE` reader - I2C data value"]
pub type ValueR = crate::FieldReader;
#[doc = "Field `VALUE` writer - I2C data value"]
pub type ValueW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - I2C data value"]
    #[inline(always)]
    pub fn value(&self) -> ValueR {
        ValueR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - I2C data value"]
    #[inline(always)]
    #[must_use]
    pub fn value(&mut self) -> ValueW<S0DataSpec> {
        ValueW::new(self, 0)
    }
}
#[doc = "Slave Data Input/Output\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`s0_data::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`s0_data::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct S0DataSpec;
impl crate::RegisterSpec for S0DataSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`s0_data::R`](R) reader structure"]
impl crate::Readable for S0DataSpec {}
#[doc = "`write(|w| ..)` method takes [`s0_data::W`](W) writer structure"]
impl crate::Writable for S0DataSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets S0_DATA to value 0"]
impl crate::Resettable for S0DataSpec {
    const RESET_VALUE: u32 = 0;
}
