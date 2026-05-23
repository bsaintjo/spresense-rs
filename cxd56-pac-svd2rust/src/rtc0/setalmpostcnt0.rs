///Register `SETALMPOSTCNT0` reader
pub type R = crate::R<Setalmpostcnt0Spec>;
///Register `SETALMPOSTCNT0` writer
pub type W = crate::W<Setalmpostcnt0Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
/**Alarm 0 post-counter compare value

You can [`read`](crate::Reg::read) this register and get [`setalmpostcnt0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`setalmpostcnt0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct Setalmpostcnt0Spec;
impl crate::RegisterSpec for Setalmpostcnt0Spec {
    type Ux = u32;
}
///`read()` method returns [`setalmpostcnt0::R`](R) reader structure
impl crate::Readable for Setalmpostcnt0Spec {}
///`write(|w| ..)` method takes [`setalmpostcnt0::W`](W) writer structure
impl crate::Writable for Setalmpostcnt0Spec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SETALMPOSTCNT0 to value 0
impl crate::Resettable for Setalmpostcnt0Spec {}
