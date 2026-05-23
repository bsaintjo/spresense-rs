///Register `WRREGPRECNT` reader
pub type R = crate::R<WrregprecntSpec>;
///Register `WRREGPRECNT` writer
pub type W = crate::W<WrregprecntSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
/**Write pre-counter (15-bit fractional 32 kHz ticks to set)

You can [`read`](crate::Reg::read) this register and get [`wrregprecnt::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wrregprecnt::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct WrregprecntSpec;
impl crate::RegisterSpec for WrregprecntSpec {
    type Ux = u32;
}
///`read()` method returns [`wrregprecnt::R`](R) reader structure
impl crate::Readable for WrregprecntSpec {}
///`write(|w| ..)` method takes [`wrregprecnt::W`](W) writer structure
impl crate::Writable for WrregprecntSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets WRREGPRECNT to value 0
impl crate::Resettable for WrregprecntSpec {}
