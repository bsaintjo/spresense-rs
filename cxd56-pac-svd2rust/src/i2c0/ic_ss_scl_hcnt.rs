///Register `IC_SS_SCL_HCNT` reader
pub type R = crate::R<IcSsSclHcntSpec>;
///Register `IC_SS_SCL_HCNT` writer
pub type W = crate::W<IcSsSclHcntSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
/**Standard speed SCL high period count

You can [`read`](crate::Reg::read) this register and get [`ic_ss_scl_hcnt::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ic_ss_scl_hcnt::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct IcSsSclHcntSpec;
impl crate::RegisterSpec for IcSsSclHcntSpec {
    type Ux = u32;
}
///`read()` method returns [`ic_ss_scl_hcnt::R`](R) reader structure
impl crate::Readable for IcSsSclHcntSpec {}
///`write(|w| ..)` method takes [`ic_ss_scl_hcnt::W`](W) writer structure
impl crate::Writable for IcSsSclHcntSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets IC_SS_SCL_HCNT to value 0x0190
impl crate::Resettable for IcSsSclHcntSpec {
    const RESET_VALUE: u32 = 0x0190;
}
