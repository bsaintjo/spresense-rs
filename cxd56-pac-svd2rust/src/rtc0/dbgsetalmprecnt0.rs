///Register `DBGSETALMPRECNT0` reader
pub type R = crate::R<Dbgsetalmprecnt0Spec>;
///Register `DBGSETALMPRECNT0` writer
pub type W = crate::W<Dbgsetalmprecnt0Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
/**Debug alarm 0 pre-counter compare

You can [`read`](crate::Reg::read) this register and get [`dbgsetalmprecnt0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dbgsetalmprecnt0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct Dbgsetalmprecnt0Spec;
impl crate::RegisterSpec for Dbgsetalmprecnt0Spec {
    type Ux = u32;
}
///`read()` method returns [`dbgsetalmprecnt0::R`](R) reader structure
impl crate::Readable for Dbgsetalmprecnt0Spec {}
///`write(|w| ..)` method takes [`dbgsetalmprecnt0::W`](W) writer structure
impl crate::Writable for Dbgsetalmprecnt0Spec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DBGSETALMPRECNT0 to value 0
impl crate::Resettable for Dbgsetalmprecnt0Spec {}
