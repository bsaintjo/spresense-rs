///Register `IC_SDA_SETUP` reader
pub type R = crate::R<IcSdaSetupSpec>;
///Register `IC_SDA_SETUP` writer
pub type W = crate::W<IcSdaSetupSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
/**I2C SDA setup time register

You can [`read`](crate::Reg::read) this register and get [`ic_sda_setup::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ic_sda_setup::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct IcSdaSetupSpec;
impl crate::RegisterSpec for IcSdaSetupSpec {
    type Ux = u32;
}
///`read()` method returns [`ic_sda_setup::R`](R) reader structure
impl crate::Readable for IcSdaSetupSpec {}
///`write(|w| ..)` method takes [`ic_sda_setup::W`](W) writer structure
impl crate::Writable for IcSdaSetupSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets IC_SDA_SETUP to value 0x64
impl crate::Resettable for IcSdaSetupSpec {
    const RESET_VALUE: u32 = 0x64;
}
