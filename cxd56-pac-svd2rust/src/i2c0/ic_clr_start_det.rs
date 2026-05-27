///Register `IC_CLR_START_DET` reader
pub type R = crate::R<IcClrStartDetSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
/**Clear START_DET interrupt (read-to-clear)

You can [`read`](crate::Reg::read) this register and get [`ic_clr_start_det::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct IcClrStartDetSpec;
impl crate::RegisterSpec for IcClrStartDetSpec {
    type Ux = u32;
}
///`read()` method returns [`ic_clr_start_det::R`](R) reader structure
impl crate::Readable for IcClrStartDetSpec {}
///`reset()` method sets IC_CLR_START_DET to value 0
impl crate::Resettable for IcClrStartDetSpec {}
