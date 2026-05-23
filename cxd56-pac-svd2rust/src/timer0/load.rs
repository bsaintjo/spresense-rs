///Register `LOAD` reader
pub type R = crate::R<LoadSpec>;
///Register `LOAD` writer
pub type W = crate::W<LoadSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
/**Counter reload value

You can [`read`](crate::Reg::read) this register and get [`load::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`load::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct LoadSpec;
impl crate::RegisterSpec for LoadSpec {
    type Ux = u32;
}
///`read()` method returns [`load::R`](R) reader structure
impl crate::Readable for LoadSpec {}
///`write(|w| ..)` method takes [`load::W`](W) writer structure
impl crate::Writable for LoadSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets LOAD to value 0
impl crate::Resettable for LoadSpec {}
