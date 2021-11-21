#[doc = "Register `DATA0_CMB6` reader"]
pub struct R(crate::R<DATA0_CMB6_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DATA0_CMB6_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DATA0_CMB6_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DATA0_CMB6_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DATA0_CMB6` writer"]
pub struct W(crate::W<DATA0_CMB6_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DATA0_CMB6_SPEC>;
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
impl From<crate::W<DATA0_CMB6_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DATA0_CMB6_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BYTE1` reader - Data Byte 1"]
pub struct BYTE1_R(crate::FieldReader<u8, u8>);
impl BYTE1_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        BYTE1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BYTE1_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BYTE1` writer - Data Byte 1"]
pub struct BYTE1_W<'a> {
    w: &'a mut W,
}
impl<'a> BYTE1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | ((value as u32 & 0xff) << 8);
        self.w
    }
}
#[doc = "Field `BYTE2` reader - Data Byte 2"]
pub struct BYTE2_R(crate::FieldReader<u8, u8>);
impl BYTE2_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        BYTE2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BYTE2_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BYTE2` writer - Data Byte 2"]
pub struct BYTE2_W<'a> {
    w: &'a mut W,
}
impl<'a> BYTE2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 8:15 - Data Byte 1"]
    #[inline(always)]
    pub fn byte1(&self) -> BYTE1_R {
        BYTE1_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 0:7 - Data Byte 2"]
    #[inline(always)]
    pub fn byte2(&self) -> BYTE2_R {
        BYTE2_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 8:15 - Data Byte 1"]
    #[inline(always)]
    pub fn byte1(&mut self) -> BYTE1_W {
        BYTE1_W { w: self }
    }
    #[doc = "Bits 0:7 - Data Byte 2"]
    #[inline(always)]
    pub fn byte2(&mut self) -> BYTE2_W {
        BYTE2_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CAN Frame Data Word 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [data0_cmb6](index.html) module"]
pub struct DATA0_CMB6_SPEC;
impl crate::RegisterSpec for DATA0_CMB6_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [data0_cmb6::R](R) reader structure"]
impl crate::Readable for DATA0_CMB6_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [data0_cmb6::W](W) writer structure"]
impl crate::Writable for DATA0_CMB6_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DATA0_CMB6 to value 0"]
impl crate::Resettable for DATA0_CMB6_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
