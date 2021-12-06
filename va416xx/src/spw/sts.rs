#[doc = "Register `STS` reader"]
pub type R = crate::R<StsSpec>;
#[doc = "Register `STS` writer"]
pub type W = crate::W<StsSpec>;
#[doc = "Field `TO` reader - A new time count value was received"]
pub type ToR = crate::BitReader;
#[doc = "Field `TO` writer - A new time count value was received"]
pub type ToW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CE` reader - Credit has occurred"]
pub type CeR = crate::BitReader;
#[doc = "Field `CE` writer - Credit has occurred"]
pub type CeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ER` reader - Escape error has occurred"]
pub type ErR = crate::BitReader;
#[doc = "Field `ER` writer - Escape error has occurred"]
pub type ErW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DE` reader - Disconnection error has occurred"]
pub type DeR = crate::BitReader;
#[doc = "Field `DE` writer - Disconnection error has occurred"]
pub type DeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PE` reader - Parity error has occurred"]
pub type PeR = crate::BitReader;
#[doc = "Field `PE` writer - Parity error has occurred"]
pub type PeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WE` reader - A synchronization problem has occurred when receiving NChars"]
pub type WeR = crate::BitReader;
#[doc = "Field `WE` writer - A synchronization problem has occurred when receiving NChars"]
pub type WeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IA` reader - Packet is received with an invalid destination address field"]
pub type IaR = crate::BitReader;
#[doc = "Field `IA` writer - Packet is received with an invalid destination address field"]
pub type IaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EE` reader - Set to one when a packet is received with an EOP after the first byte for a non-RMAP packet and after the second byte for a RMAP packet"]
pub type EeR = crate::BitReader;
#[doc = "Field `EE` writer - Set to one when a packet is received with an EOP after the first byte for a non-RMAP packet and after the second byte for a RMAP packet"]
pub type EeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AP` reader - Active port"]
pub type ApR = crate::BitReader;
#[doc = "Field `AP` writer - Active port"]
pub type ApW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LS` reader - Link State"]
pub type LsR = crate::FieldReader;
#[doc = "Field `LS` writer - Link State"]
pub type LsW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `NTXD` reader - Number of Transmit Descriptors"]
pub type NtxdR = crate::FieldReader;
#[doc = "Field `NTXD` writer - Number of Transmit Descriptors"]
pub type NtxdW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `NRXD` reader - Number of Receive Descriptors"]
pub type NrxdR = crate::FieldReader;
#[doc = "Field `NRXD` writer - Number of Receive Descriptors"]
pub type NrxdW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bit 0 - A new time count value was received"]
    #[inline(always)]
    pub fn to(&self) -> ToR {
        ToR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Credit has occurred"]
    #[inline(always)]
    pub fn ce(&self) -> CeR {
        CeR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Escape error has occurred"]
    #[inline(always)]
    pub fn er(&self) -> ErR {
        ErR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Disconnection error has occurred"]
    #[inline(always)]
    pub fn de(&self) -> DeR {
        DeR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Parity error has occurred"]
    #[inline(always)]
    pub fn pe(&self) -> PeR {
        PeR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - A synchronization problem has occurred when receiving NChars"]
    #[inline(always)]
    pub fn we(&self) -> WeR {
        WeR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Packet is received with an invalid destination address field"]
    #[inline(always)]
    pub fn ia(&self) -> IaR {
        IaR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Set to one when a packet is received with an EOP after the first byte for a non-RMAP packet and after the second byte for a RMAP packet"]
    #[inline(always)]
    pub fn ee(&self) -> EeR {
        EeR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Active port"]
    #[inline(always)]
    pub fn ap(&self) -> ApR {
        ApR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 21:23 - Link State"]
    #[inline(always)]
    pub fn ls(&self) -> LsR {
        LsR::new(((self.bits >> 21) & 7) as u8)
    }
    #[doc = "Bits 24:25 - Number of Transmit Descriptors"]
    #[inline(always)]
    pub fn ntxd(&self) -> NtxdR {
        NtxdR::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 26:27 - Number of Receive Descriptors"]
    #[inline(always)]
    pub fn nrxd(&self) -> NrxdR {
        NrxdR::new(((self.bits >> 26) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - A new time count value was received"]
    #[inline(always)]
    #[must_use]
    pub fn to(&mut self) -> ToW<StsSpec> {
        ToW::new(self, 0)
    }
    #[doc = "Bit 1 - Credit has occurred"]
    #[inline(always)]
    #[must_use]
    pub fn ce(&mut self) -> CeW<StsSpec> {
        CeW::new(self, 1)
    }
    #[doc = "Bit 2 - Escape error has occurred"]
    #[inline(always)]
    #[must_use]
    pub fn er(&mut self) -> ErW<StsSpec> {
        ErW::new(self, 2)
    }
    #[doc = "Bit 3 - Disconnection error has occurred"]
    #[inline(always)]
    #[must_use]
    pub fn de(&mut self) -> DeW<StsSpec> {
        DeW::new(self, 3)
    }
    #[doc = "Bit 4 - Parity error has occurred"]
    #[inline(always)]
    #[must_use]
    pub fn pe(&mut self) -> PeW<StsSpec> {
        PeW::new(self, 4)
    }
    #[doc = "Bit 6 - A synchronization problem has occurred when receiving NChars"]
    #[inline(always)]
    #[must_use]
    pub fn we(&mut self) -> WeW<StsSpec> {
        WeW::new(self, 6)
    }
    #[doc = "Bit 7 - Packet is received with an invalid destination address field"]
    #[inline(always)]
    #[must_use]
    pub fn ia(&mut self) -> IaW<StsSpec> {
        IaW::new(self, 7)
    }
    #[doc = "Bit 8 - Set to one when a packet is received with an EOP after the first byte for a non-RMAP packet and after the second byte for a RMAP packet"]
    #[inline(always)]
    #[must_use]
    pub fn ee(&mut self) -> EeW<StsSpec> {
        EeW::new(self, 8)
    }
    #[doc = "Bit 9 - Active port"]
    #[inline(always)]
    #[must_use]
    pub fn ap(&mut self) -> ApW<StsSpec> {
        ApW::new(self, 9)
    }
    #[doc = "Bits 21:23 - Link State"]
    #[inline(always)]
    #[must_use]
    pub fn ls(&mut self) -> LsW<StsSpec> {
        LsW::new(self, 21)
    }
    #[doc = "Bits 24:25 - Number of Transmit Descriptors"]
    #[inline(always)]
    #[must_use]
    pub fn ntxd(&mut self) -> NtxdW<StsSpec> {
        NtxdW::new(self, 24)
    }
    #[doc = "Bits 26:27 - Number of Receive Descriptors"]
    #[inline(always)]
    #[must_use]
    pub fn nrxd(&mut self) -> NrxdW<StsSpec> {
        NrxdW::new(self, 26)
    }
}
#[doc = "Status/Interrupt Source Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sts::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sts::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StsSpec;
impl crate::RegisterSpec for StsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sts::R`](R) reader structure"]
impl crate::Readable for StsSpec {}
#[doc = "`write(|w| ..)` method takes [`sts::W`](W) writer structure"]
impl crate::Writable for StsSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets STS to value 0x0640_0000"]
impl crate::Resettable for StsSpec {
    const RESET_VALUE: u32 = 0x0640_0000;
}
