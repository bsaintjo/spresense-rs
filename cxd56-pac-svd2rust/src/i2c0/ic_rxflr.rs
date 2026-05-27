///Register `IC_RXFLR` reader
pub type R = crate::R<IcRxflrSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
/**I2C receive FIFO level (read-only)

You can [`read`](crate::Reg::read) this register and get [`ic_rxflr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct IcRxflrSpec;
impl crate::RegisterSpec for IcRxflrSpec {
    type Ux = u32;
}
///`read()` method returns [`ic_rxflr::R`](R) reader structure
impl crate::Readable for IcRxflrSpec {}
///`reset()` method sets IC_RXFLR to value 0
impl crate::Resettable for IcRxflrSpec {}
