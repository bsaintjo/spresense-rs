///Register `RDPOSTCNT` reader
pub type R = crate::R<RdpostcntSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
/**Captured post-counter value (integer seconds)

You can [`read`](crate::Reg::read) this register and get [`rdpostcnt::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct RdpostcntSpec;
impl crate::RegisterSpec for RdpostcntSpec {
    type Ux = u32;
}
///`read()` method returns [`rdpostcnt::R`](R) reader structure
impl crate::Readable for RdpostcntSpec {}
///`reset()` method sets RDPOSTCNT to value 0
impl crate::Resettable for RdpostcntSpec {}
