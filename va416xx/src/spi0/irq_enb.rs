#[doc = "Register `IRQ_ENB` reader"]
pub type R = crate::R<IrqEnbSpec>;
#[doc = "Register `IRQ_ENB` writer"]
pub type W = crate::W<IrqEnbSpec>;
#[doc = "Field `RORIM` reader - RX Overrun"]
pub type RorimR = crate::BitReader;
#[doc = "Field `RORIM` writer - RX Overrun"]
pub type RorimW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RTIM` reader - RX Timeout"]
pub type RtimR = crate::BitReader;
#[doc = "Field `RTIM` writer - RX Timeout"]
pub type RtimW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXIM` reader - RX Fifo is at least half full"]
pub type RximR = crate::BitReader;
#[doc = "Field `RXIM` writer - RX Fifo is at least half full"]
pub type RximW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXIM` reader - TX Fifo is at least half empty"]
pub type TximR = crate::BitReader;
#[doc = "Field `TXIM` writer - TX Fifo is at least half empty"]
pub type TximW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - RX Overrun"]
    #[inline(always)]
    pub fn rorim(&self) -> RorimR {
        RorimR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - RX Timeout"]
    #[inline(always)]
    pub fn rtim(&self) -> RtimR {
        RtimR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - RX Fifo is at least half full"]
    #[inline(always)]
    pub fn rxim(&self) -> RximR {
        RximR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - TX Fifo is at least half empty"]
    #[inline(always)]
    pub fn txim(&self) -> TximR {
        TximR::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - RX Overrun"]
    #[inline(always)]
    #[must_use]
    pub fn rorim(&mut self) -> RorimW<IrqEnbSpec> {
        RorimW::new(self, 0)
    }
    #[doc = "Bit 1 - RX Timeout"]
    #[inline(always)]
    #[must_use]
    pub fn rtim(&mut self) -> RtimW<IrqEnbSpec> {
        RtimW::new(self, 1)
    }
    #[doc = "Bit 2 - RX Fifo is at least half full"]
    #[inline(always)]
    #[must_use]
    pub fn rxim(&mut self) -> RximW<IrqEnbSpec> {
        RximW::new(self, 2)
    }
    #[doc = "Bit 3 - TX Fifo is at least half empty"]
    #[inline(always)]
    #[must_use]
    pub fn txim(&mut self) -> TximW<IrqEnbSpec> {
        TximW::new(self, 3)
    }
}
#[doc = "Interrupt Enable Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`irq_enb::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`irq_enb::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IrqEnbSpec;
impl crate::RegisterSpec for IrqEnbSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`irq_enb::R`](R) reader structure"]
impl crate::Readable for IrqEnbSpec {}
#[doc = "`write(|w| ..)` method takes [`irq_enb::W`](W) writer structure"]
impl crate::Writable for IrqEnbSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IRQ_ENB to value 0"]
impl crate::Resettable for IrqEnbSpec {
    const RESET_VALUE: u32 = 0;
}
