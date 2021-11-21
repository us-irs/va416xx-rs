#[doc = "Register `REFRESH_CONFIG_H` reader"]
pub struct R(crate::R<REFRESH_CONFIG_H_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<REFRESH_CONFIG_H_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<REFRESH_CONFIG_H_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<REFRESH_CONFIG_H_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `REFRESH_CONFIG_H` writer"]
pub struct W(crate::W<REFRESH_CONFIG_H_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<REFRESH_CONFIG_H_SPEC>;
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
impl From<crate::W<REFRESH_CONFIG_H_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<REFRESH_CONFIG_H_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DIVCOUNT` reader - Upper 8-bits of the Refresh Rate Counter. Registers are refreshed every DIVCOUNT+1 cycles"]
pub struct DIVCOUNT_R(crate::FieldReader<u8, u8>);
impl DIVCOUNT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DIVCOUNT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DIVCOUNT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DIVCOUNT` writer - Upper 8-bits of the Refresh Rate Counter. Registers are refreshed every DIVCOUNT+1 cycles"]
pub struct DIVCOUNT_W<'a> {
    w: &'a mut W,
}
impl<'a> DIVCOUNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
#[doc = "Field `TESTMODE` reader - Special Test Mode Configuration. 00/01=normal. 10=Force refresh off. 11=Force refresh on constantly."]
pub struct TESTMODE_R(crate::FieldReader<u8, u8>);
impl TESTMODE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        TESTMODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TESTMODE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TESTMODE` writer - Special Test Mode Configuration. 00/01=normal. 10=Force refresh off. 11=Force refresh on constantly."]
pub struct TESTMODE_W<'a> {
    w: &'a mut W,
}
impl<'a> TESTMODE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 30)) | ((value as u32 & 0x03) << 30);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Upper 8-bits of the Refresh Rate Counter. Registers are refreshed every DIVCOUNT+1 cycles"]
    #[inline(always)]
    pub fn divcount(&self) -> DIVCOUNT_R {
        DIVCOUNT_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 30:31 - Special Test Mode Configuration. 00/01=normal. 10=Force refresh off. 11=Force refresh on constantly."]
    #[inline(always)]
    pub fn testmode(&self) -> TESTMODE_R {
        TESTMODE_R::new(((self.bits >> 30) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Upper 8-bits of the Refresh Rate Counter. Registers are refreshed every DIVCOUNT+1 cycles"]
    #[inline(always)]
    pub fn divcount(&mut self) -> DIVCOUNT_W {
        DIVCOUNT_W { w: self }
    }
    #[doc = "Bits 30:31 - Special Test Mode Configuration. 00/01=normal. 10=Force refresh off. 11=Force refresh on constantly."]
    #[inline(always)]
    pub fn testmode(&mut self) -> TESTMODE_W {
        TESTMODE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Register Refresh Rate for TMR registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [refresh_config_h](index.html) module"]
pub struct REFRESH_CONFIG_H_SPEC;
impl crate::RegisterSpec for REFRESH_CONFIG_H_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [refresh_config_h::R](R) reader structure"]
impl crate::Readable for REFRESH_CONFIG_H_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [refresh_config_h::W](W) writer structure"]
impl crate::Writable for REFRESH_CONFIG_H_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets REFRESH_CONFIG_H to value 0"]
impl crate::Resettable for REFRESH_CONFIG_H_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
