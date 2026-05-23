///Register `RTPRECNT` reader
pub type R = crate::R<RtprecntSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
/**Real-time pre-counter (free-running read)

You can [`read`](crate::Reg::read) this register and get [`rtprecnt::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct RtprecntSpec;
impl crate::RegisterSpec for RtprecntSpec {
    type Ux = u32;
}
///`read()` method returns [`rtprecnt::R`](R) reader structure
impl crate::Readable for RtprecntSpec {}
///`reset()` method sets RTPRECNT to value 0
impl crate::Resettable for RtprecntSpec {}
