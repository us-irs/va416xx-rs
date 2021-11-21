#[doc = "Register `GMSKX` reader"]
pub struct R(crate::R<GMSKX_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GMSKX_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GMSKX_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GMSKX_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GMSKX` writer"]
pub struct W(crate::W<GMSKX_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GMSKX_SPEC>;
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
impl From<crate::W<GMSKX_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GMSKX_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `GM` reader - GM\\[14:0\\]
used when an extended frame is received. ID\\[14:0\\]
in extended, unused standard"]
pub struct GM_R(crate::FieldReader<u16, u16>);
impl GM_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        GM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GM_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GM` writer - GM\\[14:0\\]
used when an extended frame is received. ID\\[14:0\\]
in extended, unused standard"]
pub struct GM_W<'a> {
    w: &'a mut W,
}
impl<'a> GM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7fff << 1)) | ((value as u32 & 0x7fff) << 1);
        self.w
    }
}
#[doc = "Field `XRTR` reader - Extended Remote transmission Request Bit"]
pub struct XRTR_R(crate::FieldReader<bool, bool>);
impl XRTR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        XRTR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for XRTR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `XRTR` writer - Extended Remote transmission Request Bit"]
pub struct XRTR_W<'a> {
    w: &'a mut W,
}
impl<'a> XRTR_W<'a> {
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
    #[doc = "Bits 1:15 - GM\\[14:0\\]
used when an extended frame is received. ID\\[14:0\\]
in extended, unused standard"]
    #[inline(always)]
    pub fn gm(&self) -> GM_R {
        GM_R::new(((self.bits >> 1) & 0x7fff) as u16)
    }
    #[doc = "Bit 0 - Extended Remote transmission Request Bit"]
    #[inline(always)]
    pub fn xrtr(&self) -> XRTR_R {
        XRTR_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 1:15 - GM\\[14:0\\]
used when an extended frame is received. ID\\[14:0\\]
in extended, unused standard"]
    #[inline(always)]
    pub fn gm(&mut self) -> GM_W {
        GM_W { w: self }
    }
    #[doc = "Bit 0 - Extended Remote transmission Request Bit"]
    #[inline(always)]
    pub fn xrtr(&mut self) -> XRTR_W {
        XRTR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CAN Global Mask Extension\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gmskx](index.html) module"]
pub struct GMSKX_SPEC;
impl crate::RegisterSpec for GMSKX_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gmskx::R](R) reader structure"]
impl crate::Readable for GMSKX_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gmskx::W](W) writer structure"]
impl crate::Writable for GMSKX_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets GMSKX to value 0"]
impl crate::Resettable for GMSKX_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
