#[doc = "Register `DEBUG_CONTROL` reader"]
pub struct R(crate::R<DEBUG_CONTROL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DEBUG_CONTROL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DEBUG_CONTROL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DEBUG_CONTROL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DEBUG_CONTROL` writer"]
pub struct W(crate::W<DEBUG_CONTROL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DEBUG_CONTROL_SPEC>;
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
impl From<crate::W<DEBUG_CONTROL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DEBUG_CONTROL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `AUTO_CORRELATE_BYPASS` reader - The autocorrelation test in the TRNG module is bypassed"]
pub struct AUTO_CORRELATE_BYPASS_R(crate::FieldReader<bool, bool>);
impl AUTO_CORRELATE_BYPASS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        AUTO_CORRELATE_BYPASS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AUTO_CORRELATE_BYPASS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AUTO_CORRELATE_BYPASS` writer - The autocorrelation test in the TRNG module is bypassed"]
pub struct AUTO_CORRELATE_BYPASS_W<'a> {
    w: &'a mut W,
}
impl<'a> AUTO_CORRELATE_BYPASS_W<'a> {
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
#[doc = "Field `CRNGT_BYPASS` reader - The CRNGT test in the TRNG is bypassed"]
pub struct CRNGT_BYPASS_R(crate::FieldReader<bool, bool>);
impl CRNGT_BYPASS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CRNGT_BYPASS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CRNGT_BYPASS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CRNGT_BYPASS` writer - The CRNGT test in the TRNG is bypassed"]
pub struct CRNGT_BYPASS_W<'a> {
    w: &'a mut W,
}
impl<'a> CRNGT_BYPASS_W<'a> {
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
#[doc = "Field `VNC_PYPASS` reader - The Von Neumann balancer is bypassed"]
pub struct VNC_PYPASS_R(crate::FieldReader<bool, bool>);
impl VNC_PYPASS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        VNC_PYPASS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VNC_PYPASS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VNC_PYPASS` writer - The Von Neumann balancer is bypassed"]
pub struct VNC_PYPASS_W<'a> {
    w: &'a mut W,
}
impl<'a> VNC_PYPASS_W<'a> {
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
impl R {
    #[doc = "Bit 3 - The autocorrelation test in the TRNG module is bypassed"]
    #[inline(always)]
    pub fn auto_correlate_bypass(&self) -> AUTO_CORRELATE_BYPASS_R {
        AUTO_CORRELATE_BYPASS_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - The CRNGT test in the TRNG is bypassed"]
    #[inline(always)]
    pub fn crngt_bypass(&self) -> CRNGT_BYPASS_R {
        CRNGT_BYPASS_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - The Von Neumann balancer is bypassed"]
    #[inline(always)]
    pub fn vnc_pypass(&self) -> VNC_PYPASS_R {
        VNC_PYPASS_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 3 - The autocorrelation test in the TRNG module is bypassed"]
    #[inline(always)]
    pub fn auto_correlate_bypass(&mut self) -> AUTO_CORRELATE_BYPASS_W {
        AUTO_CORRELATE_BYPASS_W { w: self }
    }
    #[doc = "Bit 2 - The CRNGT test in the TRNG is bypassed"]
    #[inline(always)]
    pub fn crngt_bypass(&mut self) -> CRNGT_BYPASS_W {
        CRNGT_BYPASS_W { w: self }
    }
    #[doc = "Bit 1 - The Von Neumann balancer is bypassed"]
    #[inline(always)]
    pub fn vnc_pypass(&mut self) -> VNC_PYPASS_W {
        VNC_PYPASS_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Section TBD\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [debug_control](index.html) module"]
pub struct DEBUG_CONTROL_SPEC;
impl crate::RegisterSpec for DEBUG_CONTROL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [debug_control::R](R) reader structure"]
impl crate::Readable for DEBUG_CONTROL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [debug_control::W](W) writer structure"]
impl crate::Writable for DEBUG_CONTROL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DEBUG_CONTROL to value 0"]
impl crate::Resettable for DEBUG_CONTROL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
