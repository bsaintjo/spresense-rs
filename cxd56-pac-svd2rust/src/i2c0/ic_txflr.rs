///Register `IC_TXFLR` reader
pub type R = crate::R<IcTxflrSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
/**I2C transmit FIFO level (read-only)

You can [`read`](crate::Reg::read) this register and get [`ic_txflr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct IcTxflrSpec;
impl crate::RegisterSpec for IcTxflrSpec {
    type Ux = u32;
}
///`read()` method returns [`ic_txflr::R`](R) reader structure
impl crate::Readable for IcTxflrSpec {}
///`reset()` method sets IC_TXFLR to value 0
impl crate::Resettable for IcTxflrSpec {}
