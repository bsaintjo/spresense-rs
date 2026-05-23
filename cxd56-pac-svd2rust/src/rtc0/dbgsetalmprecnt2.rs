///Register `DBGSETALMPRECNT2` reader
pub type R = crate::R<Dbgsetalmprecnt2Spec>;
///Register `DBGSETALMPRECNT2` writer
pub type W = crate::W<Dbgsetalmprecnt2Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
/**Debug alarm 2 pre-counter compare

You can [`read`](crate::Reg::read) this register and get [`dbgsetalmprecnt2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dbgsetalmprecnt2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct Dbgsetalmprecnt2Spec;
impl crate::RegisterSpec for Dbgsetalmprecnt2Spec {
    type Ux = u32;
}
///`read()` method returns [`dbgsetalmprecnt2::R`](R) reader structure
impl crate::Readable for Dbgsetalmprecnt2Spec {}
///`write(|w| ..)` method takes [`dbgsetalmprecnt2::W`](W) writer structure
impl crate::Writable for Dbgsetalmprecnt2Spec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DBGSETALMPRECNT2 to value 0
impl crate::Resettable for Dbgsetalmprecnt2Spec {}
