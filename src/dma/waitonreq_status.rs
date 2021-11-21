#[doc = "Register `WAITONREQ_STATUS` reader"]
pub struct R(crate::R<WAITONREQ_STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WAITONREQ_STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WAITONREQ_STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WAITONREQ_STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CH3` reader - DMA wait on request"]
pub struct CH3_R(crate::FieldReader<bool, bool>);
impl CH3_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CH3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CH3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CH2` reader - DMA wait on request"]
pub struct CH2_R(crate::FieldReader<bool, bool>);
impl CH2_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CH2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CH2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CH1` reader - DMA wait on request"]
pub struct CH1_R(crate::FieldReader<bool, bool>);
impl CH1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CH1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CH1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CH0` reader - DMA wait on request"]
pub struct CH0_R(crate::FieldReader<bool, bool>);
impl CH0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CH0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CH0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 3 - DMA wait on request"]
    #[inline(always)]
    pub fn ch3(&self) -> CH3_R {
        CH3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - DMA wait on request"]
    #[inline(always)]
    pub fn ch2(&self) -> CH2_R {
        CH2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - DMA wait on request"]
    #[inline(always)]
    pub fn ch1(&self) -> CH1_R {
        CH1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - DMA wait on request"]
    #[inline(always)]
    pub fn ch0(&self) -> CH0_R {
        CH0_R::new((self.bits & 0x01) != 0)
    }
}
#[doc = "DMA channel wait on request status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [waitonreq_status](index.html) module"]
pub struct WAITONREQ_STATUS_SPEC;
impl crate::RegisterSpec for WAITONREQ_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [waitonreq_status::R](R) reader structure"]
impl crate::Readable for WAITONREQ_STATUS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets WAITONREQ_STATUS to value 0"]
impl crate::Resettable for WAITONREQ_STATUS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
