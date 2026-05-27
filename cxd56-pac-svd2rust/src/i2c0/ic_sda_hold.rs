///Register `IC_SDA_HOLD` reader
pub type R = crate::R<IcSdaHoldSpec>;
///Register `IC_SDA_HOLD` writer
pub type W = crate::W<IcSdaHoldSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
/**I2C SDA hold time length register

You can [`read`](crate::Reg::read) this register and get [`ic_sda_hold::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ic_sda_hold::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct IcSdaHoldSpec;
impl crate::RegisterSpec for IcSdaHoldSpec {
    type Ux = u32;
}
///`read()` method returns [`ic_sda_hold::R`](R) reader structure
impl crate::Readable for IcSdaHoldSpec {}
///`write(|w| ..)` method takes [`ic_sda_hold::W`](W) writer structure
impl crate::Writable for IcSdaHoldSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets IC_SDA_HOLD to value 0x01
impl crate::Resettable for IcSdaHoldSpec {
    const RESET_VALUE: u32 = 0x01;
}
