///Register `INTCLR` writer
pub type W = crate::W<IntclrSpec>;
impl core::fmt::Debug for crate::generic::Reg<IntclrSpec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {}
/**Interrupt clear (write any value to clear)

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intclr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct IntclrSpec;
impl crate::RegisterSpec for IntclrSpec {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`intclr::W`](W) writer structure
impl crate::Writable for IntclrSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets INTCLR to value 0
impl crate::Resettable for IntclrSpec {}
