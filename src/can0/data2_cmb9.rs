#[doc = "Register `DATA2_CMB9` reader"]
pub struct R(crate::R<DATA2_CMB9_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DATA2_CMB9_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DATA2_CMB9_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DATA2_CMB9_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DATA2_CMB9` writer"]
pub struct W(crate::W<DATA2_CMB9_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DATA2_CMB9_SPEC>;
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
impl From<crate::W<DATA2_CMB9_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DATA2_CMB9_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BYTE5` reader - Data Byte 5"]
pub struct BYTE5_R(crate::FieldReader<u8, u8>);
impl BYTE5_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        BYTE5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BYTE5_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BYTE5` writer - Data Byte 5"]
pub struct BYTE5_W<'a> {
    w: &'a mut W,
}
impl<'a> BYTE5_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | ((value as u32 & 0xff) << 8);
        self.w
    }
}
#[doc = "Field `BYTE6` reader - Data Byte 6"]
pub struct BYTE6_R(crate::FieldReader<u8, u8>);
impl BYTE6_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        BYTE6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BYTE6_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BYTE6` writer - Data Byte 6"]
pub struct BYTE6_W<'a> {
    w: &'a mut W,
}
impl<'a> BYTE6_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 8:15 - Data Byte 5"]
    #[inline(always)]
    pub fn byte5(&self) -> BYTE5_R {
        BYTE5_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 0:7 - Data Byte 6"]
    #[inline(always)]
    pub fn byte6(&self) -> BYTE6_R {
        BYTE6_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 8:15 - Data Byte 5"]
    #[inline(always)]
    pub fn byte5(&mut self) -> BYTE5_W {
        BYTE5_W { w: self }
    }
    #[doc = "Bits 0:7 - Data Byte 6"]
    #[inline(always)]
    pub fn byte6(&mut self) -> BYTE6_W {
        BYTE6_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CAN Frame Data Word 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [data2_cmb9](index.html) module"]
pub struct DATA2_CMB9_SPEC;
impl crate::RegisterSpec for DATA2_CMB9_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [data2_cmb9::R](R) reader structure"]
impl crate::Readable for DATA2_CMB9_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [data2_cmb9::W](W) writer structure"]
impl crate::Writable for DATA2_CMB9_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DATA2_CMB9 to value 0"]
impl crate::Resettable for DATA2_CMB9_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
