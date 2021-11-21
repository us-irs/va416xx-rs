#[doc = "Register `ID0_CMB8` reader"]
pub struct R(crate::R<ID0_CMB8_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ID0_CMB8_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ID0_CMB8_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ID0_CMB8_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ID0_CMB8` writer"]
pub struct W(crate::W<ID0_CMB8_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ID0_CMB8_SPEC>;
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
impl From<crate::W<ID0_CMB8_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ID0_CMB8_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ID0` reader - Half of CAN Frame ID. Format Varies for Standard or Extended Frames"]
pub struct ID0_R(crate::FieldReader<u16, u16>);
impl ID0_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        ID0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ID0_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ID0` writer - Half of CAN Frame ID. Format Varies for Standard or Extended Frames"]
pub struct ID0_W<'a> {
    w: &'a mut W,
}
impl<'a> ID0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Half of CAN Frame ID. Format Varies for Standard or Extended Frames"]
    #[inline(always)]
    pub fn id0(&self) -> ID0_R {
        ID0_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Half of CAN Frame ID. Format Varies for Standard or Extended Frames"]
    #[inline(always)]
    pub fn id0(&mut self) -> ID0_W {
        ID0_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CAN Frame Identifier Word 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [id0_cmb8](index.html) module"]
pub struct ID0_CMB8_SPEC;
impl crate::RegisterSpec for ID0_CMB8_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [id0_cmb8::R](R) reader structure"]
impl crate::Readable for ID0_CMB8_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [id0_cmb8::W](W) writer structure"]
impl crate::Writable for ID0_CMB8_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ID0_CMB8 to value 0"]
impl crate::Resettable for ID0_CMB8_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
