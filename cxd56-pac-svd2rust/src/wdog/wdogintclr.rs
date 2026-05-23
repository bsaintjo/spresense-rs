///Register `WDOGINTCLR` writer
pub type W = crate::W<WdogintclrSpec>;
impl core::fmt::Debug for crate::generic::Reg<WdogintclrSpec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {}
/**Watchdog interrupt clear (write any value to clear and reload)

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wdogintclr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct WdogintclrSpec;
impl crate::RegisterSpec for WdogintclrSpec {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`wdogintclr::W`](W) writer structure
impl crate::Writable for WdogintclrSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets WDOGINTCLR to value 0
impl crate::Resettable for WdogintclrSpec {}
