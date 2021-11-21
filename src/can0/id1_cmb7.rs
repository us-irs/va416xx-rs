#[doc = "Register `ID1_CMB7` reader"]
pub struct R(crate::R<ID1_CMB7_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ID1_CMB7_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ID1_CMB7_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ID1_CMB7_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ID1_CMB7` writer"]
pub struct W(crate::W<ID1_CMB7_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ID1_CMB7_SPEC>;
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
impl From<crate::W<ID1_CMB7_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ID1_CMB7_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ID1` reader - Half of CAN Frame ID. Format Varies for Standard or Extended Frames"]
pub struct ID1_R(crate::FieldReader<u16, u16>);
impl ID1_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        ID1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ID1_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ID1` writer - Half of CAN Frame ID. Format Varies for Standard or Extended Frames"]
pub struct ID1_W<'a> {
    w: &'a mut W,
}
impl<'a> ID1_W<'a> {
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
    pub fn id1(&self) -> ID1_R {
        ID1_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Half of CAN Frame ID. Format Varies for Standard or Extended Frames"]
    #[inline(always)]
    pub fn id1(&mut self) -> ID1_W {
        ID1_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CAN Frame Identifier Word 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [id1_cmb7](index.html) module"]
pub struct ID1_CMB7_SPEC;
impl crate::RegisterSpec for ID1_CMB7_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [id1_cmb7::R](R) reader structure"]
impl crate::Readable for ID1_CMB7_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [id1_cmb7::W](W) writer structure"]
impl crate::Writable for ID1_CMB7_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ID1_CMB7 to value 0"]
impl crate::Resettable for ID1_CMB7_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
