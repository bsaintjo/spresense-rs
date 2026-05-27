///Register `IC_CLR_RX_OVER` reader
pub type R = crate::R<IcClrRxOverSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
/**Clear RX_OVER interrupt (read-to-clear)

You can [`read`](crate::Reg::read) this register and get [`ic_clr_rx_over::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct IcClrRxOverSpec;
impl crate::RegisterSpec for IcClrRxOverSpec {
    type Ux = u32;
}
///`read()` method returns [`ic_clr_rx_over::R`](R) reader structure
impl crate::Readable for IcClrRxOverSpec {}
///`reset()` method sets IC_CLR_RX_OVER to value 0
impl crate::Resettable for IcClrRxOverSpec {}
