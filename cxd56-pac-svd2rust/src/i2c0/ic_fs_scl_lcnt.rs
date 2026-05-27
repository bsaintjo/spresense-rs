///Register `IC_FS_SCL_LCNT` reader
pub type R = crate::R<IcFsSclLcntSpec>;
///Register `IC_FS_SCL_LCNT` writer
pub type W = crate::W<IcFsSclLcntSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
/**Fast speed SCL low period count

You can [`read`](crate::Reg::read) this register and get [`ic_fs_scl_lcnt::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ic_fs_scl_lcnt::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct IcFsSclLcntSpec;
impl crate::RegisterSpec for IcFsSclLcntSpec {
    type Ux = u32;
}
///`read()` method returns [`ic_fs_scl_lcnt::R`](R) reader structure
impl crate::Readable for IcFsSclLcntSpec {}
///`write(|w| ..)` method takes [`ic_fs_scl_lcnt::W`](W) writer structure
impl crate::Writable for IcFsSclLcntSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets IC_FS_SCL_LCNT to value 0x82
impl crate::Resettable for IcFsSclLcntSpec {
    const RESET_VALUE: u32 = 0x82;
}
