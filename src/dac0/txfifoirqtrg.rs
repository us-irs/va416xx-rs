#[doc = "Register `TXFIFOIRQTRG` reader"]
pub struct R(crate::R<TXFIFOIRQTRG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TXFIFOIRQTRG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TXFIFOIRQTRG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TXFIFOIRQTRG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TXFIFOIRQTRG` writer"]
pub struct W(crate::W<TXFIFOIRQTRG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TXFIFOIRQTRG_SPEC>;
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
impl From<crate::W<TXFIFOIRQTRG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TXFIFOIRQTRG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LEVEL` reader - Sets the FIFO_ENTRY_CNT value that asserts the FIFO_DEPTH_TRIG interrupt"]
pub struct LEVEL_R(crate::FieldReader<u8, u8>);
impl LEVEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        LEVEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LEVEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LEVEL` writer - Sets the FIFO_ENTRY_CNT value that asserts the FIFO_DEPTH_TRIG interrupt"]
pub struct LEVEL_W<'a> {
    w: &'a mut W,
}
impl<'a> LEVEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | (value as u32 & 0x1f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:4 - Sets the FIFO_ENTRY_CNT value that asserts the FIFO_DEPTH_TRIG interrupt"]
    #[inline(always)]
    pub fn level(&self) -> LEVEL_R {
        LEVEL_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Sets the FIFO_ENTRY_CNT value that asserts the FIFO_DEPTH_TRIG interrupt"]
    #[inline(always)]
    pub fn level(&mut self) -> LEVEL_W {
        LEVEL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Receive FIFO Interrupt Trigger Value\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [txfifoirqtrg](index.html) module"]
pub struct TXFIFOIRQTRG_SPEC;
impl crate::RegisterSpec for TXFIFOIRQTRG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [txfifoirqtrg::R](R) reader structure"]
impl crate::Readable for TXFIFOIRQTRG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [txfifoirqtrg::W](W) writer structure"]
impl crate::Writable for TXFIFOIRQTRG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TXFIFOIRQTRG to value 0x10"]
impl crate::Resettable for TXFIFOIRQTRG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x10
    }
}
