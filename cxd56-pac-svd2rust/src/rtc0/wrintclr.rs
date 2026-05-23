///Register `WRINTCLR` reader
pub type R = crate::R<WrintclrSpec>;
///Register `WRINTCLR` writer
pub type W = crate::W<WrintclrSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
/**Write interrupt clear

You can [`read`](crate::Reg::read) this register and get [`wrintclr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wrintclr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct WrintclrSpec;
impl crate::RegisterSpec for WrintclrSpec {
    type Ux = u32;
}
///`read()` method returns [`wrintclr::R`](R) reader structure
impl crate::Readable for WrintclrSpec {}
///`write(|w| ..)` method takes [`wrintclr::W`](W) writer structure
impl crate::Writable for WrintclrSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets WRINTCLR to value 0
impl crate::Resettable for WrintclrSpec {}
