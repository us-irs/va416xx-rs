#[doc = "Register `IMR` reader"]
pub struct R(crate::R<IMR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IMR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IMR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IMR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IMR` writer"]
pub struct W(crate::W<IMR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IMR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<IMR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IMR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `VN_ERR_INT_MASK` reader - Mask the Von Neumann error"]
pub struct VN_ERR_INT_MASK_R(crate::FieldReader<bool, bool>);
impl VN_ERR_INT_MASK_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        VN_ERR_INT_MASK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VN_ERR_INT_MASK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VN_ERR_INT_MASK` writer - Mask the Von Neumann error"]
pub struct VN_ERR_INT_MASK_W<'a> {
    w: &'a mut W,
}
impl<'a> VN_ERR_INT_MASK_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u32 & 0x01) << 3);
        self.w
    }
}
#[doc = "Field `CRNGT_ERR_INT_MASK` reader - Mask the CRNGT error"]
pub struct CRNGT_ERR_INT_MASK_R(crate::FieldReader<bool, bool>);
impl CRNGT_ERR_INT_MASK_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CRNGT_ERR_INT_MASK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CRNGT_ERR_INT_MASK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CRNGT_ERR_INT_MASK` writer - Mask the CRNGT error"]
pub struct CRNGT_ERR_INT_MASK_W<'a> {
    w: &'a mut W,
}
impl<'a> CRNGT_ERR_INT_MASK_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
        self.w
    }
}
#[doc = "Field `AUTOCORR_ERR_INT_MASK` reader - Mask the Autocorrelation error"]
pub struct AUTOCORR_ERR_INT_MASK_R(crate::FieldReader<bool, bool>);
impl AUTOCORR_ERR_INT_MASK_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        AUTOCORR_ERR_INT_MASK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AUTOCORR_ERR_INT_MASK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AUTOCORR_ERR_INT_MASK` writer - Mask the Autocorrelation error"]
pub struct AUTOCORR_ERR_INT_MASK_W<'a> {
    w: &'a mut W,
}
impl<'a> AUTOCORR_ERR_INT_MASK_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
#[doc = "Field `EHR_VALID_INT_MASK` reader - Mask when the TRNG has collected 192 bits"]
pub struct EHR_VALID_INT_MASK_R(crate::FieldReader<bool, bool>);
impl EHR_VALID_INT_MASK_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EHR_VALID_INT_MASK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EHR_VALID_INT_MASK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EHR_VALID_INT_MASK` writer - Mask when the TRNG has collected 192 bits"]
pub struct EHR_VALID_INT_MASK_W<'a> {
    w: &'a mut W,
}
impl<'a> EHR_VALID_INT_MASK_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
impl R {
    #[doc = "Bit 3 - Mask the Von Neumann error"]
    #[inline(always)]
    pub fn vn_err_int_mask(&self) -> VN_ERR_INT_MASK_R {
        VN_ERR_INT_MASK_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Mask the CRNGT error"]
    #[inline(always)]
    pub fn crngt_err_int_mask(&self) -> CRNGT_ERR_INT_MASK_R {
        CRNGT_ERR_INT_MASK_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - Mask the Autocorrelation error"]
    #[inline(always)]
    pub fn autocorr_err_int_mask(&self) -> AUTOCORR_ERR_INT_MASK_R {
        AUTOCORR_ERR_INT_MASK_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Mask when the TRNG has collected 192 bits"]
    #[inline(always)]
    pub fn ehr_valid_int_mask(&self) -> EHR_VALID_INT_MASK_R {
        EHR_VALID_INT_MASK_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 3 - Mask the Von Neumann error"]
    #[inline(always)]
    pub fn vn_err_int_mask(&mut self) -> VN_ERR_INT_MASK_W {
        VN_ERR_INT_MASK_W { w: self }
    }
    #[doc = "Bit 2 - Mask the CRNGT error"]
    #[inline(always)]
    pub fn crngt_err_int_mask(&mut self) -> CRNGT_ERR_INT_MASK_W {
        CRNGT_ERR_INT_MASK_W { w: self }
    }
    #[doc = "Bit 1 - Mask the Autocorrelation error"]
    #[inline(always)]
    pub fn autocorr_err_int_mask(&mut self) -> AUTOCORR_ERR_INT_MASK_W {
        AUTOCORR_ERR_INT_MASK_W { w: self }
    }
    #[doc = "Bit 0 - Mask when the TRNG has collected 192 bits"]
    #[inline(always)]
    pub fn ehr_valid_int_mask(&mut self) -> EHR_VALID_INT_MASK_W {
        EHR_VALID_INT_MASK_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Mask Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [imr](index.html) module"]
pub struct IMR_SPEC;
impl crate::RegisterSpec for IMR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [imr::R](R) reader structure"]
impl crate::Readable for IMR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [imr::W](W) writer structure"]
impl crate::Writable for IMR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets IMR to value 0x0f"]
impl crate::Resettable for IMR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0f
    }
}
