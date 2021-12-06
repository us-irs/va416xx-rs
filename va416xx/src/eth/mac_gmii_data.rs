#[doc = "Register `MAC_GMII_DATA` reader"]
pub type R = crate::R<MacGmiiDataSpec>;
#[doc = "Register `MAC_GMII_DATA` writer"]
pub type W = crate::W<MacGmiiDataSpec>;
#[doc = "Field `GD` reader - GMII Data"]
pub type GdR = crate::FieldReader<u16>;
#[doc = "Field `GD` writer - GMII Data"]
pub type GdW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - GMII Data"]
    #[inline(always)]
    pub fn gd(&self) -> GdR {
        GdR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - GMII Data"]
    #[inline(always)]
    #[must_use]
    pub fn gd(&mut self) -> GdW<MacGmiiDataSpec> {
        GdW::new(self, 0)
    }
}
#[doc = "Contains the data to be written to or read from the PHY register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mac_gmii_data::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mac_gmii_data::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MacGmiiDataSpec;
impl crate::RegisterSpec for MacGmiiDataSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mac_gmii_data::R`](R) reader structure"]
impl crate::Readable for MacGmiiDataSpec {}
#[doc = "`write(|w| ..)` method takes [`mac_gmii_data::W`](W) writer structure"]
impl crate::Writable for MacGmiiDataSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MAC_GMII_DATA to value 0"]
impl crate::Resettable for MacGmiiDataSpec {
    const RESET_VALUE: u32 = 0;
}
