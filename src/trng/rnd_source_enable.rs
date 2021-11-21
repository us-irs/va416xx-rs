#[doc = "Register `RND_SOURCE_ENABLE` reader"]
pub struct R(crate::R<RND_SOURCE_ENABLE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RND_SOURCE_ENABLE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RND_SOURCE_ENABLE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RND_SOURCE_ENABLE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RND_SOURCE_ENABLE` writer"]
pub struct W(crate::W<RND_SOURCE_ENABLE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RND_SOURCE_ENABLE_SPEC>;
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
impl From<crate::W<RND_SOURCE_ENABLE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RND_SOURCE_ENABLE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RND_SRC_EN` reader - The entropy source, ring oscillator, is enabled"]
pub struct RND_SRC_EN_R(crate::FieldReader<bool, bool>);
impl RND_SRC_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RND_SRC_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RND_SRC_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RND_SRC_EN` writer - The entropy source, ring oscillator, is enabled"]
pub struct RND_SRC_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> RND_SRC_EN_W<'a> {
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
    #[doc = "Bit 0 - The entropy source, ring oscillator, is enabled"]
    #[inline(always)]
    pub fn rnd_src_en(&self) -> RND_SRC_EN_R {
        RND_SRC_EN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - The entropy source, ring oscillator, is enabled"]
    #[inline(always)]
    pub fn rnd_src_en(&mut self) -> RND_SRC_EN_W {
        RND_SRC_EN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Random Source Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rnd_source_enable](index.html) module"]
pub struct RND_SOURCE_ENABLE_SPEC;
impl crate::RegisterSpec for RND_SOURCE_ENABLE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rnd_source_enable::R](R) reader structure"]
impl crate::Readable for RND_SOURCE_ENABLE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rnd_source_enable::W](W) writer structure"]
impl crate::Writable for RND_SOURCE_ENABLE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RND_SOURCE_ENABLE to value 0"]
impl crate::Resettable for RND_SOURCE_ENABLE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
