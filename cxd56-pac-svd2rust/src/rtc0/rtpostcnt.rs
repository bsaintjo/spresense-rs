///Register `RTPOSTCNT` reader
pub type R = crate::R<RtpostcntSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
/**Real-time post-counter (free-running read)

You can [`read`](crate::Reg::read) this register and get [`rtpostcnt::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct RtpostcntSpec;
impl crate::RegisterSpec for RtpostcntSpec {
    type Ux = u32;
}
///`read()` method returns [`rtpostcnt::R`](R) reader structure
impl crate::Readable for RtpostcntSpec {}
///`reset()` method sets RTPOSTCNT to value 0
impl crate::Resettable for RtpostcntSpec {}
