///Register `ALMCLR` reader
pub type R = crate::R<AlmclrSpec>;
///Register `ALMCLR` writer
pub type W = crate::W<AlmclrSpec>;
///Field `ALM0` reader - Clear alarm 0 flag
pub type Alm0R = crate::BitReader;
///Field `ALM0` writer - Clear alarm 0 flag
pub type Alm0W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ALM1` reader - Clear alarm 1 flag
pub type Alm1R = crate::BitReader;
///Field `ALM1` writer - Clear alarm 1 flag
pub type Alm1W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ALM2` reader - Clear alarm 2 flag
pub type Alm2R = crate::BitReader;
///Field `ALM2` writer - Clear alarm 2 flag
pub type Alm2W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ALM0_ERR` reader - Clear alarm 0 error flag
pub type Alm0ErrR = crate::BitReader;
///Field `ALM0_ERR` writer - Clear alarm 0 error flag
pub type Alm0ErrW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ALM1_ERR` reader - Clear alarm 1 error flag
pub type Alm1ErrR = crate::BitReader;
///Field `ALM1_ERR` writer - Clear alarm 1 error flag
pub type Alm1ErrW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Clear alarm 0 flag
    #[inline(always)]
    pub fn alm0(&self) -> Alm0R {
        Alm0R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Clear alarm 1 flag
    #[inline(always)]
    pub fn alm1(&self) -> Alm1R {
        Alm1R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Clear alarm 2 flag
    #[inline(always)]
    pub fn alm2(&self) -> Alm2R {
        Alm2R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 16 - Clear alarm 0 error flag
    #[inline(always)]
    pub fn alm0_err(&self) -> Alm0ErrR {
        Alm0ErrR::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - Clear alarm 1 error flag
    #[inline(always)]
    pub fn alm1_err(&self) -> Alm1ErrR {
        Alm1ErrR::new(((self.bits >> 17) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - Clear alarm 0 flag
    #[inline(always)]
    pub fn alm0(&mut self) -> Alm0W<'_, AlmclrSpec> {
        Alm0W::new(self, 0)
    }
    ///Bit 1 - Clear alarm 1 flag
    #[inline(always)]
    pub fn alm1(&mut self) -> Alm1W<'_, AlmclrSpec> {
        Alm1W::new(self, 1)
    }
    ///Bit 2 - Clear alarm 2 flag
    #[inline(always)]
    pub fn alm2(&mut self) -> Alm2W<'_, AlmclrSpec> {
        Alm2W::new(self, 2)
    }
    ///Bit 16 - Clear alarm 0 error flag
    #[inline(always)]
    pub fn alm0_err(&mut self) -> Alm0ErrW<'_, AlmclrSpec> {
        Alm0ErrW::new(self, 16)
    }
    ///Bit 17 - Clear alarm 1 error flag
    #[inline(always)]
    pub fn alm1_err(&mut self) -> Alm1ErrW<'_, AlmclrSpec> {
        Alm1ErrW::new(self, 17)
    }
}
/**Alarm flag clear register

You can [`read`](crate::Reg::read) this register and get [`almclr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`almclr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct AlmclrSpec;
impl crate::RegisterSpec for AlmclrSpec {
    type Ux = u32;
}
///`read()` method returns [`almclr::R`](R) reader structure
impl crate::Readable for AlmclrSpec {}
///`write(|w| ..)` method takes [`almclr::W`](W) writer structure
impl crate::Writable for AlmclrSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets ALMCLR to value 0
impl crate::Resettable for AlmclrSpec {}
