#[doc = "Register `REFRESH_CONFIG_L` reader"]
pub struct R(crate::R<REFRESH_CONFIG_L_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<REFRESH_CONFIG_L_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<REFRESH_CONFIG_L_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<REFRESH_CONFIG_L_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `REFRESH_CONFIG_L` writer"]
pub struct W(crate::W<REFRESH_CONFIG_L_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<REFRESH_CONFIG_L_SPEC>;
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
impl From<crate::W<REFRESH_CONFIG_L_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<REFRESH_CONFIG_L_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DIVCOUNT` reader - Lower 32-bits of the Refresh Rate Counter. Registers are refreshed every DIVCOUNT+1 cycles"]
pub struct DIVCOUNT_R(crate::FieldReader<u32, u32>);
impl DIVCOUNT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        DIVCOUNT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DIVCOUNT_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DIVCOUNT` writer - Lower 32-bits of the Refresh Rate Counter. Registers are refreshed every DIVCOUNT+1 cycles"]
pub struct DIVCOUNT_W<'a> {
    w: &'a mut W,
}
impl<'a> DIVCOUNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = value as u32;
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Lower 32-bits of the Refresh Rate Counter. Registers are refreshed every DIVCOUNT+1 cycles"]
    #[inline(always)]
    pub fn divcount(&self) -> DIVCOUNT_R {
        DIVCOUNT_R::new(self.bits as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Lower 32-bits of the Refresh Rate Counter. Registers are refreshed every DIVCOUNT+1 cycles"]
    #[inline(always)]
    pub fn divcount(&mut self) -> DIVCOUNT_W {
        DIVCOUNT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Register Refresh Rate for TMR registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [refresh_config_l](index.html) module"]
pub struct REFRESH_CONFIG_L_SPEC;
impl crate::RegisterSpec for REFRESH_CONFIG_L_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [refresh_config_l::R](R) reader structure"]
impl crate::Readable for REFRESH_CONFIG_L_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [refresh_config_l::W](W) writer structure"]
impl crate::Writable for REFRESH_CONFIG_L_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets REFRESH_CONFIG_L to value 0x0f"]
impl crate::Resettable for REFRESH_CONFIG_L_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0f
    }
}
