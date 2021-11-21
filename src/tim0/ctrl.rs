#[doc = "Register `CTRL` reader"]
pub struct R(crate::R<CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTRL` writer"]
pub struct W(crate::W<CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTRL_SPEC>;
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
impl From<crate::W<CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ENABLE` reader - Counter Enable"]
pub struct ENABLE_R(crate::FieldReader<bool, bool>);
impl ENABLE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ENABLE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ENABLE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ENABLE` writer - Counter Enable"]
pub struct ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> ENABLE_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
#[doc = "Field `ACTIVE` reader - Counter Active"]
pub struct ACTIVE_R(crate::FieldReader<bool, bool>);
impl ACTIVE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ACTIVE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ACTIVE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AUTO_DISABLE` reader - Auto Disables the counter (set ENABLE to 0) when the count reaches 0"]
pub struct AUTO_DISABLE_R(crate::FieldReader<bool, bool>);
impl AUTO_DISABLE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        AUTO_DISABLE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AUTO_DISABLE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AUTO_DISABLE` writer - Auto Disables the counter (set ENABLE to 0) when the count reaches 0"]
pub struct AUTO_DISABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> AUTO_DISABLE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
        self.w
    }
}
#[doc = "Field `AUTO_DEACTIVATE` reader - Auto Deactivate the counter (set ACTIVE to 0) when the count reaches 0"]
pub struct AUTO_DEACTIVATE_R(crate::FieldReader<bool, bool>);
impl AUTO_DEACTIVATE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        AUTO_DEACTIVATE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AUTO_DEACTIVATE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AUTO_DEACTIVATE` writer - Auto Deactivate the counter (set ACTIVE to 0) when the count reaches 0"]
pub struct AUTO_DEACTIVATE_W<'a> {
    w: &'a mut W,
}
impl<'a> AUTO_DEACTIVATE_W<'a> {
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
#[doc = "Field `IRQ_ENB` reader - Interrupt Enable"]
pub struct IRQ_ENB_R(crate::FieldReader<bool, bool>);
impl IRQ_ENB_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        IRQ_ENB_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IRQ_ENB_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IRQ_ENB` writer - Interrupt Enable"]
pub struct IRQ_ENB_W<'a> {
    w: &'a mut W,
}
impl<'a> IRQ_ENB_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u32 & 0x01) << 4);
        self.w
    }
}
#[doc = "Counter Status Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum STATUS_SEL_A {
    #[doc = "0: Single cycle pulse when the counter reaches 0"]
    DONE = 0,
    #[doc = "1: Returns the counter ACTIVE bit"]
    ACTIVE = 1,
    #[doc = "2: Toggles the STATUS bit everytime the counter reaches 0. Basically a divide by 2 counter output."]
    TOGGLE = 2,
    #[doc = "3: Selects the Pulse Width Modulated output. It 1 when the counter value is >= the PWMA_VALUE"]
    PWMA = 3,
    #[doc = "4: Selects the Pulse Width Modulated output. It 1 when the counter value is < the PWMA_VALUE and value is > PWMA_VALUE"]
    PWMB = 4,
    #[doc = "5: Returns the counter ENABLED bit"]
    ENABLED = 5,
    #[doc = "6: Selects the Pulse Width Modulated output. It 1 when the counter value is <= the PWMA_VALUE and value is >= 0"]
    PWMA_ACTIVE = 6,
}
impl From<STATUS_SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: STATUS_SEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `STATUS_SEL` reader - Counter Status Selection"]
pub struct STATUS_SEL_R(crate::FieldReader<u8, STATUS_SEL_A>);
impl STATUS_SEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        STATUS_SEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<STATUS_SEL_A> {
        match self.bits {
            0 => Some(STATUS_SEL_A::DONE),
            1 => Some(STATUS_SEL_A::ACTIVE),
            2 => Some(STATUS_SEL_A::TOGGLE),
            3 => Some(STATUS_SEL_A::PWMA),
            4 => Some(STATUS_SEL_A::PWMB),
            5 => Some(STATUS_SEL_A::ENABLED),
            6 => Some(STATUS_SEL_A::PWMA_ACTIVE),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `DONE`"]
    #[inline(always)]
    pub fn is_done(&self) -> bool {
        **self == STATUS_SEL_A::DONE
    }
    #[doc = "Checks if the value of the field is `ACTIVE`"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        **self == STATUS_SEL_A::ACTIVE
    }
    #[doc = "Checks if the value of the field is `TOGGLE`"]
    #[inline(always)]
    pub fn is_toggle(&self) -> bool {
        **self == STATUS_SEL_A::TOGGLE
    }
    #[doc = "Checks if the value of the field is `PWMA`"]
    #[inline(always)]
    pub fn is_pwma(&self) -> bool {
        **self == STATUS_SEL_A::PWMA
    }
    #[doc = "Checks if the value of the field is `PWMB`"]
    #[inline(always)]
    pub fn is_pwmb(&self) -> bool {
        **self == STATUS_SEL_A::PWMB
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == STATUS_SEL_A::ENABLED
    }
    #[doc = "Checks if the value of the field is `PWMA_ACTIVE`"]
    #[inline(always)]
    pub fn is_pwma_active(&self) -> bool {
        **self == STATUS_SEL_A::PWMA_ACTIVE
    }
}
impl core::ops::Deref for STATUS_SEL_R {
    type Target = crate::FieldReader<u8, STATUS_SEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `STATUS_SEL` writer - Counter Status Selection"]
pub struct STATUS_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> STATUS_SEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: STATUS_SEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Single cycle pulse when the counter reaches 0"]
    #[inline(always)]
    pub fn done(self) -> &'a mut W {
        self.variant(STATUS_SEL_A::DONE)
    }
    #[doc = "Returns the counter ACTIVE bit"]
    #[inline(always)]
    pub fn active(self) -> &'a mut W {
        self.variant(STATUS_SEL_A::ACTIVE)
    }
    #[doc = "Toggles the STATUS bit everytime the counter reaches 0. Basically a divide by 2 counter output."]
    #[inline(always)]
    pub fn toggle(self) -> &'a mut W {
        self.variant(STATUS_SEL_A::TOGGLE)
    }
    #[doc = "Selects the Pulse Width Modulated output. It 1 when the counter value is >= the PWMA_VALUE"]
    #[inline(always)]
    pub fn pwma(self) -> &'a mut W {
        self.variant(STATUS_SEL_A::PWMA)
    }
    #[doc = "Selects the Pulse Width Modulated output. It 1 when the counter value is < the PWMA_VALUE and value is > PWMA_VALUE"]
    #[inline(always)]
    pub fn pwmb(self) -> &'a mut W {
        self.variant(STATUS_SEL_A::PWMB)
    }
    #[doc = "Returns the counter ENABLED bit"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(STATUS_SEL_A::ENABLED)
    }
    #[doc = "Selects the Pulse Width Modulated output. It 1 when the counter value is <= the PWMA_VALUE and value is >= 0"]
    #[inline(always)]
    pub fn pwma_active(self) -> &'a mut W {
        self.variant(STATUS_SEL_A::PWMA_ACTIVE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 5)) | ((value as u32 & 0x07) << 5);
        self.w
    }
}
#[doc = "Field `STATUS_INV` reader - Invert the Output Status"]
pub struct STATUS_INV_R(crate::FieldReader<bool, bool>);
impl STATUS_INV_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        STATUS_INV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for STATUS_INV_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `STATUS_INV` writer - Invert the Output Status"]
pub struct STATUS_INV_W<'a> {
    w: &'a mut W,
}
impl<'a> STATUS_INV_W<'a> {
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
#[doc = "Field `REQ_STOP` reader - Stop Request"]
pub struct REQ_STOP_R(crate::FieldReader<bool, bool>);
impl REQ_STOP_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        REQ_STOP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for REQ_STOP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `REQ_STOP` writer - Stop Request"]
pub struct REQ_STOP_W<'a> {
    w: &'a mut W,
}
impl<'a> REQ_STOP_W<'a> {
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
impl R {
    #[doc = "Bit 0 - Counter Enable"]
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Counter Active"]
    #[inline(always)]
    pub fn active(&self) -> ACTIVE_R {
        ACTIVE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Auto Disables the counter (set ENABLE to 0) when the count reaches 0"]
    #[inline(always)]
    pub fn auto_disable(&self) -> AUTO_DISABLE_R {
        AUTO_DISABLE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Auto Deactivate the counter (set ACTIVE to 0) when the count reaches 0"]
    #[inline(always)]
    pub fn auto_deactivate(&self) -> AUTO_DEACTIVATE_R {
        AUTO_DEACTIVATE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Interrupt Enable"]
    #[inline(always)]
    pub fn irq_enb(&self) -> IRQ_ENB_R {
        IRQ_ENB_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bits 5:7 - Counter Status Selection"]
    #[inline(always)]
    pub fn status_sel(&self) -> STATUS_SEL_R {
        STATUS_SEL_R::new(((self.bits >> 5) & 0x07) as u8)
    }
    #[doc = "Bit 8 - Invert the Output Status"]
    #[inline(always)]
    pub fn status_inv(&self) -> STATUS_INV_R {
        STATUS_INV_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Stop Request"]
    #[inline(always)]
    pub fn req_stop(&self) -> REQ_STOP_R {
        REQ_STOP_R::new(((self.bits >> 9) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Counter Enable"]
    #[inline(always)]
    pub fn enable(&mut self) -> ENABLE_W {
        ENABLE_W { w: self }
    }
    #[doc = "Bit 2 - Auto Disables the counter (set ENABLE to 0) when the count reaches 0"]
    #[inline(always)]
    pub fn auto_disable(&mut self) -> AUTO_DISABLE_W {
        AUTO_DISABLE_W { w: self }
    }
    #[doc = "Bit 3 - Auto Deactivate the counter (set ACTIVE to 0) when the count reaches 0"]
    #[inline(always)]
    pub fn auto_deactivate(&mut self) -> AUTO_DEACTIVATE_W {
        AUTO_DEACTIVATE_W { w: self }
    }
    #[doc = "Bit 4 - Interrupt Enable"]
    #[inline(always)]
    pub fn irq_enb(&mut self) -> IRQ_ENB_W {
        IRQ_ENB_W { w: self }
    }
    #[doc = "Bits 5:7 - Counter Status Selection"]
    #[inline(always)]
    pub fn status_sel(&mut self) -> STATUS_SEL_W {
        STATUS_SEL_W { w: self }
    }
    #[doc = "Bit 8 - Invert the Output Status"]
    #[inline(always)]
    pub fn status_inv(&mut self) -> STATUS_INV_W {
        STATUS_INV_W { w: self }
    }
    #[doc = "Bit 9 - Stop Request"]
    #[inline(always)]
    pub fn req_stop(&mut self) -> REQ_STOP_W {
        REQ_STOP_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl](index.html) module"]
pub struct CTRL_SPEC;
impl crate::RegisterSpec for CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctrl::R](R) reader structure"]
impl crate::Readable for CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctrl::W](W) writer structure"]
impl crate::Writable for CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CTRL to value 0"]
impl crate::Resettable for CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
