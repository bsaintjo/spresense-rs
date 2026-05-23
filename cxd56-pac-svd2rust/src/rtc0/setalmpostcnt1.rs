///Register `SETALMPOSTCNT1` reader
pub type R = crate::R<Setalmpostcnt1Spec>;
///Register `SETALMPOSTCNT1` writer
pub type W = crate::W<Setalmpostcnt1Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
/**Alarm 1 post-counter compare value

You can [`read`](crate::Reg::read) this register and get [`setalmpostcnt1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`setalmpostcnt1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct Setalmpostcnt1Spec;
impl crate::RegisterSpec for Setalmpostcnt1Spec {
    type Ux = u32;
}
///`read()` method returns [`setalmpostcnt1::R`](R) reader structure
impl crate::Readable for Setalmpostcnt1Spec {}
///`write(|w| ..)` method takes [`setalmpostcnt1::W`](W) writer structure
impl crate::Writable for Setalmpostcnt1Spec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SETALMPOSTCNT1 to value 0
impl crate::Resettable for Setalmpostcnt1Spec {}
