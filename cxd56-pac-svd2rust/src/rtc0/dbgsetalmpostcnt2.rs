///Register `DBGSETALMPOSTCNT2` reader
pub type R = crate::R<Dbgsetalmpostcnt2Spec>;
///Register `DBGSETALMPOSTCNT2` writer
pub type W = crate::W<Dbgsetalmpostcnt2Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
/**Debug alarm 2 post-counter compare

You can [`read`](crate::Reg::read) this register and get [`dbgsetalmpostcnt2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dbgsetalmpostcnt2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct Dbgsetalmpostcnt2Spec;
impl crate::RegisterSpec for Dbgsetalmpostcnt2Spec {
    type Ux = u32;
}
///`read()` method returns [`dbgsetalmpostcnt2::R`](R) reader structure
impl crate::Readable for Dbgsetalmpostcnt2Spec {}
///`write(|w| ..)` method takes [`dbgsetalmpostcnt2::W`](W) writer structure
impl crate::Writable for Dbgsetalmpostcnt2Spec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DBGSETALMPOSTCNT2 to value 0
impl crate::Resettable for Dbgsetalmpostcnt2Spec {}
