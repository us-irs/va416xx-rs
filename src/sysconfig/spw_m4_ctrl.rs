#[doc = "Register `SPW_M4_CTRL` reader"]
pub struct R(crate::R<SPW_M4_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SPW_M4_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SPW_M4_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SPW_M4_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SPW_M4_CTRL` writer"]
pub struct W(crate::W<SPW_M4_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SPW_M4_CTRL_SPEC>;
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
impl From<crate::W<SPW_M4_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SPW_M4_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LREN` reader - Lockup reset enable"]
pub struct LREN_R(crate::FieldReader<bool, bool>);
impl LREN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LREN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LREN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LREN` writer - Lockup reset enable"]
pub struct LREN_W<'a> {
    w: &'a mut W,
}
impl<'a> LREN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | ((value as u32 & 0x01) << 17);
        self.w
    }
}
#[doc = "Field `SPW_PAD_EN` reader - SPW pad enable"]
pub struct SPW_PAD_EN_R(crate::FieldReader<bool, bool>);
impl SPW_PAD_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SPW_PAD_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SPW_PAD_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SPW_PAD_EN` writer - SPW pad enable"]
pub struct SPW_PAD_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SPW_PAD_EN_W<'a> {
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
#[doc = "Field `REG_WR_KEY` reader - Fuse-analog register writes enabled when key = 0xfeed"]
pub struct REG_WR_KEY_R(crate::FieldReader<u16, u16>);
impl REG_WR_KEY_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        REG_WR_KEY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for REG_WR_KEY_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `REG_WR_KEY` writer - Fuse-analog register writes enabled when key = 0xfeed"]
pub struct REG_WR_KEY_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_WR_KEY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bit 17 - Lockup reset enable"]
    #[inline(always)]
    pub fn lren(&self) -> LREN_R {
        LREN_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 16 - SPW pad enable"]
    #[inline(always)]
    pub fn spw_pad_en(&self) -> SPW_PAD_EN_R {
        SPW_PAD_EN_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bits 0:15 - Fuse-analog register writes enabled when key = 0xfeed"]
    #[inline(always)]
    pub fn reg_wr_key(&self) -> REG_WR_KEY_R {
        REG_WR_KEY_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bit 17 - Lockup reset enable"]
    #[inline(always)]
    pub fn lren(&mut self) -> LREN_W {
        LREN_W { w: self }
    }
    #[doc = "Bit 16 - SPW pad enable"]
    #[inline(always)]
    pub fn spw_pad_en(&mut self) -> SPW_PAD_EN_W {
        SPW_PAD_EN_W { w: self }
    }
    #[doc = "Bits 0:15 - Fuse-analog register writes enabled when key = 0xfeed"]
    #[inline(always)]
    pub fn reg_wr_key(&mut self) -> REG_WR_KEY_W {
        REG_WR_KEY_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SPW M4 control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spw_m4_ctrl](index.html) module"]
pub struct SPW_M4_CTRL_SPEC;
impl crate::RegisterSpec for SPW_M4_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [spw_m4_ctrl::R](R) reader structure"]
impl crate::Readable for SPW_M4_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [spw_m4_ctrl::W](W) writer structure"]
impl crate::Writable for SPW_M4_CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SPW_M4_CTRL to value 0x0003_0000"]
impl crate::Resettable for SPW_M4_CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0003_0000
    }
}
