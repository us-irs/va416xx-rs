#[doc = "Register `STALL_STATUS` reader"]
pub struct R(crate::R<STALL_STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STALL_STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STALL_STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STALL_STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `STALL_STATUS` writer"]
pub struct W(crate::W<STALL_STATUS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<STALL_STATUS_SPEC>;
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
impl From<crate::W<STALL_STATUS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<STALL_STATUS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `STALL_STATUS` reader - DMA is stalled"]
pub struct STALL_STATUS_R(crate::FieldReader<bool, bool>);
impl STALL_STATUS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        STALL_STATUS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for STALL_STATUS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - DMA is stalled"]
    #[inline(always)]
    pub fn stall_status(&self) -> STALL_STATUS_R {
        STALL_STATUS_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DMA stall status\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [stall_status](index.html) module"]
pub struct STALL_STATUS_SPEC;
impl crate::RegisterSpec for STALL_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [stall_status::R](R) reader structure"]
impl crate::Readable for STALL_STATUS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [stall_status::W](W) writer structure"]
impl crate::Writable for STALL_STATUS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets STALL_STATUS to value 0"]
impl crate::Resettable for STALL_STATUS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
