///Register `IC_FS_SCL_HCNT` reader
pub type R = crate::R<IcFsSclHcntSpec>;
///Register `IC_FS_SCL_HCNT` writer
pub type W = crate::W<IcFsSclHcntSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
/**Fast speed SCL high period count

You can [`read`](crate::Reg::read) this register and get [`ic_fs_scl_hcnt::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ic_fs_scl_hcnt::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct IcFsSclHcntSpec;
impl crate::RegisterSpec for IcFsSclHcntSpec {
    type Ux = u32;
}
///`read()` method returns [`ic_fs_scl_hcnt::R`](R) reader structure
impl crate::Readable for IcFsSclHcntSpec {}
///`write(|w| ..)` method takes [`ic_fs_scl_hcnt::W`](W) writer structure
impl crate::Writable for IcFsSclHcntSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets IC_FS_SCL_HCNT to value 0x3c
impl crate::Resettable for IcFsSclHcntSpec {
    const RESET_VALUE: u32 = 0x3c;
}
