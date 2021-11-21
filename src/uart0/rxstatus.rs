#[doc = "Register `RXSTATUS` reader"]
pub struct R(crate::R<RXSTATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RXSTATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RXSTATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RXSTATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RDAVL` reader - Read Data Available"]
pub struct RDAVL_R(crate::FieldReader<bool, bool>);
impl RDAVL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RDAVL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RDAVL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RDNFULL` reader - Read Fifo NOT Full"]
pub struct RDNFULL_R(crate::FieldReader<bool, bool>);
impl RDNFULL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RDNFULL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RDNFULL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXBUSY` reader - RX Busy Receiving"]
pub struct RXBUSY_R(crate::FieldReader<bool, bool>);
impl RXBUSY_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RXBUSY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXBUSY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXTO` reader - RX Receive Timeout"]
pub struct RXTO_R(crate::FieldReader<bool, bool>);
impl RXTO_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RXTO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXTO_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXOVR` reader - Read Fifo Overflow"]
pub struct RXOVR_R(crate::FieldReader<bool, bool>);
impl RXOVR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RXOVR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXOVR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXFRM` reader - RX Framing Error"]
pub struct RXFRM_R(crate::FieldReader<bool, bool>);
impl RXFRM_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RXFRM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXFRM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXPAR` reader - RX Parity Error"]
pub struct RXPAR_R(crate::FieldReader<bool, bool>);
impl RXPAR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RXPAR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXPAR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXBRK` reader - RX Break Error"]
pub struct RXBRK_R(crate::FieldReader<bool, bool>);
impl RXBRK_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RXBRK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXBRK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXBUSYBRK` reader - RX Busy Receiving Break"]
pub struct RXBUSYBRK_R(crate::FieldReader<bool, bool>);
impl RXBUSYBRK_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RXBUSYBRK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXBUSYBRK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXADDR9` reader - Address Match for 9 bit mode"]
pub struct RXADDR9_R(crate::FieldReader<bool, bool>);
impl RXADDR9_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RXADDR9_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXADDR9_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXRTSN` reader - RX RTSn Output Value"]
pub struct RXRTSN_R(crate::FieldReader<bool, bool>);
impl RXRTSN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RXRTSN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXRTSN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - Read Data Available"]
    #[inline(always)]
    pub fn rdavl(&self) -> RDAVL_R {
        RDAVL_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Read Fifo NOT Full"]
    #[inline(always)]
    pub fn rdnfull(&self) -> RDNFULL_R {
        RDNFULL_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - RX Busy Receiving"]
    #[inline(always)]
    pub fn rxbusy(&self) -> RXBUSY_R {
        RXBUSY_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - RX Receive Timeout"]
    #[inline(always)]
    pub fn rxto(&self) -> RXTO_R {
        RXTO_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Read Fifo Overflow"]
    #[inline(always)]
    pub fn rxovr(&self) -> RXOVR_R {
        RXOVR_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - RX Framing Error"]
    #[inline(always)]
    pub fn rxfrm(&self) -> RXFRM_R {
        RXFRM_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - RX Parity Error"]
    #[inline(always)]
    pub fn rxpar(&self) -> RXPAR_R {
        RXPAR_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - RX Break Error"]
    #[inline(always)]
    pub fn rxbrk(&self) -> RXBRK_R {
        RXBRK_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - RX Busy Receiving Break"]
    #[inline(always)]
    pub fn rxbusybrk(&self) -> RXBUSYBRK_R {
        RXBUSYBRK_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Address Match for 9 bit mode"]
    #[inline(always)]
    pub fn rxaddr9(&self) -> RXADDR9_R {
        RXADDR9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 15 - RX RTSn Output Value"]
    #[inline(always)]
    pub fn rxrtsn(&self) -> RXRTSN_R {
        RXRTSN_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
#[doc = "Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxstatus](index.html) module"]
pub struct RXSTATUS_SPEC;
impl crate::RegisterSpec for RXSTATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rxstatus::R](R) reader structure"]
impl crate::Readable for RXSTATUS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RXSTATUS to value 0"]
impl crate::Resettable for RXSTATUS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
