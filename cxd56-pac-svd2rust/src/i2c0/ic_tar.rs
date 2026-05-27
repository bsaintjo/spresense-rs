///Register `IC_TAR` reader
pub type R = crate::R<IcTarSpec>;
///Register `IC_TAR` writer
pub type W = crate::W<IcTarSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
/**I2C target address register

You can [`read`](crate::Reg::read) this register and get [`ic_tar::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ic_tar::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct IcTarSpec;
impl crate::RegisterSpec for IcTarSpec {
    type Ux = u32;
}
///`read()` method returns [`ic_tar::R`](R) reader structure
impl crate::Readable for IcTarSpec {}
///`write(|w| ..)` method takes [`ic_tar::W`](W) writer structure
impl crate::Writable for IcTarSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets IC_TAR to value 0x1055
impl crate::Resettable for IcTarSpec {
    const RESET_VALUE: u32 = 0x1055;
}
