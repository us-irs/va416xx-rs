#[doc = "Register `ICR` reader"]
pub struct R(crate::R<ICR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ICR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ICR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ICR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ICR` writer"]
pub struct W(crate::W<ICR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ICR_SPEC>;
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
impl From<crate::W<ICR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ICR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `VN_ERR` reader - Clears a Von Neumann error"]
pub struct VN_ERR_R(crate::FieldReader<bool, bool>);
impl VN_ERR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        VN_ERR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VN_ERR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VN_ERR` writer - Clears a Von Neumann error"]
pub struct VN_ERR_W<'a> {
    w: &'a mut W,
}
impl<'a> VN_ERR_W<'a> {
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
#[doc = "Field `CRNGT_ERR` reader - Clear a Continuous Random Number Generation Testing (CRNGT) error"]
pub struct CRNGT_ERR_R(crate::FieldReader<bool, bool>);
impl CRNGT_ERR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CRNGT_ERR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CRNGT_ERR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CRNGT_ERR` writer - Clear a Continuous Random Number Generation Testing (CRNGT) error"]
pub struct CRNGT_ERR_W<'a> {
    w: &'a mut W,
}
impl<'a> CRNGT_ERR_W<'a> {
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
#[doc = "Field `AUTOCORR_ERR` reader - Software cannot clear this bit. Only a TRNG reset can clear this bit"]
pub struct AUTOCORR_ERR_R(crate::FieldReader<bool, bool>);
impl AUTOCORR_ERR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        AUTOCORR_ERR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AUTOCORR_ERR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AUTOCORR_ERR` writer - Software cannot clear this bit. Only a TRNG reset can clear this bit"]
pub struct AUTOCORR_ERR_W<'a> {
    w: &'a mut W,
}
impl<'a> AUTOCORR_ERR_W<'a> {
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
#[doc = "Field `EHR_VALID` reader - Set to 1 after the EHR_DATA\\[0,1,2,3,4,5\\]
registers have been read"]
pub struct EHR_VALID_R(crate::FieldReader<bool, bool>);
impl EHR_VALID_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EHR_VALID_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EHR_VALID_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EHR_VALID` writer - Set to 1 after the EHR_DATA\\[0,1,2,3,4,5\\]
registers have been read"]
pub struct EHR_VALID_W<'a> {
    w: &'a mut W,
}
impl<'a> EHR_VALID_W<'a> {
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
    #[doc = "Bit 3 - Clears a Von Neumann error"]
    #[inline(always)]
    pub fn vn_err(&self) -> VN_ERR_R {
        VN_ERR_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Clear a Continuous Random Number Generation Testing (CRNGT) error"]
    #[inline(always)]
    pub fn crngt_err(&self) -> CRNGT_ERR_R {
        CRNGT_ERR_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - Software cannot clear this bit. Only a TRNG reset can clear this bit"]
    #[inline(always)]
    pub fn autocorr_err(&self) -> AUTOCORR_ERR_R {
        AUTOCORR_ERR_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Set to 1 after the EHR_DATA\\[0,1,2,3,4,5\\]
registers have been read"]
    #[inline(always)]
    pub fn ehr_valid(&self) -> EHR_VALID_R {
        EHR_VALID_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 3 - Clears a Von Neumann error"]
    #[inline(always)]
    pub fn vn_err(&mut self) -> VN_ERR_W {
        VN_ERR_W { w: self }
    }
    #[doc = "Bit 2 - Clear a Continuous Random Number Generation Testing (CRNGT) error"]
    #[inline(always)]
    pub fn crngt_err(&mut self) -> CRNGT_ERR_W {
        CRNGT_ERR_W { w: self }
    }
    #[doc = "Bit 1 - Software cannot clear this bit. Only a TRNG reset can clear this bit"]
    #[inline(always)]
    pub fn autocorr_err(&mut self) -> AUTOCORR_ERR_W {
        AUTOCORR_ERR_W { w: self }
    }
    #[doc = "Bit 0 - Set to 1 after the EHR_DATA\\[0,1,2,3,4,5\\]
registers have been read"]
    #[inline(always)]
    pub fn ehr_valid(&mut self) -> EHR_VALID_W {
        EHR_VALID_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Clear Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [icr](index.html) module"]
pub struct ICR_SPEC;
impl crate::RegisterSpec for ICR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [icr::R](R) reader structure"]
impl crate::Readable for ICR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [icr::W](W) writer structure"]
impl crate::Writable for ICR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ICR to value 0"]
impl crate::Resettable for ICR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
