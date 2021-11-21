#[doc = "Register `MAC_DEBUG` reader"]
pub struct R(crate::R<MAC_DEBUG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MAC_DEBUG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MAC_DEBUG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MAC_DEBUG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `TXSTSFSTS` reader - MTL TxStatus FIFO Full Status"]
pub struct TXSTSFSTS_R(crate::FieldReader<bool, bool>);
impl TXSTSFSTS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TXSTSFSTS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXSTSFSTS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXFSTS` reader - MTL Tx FIFO Not Empty Status"]
pub struct TXFSTS_R(crate::FieldReader<bool, bool>);
impl TXFSTS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TXFSTS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXFSTS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TWCSTS` reader - MTL Tx FIFO Write Controller Status"]
pub struct TWCSTS_R(crate::FieldReader<bool, bool>);
impl TWCSTS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TWCSTS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TWCSTS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TRCSTS` reader - MTL Tx FIFO Read Controller Status"]
pub struct TRCSTS_R(crate::FieldReader<u8, u8>);
impl TRCSTS_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        TRCSTS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TRCSTS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXPAUSED` reader - MAC Transmitter in Pause"]
pub struct TXPAUSED_R(crate::FieldReader<bool, bool>);
impl TXPAUSED_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TXPAUSED_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXPAUSED_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TFCSTS` reader - PAC Transmit Frame Controller Status"]
pub struct TFCSTS_R(crate::FieldReader<u8, u8>);
impl TFCSTS_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        TFCSTS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TFCSTS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TPESTS` reader - MAC GMII or MII Transmit Protocol Engine Status"]
pub struct TPESTS_R(crate::FieldReader<bool, bool>);
impl TPESTS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TPESTS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TPESTS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXFSTS` reader - MTL RxFIFO Fill-Level Status"]
pub struct RXFSTS_R(crate::FieldReader<u8, u8>);
impl RXFSTS_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        RXFSTS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXFSTS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RRCSTS` reader - MTL RxFIFO Read Controller State"]
pub struct RRCSTS_R(crate::FieldReader<u8, u8>);
impl RRCSTS_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        RRCSTS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RRCSTS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RWCSTS` reader - MTL Rx FIFO Write Controller Active Status"]
pub struct RWCSTS_R(crate::FieldReader<bool, bool>);
impl RWCSTS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RWCSTS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RWCSTS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RFCFCSTS` reader - MAC Receive Frame FIFO Controller Status"]
pub struct RFCFCSTS_R(crate::FieldReader<u8, u8>);
impl RFCFCSTS_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        RFCFCSTS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RFCFCSTS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RPESTS` reader - MAC GMII or MII Receive Protocol Engine Status"]
pub struct RPESTS_R(crate::FieldReader<bool, bool>);
impl RPESTS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RPESTS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RPESTS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 25 - MTL TxStatus FIFO Full Status"]
    #[inline(always)]
    pub fn txstsfsts(&self) -> TXSTSFSTS_R {
        TXSTSFSTS_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 24 - MTL Tx FIFO Not Empty Status"]
    #[inline(always)]
    pub fn txfsts(&self) -> TXFSTS_R {
        TXFSTS_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 22 - MTL Tx FIFO Write Controller Status"]
    #[inline(always)]
    pub fn twcsts(&self) -> TWCSTS_R {
        TWCSTS_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bits 20:21 - MTL Tx FIFO Read Controller Status"]
    #[inline(always)]
    pub fn trcsts(&self) -> TRCSTS_R {
        TRCSTS_R::new(((self.bits >> 20) & 0x03) as u8)
    }
    #[doc = "Bit 19 - MAC Transmitter in Pause"]
    #[inline(always)]
    pub fn txpaused(&self) -> TXPAUSED_R {
        TXPAUSED_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bits 17:18 - PAC Transmit Frame Controller Status"]
    #[inline(always)]
    pub fn tfcsts(&self) -> TFCSTS_R {
        TFCSTS_R::new(((self.bits >> 17) & 0x03) as u8)
    }
    #[doc = "Bit 16 - MAC GMII or MII Transmit Protocol Engine Status"]
    #[inline(always)]
    pub fn tpests(&self) -> TPESTS_R {
        TPESTS_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bits 8:9 - MTL RxFIFO Fill-Level Status"]
    #[inline(always)]
    pub fn rxfsts(&self) -> RXFSTS_R {
        RXFSTS_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 5:6 - MTL RxFIFO Read Controller State"]
    #[inline(always)]
    pub fn rrcsts(&self) -> RRCSTS_R {
        RRCSTS_R::new(((self.bits >> 5) & 0x03) as u8)
    }
    #[doc = "Bit 4 - MTL Rx FIFO Write Controller Active Status"]
    #[inline(always)]
    pub fn rwcsts(&self) -> RWCSTS_R {
        RWCSTS_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bits 1:2 - MAC Receive Frame FIFO Controller Status"]
    #[inline(always)]
    pub fn rfcfcsts(&self) -> RFCFCSTS_R {
        RFCFCSTS_R::new(((self.bits >> 1) & 0x03) as u8)
    }
    #[doc = "Bit 0 - MAC GMII or MII Receive Protocol Engine Status"]
    #[inline(always)]
    pub fn rpests(&self) -> RPESTS_R {
        RPESTS_R::new((self.bits & 0x01) != 0)
    }
}
#[doc = "Gives the status of the various internal blocks for debugging\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mac_debug](index.html) module"]
pub struct MAC_DEBUG_SPEC;
impl crate::RegisterSpec for MAC_DEBUG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mac_debug::R](R) reader structure"]
impl crate::Readable for MAC_DEBUG_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets MAC_DEBUG to value 0"]
impl crate::Resettable for MAC_DEBUG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
