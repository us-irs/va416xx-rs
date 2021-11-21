#[doc = "Register `PERIPH_ID_2` reader"]
pub struct R(crate::R<PERIPH_ID_2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PERIPH_ID_2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PERIPH_ID_2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PERIPH_ID_2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PERIPH_ID_2` writer"]
pub struct W(crate::W<PERIPH_ID_2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PERIPH_ID_2_SPEC>;
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
impl From<crate::W<PERIPH_ID_2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PERIPH_ID_2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `REVISION` reader - Revision"]
pub struct REVISION_R(crate::FieldReader<u8, u8>);
impl REVISION_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        REVISION_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for REVISION_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `REVISION` writer - Revision"]
pub struct REVISION_W<'a> {
    w: &'a mut W,
}
impl<'a> REVISION_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | ((value as u32 & 0x0f) << 4);
        self.w
    }
}
#[doc = "Field `JEDEC_USED` reader - JEDEC"]
pub struct JEDEC_USED_R(crate::FieldReader<bool, bool>);
impl JEDEC_USED_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        JEDEC_USED_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for JEDEC_USED_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `JEDEC_USED` writer - JEDEC"]
pub struct JEDEC_USED_W<'a> {
    w: &'a mut W,
}
impl<'a> JEDEC_USED_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u32 & 0x01) << 3);
        self.w
    }
}
#[doc = "Field `JEP106_ID_6_4` reader - JEP106"]
pub struct JEP106_ID_6_4_R(crate::FieldReader<u8, u8>);
impl JEP106_ID_6_4_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        JEP106_ID_6_4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for JEP106_ID_6_4_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `JEP106_ID_6_4` writer - JEP106"]
pub struct JEP106_ID_6_4_W<'a> {
    w: &'a mut W,
}
impl<'a> JEP106_ID_6_4_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | (value as u32 & 0x07);
        self.w
    }
}
impl R {
    #[doc = "Bits 4:7 - Revision"]
    #[inline(always)]
    pub fn revision(&self) -> REVISION_R {
        REVISION_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 3 - JEDEC"]
    #[inline(always)]
    pub fn jedec_used(&self) -> JEDEC_USED_R {
        JEDEC_USED_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bits 0:2 - JEP106"]
    #[inline(always)]
    pub fn jep106_id_6_4(&self) -> JEP106_ID_6_4_R {
        JEP106_ID_6_4_R::new((self.bits & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 4:7 - Revision"]
    #[inline(always)]
    pub fn revision(&mut self) -> REVISION_W {
        REVISION_W { w: self }
    }
    #[doc = "Bit 3 - JEDEC"]
    #[inline(always)]
    pub fn jedec_used(&mut self) -> JEDEC_USED_W {
        JEDEC_USED_W { w: self }
    }
    #[doc = "Bits 0:2 - JEP106"]
    #[inline(always)]
    pub fn jep106_id_6_4(&mut self) -> JEP106_ID_6_4_W {
        JEP106_ID_6_4_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DMA Peripheral ID 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [periph_id_2](index.html) module"]
pub struct PERIPH_ID_2_SPEC;
impl crate::RegisterSpec for PERIPH_ID_2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [periph_id_2::R](R) reader structure"]
impl crate::Readable for PERIPH_ID_2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [periph_id_2::W](W) writer structure"]
impl crate::Writable for PERIPH_ID_2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PERIPH_ID_2 to value 0xbc"]
impl crate::Resettable for PERIPH_ID_2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xbc
    }
}
