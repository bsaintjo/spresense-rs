///Register `IC_SLV_DATA_NACK_ONLY` reader
pub type R = crate::R<IcSlvDataNackOnlySpec>;
///Register `IC_SLV_DATA_NACK_ONLY` writer
pub type W = crate::W<IcSlvDataNackOnlySpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
/**Generate NACK for data bytes in slave mode

You can [`read`](crate::Reg::read) this register and get [`ic_slv_data_nack_only::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ic_slv_data_nack_only::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct IcSlvDataNackOnlySpec;
impl crate::RegisterSpec for IcSlvDataNackOnlySpec {
    type Ux = u32;
}
///`read()` method returns [`ic_slv_data_nack_only::R`](R) reader structure
impl crate::Readable for IcSlvDataNackOnlySpec {}
///`write(|w| ..)` method takes [`ic_slv_data_nack_only::W`](W) writer structure
impl crate::Writable for IcSlvDataNackOnlySpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets IC_SLV_DATA_NACK_ONLY to value 0
impl crate::Resettable for IcSlvDataNackOnlySpec {}
