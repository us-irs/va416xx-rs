#[doc = "Register `PMU_CTRL` reader"]
pub struct R(crate::R<PMU_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PMU_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PMU_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PMU_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PMU_CTRL` writer"]
pub struct W(crate::W<PMU_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PMU_CTRL_SPEC>;
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
impl From<crate::W<PMU_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PMU_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LVL_SLCT` reader - Select the POK detect level"]
pub struct LVL_SLCT_R(crate::FieldReader<u8, u8>);
impl LVL_SLCT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        LVL_SLCT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LVL_SLCT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LVL_SLCT` writer - Select the POK detect level"]
pub struct LVL_SLCT_W<'a> {
    w: &'a mut W,
}
impl<'a> LVL_SLCT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - Select the POK detect level"]
    #[inline(always)]
    pub fn lvl_slct(&self) -> LVL_SLCT_R {
        LVL_SLCT_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Select the POK detect level"]
    #[inline(always)]
    pub fn lvl_slct(&mut self) -> LVL_SLCT_W {
        LVL_SLCT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PMU Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pmu_ctrl](index.html) module"]
pub struct PMU_CTRL_SPEC;
impl crate::RegisterSpec for PMU_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pmu_ctrl::R](R) reader structure"]
impl crate::Readable for PMU_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pmu_ctrl::W](W) writer structure"]
impl crate::Writable for PMU_CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PMU_CTRL to value 0"]
impl crate::Resettable for PMU_CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
