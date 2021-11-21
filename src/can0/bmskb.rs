#[doc = "Register `BMSKB` reader"]
pub struct R(crate::R<BMSKB_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BMSKB_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BMSKB_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BMSKB_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BMSKB` writer"]
pub struct W(crate::W<BMSKB_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BMSKB_SPEC>;
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
impl From<crate::W<BMSKB_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BMSKB_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BM1` reader - BM\\[28:18\\]
- ID\\[10:0\\]
in standard, ID\\[28:18\\]
in extended"]
pub struct BM1_R(crate::FieldReader<u16, u16>);
impl BM1_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        BM1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BM1_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BM1` writer - BM\\[28:18\\]
- ID\\[10:0\\]
in standard, ID\\[28:18\\]
in extended"]
pub struct BM1_W<'a> {
    w: &'a mut W,
}
impl<'a> BM1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07ff << 5)) | ((value as u32 & 0x07ff) << 5);
        self.w
    }
}
#[doc = "Field `RTR` reader - Remote Transmission Request in Standard, Substitute Remote Request (SRR) in extended"]
pub struct RTR_R(crate::FieldReader<bool, bool>);
impl RTR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RTR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RTR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RTR` writer - Remote Transmission Request in Standard, Substitute Remote Request (SRR) in extended"]
pub struct RTR_W<'a> {
    w: &'a mut W,
}
impl<'a> RTR_W<'a> {
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
#[doc = "Field `IDE` reader - Identifier Extension Bit"]
pub struct IDE_R(crate::FieldReader<bool, bool>);
impl IDE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        IDE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IDE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IDE` writer - Identifier Extension Bit"]
pub struct IDE_W<'a> {
    w: &'a mut W,
}
impl<'a> IDE_W<'a> {
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
#[doc = "Field `BM0` reader - BM\\[17:15\\]
- Unused in standard, ID\\[17:15\\]
in extended"]
pub struct BM0_R(crate::FieldReader<u8, u8>);
impl BM0_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        BM0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BM0_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BM0` writer - BM\\[17:15\\]
- Unused in standard, ID\\[17:15\\]
in extended"]
pub struct BM0_W<'a> {
    w: &'a mut W,
}
impl<'a> BM0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | (value as u32 & 0x07);
        self.w
    }
}
impl R {
    #[doc = "Bits 5:15 - BM\\[28:18\\]
- ID\\[10:0\\]
in standard, ID\\[28:18\\]
in extended"]
    #[inline(always)]
    pub fn bm1(&self) -> BM1_R {
        BM1_R::new(((self.bits >> 5) & 0x07ff) as u16)
    }
    #[doc = "Bit 4 - Remote Transmission Request in Standard, Substitute Remote Request (SRR) in extended"]
    #[inline(always)]
    pub fn rtr(&self) -> RTR_R {
        RTR_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Identifier Extension Bit"]
    #[inline(always)]
    pub fn ide(&self) -> IDE_R {
        IDE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bits 0:2 - BM\\[17:15\\]
- Unused in standard, ID\\[17:15\\]
in extended"]
    #[inline(always)]
    pub fn bm0(&self) -> BM0_R {
        BM0_R::new((self.bits & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 5:15 - BM\\[28:18\\]
- ID\\[10:0\\]
in standard, ID\\[28:18\\]
in extended"]
    #[inline(always)]
    pub fn bm1(&mut self) -> BM1_W {
        BM1_W { w: self }
    }
    #[doc = "Bit 4 - Remote Transmission Request in Standard, Substitute Remote Request (SRR) in extended"]
    #[inline(always)]
    pub fn rtr(&mut self) -> RTR_W {
        RTR_W { w: self }
    }
    #[doc = "Bit 3 - Identifier Extension Bit"]
    #[inline(always)]
    pub fn ide(&mut self) -> IDE_W {
        IDE_W { w: self }
    }
    #[doc = "Bits 0:2 - BM\\[17:15\\]
- Unused in standard, ID\\[17:15\\]
in extended"]
    #[inline(always)]
    pub fn bm0(&mut self) -> BM0_W {
        BM0_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CAN Basic Mask Base\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bmskb](index.html) module"]
pub struct BMSKB_SPEC;
impl crate::RegisterSpec for BMSKB_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [bmskb::R](R) reader structure"]
impl crate::Readable for BMSKB_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [bmskb::W](W) writer structure"]
impl crate::Writable for BMSKB_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets BMSKB to value 0"]
impl crate::Resettable for BMSKB_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
