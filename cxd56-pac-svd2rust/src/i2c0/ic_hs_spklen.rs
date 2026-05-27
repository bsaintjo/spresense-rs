///Register `IC_HS_SPKLEN` reader
pub type R = crate::R<IcHsSpklenSpec>;
///Register `IC_HS_SPKLEN` writer
pub type W = crate::W<IcHsSpklenSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
/**I2C HS spike suppression limit register

You can [`read`](crate::Reg::read) this register and get [`ic_hs_spklen::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ic_hs_spklen::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct IcHsSpklenSpec;
impl crate::RegisterSpec for IcHsSpklenSpec {
    type Ux = u32;
}
///`read()` method returns [`ic_hs_spklen::R`](R) reader structure
impl crate::Readable for IcHsSpklenSpec {}
///`write(|w| ..)` method takes [`ic_hs_spklen::W`](W) writer structure
impl crate::Writable for IcHsSpklenSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets IC_HS_SPKLEN to value 0x01
impl crate::Resettable for IcHsSpklenSpec {
    const RESET_VALUE: u32 = 0x01;
}
