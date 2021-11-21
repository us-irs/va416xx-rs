#[doc = "Register `RAM0_MBE` reader"]
pub struct R(crate::R<RAM0_MBE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RAM0_MBE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RAM0_MBE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RAM0_MBE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RAM0_MBE` writer"]
pub struct W(crate::W<RAM0_MBE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RAM0_MBE_SPEC>;
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
impl From<crate::W<RAM0_MBE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RAM0_MBE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `COUNT` reader - RAM0 Multi Bit Errors"]
pub struct COUNT_R(crate::FieldReader<u16, u16>);
impl COUNT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        COUNT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for COUNT_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `COUNT` writer - RAM0 Multi Bit Errors"]
pub struct COUNT_W<'a> {
    w: &'a mut W,
}
impl<'a> COUNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - RAM0 Multi Bit Errors"]
    #[inline(always)]
    pub fn count(&self) -> COUNT_R {
        COUNT_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - RAM0 Multi Bit Errors"]
    #[inline(always)]
    pub fn count(&mut self) -> COUNT_W {
        COUNT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Count of RAM0 EDAC Multi Bit Errors\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ram0_mbe](index.html) module"]
pub struct RAM0_MBE_SPEC;
impl crate::RegisterSpec for RAM0_MBE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ram0_mbe::R](R) reader structure"]
impl crate::Readable for RAM0_MBE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ram0_mbe::W](W) writer structure"]
impl crate::Writable for RAM0_MBE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RAM0_MBE to value 0"]
impl crate::Resettable for RAM0_MBE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
