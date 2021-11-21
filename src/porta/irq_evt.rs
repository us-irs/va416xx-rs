#[doc = "Register `IRQ_EVT` reader"]
pub struct R(crate::R<IRQ_EVT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IRQ_EVT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IRQ_EVT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IRQ_EVT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IRQ_EVT` writer"]
pub struct W(crate::W<IRQ_EVT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IRQ_EVT_SPEC>;
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
impl From<crate::W<IRQ_EVT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IRQ_EVT_SPEC>) -> Self {
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
#[doc = "Interrupt Event Register (1:HighLevel/L->H Edge, 0:LowLevel/H->L Edge)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [irq_evt](index.html) module"]
pub struct IRQ_EVT_SPEC;
impl crate::RegisterSpec for IRQ_EVT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [irq_evt::R](R) reader structure"]
impl crate::Readable for IRQ_EVT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [irq_evt::W](W) writer structure"]
impl crate::Writable for IRQ_EVT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets IRQ_EVT to value 0"]
impl crate::Resettable for IRQ_EVT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
