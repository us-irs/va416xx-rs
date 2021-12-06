#[doc = "Register `ALT_CTRL_BASE_PTR` reader"]
pub type R = crate::R<AltCtrlBasePtrSpec>;
#[doc = "Register `ALT_CTRL_BASE_PTR` writer"]
pub type W = crate::W<AltCtrlBasePtrSpec>;
#[doc = "Field `ALT_CTRL_BASE_PTR` reader - Base Pointer for Alternate DMA Control Register"]
pub type AltCtrlBasePtrR = crate::FieldReader<u32>;
#[doc = "Field `ALT_CTRL_BASE_PTR` writer - Base Pointer for Alternate DMA Control Register"]
pub type AltCtrlBasePtrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Base Pointer for Alternate DMA Control Register"]
    #[inline(always)]
    pub fn alt_ctrl_base_ptr(&self) -> AltCtrlBasePtrR {
        AltCtrlBasePtrR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Base Pointer for Alternate DMA Control Register"]
    #[inline(always)]
    #[must_use]
    pub fn alt_ctrl_base_ptr(&mut self) -> AltCtrlBasePtrW<AltCtrlBasePtrSpec> {
        AltCtrlBasePtrW::new(self, 0)
    }
}
#[doc = "DMA Channel alternate control data base pointer\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`alt_ctrl_base_ptr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`alt_ctrl_base_ptr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AltCtrlBasePtrSpec;
impl crate::RegisterSpec for AltCtrlBasePtrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`alt_ctrl_base_ptr::R`](R) reader structure"]
impl crate::Readable for AltCtrlBasePtrSpec {}
#[doc = "`write(|w| ..)` method takes [`alt_ctrl_base_ptr::W`](W) writer structure"]
impl crate::Writable for AltCtrlBasePtrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ALT_CTRL_BASE_PTR to value 0"]
impl crate::Resettable for AltCtrlBasePtrSpec {
    const RESET_VALUE: u32 = 0;
}
