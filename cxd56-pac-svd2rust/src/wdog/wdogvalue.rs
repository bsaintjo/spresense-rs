///Register `WDOGVALUE` reader
pub type R = crate::R<WdogvalueSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
/**Watchdog current value register (read-only)

You can [`read`](crate::Reg::read) this register and get [`wdogvalue::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct WdogvalueSpec;
impl crate::RegisterSpec for WdogvalueSpec {
    type Ux = u32;
}
///`read()` method returns [`wdogvalue::R`](R) reader structure
impl crate::Readable for WdogvalueSpec {}
///`reset()` method sets WDOGVALUE to value 0xffff_ffff
impl crate::Resettable for WdogvalueSpec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
