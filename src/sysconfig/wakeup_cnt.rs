#[doc = "Register `WAKEUP_CNT` reader"]
pub struct R(crate::R<WAKEUP_CNT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WAKEUP_CNT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WAKEUP_CNT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WAKEUP_CNT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `WAKEUP_CNT` writer"]
pub struct W(crate::W<WAKEUP_CNT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WAKEUP_CNT_SPEC>;
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
impl From<crate::W<WAKEUP_CNT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WAKEUP_CNT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CNTSTRT` reader - Launch SLP mode in analog block"]
pub struct CNTSTRT_R(crate::FieldReader<bool, bool>);
impl CNTSTRT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CNTSTRT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CNTSTRT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CNTSTRT` writer - Launch SLP mode in analog block"]
pub struct CNTSTRT_W<'a> {
    w: &'a mut W,
}
impl<'a> CNTSTRT_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u32 & 0x01) << 3);
        self.w
    }
}
#[doc = "Field `WKUP_CNT` reader - Used to set a time to wake up the processor after the device has been put in a low power state"]
pub struct WKUP_CNT_R(crate::FieldReader<u8, u8>);
impl WKUP_CNT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        WKUP_CNT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WKUP_CNT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WKUP_CNT` writer - Used to set a time to wake up the processor after the device has been put in a low power state"]
pub struct WKUP_CNT_W<'a> {
    w: &'a mut W,
}
impl<'a> WKUP_CNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | (value as u32 & 0x07);
        self.w
    }
}
impl R {
    #[doc = "Bit 3 - Launch SLP mode in analog block"]
    #[inline(always)]
    pub fn cntstrt(&self) -> CNTSTRT_R {
        CNTSTRT_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bits 0:2 - Used to set a time to wake up the processor after the device has been put in a low power state"]
    #[inline(always)]
    pub fn wkup_cnt(&self) -> WKUP_CNT_R {
        WKUP_CNT_R::new((self.bits & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bit 3 - Launch SLP mode in analog block"]
    #[inline(always)]
    pub fn cntstrt(&mut self) -> CNTSTRT_W {
        CNTSTRT_W { w: self }
    }
    #[doc = "Bits 0:2 - Used to set a time to wake up the processor after the device has been put in a low power state"]
    #[inline(always)]
    pub fn wkup_cnt(&mut self) -> WKUP_CNT_W {
        WKUP_CNT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Wakeup Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wakeup_cnt](index.html) module"]
pub struct WAKEUP_CNT_SPEC;
impl crate::RegisterSpec for WAKEUP_CNT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [wakeup_cnt::R](R) reader structure"]
impl crate::Readable for WAKEUP_CNT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [wakeup_cnt::W](W) writer structure"]
impl crate::Writable for WAKEUP_CNT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets WAKEUP_CNT to value 0x07"]
impl crate::Resettable for WAKEUP_CNT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x07
    }
}
