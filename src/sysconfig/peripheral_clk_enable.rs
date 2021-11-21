#[doc = "Register `PERIPHERAL_CLK_ENABLE` reader"]
pub struct R(crate::R<PERIPHERAL_CLK_ENABLE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PERIPHERAL_CLK_ENABLE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PERIPHERAL_CLK_ENABLE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PERIPHERAL_CLK_ENABLE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PERIPHERAL_CLK_ENABLE` writer"]
pub struct W(crate::W<PERIPHERAL_CLK_ENABLE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PERIPHERAL_CLK_ENABLE_SPEC>;
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
impl From<crate::W<PERIPHERAL_CLK_ENABLE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PERIPHERAL_CLK_ENABLE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SPI0` reader - Resetn of SPI0"]
pub struct SPI0_R(crate::FieldReader<bool, bool>);
impl SPI0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SPI0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SPI0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SPI0` writer - Resetn of SPI0"]
pub struct SPI0_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI0_W<'a> {
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
#[doc = "Field `SPI1` reader - Resetn of SPI1"]
pub struct SPI1_R(crate::FieldReader<bool, bool>);
impl SPI1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SPI1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SPI1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SPI1` writer - Resetn of SPI1"]
pub struct SPI1_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI1_W<'a> {
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
#[doc = "Field `SPI2` reader - Resetn of SPI2"]
pub struct SPI2_R(crate::FieldReader<bool, bool>);
impl SPI2_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SPI2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SPI2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SPI2` writer - Resetn of SPI2"]
pub struct SPI2_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI2_W<'a> {
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
#[doc = "Field `SPI3` reader - Resetn of SPI3"]
pub struct SPI3_R(crate::FieldReader<bool, bool>);
impl SPI3_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SPI3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SPI3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SPI3` writer - Resetn of SPI3"]
pub struct SPI3_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI3_W<'a> {
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
#[doc = "Field `UART0` reader - Resetn of UART0"]
pub struct UART0_R(crate::FieldReader<bool, bool>);
impl UART0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        UART0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UART0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UART0` writer - Resetn of UART0"]
pub struct UART0_W<'a> {
    w: &'a mut W,
}
impl<'a> UART0_W<'a> {
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
#[doc = "Field `UART1` reader - Resetn of UART1"]
pub struct UART1_R(crate::FieldReader<bool, bool>);
impl UART1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        UART1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UART1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UART1` writer - Resetn of UART1"]
pub struct UART1_W<'a> {
    w: &'a mut W,
}
impl<'a> UART1_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u32 & 0x01) << 5);
        self.w
    }
}
#[doc = "Field `UART2` reader - Resetn of UART2"]
pub struct UART2_R(crate::FieldReader<bool, bool>);
impl UART2_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        UART2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UART2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UART2` writer - Resetn of UART2"]
pub struct UART2_W<'a> {
    w: &'a mut W,
}
impl<'a> UART2_W<'a> {
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
#[doc = "Field `I2C0` reader - Resetn of I2C0"]
pub struct I2C0_R(crate::FieldReader<bool, bool>);
impl I2C0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        I2C0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for I2C0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `I2C0` writer - Resetn of I2C0"]
pub struct I2C0_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C0_W<'a> {
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
#[doc = "Field `I2C1` reader - Resetn of I2C1"]
pub struct I2C1_R(crate::FieldReader<bool, bool>);
impl I2C1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        I2C1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for I2C1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `I2C1` writer - Resetn of I2C1"]
pub struct I2C1_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C1_W<'a> {
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
#[doc = "Field `I2C2` reader - Resetn of I2C2"]
pub struct I2C2_R(crate::FieldReader<bool, bool>);
impl I2C2_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        I2C2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for I2C2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `I2C2` writer - Resetn of I2C2"]
pub struct I2C2_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C2_W<'a> {
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
#[doc = "Field `CAN0` reader - Resetn of CAN0"]
pub struct CAN0_R(crate::FieldReader<bool, bool>);
impl CAN0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CAN0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CAN0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CAN0` writer - Resetn of CAN0"]
pub struct CAN0_W<'a> {
    w: &'a mut W,
}
impl<'a> CAN0_W<'a> {
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
#[doc = "Field `CAN1` reader - Resetn of CAN1"]
pub struct CAN1_R(crate::FieldReader<bool, bool>);
impl CAN1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CAN1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CAN1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CAN1` writer - Resetn of CAN1"]
pub struct CAN1_W<'a> {
    w: &'a mut W,
}
impl<'a> CAN1_W<'a> {
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
#[doc = "Field `TRNG` reader - Resetn of TRNG"]
pub struct TRNG_R(crate::FieldReader<bool, bool>);
impl TRNG_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TRNG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TRNG_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TRNG` writer - Resetn of TRNG"]
pub struct TRNG_W<'a> {
    w: &'a mut W,
}
impl<'a> TRNG_W<'a> {
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
#[doc = "Field `ADC` reader - Resetn of ADC"]
pub struct ADC_R(crate::FieldReader<bool, bool>);
impl ADC_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ADC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADC` writer - Resetn of ADC"]
pub struct ADC_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | ((value as u32 & 0x01) << 13);
        self.w
    }
}
#[doc = "Field `DAC` reader - Resetn of DAC"]
pub struct DAC_R(crate::FieldReader<bool, bool>);
impl DAC_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DAC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DAC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DAC` writer - Resetn of DAC"]
pub struct DAC_W<'a> {
    w: &'a mut W,
}
impl<'a> DAC_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | ((value as u32 & 0x01) << 14);
        self.w
    }
}
#[doc = "Field `DMA` reader - Resetn of DMA"]
pub struct DMA_R(crate::FieldReader<bool, bool>);
impl DMA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DMA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DMA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DMA` writer - Resetn of DMA"]
pub struct DMA_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | ((value as u32 & 0x01) << 15);
        self.w
    }
}
#[doc = "Field `EBI` reader - Resetn of EBI"]
pub struct EBI_R(crate::FieldReader<bool, bool>);
impl EBI_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EBI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EBI_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EBI` writer - Resetn of EBI"]
pub struct EBI_W<'a> {
    w: &'a mut W,
}
impl<'a> EBI_W<'a> {
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
#[doc = "Field `ETH` reader - Resetn of Ethernet"]
pub struct ETH_R(crate::FieldReader<bool, bool>);
impl ETH_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ETH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ETH_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ETH` writer - Resetn of Ethernet"]
pub struct ETH_W<'a> {
    w: &'a mut W,
}
impl<'a> ETH_W<'a> {
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
#[doc = "Field `SPW` reader - Resetn of SpaceWire"]
pub struct SPW_R(crate::FieldReader<bool, bool>);
impl SPW_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SPW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SPW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SPW` writer - Resetn of SpaceWire"]
pub struct SPW_W<'a> {
    w: &'a mut W,
}
impl<'a> SPW_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | ((value as u32 & 0x01) << 18);
        self.w
    }
}
#[doc = "Field `CLKGEN` reader - RESETn of PLL in Clock Generation Module"]
pub struct CLKGEN_R(crate::FieldReader<bool, bool>);
impl CLKGEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CLKGEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CLKGEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CLKGEN` writer - RESETn of PLL in Clock Generation Module"]
pub struct CLKGEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CLKGEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | ((value as u32 & 0x01) << 19);
        self.w
    }
}
#[doc = "Field `IRQ` reader - Resetn of IRQ Router"]
pub struct IRQ_R(crate::FieldReader<bool, bool>);
impl IRQ_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        IRQ_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IRQ_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IRQ` writer - Resetn of IRQ Router"]
pub struct IRQ_W<'a> {
    w: &'a mut W,
}
impl<'a> IRQ_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | ((value as u32 & 0x01) << 20);
        self.w
    }
}
#[doc = "Field `IOCONFIG` reader - Resetn of IO CONFIG"]
pub struct IOCONFIG_R(crate::FieldReader<bool, bool>);
impl IOCONFIG_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        IOCONFIG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IOCONFIG_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IOCONFIG` writer - Resetn of IO CONFIG"]
pub struct IOCONFIG_W<'a> {
    w: &'a mut W,
}
impl<'a> IOCONFIG_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | ((value as u32 & 0x01) << 21);
        self.w
    }
}
#[doc = "Field `UTILITY` reader - Resetn of UTILITY peripheral"]
pub struct UTILITY_R(crate::FieldReader<bool, bool>);
impl UTILITY_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        UTILITY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UTILITY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UTILITY` writer - Resetn of UTILITY peripheral"]
pub struct UTILITY_W<'a> {
    w: &'a mut W,
}
impl<'a> UTILITY_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | ((value as u32 & 0x01) << 22);
        self.w
    }
}
#[doc = "Field `WDOG` reader - Resetn of WDOG"]
pub struct WDOG_R(crate::FieldReader<bool, bool>);
impl WDOG_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        WDOG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WDOG_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WDOG` writer - Resetn of WDOG"]
pub struct WDOG_W<'a> {
    w: &'a mut W,
}
impl<'a> WDOG_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | ((value as u32 & 0x01) << 23);
        self.w
    }
}
#[doc = "Field `PORTA` reader - Resetn of PORTA"]
pub struct PORTA_R(crate::FieldReader<bool, bool>);
impl PORTA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PORTA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PORTA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PORTA` writer - Resetn of PORTA"]
pub struct PORTA_W<'a> {
    w: &'a mut W,
}
impl<'a> PORTA_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | ((value as u32 & 0x01) << 24);
        self.w
    }
}
#[doc = "Field `PORTB` reader - Resetn of PORTB"]
pub struct PORTB_R(crate::FieldReader<bool, bool>);
impl PORTB_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PORTB_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PORTB_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PORTB` writer - Resetn of PORTB"]
pub struct PORTB_W<'a> {
    w: &'a mut W,
}
impl<'a> PORTB_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | ((value as u32 & 0x01) << 25);
        self.w
    }
}
#[doc = "Field `PORTC` reader - Resetn of PORTC"]
pub struct PORTC_R(crate::FieldReader<bool, bool>);
impl PORTC_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PORTC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PORTC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PORTC` writer - Resetn of PORTC"]
pub struct PORTC_W<'a> {
    w: &'a mut W,
}
impl<'a> PORTC_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 26)) | ((value as u32 & 0x01) << 26);
        self.w
    }
}
#[doc = "Field `PORTD` reader - Resetn of PORTD"]
pub struct PORTD_R(crate::FieldReader<bool, bool>);
impl PORTD_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PORTD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PORTD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PORTD` writer - Resetn of PORTD"]
pub struct PORTD_W<'a> {
    w: &'a mut W,
}
impl<'a> PORTD_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 27)) | ((value as u32 & 0x01) << 27);
        self.w
    }
}
#[doc = "Field `PORTE` reader - Resetn of PORTE"]
pub struct PORTE_R(crate::FieldReader<bool, bool>);
impl PORTE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PORTE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PORTE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PORTE` writer - Resetn of PORTE"]
pub struct PORTE_W<'a> {
    w: &'a mut W,
}
impl<'a> PORTE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 28)) | ((value as u32 & 0x01) << 28);
        self.w
    }
}
#[doc = "Field `PORTF` reader - Resetn of PORTF"]
pub struct PORTF_R(crate::FieldReader<bool, bool>);
impl PORTF_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PORTF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PORTF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PORTF` writer - Resetn of PORTF"]
pub struct PORTF_W<'a> {
    w: &'a mut W,
}
impl<'a> PORTF_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 29)) | ((value as u32 & 0x01) << 29);
        self.w
    }
}
#[doc = "Field `PORTG` reader - Resetn of PORTG"]
pub struct PORTG_R(crate::FieldReader<bool, bool>);
impl PORTG_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PORTG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PORTG_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PORTG` writer - Resetn of PORTG"]
pub struct PORTG_W<'a> {
    w: &'a mut W,
}
impl<'a> PORTG_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | ((value as u32 & 0x01) << 30);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Resetn of SPI0"]
    #[inline(always)]
    pub fn spi0(&self) -> SPI0_R {
        SPI0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Resetn of SPI1"]
    #[inline(always)]
    pub fn spi1(&self) -> SPI1_R {
        SPI1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Resetn of SPI2"]
    #[inline(always)]
    pub fn spi2(&self) -> SPI2_R {
        SPI2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Resetn of SPI3"]
    #[inline(always)]
    pub fn spi3(&self) -> SPI3_R {
        SPI3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Resetn of UART0"]
    #[inline(always)]
    pub fn uart0(&self) -> UART0_R {
        UART0_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Resetn of UART1"]
    #[inline(always)]
    pub fn uart1(&self) -> UART1_R {
        UART1_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Resetn of UART2"]
    #[inline(always)]
    pub fn uart2(&self) -> UART2_R {
        UART2_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Resetn of I2C0"]
    #[inline(always)]
    pub fn i2c0(&self) -> I2C0_R {
        I2C0_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Resetn of I2C1"]
    #[inline(always)]
    pub fn i2c1(&self) -> I2C1_R {
        I2C1_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Resetn of I2C2"]
    #[inline(always)]
    pub fn i2c2(&self) -> I2C2_R {
        I2C2_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Resetn of CAN0"]
    #[inline(always)]
    pub fn can0(&self) -> CAN0_R {
        CAN0_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Resetn of CAN1"]
    #[inline(always)]
    pub fn can1(&self) -> CAN1_R {
        CAN1_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Resetn of TRNG"]
    #[inline(always)]
    pub fn trng(&self) -> TRNG_R {
        TRNG_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Resetn of ADC"]
    #[inline(always)]
    pub fn adc(&self) -> ADC_R {
        ADC_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Resetn of DAC"]
    #[inline(always)]
    pub fn dac(&self) -> DAC_R {
        DAC_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Resetn of DMA"]
    #[inline(always)]
    pub fn dma(&self) -> DMA_R {
        DMA_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Resetn of EBI"]
    #[inline(always)]
    pub fn ebi(&self) -> EBI_R {
        EBI_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Resetn of Ethernet"]
    #[inline(always)]
    pub fn eth(&self) -> ETH_R {
        ETH_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Resetn of SpaceWire"]
    #[inline(always)]
    pub fn spw(&self) -> SPW_R {
        SPW_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - RESETn of PLL in Clock Generation Module"]
    #[inline(always)]
    pub fn clkgen(&self) -> CLKGEN_R {
        CLKGEN_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Resetn of IRQ Router"]
    #[inline(always)]
    pub fn irq(&self) -> IRQ_R {
        IRQ_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Resetn of IO CONFIG"]
    #[inline(always)]
    pub fn ioconfig(&self) -> IOCONFIG_R {
        IOCONFIG_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Resetn of UTILITY peripheral"]
    #[inline(always)]
    pub fn utility(&self) -> UTILITY_R {
        UTILITY_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Resetn of WDOG"]
    #[inline(always)]
    pub fn wdog(&self) -> WDOG_R {
        WDOG_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Resetn of PORTA"]
    #[inline(always)]
    pub fn porta(&self) -> PORTA_R {
        PORTA_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Resetn of PORTB"]
    #[inline(always)]
    pub fn portb(&self) -> PORTB_R {
        PORTB_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Resetn of PORTC"]
    #[inline(always)]
    pub fn portc(&self) -> PORTC_R {
        PORTC_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - Resetn of PORTD"]
    #[inline(always)]
    pub fn portd(&self) -> PORTD_R {
        PORTD_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - Resetn of PORTE"]
    #[inline(always)]
    pub fn porte(&self) -> PORTE_R {
        PORTE_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - Resetn of PORTF"]
    #[inline(always)]
    pub fn portf(&self) -> PORTF_R {
        PORTF_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - Resetn of PORTG"]
    #[inline(always)]
    pub fn portg(&self) -> PORTG_R {
        PORTG_R::new(((self.bits >> 30) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Resetn of SPI0"]
    #[inline(always)]
    pub fn spi0(&mut self) -> SPI0_W {
        SPI0_W { w: self }
    }
    #[doc = "Bit 1 - Resetn of SPI1"]
    #[inline(always)]
    pub fn spi1(&mut self) -> SPI1_W {
        SPI1_W { w: self }
    }
    #[doc = "Bit 2 - Resetn of SPI2"]
    #[inline(always)]
    pub fn spi2(&mut self) -> SPI2_W {
        SPI2_W { w: self }
    }
    #[doc = "Bit 3 - Resetn of SPI3"]
    #[inline(always)]
    pub fn spi3(&mut self) -> SPI3_W {
        SPI3_W { w: self }
    }
    #[doc = "Bit 4 - Resetn of UART0"]
    #[inline(always)]
    pub fn uart0(&mut self) -> UART0_W {
        UART0_W { w: self }
    }
    #[doc = "Bit 5 - Resetn of UART1"]
    #[inline(always)]
    pub fn uart1(&mut self) -> UART1_W {
        UART1_W { w: self }
    }
    #[doc = "Bit 6 - Resetn of UART2"]
    #[inline(always)]
    pub fn uart2(&mut self) -> UART2_W {
        UART2_W { w: self }
    }
    #[doc = "Bit 7 - Resetn of I2C0"]
    #[inline(always)]
    pub fn i2c0(&mut self) -> I2C0_W {
        I2C0_W { w: self }
    }
    #[doc = "Bit 8 - Resetn of I2C1"]
    #[inline(always)]
    pub fn i2c1(&mut self) -> I2C1_W {
        I2C1_W { w: self }
    }
    #[doc = "Bit 9 - Resetn of I2C2"]
    #[inline(always)]
    pub fn i2c2(&mut self) -> I2C2_W {
        I2C2_W { w: self }
    }
    #[doc = "Bit 10 - Resetn of CAN0"]
    #[inline(always)]
    pub fn can0(&mut self) -> CAN0_W {
        CAN0_W { w: self }
    }
    #[doc = "Bit 11 - Resetn of CAN1"]
    #[inline(always)]
    pub fn can1(&mut self) -> CAN1_W {
        CAN1_W { w: self }
    }
    #[doc = "Bit 12 - Resetn of TRNG"]
    #[inline(always)]
    pub fn trng(&mut self) -> TRNG_W {
        TRNG_W { w: self }
    }
    #[doc = "Bit 13 - Resetn of ADC"]
    #[inline(always)]
    pub fn adc(&mut self) -> ADC_W {
        ADC_W { w: self }
    }
    #[doc = "Bit 14 - Resetn of DAC"]
    #[inline(always)]
    pub fn dac(&mut self) -> DAC_W {
        DAC_W { w: self }
    }
    #[doc = "Bit 15 - Resetn of DMA"]
    #[inline(always)]
    pub fn dma(&mut self) -> DMA_W {
        DMA_W { w: self }
    }
    #[doc = "Bit 16 - Resetn of EBI"]
    #[inline(always)]
    pub fn ebi(&mut self) -> EBI_W {
        EBI_W { w: self }
    }
    #[doc = "Bit 17 - Resetn of Ethernet"]
    #[inline(always)]
    pub fn eth(&mut self) -> ETH_W {
        ETH_W { w: self }
    }
    #[doc = "Bit 18 - Resetn of SpaceWire"]
    #[inline(always)]
    pub fn spw(&mut self) -> SPW_W {
        SPW_W { w: self }
    }
    #[doc = "Bit 19 - RESETn of PLL in Clock Generation Module"]
    #[inline(always)]
    pub fn clkgen(&mut self) -> CLKGEN_W {
        CLKGEN_W { w: self }
    }
    #[doc = "Bit 20 - Resetn of IRQ Router"]
    #[inline(always)]
    pub fn irq(&mut self) -> IRQ_W {
        IRQ_W { w: self }
    }
    #[doc = "Bit 21 - Resetn of IO CONFIG"]
    #[inline(always)]
    pub fn ioconfig(&mut self) -> IOCONFIG_W {
        IOCONFIG_W { w: self }
    }
    #[doc = "Bit 22 - Resetn of UTILITY peripheral"]
    #[inline(always)]
    pub fn utility(&mut self) -> UTILITY_W {
        UTILITY_W { w: self }
    }
    #[doc = "Bit 23 - Resetn of WDOG"]
    #[inline(always)]
    pub fn wdog(&mut self) -> WDOG_W {
        WDOG_W { w: self }
    }
    #[doc = "Bit 24 - Resetn of PORTA"]
    #[inline(always)]
    pub fn porta(&mut self) -> PORTA_W {
        PORTA_W { w: self }
    }
    #[doc = "Bit 25 - Resetn of PORTB"]
    #[inline(always)]
    pub fn portb(&mut self) -> PORTB_W {
        PORTB_W { w: self }
    }
    #[doc = "Bit 26 - Resetn of PORTC"]
    #[inline(always)]
    pub fn portc(&mut self) -> PORTC_W {
        PORTC_W { w: self }
    }
    #[doc = "Bit 27 - Resetn of PORTD"]
    #[inline(always)]
    pub fn portd(&mut self) -> PORTD_W {
        PORTD_W { w: self }
    }
    #[doc = "Bit 28 - Resetn of PORTE"]
    #[inline(always)]
    pub fn porte(&mut self) -> PORTE_W {
        PORTE_W { w: self }
    }
    #[doc = "Bit 29 - Resetn of PORTF"]
    #[inline(always)]
    pub fn portf(&mut self) -> PORTF_W {
        PORTF_W { w: self }
    }
    #[doc = "Bit 30 - Resetn of PORTG"]
    #[inline(always)]
    pub fn portg(&mut self) -> PORTG_W {
        PORTG_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Peripheral Enable Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [peripheral_clk_enable](index.html) module"]
pub struct PERIPHERAL_CLK_ENABLE_SPEC;
impl crate::RegisterSpec for PERIPHERAL_CLK_ENABLE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [peripheral_clk_enable::R](R) reader structure"]
impl crate::Readable for PERIPHERAL_CLK_ENABLE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [peripheral_clk_enable::W](W) writer structure"]
impl crate::Writable for PERIPHERAL_CLK_ENABLE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PERIPHERAL_CLK_ENABLE to value 0x0088_0000"]
impl crate::Resettable for PERIPHERAL_CLK_ENABLE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0088_0000
    }
}
