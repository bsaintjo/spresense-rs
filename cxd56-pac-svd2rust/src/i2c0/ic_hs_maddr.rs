///Register `IC_HS_MADDR` reader
pub type R = crate::R<IcHsMaddrSpec>;
///Register `IC_HS_MADDR` writer
pub type W = crate::W<IcHsMaddrSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
/**I2C high speed master mode code address register

You can [`read`](crate::Reg::read) this register and get [`ic_hs_maddr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ic_hs_maddr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct IcHsMaddrSpec;
impl crate::RegisterSpec for IcHsMaddrSpec {
    type Ux = u32;
}
///`read()` method returns [`ic_hs_maddr::R`](R) reader structure
impl crate::Readable for IcHsMaddrSpec {}
///`write(|w| ..)` method takes [`ic_hs_maddr::W`](W) writer structure
impl crate::Writable for IcHsMaddrSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets IC_HS_MADDR to value 0x01
impl crate::Resettable for IcHsMaddrSpec {
    const RESET_VALUE: u32 = 0x01;
}
