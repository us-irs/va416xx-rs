#[doc = "Register `TXBREAK` writer"]
pub type W = crate::W<TxbreakSpec>;
impl core::fmt::Debug for crate::generic::Reg<TxbreakSpec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {}
#[doc = "Break Transmit Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`txbreak::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TxbreakSpec;
impl crate::RegisterSpec for TxbreakSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`txbreak::W`](W) writer structure"]
impl crate::Writable for TxbreakSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TXBREAK to value 0"]
impl crate::Resettable for TxbreakSpec {
    const RESET_VALUE: u32 = 0;
}
