///Register `IC_COMP_TYPE` reader
pub type R = crate::R<IcCompTypeSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
/**Component type register (read-only, 0x44570140 = DW_apb_i2c)

You can [`read`](crate::Reg::read) this register and get [`ic_comp_type::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct IcCompTypeSpec;
impl crate::RegisterSpec for IcCompTypeSpec {
    type Ux = u32;
}
///`read()` method returns [`ic_comp_type::R`](R) reader structure
impl crate::Readable for IcCompTypeSpec {}
///`reset()` method sets IC_COMP_TYPE to value 0x4457_0140
impl crate::Resettable for IcCompTypeSpec {
    const RESET_VALUE: u32 = 0x4457_0140;
}
