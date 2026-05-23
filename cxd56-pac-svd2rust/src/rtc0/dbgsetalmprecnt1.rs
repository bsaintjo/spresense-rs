///Register `DBGSETALMPRECNT1` reader
pub type R = crate::R<Dbgsetalmprecnt1Spec>;
///Register `DBGSETALMPRECNT1` writer
pub type W = crate::W<Dbgsetalmprecnt1Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
/**Debug alarm 1 pre-counter compare

You can [`read`](crate::Reg::read) this register and get [`dbgsetalmprecnt1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dbgsetalmprecnt1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct Dbgsetalmprecnt1Spec;
impl crate::RegisterSpec for Dbgsetalmprecnt1Spec {
    type Ux = u32;
}
///`read()` method returns [`dbgsetalmprecnt1::R`](R) reader structure
impl crate::Readable for Dbgsetalmprecnt1Spec {}
///`write(|w| ..)` method takes [`dbgsetalmprecnt1::W`](W) writer structure
impl crate::Writable for Dbgsetalmprecnt1Spec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DBGSETALMPRECNT1 to value 0
impl crate::Resettable for Dbgsetalmprecnt1Spec {}
