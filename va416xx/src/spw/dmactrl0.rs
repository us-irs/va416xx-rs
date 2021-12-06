#[doc = "Register `DMACTRL0` reader"]
pub type R = crate::R<Dmactrl0Spec>;
#[doc = "Register `DMACTRL0` writer"]
pub type W = crate::W<Dmactrl0Spec>;
#[doc = "Field `TE` reader - Write a one to this bit each time new descriptors are activated in the table"]
pub type TeR = crate::BitReader;
#[doc = "Field `TE` writer - Write a one to this bit each time new descriptors are activated in the table"]
pub type TeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RE` reader - Packets are allowed to be received to this channel"]
pub type ReR = crate::BitReader;
#[doc = "Field `RE` writer - Packets are allowed to be received to this channel"]
pub type ReW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TI` reader - An interrupt will be generated each time a packet is transmitted"]
pub type TiR = crate::BitReader;
#[doc = "Field `TI` writer - An interrupt will be generated each time a packet is transmitted"]
pub type TiW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RI` reader - An interrupt will be generated each time a packet has been received"]
pub type RiR = crate::BitReader;
#[doc = "Field `RI` writer - An interrupt will be generated each time a packet has been received"]
pub type RiW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AI` reader - An interrupt will be generated each time an AHB error occurs when this DMA channel is accessing the bus"]
pub type AiR = crate::BitReader;
#[doc = "Field `AI` writer - An interrupt will be generated each time an AHB error occurs when this DMA channel is accessing the bus"]
pub type AiW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PS` reader - Set each time a packet has been sent"]
pub type PsR = crate::BitReader;
#[doc = "Field `PS` writer - Set each time a packet has been sent"]
pub type PsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PR` reader - Set each time a packet has been received"]
pub type PrR = crate::BitReader;
#[doc = "Field `PR` writer - Set each time a packet has been received"]
pub type PrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TA` reader - An error response was detected on the AHB bus - DMA transmit"]
pub type TaR = crate::BitReader;
#[doc = "Field `TA` writer - An error response was detected on the AHB bus - DMA transmit"]
pub type TaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RA` reader - An error response was detected on the AHB bus - DMA receive"]
pub type RaR = crate::BitReader;
#[doc = "Field `RA` writer - An error response was detected on the AHB bus - DMA receive"]
pub type RaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AT` reader - Abort the currently transmitting packet and disable transmissions"]
pub type AtR = crate::BitReader;
#[doc = "Field `RX` reader - Reception to the DMA channel is currently active"]
pub type RxR = crate::BitReader;
#[doc = "Field `RD` reader - Indicates to the GRSPW that there are enabled descriptors in the descriptor table"]
pub type RdR = crate::BitReader;
#[doc = "Field `RD` writer - Indicates to the GRSPW that there are enabled descriptors in the descriptor table"]
pub type RdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NS` reader - If cleared, packets will be discarded when a packet is arriving and there are no active descriptors. If set, the GRSPW will wait for a descriptor to be activated"]
pub type NsR = crate::BitReader;
#[doc = "Field `NS` writer - If cleared, packets will be discarded when a packet is arriving and there are no active descriptors. If set, the GRSPW will wait for a descriptor to be activated"]
pub type NsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EN` reader - Enable Address"]
pub type EnR = crate::BitReader;
#[doc = "Field `EN` writer - Enable Address"]
pub type EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SA` reader - Strip Address"]
pub type SaR = crate::BitReader;
#[doc = "Field `SA` writer - Strip Address"]
pub type SaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SP` reader - Strip PID"]
pub type SpR = crate::BitReader;
#[doc = "Field `SP` writer - Strip PID"]
pub type SpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LE` reader - Disable transmitter when a link error occurs"]
pub type LeR = crate::BitReader;
#[doc = "Field `LE` writer - Disable transmitter when a link error occurs"]
pub type LeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TL` reader - Transmit Enable Lock"]
pub type TlR = crate::BitReader;
#[doc = "Field `TL` writer - Transmit Enable Lock"]
pub type TlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TP` reader - Transmit Packet IRQ"]
pub type TpR = crate::BitReader;
#[doc = "Field `TP` writer - Transmit Packet IRQ"]
pub type TpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RP` reader - Receive Packet IRQ"]
pub type RpR = crate::BitReader;
#[doc = "Field `RP` writer - Receive Packet IRQ"]
pub type RpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IT` reader - Interrupt code transmit enable on truncation"]
pub type ItR = crate::BitReader;
#[doc = "Field `IT` writer - Interrupt code transmit enable on truncation"]
pub type ItW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IE` reader - Interrupt code transmit enable on EEP"]
pub type IeR = crate::BitReader;
#[doc = "Field `IE` writer - Interrupt code transmit enable on EEP"]
pub type IeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TR` reader - Truncated"]
pub type TrR = crate::BitReader;
#[doc = "Field `TR` writer - Truncated"]
pub type TrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EP` reader - EEP Termination"]
pub type EpR = crate::BitReader;
#[doc = "Field `EP` writer - EEP Termination"]
pub type EpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INTNUM` reader - Interrupt number used for this channel"]
pub type IntnumR = crate::FieldReader;
#[doc = "Field `INTNUM` writer - Interrupt number used for this channel"]
pub type IntnumW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bit 0 - Write a one to this bit each time new descriptors are activated in the table"]
    #[inline(always)]
    pub fn te(&self) -> TeR {
        TeR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Packets are allowed to be received to this channel"]
    #[inline(always)]
    pub fn re(&self) -> ReR {
        ReR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - An interrupt will be generated each time a packet is transmitted"]
    #[inline(always)]
    pub fn ti(&self) -> TiR {
        TiR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - An interrupt will be generated each time a packet has been received"]
    #[inline(always)]
    pub fn ri(&self) -> RiR {
        RiR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - An interrupt will be generated each time an AHB error occurs when this DMA channel is accessing the bus"]
    #[inline(always)]
    pub fn ai(&self) -> AiR {
        AiR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Set each time a packet has been sent"]
    #[inline(always)]
    pub fn ps(&self) -> PsR {
        PsR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Set each time a packet has been received"]
    #[inline(always)]
    pub fn pr(&self) -> PrR {
        PrR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - An error response was detected on the AHB bus - DMA transmit"]
    #[inline(always)]
    pub fn ta(&self) -> TaR {
        TaR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - An error response was detected on the AHB bus - DMA receive"]
    #[inline(always)]
    pub fn ra(&self) -> RaR {
        RaR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Abort the currently transmitting packet and disable transmissions"]
    #[inline(always)]
    pub fn at(&self) -> AtR {
        AtR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Reception to the DMA channel is currently active"]
    #[inline(always)]
    pub fn rx(&self) -> RxR {
        RxR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Indicates to the GRSPW that there are enabled descriptors in the descriptor table"]
    #[inline(always)]
    pub fn rd(&self) -> RdR {
        RdR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - If cleared, packets will be discarded when a packet is arriving and there are no active descriptors. If set, the GRSPW will wait for a descriptor to be activated"]
    #[inline(always)]
    pub fn ns(&self) -> NsR {
        NsR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Enable Address"]
    #[inline(always)]
    pub fn en(&self) -> EnR {
        EnR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Strip Address"]
    #[inline(always)]
    pub fn sa(&self) -> SaR {
        SaR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Strip PID"]
    #[inline(always)]
    pub fn sp(&self) -> SpR {
        SpR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Disable transmitter when a link error occurs"]
    #[inline(always)]
    pub fn le(&self) -> LeR {
        LeR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Transmit Enable Lock"]
    #[inline(always)]
    pub fn tl(&self) -> TlR {
        TlR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Transmit Packet IRQ"]
    #[inline(always)]
    pub fn tp(&self) -> TpR {
        TpR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Receive Packet IRQ"]
    #[inline(always)]
    pub fn rp(&self) -> RpR {
        RpR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Interrupt code transmit enable on truncation"]
    #[inline(always)]
    pub fn it(&self) -> ItR {
        ItR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Interrupt code transmit enable on EEP"]
    #[inline(always)]
    pub fn ie(&self) -> IeR {
        IeR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Truncated"]
    #[inline(always)]
    pub fn tr(&self) -> TrR {
        TrR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - EEP Termination"]
    #[inline(always)]
    pub fn ep(&self) -> EpR {
        EpR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 26:31 - Interrupt number used for this channel"]
    #[inline(always)]
    pub fn intnum(&self) -> IntnumR {
        IntnumR::new(((self.bits >> 26) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Write a one to this bit each time new descriptors are activated in the table"]
    #[inline(always)]
    #[must_use]
    pub fn te(&mut self) -> TeW<Dmactrl0Spec> {
        TeW::new(self, 0)
    }
    #[doc = "Bit 1 - Packets are allowed to be received to this channel"]
    #[inline(always)]
    #[must_use]
    pub fn re(&mut self) -> ReW<Dmactrl0Spec> {
        ReW::new(self, 1)
    }
    #[doc = "Bit 2 - An interrupt will be generated each time a packet is transmitted"]
    #[inline(always)]
    #[must_use]
    pub fn ti(&mut self) -> TiW<Dmactrl0Spec> {
        TiW::new(self, 2)
    }
    #[doc = "Bit 3 - An interrupt will be generated each time a packet has been received"]
    #[inline(always)]
    #[must_use]
    pub fn ri(&mut self) -> RiW<Dmactrl0Spec> {
        RiW::new(self, 3)
    }
    #[doc = "Bit 4 - An interrupt will be generated each time an AHB error occurs when this DMA channel is accessing the bus"]
    #[inline(always)]
    #[must_use]
    pub fn ai(&mut self) -> AiW<Dmactrl0Spec> {
        AiW::new(self, 4)
    }
    #[doc = "Bit 5 - Set each time a packet has been sent"]
    #[inline(always)]
    #[must_use]
    pub fn ps(&mut self) -> PsW<Dmactrl0Spec> {
        PsW::new(self, 5)
    }
    #[doc = "Bit 6 - Set each time a packet has been received"]
    #[inline(always)]
    #[must_use]
    pub fn pr(&mut self) -> PrW<Dmactrl0Spec> {
        PrW::new(self, 6)
    }
    #[doc = "Bit 7 - An error response was detected on the AHB bus - DMA transmit"]
    #[inline(always)]
    #[must_use]
    pub fn ta(&mut self) -> TaW<Dmactrl0Spec> {
        TaW::new(self, 7)
    }
    #[doc = "Bit 8 - An error response was detected on the AHB bus - DMA receive"]
    #[inline(always)]
    #[must_use]
    pub fn ra(&mut self) -> RaW<Dmactrl0Spec> {
        RaW::new(self, 8)
    }
    #[doc = "Bit 11 - Indicates to the GRSPW that there are enabled descriptors in the descriptor table"]
    #[inline(always)]
    #[must_use]
    pub fn rd(&mut self) -> RdW<Dmactrl0Spec> {
        RdW::new(self, 11)
    }
    #[doc = "Bit 12 - If cleared, packets will be discarded when a packet is arriving and there are no active descriptors. If set, the GRSPW will wait for a descriptor to be activated"]
    #[inline(always)]
    #[must_use]
    pub fn ns(&mut self) -> NsW<Dmactrl0Spec> {
        NsW::new(self, 12)
    }
    #[doc = "Bit 13 - Enable Address"]
    #[inline(always)]
    #[must_use]
    pub fn en(&mut self) -> EnW<Dmactrl0Spec> {
        EnW::new(self, 13)
    }
    #[doc = "Bit 14 - Strip Address"]
    #[inline(always)]
    #[must_use]
    pub fn sa(&mut self) -> SaW<Dmactrl0Spec> {
        SaW::new(self, 14)
    }
    #[doc = "Bit 15 - Strip PID"]
    #[inline(always)]
    #[must_use]
    pub fn sp(&mut self) -> SpW<Dmactrl0Spec> {
        SpW::new(self, 15)
    }
    #[doc = "Bit 16 - Disable transmitter when a link error occurs"]
    #[inline(always)]
    #[must_use]
    pub fn le(&mut self) -> LeW<Dmactrl0Spec> {
        LeW::new(self, 16)
    }
    #[doc = "Bit 17 - Transmit Enable Lock"]
    #[inline(always)]
    #[must_use]
    pub fn tl(&mut self) -> TlW<Dmactrl0Spec> {
        TlW::new(self, 17)
    }
    #[doc = "Bit 18 - Transmit Packet IRQ"]
    #[inline(always)]
    #[must_use]
    pub fn tp(&mut self) -> TpW<Dmactrl0Spec> {
        TpW::new(self, 18)
    }
    #[doc = "Bit 19 - Receive Packet IRQ"]
    #[inline(always)]
    #[must_use]
    pub fn rp(&mut self) -> RpW<Dmactrl0Spec> {
        RpW::new(self, 19)
    }
    #[doc = "Bit 20 - Interrupt code transmit enable on truncation"]
    #[inline(always)]
    #[must_use]
    pub fn it(&mut self) -> ItW<Dmactrl0Spec> {
        ItW::new(self, 20)
    }
    #[doc = "Bit 21 - Interrupt code transmit enable on EEP"]
    #[inline(always)]
    #[must_use]
    pub fn ie(&mut self) -> IeW<Dmactrl0Spec> {
        IeW::new(self, 21)
    }
    #[doc = "Bit 22 - Truncated"]
    #[inline(always)]
    #[must_use]
    pub fn tr(&mut self) -> TrW<Dmactrl0Spec> {
        TrW::new(self, 22)
    }
    #[doc = "Bit 23 - EEP Termination"]
    #[inline(always)]
    #[must_use]
    pub fn ep(&mut self) -> EpW<Dmactrl0Spec> {
        EpW::new(self, 23)
    }
    #[doc = "Bits 26:31 - Interrupt number used for this channel"]
    #[inline(always)]
    #[must_use]
    pub fn intnum(&mut self) -> IntnumW<Dmactrl0Spec> {
        IntnumW::new(self, 26)
    }
}
#[doc = "DMA Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmactrl0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmactrl0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dmactrl0Spec;
impl crate::RegisterSpec for Dmactrl0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmactrl0::R`](R) reader structure"]
impl crate::Readable for Dmactrl0Spec {}
#[doc = "`write(|w| ..)` method takes [`dmactrl0::W`](W) writer structure"]
impl crate::Writable for Dmactrl0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DMACTRL0 to value 0"]
impl crate::Resettable for Dmactrl0Spec {
    const RESET_VALUE: u32 = 0;
}
