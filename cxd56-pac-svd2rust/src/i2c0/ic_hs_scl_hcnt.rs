///Register `IC_HS_SCL_HCNT` reader
pub type R = crate::R<IcHsSclHcntSpec>;
///Register `IC_HS_SCL_HCNT` writer
pub type W = crate::W<IcHsSclHcntSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
/**High speed SCL high period count

You can [`read`](crate::Reg::read) this register and get [`ic_hs_scl_hcnt::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ic_hs_scl_hcnt::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct IcHsSclHcntSpec;
impl crate::RegisterSpec for IcHsSclHcntSpec {
    type Ux = u32;
}
///`read()` method returns [`ic_hs_scl_hcnt::R`](R) reader structure
impl crate::Readable for IcHsSclHcntSpec {}
///`write(|w| ..)` method takes [`ic_hs_scl_hcnt::W`](W) writer structure
impl crate::Writable for IcHsSclHcntSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets IC_HS_SCL_HCNT to value 0x06
impl crate::Resettable for IcHsSclHcntSpec {
    const RESET_VALUE: u32 = 0x06;
}
