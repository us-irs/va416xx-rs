#[doc = "Register `MAC_ADDR_H` reader"]
pub struct R(crate::R<MAC_ADDR_H_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MAC_ADDR_H_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MAC_ADDR_H_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MAC_ADDR_H_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MAC_ADDR_H` writer"]
pub struct W(crate::W<MAC_ADDR_H_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MAC_ADDR_H_SPEC>;
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
impl From<crate::W<MAC_ADDR_H_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MAC_ADDR_H_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `AE` reader - Address Enable, This bit is always set to 1"]
pub struct AE_R(crate::FieldReader<bool, bool>);
impl AE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        AE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADDRHI` reader - MAC Address0\\[47:32\\]"]
pub struct ADDRHI_R(crate::FieldReader<u16, u16>);
impl ADDRHI_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        ADDRHI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADDRHI_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 31 - Address Enable, This bit is always set to 1"]
    #[inline(always)]
    pub fn ae(&self) -> AE_R {
        AE_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bits 0:15 - MAC Address0\\[47:32\\]"]
    #[inline(always)]
    pub fn addrhi(&self) -> ADDRHI_R {
        ADDRHI_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Contains the high 16-bits of the first MAC Address\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mac_addr_h](index.html) module"]
pub struct MAC_ADDR_H_SPEC;
impl crate::RegisterSpec for MAC_ADDR_H_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mac_addr_h::R](R) reader structure"]
impl crate::Readable for MAC_ADDR_H_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mac_addr_h::W](W) writer structure"]
impl crate::Writable for MAC_ADDR_H_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MAC_ADDR_H to value 0x8000_ffff"]
impl crate::Resettable for MAC_ADDR_H_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x8000_ffff
    }
}
