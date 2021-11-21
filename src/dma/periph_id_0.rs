#[doc = "Register `PERIPH_ID_0` reader"]
pub struct R(crate::R<PERIPH_ID_0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PERIPH_ID_0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PERIPH_ID_0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PERIPH_ID_0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PERIPH_ID_0` writer"]
pub struct W(crate::W<PERIPH_ID_0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PERIPH_ID_0_SPEC>;
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
impl From<crate::W<PERIPH_ID_0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PERIPH_ID_0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PART_NUMBER_0` reader - Part Number"]
pub struct PART_NUMBER_0_R(crate::FieldReader<u8, u8>);
impl PART_NUMBER_0_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PART_NUMBER_0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PART_NUMBER_0_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PART_NUMBER_0` writer - Part Number"]
pub struct PART_NUMBER_0_W<'a> {
    w: &'a mut W,
}
impl<'a> PART_NUMBER_0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Part Number"]
    #[inline(always)]
    pub fn part_number_0(&self) -> PART_NUMBER_0_R {
        PART_NUMBER_0_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Part Number"]
    #[inline(always)]
    pub fn part_number_0(&mut self) -> PART_NUMBER_0_W {
        PART_NUMBER_0_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DMA Peripheral ID 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [periph_id_0](index.html) module"]
pub struct PERIPH_ID_0_SPEC;
impl crate::RegisterSpec for PERIPH_ID_0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [periph_id_0::R](R) reader structure"]
impl crate::Readable for PERIPH_ID_0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [periph_id_0::W](W) writer structure"]
impl crate::Writable for PERIPH_ID_0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PERIPH_ID_0 to value 0x30"]
impl crate::Resettable for PERIPH_ID_0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x30
    }
}
