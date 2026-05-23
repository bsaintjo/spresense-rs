///Register `BGLOAD` reader
pub type R = crate::R<BgloadSpec>;
///Register `BGLOAD` writer
pub type W = crate::W<BgloadSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
/**Background load register (no immediate counter restart)

You can [`read`](crate::Reg::read) this register and get [`bgload::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bgload::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct BgloadSpec;
impl crate::RegisterSpec for BgloadSpec {
    type Ux = u32;
}
///`read()` method returns [`bgload::R`](R) reader structure
impl crate::Readable for BgloadSpec {}
///`write(|w| ..)` method takes [`bgload::W`](W) writer structure
impl crate::Writable for BgloadSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets BGLOAD to value 0
impl crate::Resettable for BgloadSpec {}
