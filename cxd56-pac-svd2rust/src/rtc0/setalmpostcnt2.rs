///Register `SETALMPOSTCNT2` reader
pub type R = crate::R<Setalmpostcnt2Spec>;
///Register `SETALMPOSTCNT2` writer
pub type W = crate::W<Setalmpostcnt2Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
/**Alarm 2 post-counter compare value

You can [`read`](crate::Reg::read) this register and get [`setalmpostcnt2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`setalmpostcnt2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct Setalmpostcnt2Spec;
impl crate::RegisterSpec for Setalmpostcnt2Spec {
    type Ux = u32;
}
///`read()` method returns [`setalmpostcnt2::R`](R) reader structure
impl crate::Readable for Setalmpostcnt2Spec {}
///`write(|w| ..)` method takes [`setalmpostcnt2::W`](W) writer structure
impl crate::Writable for Setalmpostcnt2Spec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SETALMPOSTCNT2 to value 0
impl crate::Resettable for Setalmpostcnt2Spec {}
