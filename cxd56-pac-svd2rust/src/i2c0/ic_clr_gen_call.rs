///Register `IC_CLR_GEN_CALL` reader
pub type R = crate::R<IcClrGenCallSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
/**Clear GEN_CALL interrupt (read-to-clear)

You can [`read`](crate::Reg::read) this register and get [`ic_clr_gen_call::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct IcClrGenCallSpec;
impl crate::RegisterSpec for IcClrGenCallSpec {
    type Ux = u32;
}
///`read()` method returns [`ic_clr_gen_call::R`](R) reader structure
impl crate::Readable for IcClrGenCallSpec {}
///`reset()` method sets IC_CLR_GEN_CALL to value 0
impl crate::Resettable for IcClrGenCallSpec {}
