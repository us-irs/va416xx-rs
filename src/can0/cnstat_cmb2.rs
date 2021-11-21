#[doc = "Register `CNSTAT_CMB2` reader"]
pub struct R(crate::R<CNSTAT_CMB2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CNSTAT_CMB2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CNSTAT_CMB2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CNSTAT_CMB2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CNSTAT_CMB2` writer"]
pub struct W(crate::W<CNSTAT_CMB2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CNSTAT_CMB2_SPEC>;
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
impl From<crate::W<CNSTAT_CMB2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CNSTAT_CMB2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DLC` reader - Data Length Code"]
pub struct DLC_R(crate::FieldReader<u8, u8>);
impl DLC_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DLC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DLC_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DLC` writer - Data Length Code"]
pub struct DLC_W<'a> {
    w: &'a mut W,
}
impl<'a> DLC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 12)) | ((value as u32 & 0x0f) << 12);
        self.w
    }
}
#[doc = "Field `PRI` reader - Transmit Priority Code"]
pub struct PRI_R(crate::FieldReader<u8, u8>);
impl PRI_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PRI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PRI_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PRI` writer - Transmit Priority Code"]
pub struct PRI_W<'a> {
    w: &'a mut W,
}
impl<'a> PRI_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | ((value as u32 & 0x0f) << 4);
        self.w
    }
}
#[doc = "Field `ST` reader - Buffer Status"]
pub struct ST_R(crate::FieldReader<u8, u8>);
impl ST_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        ST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ST_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ST` writer - Buffer Status"]
pub struct ST_W<'a> {
    w: &'a mut W,
}
impl<'a> ST_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | (value as u32 & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bits 12:15 - Data Length Code"]
    #[inline(always)]
    pub fn dlc(&self) -> DLC_R {
        DLC_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Transmit Priority Code"]
    #[inline(always)]
    pub fn pri(&self) -> PRI_R {
        PRI_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 0:3 - Buffer Status"]
    #[inline(always)]
    pub fn st(&self) -> ST_R {
        ST_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 12:15 - Data Length Code"]
    #[inline(always)]
    pub fn dlc(&mut self) -> DLC_W {
        DLC_W { w: self }
    }
    #[doc = "Bits 4:7 - Transmit Priority Code"]
    #[inline(always)]
    pub fn pri(&mut self) -> PRI_W {
        PRI_W { w: self }
    }
    #[doc = "Bits 0:3 - Buffer Status"]
    #[inline(always)]
    pub fn st(&mut self) -> ST_W {
        ST_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Buffer Status / Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cnstat_cmb2](index.html) module"]
pub struct CNSTAT_CMB2_SPEC;
impl crate::RegisterSpec for CNSTAT_CMB2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cnstat_cmb2::R](R) reader structure"]
impl crate::Readable for CNSTAT_CMB2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cnstat_cmb2::W](W) writer structure"]
impl crate::Writable for CNSTAT_CMB2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CNSTAT_CMB2 to value 0"]
impl crate::Resettable for CNSTAT_CMB2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
