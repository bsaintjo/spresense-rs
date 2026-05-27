///Register `IC_CLR_INTR` reader
pub type R = crate::R<IcClrIntrSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
/**Clear combined interrupt (read-to-clear)

You can [`read`](crate::Reg::read) this register and get [`ic_clr_intr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct IcClrIntrSpec;
impl crate::RegisterSpec for IcClrIntrSpec {
    type Ux = u32;
}
///`read()` method returns [`ic_clr_intr::R`](R) reader structure
impl crate::Readable for IcClrIntrSpec {}
///`reset()` method sets IC_CLR_INTR to value 0
impl crate::Resettable for IcClrIntrSpec {}
