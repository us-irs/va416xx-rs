#[doc = "Register `SW_RESET` reader"]
pub type R = crate::R<SwResetSpec>;
#[doc = "Register `SW_RESET` writer"]
pub type W = crate::W<SwResetSpec>;
#[doc = "Field `SW_RESET` reader - Writing 1 to this register causes an internal TRNG reset"]
pub type SwResetR = crate::BitReader;
#[doc = "Field `SW_RESET` writer - Writing 1 to this register causes an internal TRNG reset"]
pub type SwResetW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Writing 1 to this register causes an internal TRNG reset"]
    #[inline(always)]
    pub fn sw_reset(&self) -> SwResetR {
        SwResetR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Writing 1 to this register causes an internal TRNG reset"]
    #[inline(always)]
    #[must_use]
    pub fn sw_reset(&mut self) -> SwResetW<SwResetSpec> {
        SwResetW::new(self, 0)
    }
}
#[doc = "Reset Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sw_reset::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sw_reset::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SwResetSpec;
impl crate::RegisterSpec for SwResetSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sw_reset::R`](R) reader structure"]
impl crate::Readable for SwResetSpec {}
#[doc = "`write(|w| ..)` method takes [`sw_reset::W`](W) writer structure"]
impl crate::Writable for SwResetSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SW_RESET to value 0"]
impl crate::Resettable for SwResetSpec {
    const RESET_VALUE: u32 = 0;
}
