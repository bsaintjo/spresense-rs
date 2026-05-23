///Register `VALUE` reader
pub type R = crate::R<ValueSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
/**Current counter value (read-only)

You can [`read`](crate::Reg::read) this register and get [`value::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct ValueSpec;
impl crate::RegisterSpec for ValueSpec {
    type Ux = u32;
}
///`read()` method returns [`value::R`](R) reader structure
impl crate::Readable for ValueSpec {}
///`reset()` method sets VALUE to value 0xffff_ffff
impl crate::Resettable for ValueSpec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
