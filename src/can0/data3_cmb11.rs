#[doc = "Register `DATA3_CMB11` reader"]
pub struct R(crate::R<DATA3_CMB11_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DATA3_CMB11_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DATA3_CMB11_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DATA3_CMB11_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DATA3_CMB11` writer"]
pub struct W(crate::W<DATA3_CMB11_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DATA3_CMB11_SPEC>;
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
impl From<crate::W<DATA3_CMB11_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DATA3_CMB11_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BYTE7` reader - Data Byte 7"]
pub struct BYTE7_R(crate::FieldReader<u8, u8>);
impl BYTE7_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        BYTE7_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BYTE7_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BYTE7` writer - Data Byte 7"]
pub struct BYTE7_W<'a> {
    w: &'a mut W,
}
impl<'a> BYTE7_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | ((value as u32 & 0xff) << 8);
        self.w
    }
}
#[doc = "Field `BYTE8` reader - Data Byte 8"]
pub struct BYTE8_R(crate::FieldReader<u8, u8>);
impl BYTE8_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        BYTE8_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BYTE8_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BYTE8` writer - Data Byte 8"]
pub struct BYTE8_W<'a> {
    w: &'a mut W,
}
impl<'a> BYTE8_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 8:15 - Data Byte 7"]
    #[inline(always)]
    pub fn byte7(&self) -> BYTE7_R {
        BYTE7_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 0:7 - Data Byte 8"]
    #[inline(always)]
    pub fn byte8(&self) -> BYTE8_R {
        BYTE8_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 8:15 - Data Byte 7"]
    #[inline(always)]
    pub fn byte7(&mut self) -> BYTE7_W {
        BYTE7_W { w: self }
    }
    #[doc = "Bits 0:7 - Data Byte 8"]
    #[inline(always)]
    pub fn byte8(&mut self) -> BYTE8_W {
        BYTE8_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CAN Frame Data Word 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [data3_cmb11](index.html) module"]
pub struct DATA3_CMB11_SPEC;
impl crate::RegisterSpec for DATA3_CMB11_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [data3_cmb11::R](R) reader structure"]
impl crate::Readable for DATA3_CMB11_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [data3_cmb11::W](W) writer structure"]
impl crate::Writable for DATA3_CMB11_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DATA3_CMB11 to value 0"]
impl crate::Resettable for DATA3_CMB11_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
