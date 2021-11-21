#[doc = "Register `CNT_VALUE` reader"]
pub struct R(crate::R<CNT_VALUE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CNT_VALUE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CNT_VALUE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CNT_VALUE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CNT_VALUE` writer"]
pub struct W(crate::W<CNT_VALUE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CNT_VALUE_SPEC>;
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
impl From<crate::W<CNT_VALUE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CNT_VALUE_SPEC>) -> Self {
        W(writer)
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
#[doc = "The current value of the counter\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cnt_value](index.html) module"]
pub struct CNT_VALUE_SPEC;
impl crate::RegisterSpec for CNT_VALUE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cnt_value::R](R) reader structure"]
impl crate::Readable for CNT_VALUE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cnt_value::W](W) writer structure"]
impl crate::Writable for CNT_VALUE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CNT_VALUE to value 0"]
impl crate::Resettable for CNT_VALUE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
