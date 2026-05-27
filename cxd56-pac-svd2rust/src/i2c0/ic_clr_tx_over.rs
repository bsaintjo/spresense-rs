///Register `IC_CLR_TX_OVER` reader
pub type R = crate::R<IcClrTxOverSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
/**Clear TX_OVER interrupt (read-to-clear)

You can [`read`](crate::Reg::read) this register and get [`ic_clr_tx_over::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct IcClrTxOverSpec;
impl crate::RegisterSpec for IcClrTxOverSpec {
    type Ux = u32;
}
///`read()` method returns [`ic_clr_tx_over::R`](R) reader structure
impl crate::Readable for IcClrTxOverSpec {}
///`reset()` method sets IC_CLR_TX_OVER to value 0
impl crate::Resettable for IcClrTxOverSpec {}
