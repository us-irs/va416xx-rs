#[doc = "Register `TIM_CLK_ENABLE` reader"]
pub struct R(crate::R<TIM_CLK_ENABLE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TIM_CLK_ENABLE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TIM_CLK_ENABLE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TIM_CLK_ENABLE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TIM_CLK_ENABLE` writer"]
pub struct W(crate::W<TIM_CLK_ENABLE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TIM_CLK_ENABLE_SPEC>;
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
impl From<crate::W<TIM_CLK_ENABLE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TIM_CLK_ENABLE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TIMERS` reader - Clock enable of a given TIMER"]
pub struct TIMERS_R(crate::FieldReader<u32, u32>);
impl TIMERS_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        TIMERS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIMERS_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIMERS` writer - Clock enable of a given TIMER"]
pub struct TIMERS_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMERS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x00ff_ffff) | (value as u32 & 0x00ff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:23 - Clock enable of a given TIMER"]
    #[inline(always)]
    pub fn timers(&self) -> TIMERS_R {
        TIMERS_R::new((self.bits & 0x00ff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:23 - Clock enable of a given TIMER"]
    #[inline(always)]
    pub fn timers(&mut self) -> TIMERS_W {
        TIMERS_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TIM Enable Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tim_clk_enable](index.html) module"]
pub struct TIM_CLK_ENABLE_SPEC;
impl crate::RegisterSpec for TIM_CLK_ENABLE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tim_clk_enable::R](R) reader structure"]
impl crate::Readable for TIM_CLK_ENABLE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tim_clk_enable::W](W) writer structure"]
impl crate::Writable for TIM_CLK_ENABLE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TIM_CLK_ENABLE to value 0"]
impl crate::Resettable for TIM_CLK_ENABLE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
