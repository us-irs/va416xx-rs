#[doc = "Register `ID0_CMB6` reader"]
pub type R = crate::R<Id0Cmb6Spec>;
#[doc = "Register `ID0_CMB6` writer"]
pub type W = crate::W<Id0Cmb6Spec>;
#[doc = "Field `ID0` reader - Half of CAN Frame ID. Format Varies for Standard or Extended Frames"]
pub type Id0R = crate::FieldReader<u16>;
#[doc = "Field `ID0` writer - Half of CAN Frame ID. Format Varies for Standard or Extended Frames"]
pub type Id0W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Half of CAN Frame ID. Format Varies for Standard or Extended Frames"]
    #[inline(always)]
    pub fn id0(&self) -> Id0R {
        Id0R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Half of CAN Frame ID. Format Varies for Standard or Extended Frames"]
    #[inline(always)]
    #[must_use]
    pub fn id0(&mut self) -> Id0W<Id0Cmb6Spec> {
        Id0W::new(self, 0)
    }
}
#[doc = "CAN Frame Identifier Word 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`id0_cmb6::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`id0_cmb6::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Id0Cmb6Spec;
impl crate::RegisterSpec for Id0Cmb6Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`id0_cmb6::R`](R) reader structure"]
impl crate::Readable for Id0Cmb6Spec {}
#[doc = "`write(|w| ..)` method takes [`id0_cmb6::W`](W) writer structure"]
impl crate::Writable for Id0Cmb6Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ID0_CMB6 to value 0"]
impl crate::Resettable for Id0Cmb6Spec {
    const RESET_VALUE: u32 = 0;
}
