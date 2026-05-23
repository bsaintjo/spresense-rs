///Register `RDPRECNT` reader
pub type R = crate::R<RdprecntSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
/**Captured pre-counter value (15-bit fractional ticks)

You can [`read`](crate::Reg::read) this register and get [`rdprecnt::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct RdprecntSpec;
impl crate::RegisterSpec for RdprecntSpec {
    type Ux = u32;
}
///`read()` method returns [`rdprecnt::R`](R) reader structure
impl crate::Readable for RdprecntSpec {}
///`reset()` method sets RDPRECNT to value 0
impl crate::Resettable for RdprecntSpec {}
