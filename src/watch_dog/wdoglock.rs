#[doc = "Register `WDOGLOCK` reader"]
pub struct R(crate::R<WDOGLOCK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WDOGLOCK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WDOGLOCK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WDOGLOCK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `WDOGLOCK` writer"]
pub struct W(crate::W<WDOGLOCK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WDOGLOCK_SPEC>;
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
impl From<crate::W<WDOGLOCK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WDOGLOCK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `REG_WR_EN` reader - Register write enable status"]
pub struct REG_WR_EN_R(crate::FieldReader<u32, u32>);
impl REG_WR_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        REG_WR_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for REG_WR_EN_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `REG_WR_EN` writer - Register write enable status"]
pub struct REG_WR_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_WR_EN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = value as u32;
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Register write enable status"]
    #[inline(always)]
    pub fn reg_wr_en(&self) -> REG_WR_EN_R {
        REG_WR_EN_R::new(self.bits as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Register write enable status"]
    #[inline(always)]
    pub fn reg_wr_en(&mut self) -> REG_WR_EN_W {
        REG_WR_EN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Lock\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wdoglock](index.html) module"]
pub struct WDOGLOCK_SPEC;
impl crate::RegisterSpec for WDOGLOCK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [wdoglock::R](R) reader structure"]
impl crate::Readable for WDOGLOCK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [wdoglock::W](W) writer structure"]
impl crate::Writable for WDOGLOCK_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets WDOGLOCK to value 0"]
impl crate::Resettable for WDOGLOCK_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
