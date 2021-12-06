#[doc = "Register `MAC_ADDR_H` reader"]
pub type R = crate::R<MacAddrHSpec>;
#[doc = "Register `MAC_ADDR_H` writer"]
pub type W = crate::W<MacAddrHSpec>;
#[doc = "Field `ADDRHI` reader - MAC Address0\\[47:32\\]"]
pub type AddrhiR = crate::FieldReader<u16>;
#[doc = "Field `AE` reader - Address Enable, This bit is always set to 1"]
pub type AeR = crate::BitReader;
impl R {
    #[doc = "Bits 0:15 - MAC Address0\\[47:32\\]"]
    #[inline(always)]
    pub fn addrhi(&self) -> AddrhiR {
        AddrhiR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 31 - Address Enable, This bit is always set to 1"]
    #[inline(always)]
    pub fn ae(&self) -> AeR {
        AeR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {}
#[doc = "Contains the high 16-bits of the first MAC Address\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mac_addr_h::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mac_addr_h::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MacAddrHSpec;
impl crate::RegisterSpec for MacAddrHSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mac_addr_h::R`](R) reader structure"]
impl crate::Readable for MacAddrHSpec {}
#[doc = "`write(|w| ..)` method takes [`mac_addr_h::W`](W) writer structure"]
impl crate::Writable for MacAddrHSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MAC_ADDR_H to value 0x8000_ffff"]
impl crate::Resettable for MacAddrHSpec {
    const RESET_VALUE: u32 = 0x8000_ffff;
}
