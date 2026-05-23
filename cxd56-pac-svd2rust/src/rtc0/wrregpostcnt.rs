///Register `WRREGPOSTCNT` reader
pub type R = crate::R<WrregpostcntSpec>;
///Register `WRREGPOSTCNT` writer
pub type W = crate::W<WrregpostcntSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
/**Write post-counter (32-bit integer seconds to set)

You can [`read`](crate::Reg::read) this register and get [`wrregpostcnt::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wrregpostcnt::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct WrregpostcntSpec;
impl crate::RegisterSpec for WrregpostcntSpec {
    type Ux = u32;
}
///`read()` method returns [`wrregpostcnt::R`](R) reader structure
impl crate::Readable for WrregpostcntSpec {}
///`write(|w| ..)` method takes [`wrregpostcnt::W`](W) writer structure
impl crate::Writable for WrregpostcntSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets WRREGPOSTCNT to value 0
impl crate::Resettable for WrregpostcntSpec {}
