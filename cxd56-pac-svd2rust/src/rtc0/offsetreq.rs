///Register `OFFSETREQ` reader
pub type R = crate::R<OffsetreqSpec>;
///Register `OFFSETREQ` writer
pub type W = crate::W<OffsetreqSpec>;
///Field `ASET_BUSY` reader - Offset correction write busy
pub type AsetBusyR = crate::BitReader;
///Field `ASET_BUSY` writer - Offset correction write busy
pub type AsetBusyW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 16 - Offset correction write busy
    #[inline(always)]
    pub fn aset_busy(&self) -> AsetBusyR {
        AsetBusyR::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    ///Bit 16 - Offset correction write busy
    #[inline(always)]
    pub fn aset_busy(&mut self) -> AsetBusyW<'_, OffsetreqSpec> {
        AsetBusyW::new(self, 16)
    }
}
/**Frequency offset correction request and busy

You can [`read`](crate::Reg::read) this register and get [`offsetreq::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`offsetreq::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct OffsetreqSpec;
impl crate::RegisterSpec for OffsetreqSpec {
    type Ux = u32;
}
///`read()` method returns [`offsetreq::R`](R) reader structure
impl crate::Readable for OffsetreqSpec {}
///`write(|w| ..)` method takes [`offsetreq::W`](W) writer structure
impl crate::Writable for OffsetreqSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets OFFSETREQ to value 0
impl crate::Resettable for OffsetreqSpec {}
