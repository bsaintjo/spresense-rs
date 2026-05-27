///Register `IC_CLR_STOP_DET` reader
pub type R = crate::R<IcClrStopDetSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
/**Clear STOP_DET interrupt (read-to-clear)

You can [`read`](crate::Reg::read) this register and get [`ic_clr_stop_det::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct IcClrStopDetSpec;
impl crate::RegisterSpec for IcClrStopDetSpec {
    type Ux = u32;
}
///`read()` method returns [`ic_clr_stop_det::R`](R) reader structure
impl crate::Readable for IcClrStopDetSpec {}
///`reset()` method sets IC_CLR_STOP_DET to value 0
impl crate::Resettable for IcClrStopDetSpec {}
