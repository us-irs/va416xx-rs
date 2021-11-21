#[doc = "Register `PRIMECELL_ID_2` reader"]
pub struct R(crate::R<PRIMECELL_ID_2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PRIMECELL_ID_2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PRIMECELL_ID_2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PRIMECELL_ID_2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PRIMECELL_ID_2` writer"]
pub struct W(crate::W<PRIMECELL_ID_2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PRIMECELL_ID_2_SPEC>;
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
impl From<crate::W<PRIMECELL_ID_2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PRIMECELL_ID_2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PRIMECELL_ID_2` reader - PrimeCell Identification"]
pub struct PRIMECELL_ID_2_R(crate::FieldReader<u8, u8>);
impl PRIMECELL_ID_2_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PRIMECELL_ID_2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PRIMECELL_ID_2_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PRIMECELL_ID_2` writer - PrimeCell Identification"]
pub struct PRIMECELL_ID_2_W<'a> {
    w: &'a mut W,
}
impl<'a> PRIMECELL_ID_2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - PrimeCell Identification"]
    #[inline(always)]
    pub fn primecell_id_2(&self) -> PRIMECELL_ID_2_R {
        PRIMECELL_ID_2_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - PrimeCell Identification"]
    #[inline(always)]
    pub fn primecell_id_2(&mut self) -> PRIMECELL_ID_2_W {
        PRIMECELL_ID_2_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DMA PrimeCell ID 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [primecell_id_2](index.html) module"]
pub struct PRIMECELL_ID_2_SPEC;
impl crate::RegisterSpec for PRIMECELL_ID_2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [primecell_id_2::R](R) reader structure"]
impl crate::Readable for PRIMECELL_ID_2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [primecell_id_2::W](W) writer structure"]
impl crate::Writable for PRIMECELL_ID_2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PRIMECELL_ID_2 to value 0x05"]
impl crate::Resettable for PRIMECELL_ID_2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x05
    }
}
