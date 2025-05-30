#[doc = "Register `ID1_CMB3` reader"]
pub type R = crate::R<Id1Cmb3Spec>;
#[doc = "Register `ID1_CMB3` writer"]
pub type W = crate::W<Id1Cmb3Spec>;
#[doc = "Field `ID1` reader - Half of CAN Frame ID. Format Varies for Standard or Extended Frames"]
pub type Id1R = crate::FieldReader<u16>;
#[doc = "Field `ID1` writer - Half of CAN Frame ID. Format Varies for Standard or Extended Frames"]
pub type Id1W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Half of CAN Frame ID. Format Varies for Standard or Extended Frames"]
    #[inline(always)]
    pub fn id1(&self) -> Id1R {
        Id1R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Half of CAN Frame ID. Format Varies for Standard or Extended Frames"]
    #[inline(always)]
    pub fn id1(&mut self) -> Id1W<Id1Cmb3Spec> {
        Id1W::new(self, 0)
    }
}
#[doc = "CAN Frame Identifier Word 1\n\nYou can [`read`](crate::Reg::read) this register and get [`id1_cmb3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`id1_cmb3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Id1Cmb3Spec;
impl crate::RegisterSpec for Id1Cmb3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`id1_cmb3::R`](R) reader structure"]
impl crate::Readable for Id1Cmb3Spec {}
#[doc = "`write(|w| ..)` method takes [`id1_cmb3::W`](W) writer structure"]
impl crate::Writable for Id1Cmb3Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ID1_CMB3 to value 0"]
impl crate::Resettable for Id1Cmb3Spec {
    const RESET_VALUE: u32 = 0;
}
