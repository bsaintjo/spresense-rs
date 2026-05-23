///Register `DBGSETALMPOSTCNT1` reader
pub type R = crate::R<Dbgsetalmpostcnt1Spec>;
///Register `DBGSETALMPOSTCNT1` writer
pub type W = crate::W<Dbgsetalmpostcnt1Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
/**Debug alarm 1 post-counter compare

You can [`read`](crate::Reg::read) this register and get [`dbgsetalmpostcnt1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dbgsetalmpostcnt1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct Dbgsetalmpostcnt1Spec;
impl crate::RegisterSpec for Dbgsetalmpostcnt1Spec {
    type Ux = u32;
}
///`read()` method returns [`dbgsetalmpostcnt1::R`](R) reader structure
impl crate::Readable for Dbgsetalmpostcnt1Spec {}
///`write(|w| ..)` method takes [`dbgsetalmpostcnt1::W`](W) writer structure
impl crate::Writable for Dbgsetalmpostcnt1Spec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DBGSETALMPOSTCNT1 to value 0
impl crate::Resettable for Dbgsetalmpostcnt1Spec {}
