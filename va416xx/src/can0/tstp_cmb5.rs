#[doc = "Register `TSTP_CMB5` reader"]
pub type R = crate::R<TstpCmb5Spec>;
#[doc = "Register `TSTP_CMB5` writer"]
pub type W = crate::W<TstpCmb5Spec>;
#[doc = "Field `TIMESTAMP` reader - Timestamp"]
pub type TimestampR = crate::FieldReader<u16>;
#[doc = "Field `TIMESTAMP` writer - Timestamp"]
pub type TimestampW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Timestamp"]
    #[inline(always)]
    pub fn timestamp(&self) -> TimestampR {
        TimestampR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Timestamp"]
    #[inline(always)]
    #[must_use]
    pub fn timestamp(&mut self) -> TimestampW<TstpCmb5Spec> {
        TimestampW::new(self, 0)
    }
}
#[doc = "CAN Frame Timestamp\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tstp_cmb5::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tstp_cmb5::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TstpCmb5Spec;
impl crate::RegisterSpec for TstpCmb5Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tstp_cmb5::R`](R) reader structure"]
impl crate::Readable for TstpCmb5Spec {}
#[doc = "`write(|w| ..)` method takes [`tstp_cmb5::W`](W) writer structure"]
impl crate::Writable for TstpCmb5Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TSTP_CMB5 to value 0"]
impl crate::Resettable for TstpCmb5Spec {
    const RESET_VALUE: u32 = 0;
}
