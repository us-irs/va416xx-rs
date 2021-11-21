#[doc = "Register `CSD_CTRL` reader"]
pub struct R(crate::R<CSD_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CSD_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CSD_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CSD_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CSD_CTRL` writer"]
pub struct W(crate::W<CSD_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CSD_CTRL_SPEC>;
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
impl From<crate::W<CSD_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CSD_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CSDEN0` reader - Cascade 0 Enable"]
pub struct CSDEN0_R(crate::FieldReader<bool, bool>);
impl CSDEN0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CSDEN0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CSDEN0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CSDEN0` writer - Cascade 0 Enable"]
pub struct CSDEN0_W<'a> {
    w: &'a mut W,
}
impl<'a> CSDEN0_W<'a> {
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
#[doc = "Field `CSDINV0` reader - Cascade 0 Invert"]
pub struct CSDINV0_R(crate::FieldReader<bool, bool>);
impl CSDINV0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CSDINV0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CSDINV0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CSDINV0` writer - Cascade 0 Invert"]
pub struct CSDINV0_W<'a> {
    w: &'a mut W,
}
impl<'a> CSDINV0_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
#[doc = "Field `CSDEN1` reader - Cascade 1 Enable"]
pub struct CSDEN1_R(crate::FieldReader<bool, bool>);
impl CSDEN1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CSDEN1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CSDEN1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CSDEN1` writer - Cascade 1 Enable"]
pub struct CSDEN1_W<'a> {
    w: &'a mut W,
}
impl<'a> CSDEN1_W<'a> {
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
#[doc = "Field `CSDINV1` reader - Cascade 1 Invert"]
pub struct CSDINV1_R(crate::FieldReader<bool, bool>);
impl CSDINV1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CSDINV1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CSDINV1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CSDINV1` writer - Cascade 1 Invert"]
pub struct CSDINV1_W<'a> {
    w: &'a mut W,
}
impl<'a> CSDINV1_W<'a> {
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
#[doc = "Field `DCASOP` reader - Dual Cascade Operation (0:AND, 1:OR)"]
pub struct DCASOP_R(crate::FieldReader<bool, bool>);
impl DCASOP_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DCASOP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DCASOP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DCASOP` writer - Dual Cascade Operation (0:AND, 1:OR)"]
pub struct DCASOP_W<'a> {
    w: &'a mut W,
}
impl<'a> DCASOP_W<'a> {
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
#[doc = "Field `CSDTRG0` reader - Cascade 0 Enabled as Trigger"]
pub struct CSDTRG0_R(crate::FieldReader<bool, bool>);
impl CSDTRG0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CSDTRG0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CSDTRG0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CSDTRG0` writer - Cascade 0 Enabled as Trigger"]
pub struct CSDTRG0_W<'a> {
    w: &'a mut W,
}
impl<'a> CSDTRG0_W<'a> {
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
#[doc = "Field `CSDTRG1` reader - Cascade 1 Enabled as Trigger"]
pub struct CSDTRG1_R(crate::FieldReader<bool, bool>);
impl CSDTRG1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CSDTRG1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CSDTRG1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CSDTRG1` writer - Cascade 1 Enabled as Trigger"]
pub struct CSDTRG1_W<'a> {
    w: &'a mut W,
}
impl<'a> CSDTRG1_W<'a> {
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
#[doc = "Field `CSDEN2` reader - Cascade 2 Enable"]
pub struct CSDEN2_R(crate::FieldReader<bool, bool>);
impl CSDEN2_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CSDEN2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CSDEN2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CSDEN2` writer - Cascade 2 Enable"]
pub struct CSDEN2_W<'a> {
    w: &'a mut W,
}
impl<'a> CSDEN2_W<'a> {
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
#[doc = "Field `CSDINV2` reader - Cascade 2 Invert"]
pub struct CSDINV2_R(crate::FieldReader<bool, bool>);
impl CSDINV2_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CSDINV2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CSDINV2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CSDINV2` writer - Cascade 2 Invert"]
pub struct CSDINV2_W<'a> {
    w: &'a mut W,
}
impl<'a> CSDINV2_W<'a> {
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
#[doc = "Field `CSDTRG2` reader - Cascade 2 Trigger mode"]
pub struct CSDTRG2_R(crate::FieldReader<bool, bool>);
impl CSDTRG2_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CSDTRG2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CSDTRG2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CSDTRG2` writer - Cascade 2 Trigger mode"]
pub struct CSDTRG2_W<'a> {
    w: &'a mut W,
}
impl<'a> CSDTRG2_W<'a> {
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
impl R {
    #[doc = "Bit 0 - Cascade 0 Enable"]
    #[inline(always)]
    pub fn csden0(&self) -> CSDEN0_R {
        CSDEN0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Cascade 0 Invert"]
    #[inline(always)]
    pub fn csdinv0(&self) -> CSDINV0_R {
        CSDINV0_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Cascade 1 Enable"]
    #[inline(always)]
    pub fn csden1(&self) -> CSDEN1_R {
        CSDEN1_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Cascade 1 Invert"]
    #[inline(always)]
    pub fn csdinv1(&self) -> CSDINV1_R {
        CSDINV1_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Dual Cascade Operation (0:AND, 1:OR)"]
    #[inline(always)]
    pub fn dcasop(&self) -> DCASOP_R {
        DCASOP_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Cascade 0 Enabled as Trigger"]
    #[inline(always)]
    pub fn csdtrg0(&self) -> CSDTRG0_R {
        CSDTRG0_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Cascade 1 Enabled as Trigger"]
    #[inline(always)]
    pub fn csdtrg1(&self) -> CSDTRG1_R {
        CSDTRG1_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Cascade 2 Enable"]
    #[inline(always)]
    pub fn csden2(&self) -> CSDEN2_R {
        CSDEN2_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Cascade 2 Invert"]
    #[inline(always)]
    pub fn csdinv2(&self) -> CSDINV2_R {
        CSDINV2_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Cascade 2 Trigger mode"]
    #[inline(always)]
    pub fn csdtrg2(&self) -> CSDTRG2_R {
        CSDTRG2_R::new(((self.bits >> 10) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Cascade 0 Enable"]
    #[inline(always)]
    pub fn csden0(&mut self) -> CSDEN0_W {
        CSDEN0_W { w: self }
    }
    #[doc = "Bit 1 - Cascade 0 Invert"]
    #[inline(always)]
    pub fn csdinv0(&mut self) -> CSDINV0_W {
        CSDINV0_W { w: self }
    }
    #[doc = "Bit 2 - Cascade 1 Enable"]
    #[inline(always)]
    pub fn csden1(&mut self) -> CSDEN1_W {
        CSDEN1_W { w: self }
    }
    #[doc = "Bit 3 - Cascade 1 Invert"]
    #[inline(always)]
    pub fn csdinv1(&mut self) -> CSDINV1_W {
        CSDINV1_W { w: self }
    }
    #[doc = "Bit 4 - Dual Cascade Operation (0:AND, 1:OR)"]
    #[inline(always)]
    pub fn dcasop(&mut self) -> DCASOP_W {
        DCASOP_W { w: self }
    }
    #[doc = "Bit 6 - Cascade 0 Enabled as Trigger"]
    #[inline(always)]
    pub fn csdtrg0(&mut self) -> CSDTRG0_W {
        CSDTRG0_W { w: self }
    }
    #[doc = "Bit 7 - Cascade 1 Enabled as Trigger"]
    #[inline(always)]
    pub fn csdtrg1(&mut self) -> CSDTRG1_W {
        CSDTRG1_W { w: self }
    }
    #[doc = "Bit 8 - Cascade 2 Enable"]
    #[inline(always)]
    pub fn csden2(&mut self) -> CSDEN2_W {
        CSDEN2_W { w: self }
    }
    #[doc = "Bit 9 - Cascade 2 Invert"]
    #[inline(always)]
    pub fn csdinv2(&mut self) -> CSDINV2_W {
        CSDINV2_W { w: self }
    }
    #[doc = "Bit 10 - Cascade 2 Trigger mode"]
    #[inline(always)]
    pub fn csdtrg2(&mut self) -> CSDTRG2_W {
        CSDTRG2_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "The Cascade Control Register. Controls the counter external enable signals\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csd_ctrl](index.html) module"]
pub struct CSD_CTRL_SPEC;
impl crate::RegisterSpec for CSD_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [csd_ctrl::R](R) reader structure"]
impl crate::Readable for CSD_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [csd_ctrl::W](W) writer structure"]
impl crate::Writable for CSD_CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CSD_CTRL to value 0"]
impl crate::Resettable for CSD_CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
