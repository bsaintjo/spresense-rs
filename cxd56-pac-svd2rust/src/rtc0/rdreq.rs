///Register `RDREQ` reader
pub type R = crate::R<RdreqSpec>;
///Register `RDREQ` writer
pub type W = crate::W<RdreqSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
/**Read request (triggers a counter capture)

You can [`read`](crate::Reg::read) this register and get [`rdreq::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rdreq::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct RdreqSpec;
impl crate::RegisterSpec for RdreqSpec {
    type Ux = u32;
}
///`read()` method returns [`rdreq::R`](R) reader structure
impl crate::Readable for RdreqSpec {}
///`write(|w| ..)` method takes [`rdreq::W`](W) writer structure
impl crate::Writable for RdreqSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets RDREQ to value 0
impl crate::Resettable for RdreqSpec {}
