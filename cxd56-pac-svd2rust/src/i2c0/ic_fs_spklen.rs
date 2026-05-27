///Register `IC_FS_SPKLEN` reader
pub type R = crate::R<IcFsSpklenSpec>;
///Register `IC_FS_SPKLEN` writer
pub type W = crate::W<IcFsSpklenSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
/**I2C SS/FS spike suppression limit register

You can [`read`](crate::Reg::read) this register and get [`ic_fs_spklen::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ic_fs_spklen::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct IcFsSpklenSpec;
impl crate::RegisterSpec for IcFsSpklenSpec {
    type Ux = u32;
}
///`read()` method returns [`ic_fs_spklen::R`](R) reader structure
impl crate::Readable for IcFsSpklenSpec {}
///`write(|w| ..)` method takes [`ic_fs_spklen::W`](W) writer structure
impl crate::Writable for IcFsSpklenSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets IC_FS_SPKLEN to value 0x01
impl crate::Resettable for IcFsSpklenSpec {
    const RESET_VALUE: u32 = 0x01;
}
