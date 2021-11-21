#[doc = "Register `S0_MAXWORDS` reader"]
pub struct R(crate::R<S0_MAXWORDS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<S0_MAXWORDS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<S0_MAXWORDS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<S0_MAXWORDS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `S0_MAXWORDS` writer"]
pub struct W(crate::W<S0_MAXWORDS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<S0_MAXWORDS_SPEC>;
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
impl From<crate::W<S0_MAXWORDS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<S0_MAXWORDS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MAXWORD` reader - Max Word Count"]
pub struct MAXWORD_R(crate::FieldReader<u16, u16>);
impl MAXWORD_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        MAXWORD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MAXWORD_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MAXWORD` writer - Max Word Count"]
pub struct MAXWORD_W<'a> {
    w: &'a mut W,
}
impl<'a> MAXWORD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07ff) | (value as u32 & 0x07ff);
        self.w
    }
}
#[doc = "Field `ENABLE` reader - Enables the max word count"]
pub struct ENABLE_R(crate::FieldReader<bool, bool>);
impl ENABLE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ENABLE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ENABLE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ENABLE` writer - Enables the max word count"]
pub struct ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> ENABLE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | ((value as u32 & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:10 - Max Word Count"]
    #[inline(always)]
    pub fn maxword(&self) -> MAXWORD_R {
        MAXWORD_R::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bit 31 - Enables the max word count"]
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:10 - Max Word Count"]
    #[inline(always)]
    pub fn maxword(&mut self) -> MAXWORD_W {
        MAXWORD_W { w: self }
    }
    #[doc = "Bit 31 - Enables the max word count"]
    #[inline(always)]
    pub fn enable(&mut self) -> ENABLE_W {
        ENABLE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Slave MaxWords Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [s0_maxwords](index.html) module"]
pub struct S0_MAXWORDS_SPEC;
impl crate::RegisterSpec for S0_MAXWORDS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [s0_maxwords::R](R) reader structure"]
impl crate::Readable for S0_MAXWORDS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [s0_maxwords::W](W) writer structure"]
impl crate::Writable for S0_MAXWORDS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets S0_MAXWORDS to value 0"]
impl crate::Resettable for S0_MAXWORDS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
