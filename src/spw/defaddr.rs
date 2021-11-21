#[doc = "Register `DEFADDR` reader"]
pub struct R(crate::R<DEFADDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DEFADDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DEFADDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DEFADDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DEFADDR` writer"]
pub struct W(crate::W<DEFADDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DEFADDR_SPEC>;
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
impl From<crate::W<DEFADDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DEFADDR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DEFMASK` reader - 8-bit default mask used for node identification on the SpaceWire network"]
pub struct DEFMASK_R(crate::FieldReader<u8, u8>);
impl DEFMASK_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DEFMASK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DEFMASK_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DEFMASK` writer - 8-bit default mask used for node identification on the SpaceWire network"]
pub struct DEFMASK_W<'a> {
    w: &'a mut W,
}
impl<'a> DEFMASK_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | ((value as u32 & 0xff) << 8);
        self.w
    }
}
#[doc = "Field `DEFADDR` reader - 8-bit node address used for node identification on the SpaceWire network"]
pub struct DEFADDR_R(crate::FieldReader<u8, u8>);
impl DEFADDR_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DEFADDR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DEFADDR_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DEFADDR` writer - 8-bit node address used for node identification on the SpaceWire network"]
pub struct DEFADDR_W<'a> {
    w: &'a mut W,
}
impl<'a> DEFADDR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 8:15 - 8-bit default mask used for node identification on the SpaceWire network"]
    #[inline(always)]
    pub fn defmask(&self) -> DEFMASK_R {
        DEFMASK_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 0:7 - 8-bit node address used for node identification on the SpaceWire network"]
    #[inline(always)]
    pub fn defaddr(&self) -> DEFADDR_R {
        DEFADDR_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 8:15 - 8-bit default mask used for node identification on the SpaceWire network"]
    #[inline(always)]
    pub fn defmask(&mut self) -> DEFMASK_W {
        DEFMASK_W { w: self }
    }
    #[doc = "Bits 0:7 - 8-bit node address used for node identification on the SpaceWire network"]
    #[inline(always)]
    pub fn defaddr(&mut self) -> DEFADDR_W {
        DEFADDR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Node Address Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [defaddr](index.html) module"]
pub struct DEFADDR_SPEC;
impl crate::RegisterSpec for DEFADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [defaddr::R](R) reader structure"]
impl crate::Readable for DEFADDR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [defaddr::W](W) writer structure"]
impl crate::Writable for DEFADDR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DEFADDR to value 0xfe"]
impl crate::Resettable for DEFADDR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xfe
    }
}
