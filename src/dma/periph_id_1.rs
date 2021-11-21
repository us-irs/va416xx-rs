#[doc = "Register `PERIPH_ID_1` reader"]
pub struct R(crate::R<PERIPH_ID_1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PERIPH_ID_1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PERIPH_ID_1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PERIPH_ID_1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `JEP106_ID_3_0` reader - Indentity Code"]
pub struct JEP106_ID_3_0_R(crate::FieldReader<u8, u8>);
impl JEP106_ID_3_0_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        JEP106_ID_3_0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for JEP106_ID_3_0_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PART_NUMBER_1` reader - Part Number 1"]
pub struct PART_NUMBER_1_R(crate::FieldReader<u8, u8>);
impl PART_NUMBER_1_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PART_NUMBER_1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PART_NUMBER_1_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 4:7 - Indentity Code"]
    #[inline(always)]
    pub fn jep106_id_3_0(&self) -> JEP106_ID_3_0_R {
        JEP106_ID_3_0_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 0:3 - Part Number 1"]
    #[inline(always)]
    pub fn part_number_1(&self) -> PART_NUMBER_1_R {
        PART_NUMBER_1_R::new((self.bits & 0x0f) as u8)
    }
}
#[doc = "DMA Peripheral ID 1\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [periph_id_1](index.html) module"]
pub struct PERIPH_ID_1_SPEC;
impl crate::RegisterSpec for PERIPH_ID_1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [periph_id_1::R](R) reader structure"]
impl crate::Readable for PERIPH_ID_1_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets PERIPH_ID_1 to value 0xb2"]
impl crate::Resettable for PERIPH_ID_1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xb2
    }
}
