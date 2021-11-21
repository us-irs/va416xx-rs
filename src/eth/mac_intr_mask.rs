#[doc = "Register `MAC_INTR_MASK` reader"]
pub struct R(crate::R<MAC_INTR_MASK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MAC_INTR_MASK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MAC_INTR_MASK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MAC_INTR_MASK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MAC_INTR_MASK` writer"]
pub struct W(crate::W<MAC_INTR_MASK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MAC_INTR_MASK_SPEC>;
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
impl From<crate::W<MAC_INTR_MASK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MAC_INTR_MASK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TSIM` reader - Timestamp Interrupt Mask"]
pub struct TSIM_R(crate::FieldReader<bool, bool>);
impl TSIM_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TSIM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TSIM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TSIM` writer - Timestamp Interrupt Mask"]
pub struct TSIM_W<'a> {
    w: &'a mut W,
}
impl<'a> TSIM_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | ((value as u32 & 0x01) << 9);
        self.w
    }
}
impl R {
    #[doc = "Bit 9 - Timestamp Interrupt Mask"]
    #[inline(always)]
    pub fn tsim(&self) -> TSIM_R {
        TSIM_R::new(((self.bits >> 9) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 9 - Timestamp Interrupt Mask"]
    #[inline(always)]
    pub fn tsim(&mut self) -> TSIM_W {
        TSIM_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Contains the masks for generating interrupt\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mac_intr_mask](index.html) module"]
pub struct MAC_INTR_MASK_SPEC;
impl crate::RegisterSpec for MAC_INTR_MASK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mac_intr_mask::R](R) reader structure"]
impl crate::Readable for MAC_INTR_MASK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mac_intr_mask::W](W) writer structure"]
impl crate::Writable for MAC_INTR_MASK_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MAC_INTR_MASK to value 0"]
impl crate::Resettable for MAC_INTR_MASK_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
