#[doc = "Register `MAC_WDOG_TO` reader"]
pub struct R(crate::R<MAC_WDOG_TO_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MAC_WDOG_TO_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MAC_WDOG_TO_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MAC_WDOG_TO_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MAC_WDOG_TO` writer"]
pub struct W(crate::W<MAC_WDOG_TO_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MAC_WDOG_TO_SPEC>;
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
impl From<crate::W<MAC_WDOG_TO_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MAC_WDOG_TO_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PWE` reader - Programmable Watchdog Enable"]
pub struct PWE_R(crate::FieldReader<bool, bool>);
impl PWE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PWE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PWE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PWE` writer - Programmable Watchdog Enable"]
pub struct PWE_W<'a> {
    w: &'a mut W,
}
impl<'a> PWE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | ((value as u32 & 0x01) << 16);
        self.w
    }
}
#[doc = "Field `WTO` reader - Watchdog Timeout"]
pub struct WTO_R(crate::FieldReader<u16, u16>);
impl WTO_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        WTO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WTO_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WTO` writer - Watchdog Timeout"]
pub struct WTO_W<'a> {
    w: &'a mut W,
}
impl<'a> WTO_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3fff) | (value as u32 & 0x3fff);
        self.w
    }
}
impl R {
    #[doc = "Bit 16 - Programmable Watchdog Enable"]
    #[inline(always)]
    pub fn pwe(&self) -> PWE_R {
        PWE_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bits 0:13 - Watchdog Timeout"]
    #[inline(always)]
    pub fn wto(&self) -> WTO_R {
        WTO_R::new((self.bits & 0x3fff) as u16)
    }
}
impl W {
    #[doc = "Bit 16 - Programmable Watchdog Enable"]
    #[inline(always)]
    pub fn pwe(&mut self) -> PWE_W {
        PWE_W { w: self }
    }
    #[doc = "Bits 0:13 - Watchdog Timeout"]
    #[inline(always)]
    pub fn wto(&mut self) -> WTO_W {
        WTO_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Controls the watchdog time-out for received frames\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mac_wdog_to](index.html) module"]
pub struct MAC_WDOG_TO_SPEC;
impl crate::RegisterSpec for MAC_WDOG_TO_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mac_wdog_to::R](R) reader structure"]
impl crate::Readable for MAC_WDOG_TO_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mac_wdog_to::W](W) writer structure"]
impl crate::Writable for MAC_WDOG_TO_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MAC_WDOG_TO to value 0"]
impl crate::Resettable for MAC_WDOG_TO_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
