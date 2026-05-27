///Register `IC_CLR_TX_ABRT` reader
pub type R = crate::R<IcClrTxAbrtSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
/**Clear TX_ABRT interrupt (read-to-clear)

You can [`read`](crate::Reg::read) this register and get [`ic_clr_tx_abrt::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct IcClrTxAbrtSpec;
impl crate::RegisterSpec for IcClrTxAbrtSpec {
    type Ux = u32;
}
///`read()` method returns [`ic_clr_tx_abrt::R`](R) reader structure
impl crate::Readable for IcClrTxAbrtSpec {}
///`reset()` method sets IC_CLR_TX_ABRT to value 0
impl crate::Resettable for IcClrTxAbrtSpec {}
