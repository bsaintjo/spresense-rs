///Register `IC_COMP_VERSION` reader
pub type R = crate::R<IcCompVersionSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
/**Component version register (read-only)

You can [`read`](crate::Reg::read) this register and get [`ic_comp_version::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct IcCompVersionSpec;
impl crate::RegisterSpec for IcCompVersionSpec {
    type Ux = u32;
}
///`read()` method returns [`ic_comp_version::R`](R) reader structure
impl crate::Readable for IcCompVersionSpec {}
///`reset()` method sets IC_COMP_VERSION to value 0x3230_302a
impl crate::Resettable for IcCompVersionSpec {
    const RESET_VALUE: u32 = 0x3230_302a;
}
