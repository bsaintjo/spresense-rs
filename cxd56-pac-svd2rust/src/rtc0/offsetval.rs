///Register `OFFSETVAL` reader
pub type R = crate::R<OffsetvalSpec>;
///Register `OFFSETVAL` writer
pub type W = crate::W<OffsetvalSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
/**Frequency offset correction value

You can [`read`](crate::Reg::read) this register and get [`offsetval::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`offsetval::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct OffsetvalSpec;
impl crate::RegisterSpec for OffsetvalSpec {
    type Ux = u32;
}
///`read()` method returns [`offsetval::R`](R) reader structure
impl crate::Readable for OffsetvalSpec {}
///`write(|w| ..)` method takes [`offsetval::W`](W) writer structure
impl crate::Writable for OffsetvalSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets OFFSETVAL to value 0
impl crate::Resettable for OffsetvalSpec {}
