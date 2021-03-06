#[doc = "Register `MAC_ADDR_L` reader"]
pub struct R(crate::R<MAC_ADDR_L_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MAC_ADDR_L_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MAC_ADDR_L_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MAC_ADDR_L_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MAC_ADDR_L` writer"]
pub struct W(crate::W<MAC_ADDR_L_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MAC_ADDR_L_SPEC>;
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
impl From<crate::W<MAC_ADDR_L_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MAC_ADDR_L_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADDRLO` reader - MAC Address0\\[31:0\\]"]
pub struct ADDRLO_R(crate::FieldReader<u32, u32>);
impl ADDRLO_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        ADDRLO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADDRLO_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:31 - MAC Address0\\[31:0\\]"]
    #[inline(always)]
    pub fn addrlo(&self) -> ADDRLO_R {
        ADDRLO_R::new(self.bits as u32)
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
#[doc = "Contains the Low 32-bits of the first MAC Address\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mac_addr_l](index.html) module"]
pub struct MAC_ADDR_L_SPEC;
impl crate::RegisterSpec for MAC_ADDR_L_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mac_addr_l::R](R) reader structure"]
impl crate::Readable for MAC_ADDR_L_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mac_addr_l::W](W) writer structure"]
impl crate::Writable for MAC_ADDR_L_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MAC_ADDR_L to value 0xffff_ffff"]
impl crate::Resettable for MAC_ADDR_L_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xffff_ffff
    }
}
