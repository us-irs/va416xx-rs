#[doc = "Register `IRQ_CLR` writer"]
pub type W = crate::W<IrqClrSpec>;
#[doc = "Field `FIFO_OFLOW` writer - Clears the FIFO overflow interrupt status. Always reads 0"]
pub type FifoOflowW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FIFO_UFLOW` writer - Clears the FIFO underflow interrupt status. Always reads 0"]
pub type FifoUflowW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DAC_DONE` writer - Clears the DAC done interrupt status. Always reads 0"]
pub type DacDoneW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TRIG_ERROR` writer - Clears the trigger error interrupt status. Always reads 0"]
pub type TrigErrorW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Clears the FIFO overflow interrupt status. Always reads 0"]
    #[inline(always)]
    #[must_use]
    pub fn fifo_oflow(&mut self) -> FifoOflowW<IrqClrSpec> {
        FifoOflowW::new(self, 0)
    }
    #[doc = "Bit 1 - Clears the FIFO underflow interrupt status. Always reads 0"]
    #[inline(always)]
    #[must_use]
    pub fn fifo_uflow(&mut self) -> FifoUflowW<IrqClrSpec> {
        FifoUflowW::new(self, 1)
    }
    #[doc = "Bit 2 - Clears the DAC done interrupt status. Always reads 0"]
    #[inline(always)]
    #[must_use]
    pub fn dac_done(&mut self) -> DacDoneW<IrqClrSpec> {
        DacDoneW::new(self, 2)
    }
    #[doc = "Bit 3 - Clears the trigger error interrupt status. Always reads 0"]
    #[inline(always)]
    #[must_use]
    pub fn trig_error(&mut self) -> TrigErrorW<IrqClrSpec> {
        TrigErrorW::new(self, 3)
    }
}
#[doc = "Clear Interrupt\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`irq_clr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IrqClrSpec;
impl crate::RegisterSpec for IrqClrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`irq_clr::W`](W) writer structure"]
impl crate::Writable for IrqClrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IRQ_CLR to value 0"]
impl crate::Resettable for IrqClrSpec {
    const RESET_VALUE: u32 = 0;
}
