#[doc = "Register `IMR` reader"]
pub type R = crate::R<ImrSpec>;
#[doc = "Register `IMR` writer"]
pub type W = crate::W<ImrSpec>;
#[doc = "Field `EHR_VALID_INT_MASK` reader - Mask when the TRNG has collected 192 bits"]
pub type EhrValidIntMaskR = crate::BitReader;
#[doc = "Field `EHR_VALID_INT_MASK` writer - Mask when the TRNG has collected 192 bits"]
pub type EhrValidIntMaskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AUTOCORR_ERR_INT_MASK` reader - Mask the Autocorrelation error"]
pub type AutocorrErrIntMaskR = crate::BitReader;
#[doc = "Field `AUTOCORR_ERR_INT_MASK` writer - Mask the Autocorrelation error"]
pub type AutocorrErrIntMaskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CRNGT_ERR_INT_MASK` reader - Mask the CRNGT error"]
pub type CrngtErrIntMaskR = crate::BitReader;
#[doc = "Field `CRNGT_ERR_INT_MASK` writer - Mask the CRNGT error"]
pub type CrngtErrIntMaskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VN_ERR_INT_MASK` reader - Mask the Von Neumann error"]
pub type VnErrIntMaskR = crate::BitReader;
#[doc = "Field `VN_ERR_INT_MASK` writer - Mask the Von Neumann error"]
pub type VnErrIntMaskW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Mask when the TRNG has collected 192 bits"]
    #[inline(always)]
    pub fn ehr_valid_int_mask(&self) -> EhrValidIntMaskR {
        EhrValidIntMaskR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Mask the Autocorrelation error"]
    #[inline(always)]
    pub fn autocorr_err_int_mask(&self) -> AutocorrErrIntMaskR {
        AutocorrErrIntMaskR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Mask the CRNGT error"]
    #[inline(always)]
    pub fn crngt_err_int_mask(&self) -> CrngtErrIntMaskR {
        CrngtErrIntMaskR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Mask the Von Neumann error"]
    #[inline(always)]
    pub fn vn_err_int_mask(&self) -> VnErrIntMaskR {
        VnErrIntMaskR::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Mask when the TRNG has collected 192 bits"]
    #[inline(always)]
    #[must_use]
    pub fn ehr_valid_int_mask(&mut self) -> EhrValidIntMaskW<ImrSpec> {
        EhrValidIntMaskW::new(self, 0)
    }
    #[doc = "Bit 1 - Mask the Autocorrelation error"]
    #[inline(always)]
    #[must_use]
    pub fn autocorr_err_int_mask(&mut self) -> AutocorrErrIntMaskW<ImrSpec> {
        AutocorrErrIntMaskW::new(self, 1)
    }
    #[doc = "Bit 2 - Mask the CRNGT error"]
    #[inline(always)]
    #[must_use]
    pub fn crngt_err_int_mask(&mut self) -> CrngtErrIntMaskW<ImrSpec> {
        CrngtErrIntMaskW::new(self, 2)
    }
    #[doc = "Bit 3 - Mask the Von Neumann error"]
    #[inline(always)]
    #[must_use]
    pub fn vn_err_int_mask(&mut self) -> VnErrIntMaskW<ImrSpec> {
        VnErrIntMaskW::new(self, 3)
    }
}
#[doc = "Interrupt Mask Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`imr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`imr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ImrSpec;
impl crate::RegisterSpec for ImrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`imr::R`](R) reader structure"]
impl crate::Readable for ImrSpec {}
#[doc = "`write(|w| ..)` method takes [`imr::W`](W) writer structure"]
impl crate::Writable for ImrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IMR to value 0x0f"]
impl crate::Resettable for ImrSpec {
    const RESET_VALUE: u32 = 0x0f;
}
