///Register `SETALMPRECNT2` reader
pub type R = crate::R<Setalmprecnt2Spec>;
///Register `SETALMPRECNT2` writer
pub type W = crate::W<Setalmprecnt2Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
/**Alarm 2 pre-counter compare value

You can [`read`](crate::Reg::read) this register and get [`setalmprecnt2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`setalmprecnt2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct Setalmprecnt2Spec;
impl crate::RegisterSpec for Setalmprecnt2Spec {
    type Ux = u32;
}
///`read()` method returns [`setalmprecnt2::R`](R) reader structure
impl crate::Readable for Setalmprecnt2Spec {}
///`write(|w| ..)` method takes [`setalmprecnt2::W`](W) writer structure
impl crate::Writable for Setalmprecnt2Spec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SETALMPRECNT2 to value 0
impl crate::Resettable for Setalmprecnt2Spec {}
