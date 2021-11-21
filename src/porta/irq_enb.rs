#[doc = "Register `IRQ_ENB` reader"]
pub struct R(crate::R<IRQ_ENB_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IRQ_ENB_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IRQ_ENB_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IRQ_ENB_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IRQ_ENB` writer"]
pub struct W(crate::W<IRQ_ENB_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IRQ_ENB_SPEC>;
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
impl From<crate::W<IRQ_ENB_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IRQ_ENB_SPEC>) -> Self {
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
#[doc = "Interrupt Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [irq_enb](index.html) module"]
pub struct IRQ_ENB_SPEC;
impl crate::RegisterSpec for IRQ_ENB_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [irq_enb::R](R) reader structure"]
impl crate::Readable for IRQ_ENB_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [irq_enb::W](W) writer structure"]
impl crate::Writable for IRQ_ENB_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets IRQ_ENB to value 0"]
impl crate::Resettable for IRQ_ENB_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
