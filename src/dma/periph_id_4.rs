#[doc = "Register `PERIPH_ID_4` reader"]
pub struct R(crate::R<PERIPH_ID_4_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PERIPH_ID_4_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PERIPH_ID_4_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PERIPH_ID_4_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PERIPH_ID_4` writer"]
pub struct W(crate::W<PERIPH_ID_4_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PERIPH_ID_4_SPEC>;
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
impl From<crate::W<PERIPH_ID_4_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PERIPH_ID_4_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BLOCK_COUNT` reader - The Number of 4k Address Blocks Required"]
pub struct BLOCK_COUNT_R(crate::FieldReader<u8, u8>);
impl BLOCK_COUNT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        BLOCK_COUNT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BLOCK_COUNT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BLOCK_COUNT` writer - The Number of 4k Address Blocks Required"]
pub struct BLOCK_COUNT_W<'a> {
    w: &'a mut W,
}
impl<'a> BLOCK_COUNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | ((value as u32 & 0x0f) << 4);
        self.w
    }
}
#[doc = "Field `JEP106_C_CODE` reader - JEP106"]
pub struct JEP106_C_CODE_R(crate::FieldReader<u8, u8>);
impl JEP106_C_CODE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        JEP106_C_CODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for JEP106_C_CODE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `JEP106_C_CODE` writer - JEP106"]
pub struct JEP106_C_CODE_W<'a> {
    w: &'a mut W,
}
impl<'a> JEP106_C_CODE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | (value as u32 & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bits 4:7 - The Number of 4k Address Blocks Required"]
    #[inline(always)]
    pub fn block_count(&self) -> BLOCK_COUNT_R {
        BLOCK_COUNT_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 0:3 - JEP106"]
    #[inline(always)]
    pub fn jep106_c_code(&self) -> JEP106_C_CODE_R {
        JEP106_C_CODE_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 4:7 - The Number of 4k Address Blocks Required"]
    #[inline(always)]
    pub fn block_count(&mut self) -> BLOCK_COUNT_W {
        BLOCK_COUNT_W { w: self }
    }
    #[doc = "Bits 0:3 - JEP106"]
    #[inline(always)]
    pub fn jep106_c_code(&mut self) -> JEP106_C_CODE_W {
        JEP106_C_CODE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DMA Peripheral ID 4\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [periph_id_4](index.html) module"]
pub struct PERIPH_ID_4_SPEC;
impl crate::RegisterSpec for PERIPH_ID_4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [periph_id_4::R](R) reader structure"]
impl crate::Readable for PERIPH_ID_4_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [periph_id_4::W](W) writer structure"]
impl crate::Writable for PERIPH_ID_4_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PERIPH_ID_4 to value 0x04"]
impl crate::Resettable for PERIPH_ID_4_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x04
    }
}
