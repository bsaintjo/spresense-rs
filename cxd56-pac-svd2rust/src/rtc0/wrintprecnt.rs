///Register `WRINTPRECNT` reader
pub type R = crate::R<WrintprecntSpec>;
///Register `WRINTPRECNT` writer
pub type W = crate::W<WrintprecntSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
/**Write interrupt pre-counter compare value

You can [`read`](crate::Reg::read) this register and get [`wrintprecnt::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wrintprecnt::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct WrintprecntSpec;
impl crate::RegisterSpec for WrintprecntSpec {
    type Ux = u32;
}
///`read()` method returns [`wrintprecnt::R`](R) reader structure
impl crate::Readable for WrintprecntSpec {}
///`write(|w| ..)` method takes [`wrintprecnt::W`](W) writer structure
impl crate::Writable for WrintprecntSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets WRINTPRECNT to value 0
impl crate::Resettable for WrintprecntSpec {}
