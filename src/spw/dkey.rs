#[doc = "Register `DKEY` reader"]
pub struct R(crate::R<DKEY_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DKEY_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DKEY_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DKEY_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DKEY` writer"]
pub struct W(crate::W<DKEY_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DKEY_SPEC>;
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
impl From<crate::W<DKEY_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DKEY_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DESTKEY` reader - RMAP destination key"]
pub struct DESTKEY_R(crate::FieldReader<u8, u8>);
impl DESTKEY_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DESTKEY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DESTKEY_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DESTKEY` writer - RMAP destination key"]
pub struct DESTKEY_W<'a> {
    w: &'a mut W,
}
impl<'a> DESTKEY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - RMAP destination key"]
    #[inline(always)]
    pub fn destkey(&self) -> DESTKEY_R {
        DESTKEY_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - RMAP destination key"]
    #[inline(always)]
    pub fn destkey(&mut self) -> DESTKEY_W {
        DESTKEY_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Destination Key\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dkey](index.html) module"]
pub struct DKEY_SPEC;
impl crate::RegisterSpec for DKEY_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dkey::R](R) reader structure"]
impl crate::Readable for DKEY_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dkey::W](W) writer structure"]
impl crate::Writable for DKEY_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DKEY to value 0"]
impl crate::Resettable for DKEY_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
