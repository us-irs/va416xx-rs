#[doc = "Register `DIRBYTE[%s]` reader"]
pub struct R(crate::R<DIRBYTE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DIRBYTE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DIRBYTE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DIRBYTE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DIRBYTE[%s]` writer"]
pub struct W(crate::W<DIRBYTE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DIRBYTE_SPEC>;
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
impl From<crate::W<DIRBYTE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DIRBYTE_SPEC>) -> Self {
        W(writer)
    }
}
impl W {
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Direction Register by Byte\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dirbyte](index.html) module"]
pub struct DIRBYTE_SPEC;
impl crate::RegisterSpec for DIRBYTE_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [dirbyte::R](R) reader structure"]
impl crate::Readable for DIRBYTE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dirbyte::W](W) writer structure"]
impl crate::Writable for DIRBYTE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DIRBYTE[%s]
to value 0"]
impl crate::Resettable for DIRBYTE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
