#[doc = "Register `TARGET_TIME_NSEC` reader"]
pub struct R(crate::R<TARGET_TIME_NSEC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TARGET_TIME_NSEC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TARGET_TIME_NSEC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TARGET_TIME_NSEC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TARGET_TIME_NSEC` writer"]
pub struct W(crate::W<TARGET_TIME_NSEC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TARGET_TIME_NSEC_SPEC>;
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
impl From<crate::W<TARGET_TIME_NSEC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TARGET_TIME_NSEC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TRGTBUSY` reader - 32 Bits of Hash Table"]
pub struct TRGTBUSY_R(crate::FieldReader<bool, bool>);
impl TRGTBUSY_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TRGTBUSY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TRGTBUSY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TRGTBUSY` writer - 32 Bits of Hash Table"]
pub struct TRGTBUSY_W<'a> {
    w: &'a mut W,
}
impl<'a> TRGTBUSY_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 31)) | ((value as u32 & 0x01) << 31);
        self.w
    }
}
#[doc = "Field `TTSLO` reader - Target Timestamp Low Register"]
pub struct TTSLO_R(crate::FieldReader<u32, u32>);
impl TTSLO_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        TTSLO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TTSLO_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TTSLO` writer - Target Timestamp Low Register"]
pub struct TTSLO_W<'a> {
    w: &'a mut W,
}
impl<'a> TTSLO_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7fff_ffff) | (value as u32 & 0x7fff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bit 31 - 32 Bits of Hash Table"]
    #[inline(always)]
    pub fn trgtbusy(&self) -> TRGTBUSY_R {
        TRGTBUSY_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bits 0:30 - Target Timestamp Low Register"]
    #[inline(always)]
    pub fn ttslo(&self) -> TTSLO_R {
        TTSLO_R::new((self.bits & 0x7fff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bit 31 - 32 Bits of Hash Table"]
    #[inline(always)]
    pub fn trgtbusy(&mut self) -> TRGTBUSY_W {
        TRGTBUSY_W { w: self }
    }
    #[doc = "Bits 0:30 - Target Timestamp Low Register"]
    #[inline(always)]
    pub fn ttslo(&mut self) -> TTSLO_W {
        TTSLO_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Holds the lower 32-bits of time to be compared with the system time\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [target_time_nsec](index.html) module"]
pub struct TARGET_TIME_NSEC_SPEC;
impl crate::RegisterSpec for TARGET_TIME_NSEC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [target_time_nsec::R](R) reader structure"]
impl crate::Readable for TARGET_TIME_NSEC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [target_time_nsec::W](W) writer structure"]
impl crate::Writable for TARGET_TIME_NSEC_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TARGET_TIME_NSEC to value 0"]
impl crate::Resettable for TARGET_TIME_NSEC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
