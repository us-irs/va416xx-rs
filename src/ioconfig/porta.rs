#[doc = "Register `PORTA[%s]` reader"]
pub struct R(crate::R<PORTA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PORTA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PORTA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PORTA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PORTA[%s]` writer"]
pub struct W(crate::W<PORTA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PORTA_SPEC>;
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
impl From<crate::W<PORTA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PORTA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Input Filter Selectoin\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum FLTTYPE_A {
    #[doc = "0: Synchronize to system clock"]
    SYNC = 0,
    #[doc = "1: Direct input, no synchronization"]
    DIRECT = 1,
    #[doc = "2: Require 2 samples to have the same value"]
    FILTER1 = 2,
    #[doc = "3: Require 3 samples to have the same value"]
    FILTER2 = 3,
    #[doc = "4: Require 4 samples to have the same value"]
    FILTER3 = 4,
    #[doc = "5: Require 5 samples to have the same value"]
    FILTER4 = 5,
}
impl From<FLTTYPE_A> for u8 {
    #[inline(always)]
    fn from(variant: FLTTYPE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `FLTTYPE` reader - Input Filter Selectoin"]
pub struct FLTTYPE_R(crate::FieldReader<u8, FLTTYPE_A>);
impl FLTTYPE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        FLTTYPE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<FLTTYPE_A> {
        match self.bits {
            0 => Some(FLTTYPE_A::SYNC),
            1 => Some(FLTTYPE_A::DIRECT),
            2 => Some(FLTTYPE_A::FILTER1),
            3 => Some(FLTTYPE_A::FILTER2),
            4 => Some(FLTTYPE_A::FILTER3),
            5 => Some(FLTTYPE_A::FILTER4),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `SYNC`"]
    #[inline(always)]
    pub fn is_sync(&self) -> bool {
        **self == FLTTYPE_A::SYNC
    }
    #[doc = "Checks if the value of the field is `DIRECT`"]
    #[inline(always)]
    pub fn is_direct(&self) -> bool {
        **self == FLTTYPE_A::DIRECT
    }
    #[doc = "Checks if the value of the field is `FILTER1`"]
    #[inline(always)]
    pub fn is_filter1(&self) -> bool {
        **self == FLTTYPE_A::FILTER1
    }
    #[doc = "Checks if the value of the field is `FILTER2`"]
    #[inline(always)]
    pub fn is_filter2(&self) -> bool {
        **self == FLTTYPE_A::FILTER2
    }
    #[doc = "Checks if the value of the field is `FILTER3`"]
    #[inline(always)]
    pub fn is_filter3(&self) -> bool {
        **self == FLTTYPE_A::FILTER3
    }
    #[doc = "Checks if the value of the field is `FILTER4`"]
    #[inline(always)]
    pub fn is_filter4(&self) -> bool {
        **self == FLTTYPE_A::FILTER4
    }
}
impl core::ops::Deref for FLTTYPE_R {
    type Target = crate::FieldReader<u8, FLTTYPE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FLTTYPE` writer - Input Filter Selectoin"]
pub struct FLTTYPE_W<'a> {
    w: &'a mut W,
}
impl<'a> FLTTYPE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FLTTYPE_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Synchronize to system clock"]
    #[inline(always)]
    pub fn sync(self) -> &'a mut W {
        self.variant(FLTTYPE_A::SYNC)
    }
    #[doc = "Direct input, no synchronization"]
    #[inline(always)]
    pub fn direct(self) -> &'a mut W {
        self.variant(FLTTYPE_A::DIRECT)
    }
    #[doc = "Require 2 samples to have the same value"]
    #[inline(always)]
    pub fn filter1(self) -> &'a mut W {
        self.variant(FLTTYPE_A::FILTER1)
    }
    #[doc = "Require 3 samples to have the same value"]
    #[inline(always)]
    pub fn filter2(self) -> &'a mut W {
        self.variant(FLTTYPE_A::FILTER2)
    }
    #[doc = "Require 4 samples to have the same value"]
    #[inline(always)]
    pub fn filter3(self) -> &'a mut W {
        self.variant(FLTTYPE_A::FILTER3)
    }
    #[doc = "Require 5 samples to have the same value"]
    #[inline(always)]
    pub fn filter4(self) -> &'a mut W {
        self.variant(FLTTYPE_A::FILTER4)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | (value as u32 & 0x07);
        self.w
    }
}
#[doc = "Field `FLTCLK` reader - Input Filter Clock Selection"]
pub struct FLTCLK_R(crate::FieldReader<u8, u8>);
impl FLTCLK_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        FLTCLK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FLTCLK_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FLTCLK` writer - Input Filter Clock Selection"]
pub struct FLTCLK_W<'a> {
    w: &'a mut W,
}
impl<'a> FLTCLK_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 3)) | ((value as u32 & 0x07) << 3);
        self.w
    }
}
#[doc = "Field `INVINP` reader - Input Invert Selection"]
pub struct INVINP_R(crate::FieldReader<bool, bool>);
impl INVINP_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        INVINP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INVINP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INVINP` writer - Input Invert Selection"]
pub struct INVINP_W<'a> {
    w: &'a mut W,
}
impl<'a> INVINP_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | ((value as u32 & 0x01) << 6);
        self.w
    }
}
#[doc = "Field `IEWO` reader - Input Enable While Output enabled"]
pub struct IEWO_R(crate::FieldReader<bool, bool>);
impl IEWO_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        IEWO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IEWO_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IEWO` writer - Input Enable While Output enabled"]
pub struct IEWO_W<'a> {
    w: &'a mut W,
}
impl<'a> IEWO_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | ((value as u32 & 0x01) << 7);
        self.w
    }
}
#[doc = "Field `OPENDRN` reader - Output Open Drain Mode"]
pub struct OPENDRN_R(crate::FieldReader<bool, bool>);
impl OPENDRN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        OPENDRN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OPENDRN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OPENDRN` writer - Output Open Drain Mode"]
pub struct OPENDRN_W<'a> {
    w: &'a mut W,
}
impl<'a> OPENDRN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | ((value as u32 & 0x01) << 8);
        self.w
    }
}
#[doc = "Field `INVOUT` reader - Output Invert Selection"]
pub struct INVOUT_R(crate::FieldReader<bool, bool>);
impl INVOUT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        INVOUT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INVOUT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INVOUT` writer - Output Invert Selection"]
pub struct INVOUT_W<'a> {
    w: &'a mut W,
}
impl<'a> INVOUT_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | ((value as u32 & 0x01) << 9);
        self.w
    }
}
#[doc = "Field `PLEVEL` reader - Internal Pull up/down level"]
pub struct PLEVEL_R(crate::FieldReader<bool, bool>);
impl PLEVEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PLEVEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PLEVEL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PLEVEL` writer - Internal Pull up/down level"]
pub struct PLEVEL_W<'a> {
    w: &'a mut W,
}
impl<'a> PLEVEL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | ((value as u32 & 0x01) << 10);
        self.w
    }
}
#[doc = "Field `PEN` reader - Enable Internal Pull up/down"]
pub struct PEN_R(crate::FieldReader<bool, bool>);
impl PEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PEN` writer - Enable Internal Pull up/down"]
pub struct PEN_W<'a> {
    w: &'a mut W,
}
impl<'a> PEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | ((value as u32 & 0x01) << 11);
        self.w
    }
}
#[doc = "Field `PWOA` reader - Enable Pull when output active"]
pub struct PWOA_R(crate::FieldReader<bool, bool>);
impl PWOA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PWOA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PWOA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PWOA` writer - Enable Pull when output active"]
pub struct PWOA_W<'a> {
    w: &'a mut W,
}
impl<'a> PWOA_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | ((value as u32 & 0x01) << 12);
        self.w
    }
}
#[doc = "Field `FUNSEL` reader - Pin Function Selection"]
pub struct FUNSEL_R(crate::FieldReader<u8, u8>);
impl FUNSEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        FUNSEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FUNSEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FUNSEL` writer - Pin Function Selection"]
pub struct FUNSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> FUNSEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 13)) | ((value as u32 & 0x07) << 13);
        self.w
    }
}
#[doc = "Field `IODIS` reader - IO Pin Disable"]
pub struct IODIS_R(crate::FieldReader<bool, bool>);
impl IODIS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        IODIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IODIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IODIS` writer - IO Pin Disable"]
pub struct IODIS_W<'a> {
    w: &'a mut W,
}
impl<'a> IODIS_W<'a> {
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
impl R {
    #[doc = "Bits 0:2 - Input Filter Selectoin"]
    #[inline(always)]
    pub fn flttype(&self) -> FLTTYPE_R {
        FLTTYPE_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bits 3:5 - Input Filter Clock Selection"]
    #[inline(always)]
    pub fn fltclk(&self) -> FLTCLK_R {
        FLTCLK_R::new(((self.bits >> 3) & 0x07) as u8)
    }
    #[doc = "Bit 6 - Input Invert Selection"]
    #[inline(always)]
    pub fn invinp(&self) -> INVINP_R {
        INVINP_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Input Enable While Output enabled"]
    #[inline(always)]
    pub fn iewo(&self) -> IEWO_R {
        IEWO_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Output Open Drain Mode"]
    #[inline(always)]
    pub fn opendrn(&self) -> OPENDRN_R {
        OPENDRN_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Output Invert Selection"]
    #[inline(always)]
    pub fn invout(&self) -> INVOUT_R {
        INVOUT_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Internal Pull up/down level"]
    #[inline(always)]
    pub fn plevel(&self) -> PLEVEL_R {
        PLEVEL_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Enable Internal Pull up/down"]
    #[inline(always)]
    pub fn pen(&self) -> PEN_R {
        PEN_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Enable Pull when output active"]
    #[inline(always)]
    pub fn pwoa(&self) -> PWOA_R {
        PWOA_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bits 13:15 - Pin Function Selection"]
    #[inline(always)]
    pub fn funsel(&self) -> FUNSEL_R {
        FUNSEL_R::new(((self.bits >> 13) & 0x07) as u8)
    }
    #[doc = "Bit 16 - IO Pin Disable"]
    #[inline(always)]
    pub fn iodis(&self) -> IODIS_R {
        IODIS_R::new(((self.bits >> 16) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Input Filter Selectoin"]
    #[inline(always)]
    pub fn flttype(&mut self) -> FLTTYPE_W {
        FLTTYPE_W { w: self }
    }
    #[doc = "Bits 3:5 - Input Filter Clock Selection"]
    #[inline(always)]
    pub fn fltclk(&mut self) -> FLTCLK_W {
        FLTCLK_W { w: self }
    }
    #[doc = "Bit 6 - Input Invert Selection"]
    #[inline(always)]
    pub fn invinp(&mut self) -> INVINP_W {
        INVINP_W { w: self }
    }
    #[doc = "Bit 7 - Input Enable While Output enabled"]
    #[inline(always)]
    pub fn iewo(&mut self) -> IEWO_W {
        IEWO_W { w: self }
    }
    #[doc = "Bit 8 - Output Open Drain Mode"]
    #[inline(always)]
    pub fn opendrn(&mut self) -> OPENDRN_W {
        OPENDRN_W { w: self }
    }
    #[doc = "Bit 9 - Output Invert Selection"]
    #[inline(always)]
    pub fn invout(&mut self) -> INVOUT_W {
        INVOUT_W { w: self }
    }
    #[doc = "Bit 10 - Internal Pull up/down level"]
    #[inline(always)]
    pub fn plevel(&mut self) -> PLEVEL_W {
        PLEVEL_W { w: self }
    }
    #[doc = "Bit 11 - Enable Internal Pull up/down"]
    #[inline(always)]
    pub fn pen(&mut self) -> PEN_W {
        PEN_W { w: self }
    }
    #[doc = "Bit 12 - Enable Pull when output active"]
    #[inline(always)]
    pub fn pwoa(&mut self) -> PWOA_W {
        PWOA_W { w: self }
    }
    #[doc = "Bits 13:15 - Pin Function Selection"]
    #[inline(always)]
    pub fn funsel(&mut self) -> FUNSEL_W {
        FUNSEL_W { w: self }
    }
    #[doc = "Bit 16 - IO Pin Disable"]
    #[inline(always)]
    pub fn iodis(&mut self) -> IODIS_W {
        IODIS_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PORTA Pin Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [porta](index.html) module"]
pub struct PORTA_SPEC;
impl crate::RegisterSpec for PORTA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [porta::R](R) reader structure"]
impl crate::Readable for PORTA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [porta::W](W) writer structure"]
impl crate::Writable for PORTA_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PORTA[%s]
to value 0"]
impl crate::Resettable for PORTA_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
