#[doc = "Register `PWMB_VALUE` reader"]
pub type R = crate::R<PwmbValueSpec>;
#[doc = "Register `PWMB_VALUE` writer"]
pub type W = crate::W<PwmbValueSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "The Pulse Width Modulation ValueB\n\nYou can [`read`](crate::Reg::read) this register and get [`pwmb_value::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwmb_value::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PwmbValueSpec;
impl crate::RegisterSpec for PwmbValueSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pwmb_value::R`](R) reader structure"]
impl crate::Readable for PwmbValueSpec {}
#[doc = "`write(|w| ..)` method takes [`pwmb_value::W`](W) writer structure"]
impl crate::Writable for PwmbValueSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PWMB_VALUE to value 0"]
impl crate::Resettable for PwmbValueSpec {
    const RESET_VALUE: u32 = 0;
}
