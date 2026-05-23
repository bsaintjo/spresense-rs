///Register `WRREGREQ` reader
pub type R = crate::R<WrregreqSpec>;
///Register `WRREGREQ` writer
pub type W = crate::W<WrregreqSpec>;
///Field `BUSYA` reader - Counter write A-side busy
pub type BusyaR = crate::BitReader;
///Field `BUSYA` writer - Counter write A-side busy
pub type BusyaW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BUSYB` reader - Counter write B-side busy
pub type BusybR = crate::BitReader;
///Field `BUSYB` writer - Counter write B-side busy
pub type BusybW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Counter write A-side busy
    #[inline(always)]
    pub fn busya(&self) -> BusyaR {
        BusyaR::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Counter write B-side busy
    #[inline(always)]
    pub fn busyb(&self) -> BusybR {
        BusybR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - Counter write A-side busy
    #[inline(always)]
    pub fn busya(&mut self) -> BusyaW<'_, WrregreqSpec> {
        BusyaW::new(self, 0)
    }
    ///Bit 1 - Counter write B-side busy
    #[inline(always)]
    pub fn busyb(&mut self) -> BusybW<'_, WrregreqSpec> {
        BusybW::new(self, 1)
    }
}
/**Counter write request and busy status

You can [`read`](crate::Reg::read) this register and get [`wrregreq::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wrregreq::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct WrregreqSpec;
impl crate::RegisterSpec for WrregreqSpec {
    type Ux = u32;
}
///`read()` method returns [`wrregreq::R`](R) reader structure
impl crate::Readable for WrregreqSpec {}
///`write(|w| ..)` method takes [`wrregreq::W`](W) writer structure
impl crate::Writable for WrregreqSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets WRREGREQ to value 0
impl crate::Resettable for WrregreqSpec {}
