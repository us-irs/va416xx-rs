#[doc = "Register `CTRL_BASE_PTR` reader"]
pub type R = crate::R<CtrlBasePtrSpec>;
#[doc = "Register `CTRL_BASE_PTR` writer"]
pub type W = crate::W<CtrlBasePtrSpec>;
#[doc = "Field `CTRL_BASE_PTR` reader - Base Pointer for DMA Control Registers"]
pub type CtrlBasePtrR = crate::FieldReader<u32>;
#[doc = "Field `CTRL_BASE_PTR` writer - Base Pointer for DMA Control Registers"]
pub type CtrlBasePtrW<'a, REG> = crate::FieldWriter<'a, REG, 25, u32>;
impl R {
    #[doc = "Bits 7:31 - Base Pointer for DMA Control Registers"]
    #[inline(always)]
    pub fn ctrl_base_ptr(&self) -> CtrlBasePtrR {
        CtrlBasePtrR::new((self.bits >> 7) & 0x01ff_ffff)
    }
}
impl W {
    #[doc = "Bits 7:31 - Base Pointer for DMA Control Registers"]
    #[inline(always)]
    #[must_use]
    pub fn ctrl_base_ptr(&mut self) -> CtrlBasePtrW<CtrlBasePtrSpec> {
        CtrlBasePtrW::new(self, 7)
    }
}
#[doc = "Base Pointer for DMA Control Registers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl_base_ptr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl_base_ptr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtrlBasePtrSpec;
impl crate::RegisterSpec for CtrlBasePtrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrl_base_ptr::R`](R) reader structure"]
impl crate::Readable for CtrlBasePtrSpec {}
#[doc = "`write(|w| ..)` method takes [`ctrl_base_ptr::W`](W) writer structure"]
impl crate::Writable for CtrlBasePtrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTRL_BASE_PTR to value 0"]
impl crate::Resettable for CtrlBasePtrSpec {
    const RESET_VALUE: u32 = 0;
}
