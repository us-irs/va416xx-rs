#[doc = "Register `TSTP_CMB13` reader"]
pub type R = crate::R<TstpCmb13Spec>;
#[doc = "Register `TSTP_CMB13` writer"]
pub type W = crate::W<TstpCmb13Spec>;
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
    pub fn timestamp(&mut self) -> TimestampW<TstpCmb13Spec> {
        TimestampW::new(self, 0)
    }
}
#[doc = "CAN Frame Timestamp\n\nYou can [`read`](crate::Reg::read) this register and get [`tstp_cmb13::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tstp_cmb13::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TstpCmb13Spec;
impl crate::RegisterSpec for TstpCmb13Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tstp_cmb13::R`](R) reader structure"]
impl crate::Readable for TstpCmb13Spec {}
#[doc = "`write(|w| ..)` method takes [`tstp_cmb13::W`](W) writer structure"]
impl crate::Writable for TstpCmb13Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TSTP_CMB13 to value 0"]
impl crate::Resettable for TstpCmb13Spec {
    const RESET_VALUE: u32 = 0;
}
