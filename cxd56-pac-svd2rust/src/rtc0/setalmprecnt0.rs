///Register `SETALMPRECNT0` reader
pub type R = crate::R<Setalmprecnt0Spec>;
///Register `SETALMPRECNT0` writer
pub type W = crate::W<Setalmprecnt0Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
/**Alarm 0 pre-counter compare value

You can [`read`](crate::Reg::read) this register and get [`setalmprecnt0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`setalmprecnt0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct Setalmprecnt0Spec;
impl crate::RegisterSpec for Setalmprecnt0Spec {
    type Ux = u32;
}
///`read()` method returns [`setalmprecnt0::R`](R) reader structure
impl crate::Readable for Setalmprecnt0Spec {}
///`write(|w| ..)` method takes [`setalmprecnt0::W`](W) writer structure
impl crate::Writable for Setalmprecnt0Spec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SETALMPRECNT0 to value 0
impl crate::Resettable for Setalmprecnt0Spec {}
