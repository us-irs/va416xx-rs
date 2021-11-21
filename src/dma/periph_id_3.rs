#[doc = "Register `PERIPH_ID_3` reader"]
pub struct R(crate::R<PERIPH_ID_3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PERIPH_ID_3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PERIPH_ID_3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PERIPH_ID_3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PERIPH_ID_3` writer"]
pub struct W(crate::W<PERIPH_ID_3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PERIPH_ID_3_SPEC>;
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
impl From<crate::W<PERIPH_ID_3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PERIPH_ID_3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MOD_NUMBER` reader - Controller Modifications"]
pub struct MOD_NUMBER_R(crate::FieldReader<u8, u8>);
impl MOD_NUMBER_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        MOD_NUMBER_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MOD_NUMBER_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MOD_NUMBER` writer - Controller Modifications"]
pub struct MOD_NUMBER_W<'a> {
    w: &'a mut W,
}
impl<'a> MOD_NUMBER_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | (value as u32 & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - Controller Modifications"]
    #[inline(always)]
    pub fn mod_number(&self) -> MOD_NUMBER_R {
        MOD_NUMBER_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Controller Modifications"]
    #[inline(always)]
    pub fn mod_number(&mut self) -> MOD_NUMBER_W {
        MOD_NUMBER_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DMA Peripheral ID 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [periph_id_3](index.html) module"]
pub struct PERIPH_ID_3_SPEC;
impl crate::RegisterSpec for PERIPH_ID_3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [periph_id_3::R](R) reader structure"]
impl crate::Readable for PERIPH_ID_3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [periph_id_3::W](W) writer structure"]
impl crate::Writable for PERIPH_ID_3_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PERIPH_ID_3 to value 0"]
impl crate::Resettable for PERIPH_ID_3_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
