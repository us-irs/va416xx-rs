#[doc = "Register `MAC_GMII_DATA` reader"]
pub struct R(crate::R<MAC_GMII_DATA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MAC_GMII_DATA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MAC_GMII_DATA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MAC_GMII_DATA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MAC_GMII_DATA` writer"]
pub struct W(crate::W<MAC_GMII_DATA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MAC_GMII_DATA_SPEC>;
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
impl From<crate::W<MAC_GMII_DATA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MAC_GMII_DATA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `GD` reader - GMII Data"]
pub struct GD_R(crate::FieldReader<u16, u16>);
impl GD_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        GD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GD_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GD` writer - GMII Data"]
pub struct GD_W<'a> {
    w: &'a mut W,
}
impl<'a> GD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - GMII Data"]
    #[inline(always)]
    pub fn gd(&self) -> GD_R {
        GD_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - GMII Data"]
    #[inline(always)]
    pub fn gd(&mut self) -> GD_W {
        GD_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Contains the data to be written to or read from the PHY register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mac_gmii_data](index.html) module"]
pub struct MAC_GMII_DATA_SPEC;
impl crate::RegisterSpec for MAC_GMII_DATA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mac_gmii_data::R](R) reader structure"]
impl crate::Readable for MAC_GMII_DATA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mac_gmii_data::W](W) writer structure"]
impl crate::Writable for MAC_GMII_DATA_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MAC_GMII_DATA to value 0"]
impl crate::Resettable for MAC_GMII_DATA_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
