#[doc = "Register `DATA1_CMB8` reader"]
pub struct R(crate::R<DATA1_CMB8_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DATA1_CMB8_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DATA1_CMB8_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DATA1_CMB8_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DATA1_CMB8` writer"]
pub struct W(crate::W<DATA1_CMB8_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DATA1_CMB8_SPEC>;
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
impl From<crate::W<DATA1_CMB8_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DATA1_CMB8_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BYTE3` reader - Data Byte 3"]
pub struct BYTE3_R(crate::FieldReader<u8, u8>);
impl BYTE3_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        BYTE3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BYTE3_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BYTE3` writer - Data Byte 3"]
pub struct BYTE3_W<'a> {
    w: &'a mut W,
}
impl<'a> BYTE3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | ((value as u32 & 0xff) << 8);
        self.w
    }
}
#[doc = "Field `BYTE4` reader - Data Byte 4"]
pub struct BYTE4_R(crate::FieldReader<u8, u8>);
impl BYTE4_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        BYTE4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BYTE4_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BYTE4` writer - Data Byte 4"]
pub struct BYTE4_W<'a> {
    w: &'a mut W,
}
impl<'a> BYTE4_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 8:15 - Data Byte 3"]
    #[inline(always)]
    pub fn byte3(&self) -> BYTE3_R {
        BYTE3_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 0:7 - Data Byte 4"]
    #[inline(always)]
    pub fn byte4(&self) -> BYTE4_R {
        BYTE4_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 8:15 - Data Byte 3"]
    #[inline(always)]
    pub fn byte3(&mut self) -> BYTE3_W {
        BYTE3_W { w: self }
    }
    #[doc = "Bits 0:7 - Data Byte 4"]
    #[inline(always)]
    pub fn byte4(&mut self) -> BYTE4_W {
        BYTE4_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CAN Frame Data Word 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [data1_cmb8](index.html) module"]
pub struct DATA1_CMB8_SPEC;
impl crate::RegisterSpec for DATA1_CMB8_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [data1_cmb8::R](R) reader structure"]
impl crate::Readable for DATA1_CMB8_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [data1_cmb8::W](W) writer structure"]
impl crate::Writable for DATA1_CMB8_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DATA1_CMB8 to value 0"]
impl crate::Resettable for DATA1_CMB8_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
