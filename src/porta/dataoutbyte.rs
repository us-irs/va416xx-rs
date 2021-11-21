#[doc = "Register `DATAOUTBYTE[%s]` writer"]
pub struct W(crate::W<DATAOUTBYTE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DATAOUTBYTE_SPEC>;
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
impl From<crate::W<DATAOUTBYTE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DATAOUTBYTE_SPEC>) -> Self {
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
#[doc = "Data Out Register by Byte\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dataoutbyte](index.html) module"]
pub struct DATAOUTBYTE_SPEC;
impl crate::RegisterSpec for DATAOUTBYTE_SPEC {
    type Ux = u8;
}
#[doc = "`write(|w| ..)` method takes [dataoutbyte::W](W) writer structure"]
impl crate::Writable for DATAOUTBYTE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DATAOUTBYTE[%s]
to value 0"]
impl crate::Resettable for DATAOUTBYTE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
