///Register `IC_CLR_RX_DONE` reader
pub type R = crate::R<IcClrRxDoneSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
/**Clear RX_DONE interrupt (read-to-clear)

You can [`read`](crate::Reg::read) this register and get [`ic_clr_rx_done::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct IcClrRxDoneSpec;
impl crate::RegisterSpec for IcClrRxDoneSpec {
    type Ux = u32;
}
///`read()` method returns [`ic_clr_rx_done::R`](R) reader structure
impl crate::Readable for IcClrRxDoneSpec {}
///`reset()` method sets IC_CLR_RX_DONE to value 0
impl crate::Resettable for IcClrRxDoneSpec {}
