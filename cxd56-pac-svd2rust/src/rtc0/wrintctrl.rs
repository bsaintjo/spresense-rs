///Register `WRINTCTRL` reader
pub type R = crate::R<WrintctrlSpec>;
///Register `WRINTCTRL` writer
pub type W = crate::W<WrintctrlSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
/**Write interrupt control

You can [`read`](crate::Reg::read) this register and get [`wrintctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wrintctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct WrintctrlSpec;
impl crate::RegisterSpec for WrintctrlSpec {
    type Ux = u32;
}
///`read()` method returns [`wrintctrl::R`](R) reader structure
impl crate::Readable for WrintctrlSpec {}
///`write(|w| ..)` method takes [`wrintctrl::W`](W) writer structure
impl crate::Writable for WrintctrlSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets WRINTCTRL to value 0
impl crate::Resettable for WrintctrlSpec {}
