#[doc = "Register `S0_DATA` reader"]
pub struct R(crate::R<S0_DATA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<S0_DATA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<S0_DATA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<S0_DATA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `S0_DATA` writer"]
pub struct W(crate::W<S0_DATA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<S0_DATA_SPEC>;
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
impl From<crate::W<S0_DATA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<S0_DATA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `VALUE` reader - I2C data value"]
pub struct VALUE_R(crate::FieldReader<u8, u8>);
impl VALUE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        VALUE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VALUE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VALUE` writer - I2C data value"]
pub struct VALUE_W<'a> {
    w: &'a mut W,
}
impl<'a> VALUE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - I2C data value"]
    #[inline(always)]
    pub fn value(&self) -> VALUE_R {
        VALUE_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - I2C data value"]
    #[inline(always)]
    pub fn value(&mut self) -> VALUE_W {
        VALUE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Slave Data Input/Output\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [s0_data](index.html) module"]
pub struct S0_DATA_SPEC;
impl crate::RegisterSpec for S0_DATA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [s0_data::R](R) reader structure"]
impl crate::Readable for S0_DATA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [s0_data::W](W) writer structure"]
impl crate::Writable for S0_DATA_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets S0_DATA to value 0"]
impl crate::Resettable for S0_DATA_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
