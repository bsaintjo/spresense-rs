///Register `DBGSETALMPOSTCNT0` reader
pub type R = crate::R<Dbgsetalmpostcnt0Spec>;
///Register `DBGSETALMPOSTCNT0` writer
pub type W = crate::W<Dbgsetalmpostcnt0Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
/**Debug alarm 0 post-counter compare

You can [`read`](crate::Reg::read) this register and get [`dbgsetalmpostcnt0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dbgsetalmpostcnt0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct Dbgsetalmpostcnt0Spec;
impl crate::RegisterSpec for Dbgsetalmpostcnt0Spec {
    type Ux = u32;
}
///`read()` method returns [`dbgsetalmpostcnt0::R`](R) reader structure
impl crate::Readable for Dbgsetalmpostcnt0Spec {}
///`write(|w| ..)` method takes [`dbgsetalmpostcnt0::W`](W) writer structure
impl crate::Writable for Dbgsetalmpostcnt0Spec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DBGSETALMPOSTCNT0 to value 0
impl crate::Resettable for Dbgsetalmpostcnt0Spec {}
