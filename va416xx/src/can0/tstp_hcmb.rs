#[doc = "Register `TSTP_HCMB` reader"]
pub type R = crate::R<TstpHcmbSpec>;
#[doc = "Register `TSTP_HCMB` writer"]
pub type W = crate::W<TstpHcmbSpec>;
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
    pub fn timestamp(&mut self) -> TimestampW<TstpHcmbSpec> {
        TimestampW::new(self, 0)
    }
}
#[doc = "CAN Frame Timestamp\n\nYou can [`read`](crate::Reg::read) this register and get [`tstp_hcmb::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tstp_hcmb::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TstpHcmbSpec;
impl crate::RegisterSpec for TstpHcmbSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tstp_hcmb::R`](R) reader structure"]
impl crate::Readable for TstpHcmbSpec {}
#[doc = "`write(|w| ..)` method takes [`tstp_hcmb::W`](W) writer structure"]
impl crate::Writable for TstpHcmbSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TSTP_HCMB to value 0"]
impl crate::Resettable for TstpHcmbSpec {
    const RESET_VALUE: u32 = 0;
}
