///Register `SETALMPRECNT1` reader
pub type R = crate::R<Setalmprecnt1Spec>;
///Register `SETALMPRECNT1` writer
pub type W = crate::W<Setalmprecnt1Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
/**Alarm 1 pre-counter compare value

You can [`read`](crate::Reg::read) this register and get [`setalmprecnt1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`setalmprecnt1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct Setalmprecnt1Spec;
impl crate::RegisterSpec for Setalmprecnt1Spec {
    type Ux = u32;
}
///`read()` method returns [`setalmprecnt1::R`](R) reader structure
impl crate::Readable for Setalmprecnt1Spec {}
///`write(|w| ..)` method takes [`setalmprecnt1::W`](W) writer structure
impl crate::Writable for Setalmprecnt1Spec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SETALMPRECNT1 to value 0
impl crate::Resettable for Setalmprecnt1Spec {}
