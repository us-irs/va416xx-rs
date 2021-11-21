#[doc = "Register `DMA_STATUS` reader"]
pub struct R(crate::R<DMA_STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMA_STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMA_STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMA_STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `TTI` reader - Timestamp Trigger Interrupt"]
pub struct TTI_R(crate::FieldReader<bool, bool>);
impl TTI_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TTI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TTI_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GMI` reader - GMAC MMC Interrupt"]
pub struct GMI_R(crate::FieldReader<bool, bool>);
impl GMI_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        GMI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GMI_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EB` reader - Error Bits"]
pub struct EB_R(crate::FieldReader<u8, u8>);
impl EB_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        EB_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EB_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TS` reader - Transmit Process State"]
pub struct TS_R(crate::FieldReader<u8, u8>);
impl TS_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        TS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RS` reader - Receive Process State"]
pub struct RS_R(crate::FieldReader<u8, u8>);
impl RS_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        RS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NIS` reader - Normal Interrupt Summary"]
pub struct NIS_R(crate::FieldReader<bool, bool>);
impl NIS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        NIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AIS` reader - Abnormal Interrupt Summary"]
pub struct AIS_R(crate::FieldReader<bool, bool>);
impl AIS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        AIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ERI` reader - Early Receive Interrupt"]
pub struct ERI_R(crate::FieldReader<bool, bool>);
impl ERI_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ERI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ERI_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FBI` reader - Fatal Bus Error Interruptble"]
pub struct FBI_R(crate::FieldReader<bool, bool>);
impl FBI_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FBI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FBI_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ETI` reader - Early Transmit Interrupt"]
pub struct ETI_R(crate::FieldReader<bool, bool>);
impl ETI_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ETI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ETI_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RWT` reader - Receive Watchdog Timeout"]
pub struct RWT_R(crate::FieldReader<bool, bool>);
impl RWT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RWT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RWT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RPS` reader - Receive Process Stopped"]
pub struct RPS_R(crate::FieldReader<bool, bool>);
impl RPS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RPS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RPS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RU` reader - Receive Buffer Unavailable"]
pub struct RU_R(crate::FieldReader<bool, bool>);
impl RU_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RU_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RU_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RI` reader - Receive Interrupt"]
pub struct RI_R(crate::FieldReader<bool, bool>);
impl RI_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RI_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UNF` reader - Transmit Underflow"]
pub struct UNF_R(crate::FieldReader<bool, bool>);
impl UNF_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        UNF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UNF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OVF` reader - Receive Underflow"]
pub struct OVF_R(crate::FieldReader<bool, bool>);
impl OVF_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        OVF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OVF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TJT` reader - Transmit Jabber Timeout"]
pub struct TJT_R(crate::FieldReader<bool, bool>);
impl TJT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TJT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TJT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TU` reader - Transmit Buffer Unavailable"]
pub struct TU_R(crate::FieldReader<bool, bool>);
impl TU_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TU_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TU_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TPS` reader - Transmit Process Stopped"]
pub struct TPS_R(crate::FieldReader<bool, bool>);
impl TPS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TPS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TPS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TI` reader - Transmit Interrupt"]
pub struct TI_R(crate::FieldReader<bool, bool>);
impl TI_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TI_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 29 - Timestamp Trigger Interrupt"]
    #[inline(always)]
    pub fn tti(&self) -> TTI_R {
        TTI_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 27 - GMAC MMC Interrupt"]
    #[inline(always)]
    pub fn gmi(&self) -> GMI_R {
        GMI_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bits 23:25 - Error Bits"]
    #[inline(always)]
    pub fn eb(&self) -> EB_R {
        EB_R::new(((self.bits >> 23) & 0x07) as u8)
    }
    #[doc = "Bits 20:22 - Transmit Process State"]
    #[inline(always)]
    pub fn ts(&self) -> TS_R {
        TS_R::new(((self.bits >> 20) & 0x07) as u8)
    }
    #[doc = "Bits 17:19 - Receive Process State"]
    #[inline(always)]
    pub fn rs(&self) -> RS_R {
        RS_R::new(((self.bits >> 17) & 0x07) as u8)
    }
    #[doc = "Bit 16 - Normal Interrupt Summary"]
    #[inline(always)]
    pub fn nis(&self) -> NIS_R {
        NIS_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Abnormal Interrupt Summary"]
    #[inline(always)]
    pub fn ais(&self) -> AIS_R {
        AIS_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Early Receive Interrupt"]
    #[inline(always)]
    pub fn eri(&self) -> ERI_R {
        ERI_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Fatal Bus Error Interruptble"]
    #[inline(always)]
    pub fn fbi(&self) -> FBI_R {
        FBI_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Early Transmit Interrupt"]
    #[inline(always)]
    pub fn eti(&self) -> ETI_R {
        ETI_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Receive Watchdog Timeout"]
    #[inline(always)]
    pub fn rwt(&self) -> RWT_R {
        RWT_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Receive Process Stopped"]
    #[inline(always)]
    pub fn rps(&self) -> RPS_R {
        RPS_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Receive Buffer Unavailable"]
    #[inline(always)]
    pub fn ru(&self) -> RU_R {
        RU_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Receive Interrupt"]
    #[inline(always)]
    pub fn ri(&self) -> RI_R {
        RI_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Transmit Underflow"]
    #[inline(always)]
    pub fn unf(&self) -> UNF_R {
        UNF_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Receive Underflow"]
    #[inline(always)]
    pub fn ovf(&self) -> OVF_R {
        OVF_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Transmit Jabber Timeout"]
    #[inline(always)]
    pub fn tjt(&self) -> TJT_R {
        TJT_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Transmit Buffer Unavailable"]
    #[inline(always)]
    pub fn tu(&self) -> TU_R {
        TU_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - Transmit Process Stopped"]
    #[inline(always)]
    pub fn tps(&self) -> TPS_R {
        TPS_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Transmit Interrupt"]
    #[inline(always)]
    pub fn ti(&self) -> TI_R {
        TI_R::new((self.bits & 0x01) != 0)
    }
}
#[doc = "Used to determine the status of the DMA\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_status](index.html) module"]
pub struct DMA_STATUS_SPEC;
impl crate::RegisterSpec for DMA_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dma_status::R](R) reader structure"]
impl crate::Readable for DMA_STATUS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DMA_STATUS to value 0"]
impl crate::Resettable for DMA_STATUS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
