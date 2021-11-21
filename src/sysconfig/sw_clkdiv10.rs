#[doc = "Register `SW_CLKDIV10` reader"]
pub struct R(crate::R<SW_CLKDIV10_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SW_CLKDIV10_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SW_CLKDIV10_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SW_CLKDIV10_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SW_CLKDIV10` writer"]
pub struct W(crate::W<SW_CLKDIV10_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SW_CLKDIV10_SPEC>;
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
impl From<crate::W<SW_CLKDIV10_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SW_CLKDIV10_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SW_CLKDIV10` reader - Defines the initial value for the SpW clock, defaults to divide by ten"]
pub struct SW_CLKDIV10_R(crate::FieldReader<u8, u8>);
impl SW_CLKDIV10_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SW_CLKDIV10_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SW_CLKDIV10_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SW_CLKDIV10` writer - Defines the initial value for the SpW clock, defaults to divide by ten"]
pub struct SW_CLKDIV10_W<'a> {
    w: &'a mut W,
}
impl<'a> SW_CLKDIV10_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Defines the initial value for the SpW clock, defaults to divide by ten"]
    #[inline(always)]
    pub fn sw_clkdiv10(&self) -> SW_CLKDIV10_R {
        SW_CLKDIV10_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Defines the initial value for the SpW clock, defaults to divide by ten"]
    #[inline(always)]
    pub fn sw_clkdiv10(&mut self) -> SW_CLKDIV10_W {
        SW_CLKDIV10_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Initial SpW Clock Divider Value\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sw_clkdiv10](index.html) module"]
pub struct SW_CLKDIV10_SPEC;
impl crate::RegisterSpec for SW_CLKDIV10_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sw_clkdiv10::R](R) reader structure"]
impl crate::Readable for SW_CLKDIV10_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sw_clkdiv10::W](W) writer structure"]
impl crate::Writable for SW_CLKDIV10_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SW_CLKDIV10 to value 0x09"]
impl crate::Resettable for SW_CLKDIV10_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x09
    }
}
