///Register `IC_CLR_RX_UNDER` reader
pub type R = crate::R<IcClrRxUnderSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
/**Clear RX_UNDER interrupt (read-to-clear)

You can [`read`](crate::Reg::read) this register and get [`ic_clr_rx_under::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct IcClrRxUnderSpec;
impl crate::RegisterSpec for IcClrRxUnderSpec {
    type Ux = u32;
}
///`read()` method returns [`ic_clr_rx_under::R`](R) reader structure
impl crate::Readable for IcClrRxUnderSpec {}
///`reset()` method sets IC_CLR_RX_UNDER to value 0
impl crate::Resettable for IcClrRxUnderSpec {}
