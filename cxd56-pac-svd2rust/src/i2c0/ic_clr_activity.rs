///Register `IC_CLR_ACTIVITY` reader
pub type R = crate::R<IcClrActivitySpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
/**Clear ACTIVITY interrupt (read-to-clear)

You can [`read`](crate::Reg::read) this register and get [`ic_clr_activity::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct IcClrActivitySpec;
impl crate::RegisterSpec for IcClrActivitySpec {
    type Ux = u32;
}
///`read()` method returns [`ic_clr_activity::R`](R) reader structure
impl crate::Readable for IcClrActivitySpec {}
///`reset()` method sets IC_CLR_ACTIVITY to value 0
impl crate::Resettable for IcClrActivitySpec {}
