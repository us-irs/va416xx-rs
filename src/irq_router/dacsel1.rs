#[doc = "Register `DACSEL1` reader"]
pub struct R(crate::R<DACSEL1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DACSEL1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DACSEL1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DACSEL1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DACSEL1` writer"]
pub struct W(crate::W<DACSEL1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DACSEL1_SPEC>;
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
impl From<crate::W<DACSEL1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DACSEL1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DACSEL` reader - DAC trigger source selection value"]
pub struct DACSEL_R(crate::FieldReader<u8, u8>);
impl DACSEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DACSEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DACSEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DACSEL` writer - DAC trigger source selection value"]
pub struct DACSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> DACSEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | (value as u32 & 0x1f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:4 - DAC trigger source selection value"]
    #[inline(always)]
    pub fn dacsel(&self) -> DACSEL_R {
        DACSEL_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - DAC trigger source selection value"]
    #[inline(always)]
    pub fn dacsel(&mut self) -> DACSEL_W {
        DACSEL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt select for DAC1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dacsel1](index.html) module"]
pub struct DACSEL1_SPEC;
impl crate::RegisterSpec for DACSEL1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dacsel1::R](R) reader structure"]
impl crate::Readable for DACSEL1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dacsel1::W](W) writer structure"]
impl crate::Writable for DACSEL1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DACSEL1 to value 0x1f"]
impl crate::Resettable for DACSEL1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x1f
    }
}
