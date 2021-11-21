#[doc = "Register `IRQ_RAW` reader"]
pub struct R(crate::R<IRQ_RAW_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IRQ_RAW_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IRQ_RAW_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IRQ_RAW_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `ROMMBE` reader - ROM Multi Bit Interrupt"]
pub struct ROMMBE_R(crate::FieldReader<bool, bool>);
impl ROMMBE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ROMMBE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ROMMBE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ROMSBE` reader - ROM Single Bit Interrupt"]
pub struct ROMSBE_R(crate::FieldReader<bool, bool>);
impl ROMSBE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ROMSBE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ROMSBE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RAM0MBE` reader - RAM0 Multi Bit Interrupt"]
pub struct RAM0MBE_R(crate::FieldReader<bool, bool>);
impl RAM0MBE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RAM0MBE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RAM0MBE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RAM0SBE` reader - RAM0 Single Bit Interrupt"]
pub struct RAM0SBE_R(crate::FieldReader<bool, bool>);
impl RAM0SBE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RAM0SBE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RAM0SBE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RAM1MBE` reader - RAM1 Multi Bit Interrupt"]
pub struct RAM1MBE_R(crate::FieldReader<bool, bool>);
impl RAM1MBE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RAM1MBE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RAM1MBE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RAM1SBE` reader - RAM1 Single Bit Interrupt"]
pub struct RAM1SBE_R(crate::FieldReader<bool, bool>);
impl RAM1SBE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RAM1SBE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RAM1SBE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - ROM Multi Bit Interrupt"]
    #[inline(always)]
    pub fn rommbe(&self) -> ROMMBE_R {
        ROMMBE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - ROM Single Bit Interrupt"]
    #[inline(always)]
    pub fn romsbe(&self) -> ROMSBE_R {
        ROMSBE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - RAM0 Multi Bit Interrupt"]
    #[inline(always)]
    pub fn ram0mbe(&self) -> RAM0MBE_R {
        RAM0MBE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - RAM0 Single Bit Interrupt"]
    #[inline(always)]
    pub fn ram0sbe(&self) -> RAM0SBE_R {
        RAM0SBE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - RAM1 Multi Bit Interrupt"]
    #[inline(always)]
    pub fn ram1mbe(&self) -> RAM1MBE_R {
        RAM1MBE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - RAM1 Single Bit Interrupt"]
    #[inline(always)]
    pub fn ram1sbe(&self) -> RAM1SBE_R {
        RAM1SBE_R::new(((self.bits >> 5) & 0x01) != 0)
    }
}
#[doc = "Raw EDAC Error Interrupt Status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [irq_raw](index.html) module"]
pub struct IRQ_RAW_SPEC;
impl crate::RegisterSpec for IRQ_RAW_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [irq_raw::R](R) reader structure"]
impl crate::Readable for IRQ_RAW_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets IRQ_RAW to value 0"]
impl crate::Resettable for IRQ_RAW_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
