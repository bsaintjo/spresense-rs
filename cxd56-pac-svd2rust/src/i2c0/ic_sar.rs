///Register `IC_SAR` reader
pub type R = crate::R<IcSarSpec>;
///Register `IC_SAR` writer
pub type W = crate::W<IcSarSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
/**I2C slave address register

You can [`read`](crate::Reg::read) this register and get [`ic_sar::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ic_sar::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct IcSarSpec;
impl crate::RegisterSpec for IcSarSpec {
    type Ux = u32;
}
///`read()` method returns [`ic_sar::R`](R) reader structure
impl crate::Readable for IcSarSpec {}
///`write(|w| ..)` method takes [`ic_sar::W`](W) writer structure
impl crate::Writable for IcSarSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets IC_SAR to value 0x55
impl crate::Resettable for IcSarSpec {
    const RESET_VALUE: u32 = 0x55;
}
