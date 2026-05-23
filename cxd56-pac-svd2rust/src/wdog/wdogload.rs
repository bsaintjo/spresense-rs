///Register `WDOGLOAD` reader
pub type R = crate::R<WdogloadSpec>;
///Register `WDOGLOAD` writer
pub type W = crate::W<WdogloadSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
/**Watchdog load register

You can [`read`](crate::Reg::read) this register and get [`wdogload::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wdogload::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct WdogloadSpec;
impl crate::RegisterSpec for WdogloadSpec {
    type Ux = u32;
}
///`read()` method returns [`wdogload::R`](R) reader structure
impl crate::Readable for WdogloadSpec {}
///`write(|w| ..)` method takes [`wdogload::W`](W) writer structure
impl crate::Writable for WdogloadSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets WDOGLOAD to value 0xffff_ffff
impl crate::Resettable for WdogloadSpec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
