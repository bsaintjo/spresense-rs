///Register `ALMOUTEN2` reader
pub type R = crate::R<Almouten2Spec>;
///Register `ALMOUTEN2` writer
pub type W = crate::W<Almouten2Spec>;
///Field `ALM_EN` reader - Alarm 2 enable
pub type AlmEnR = crate::BitReader;
///Field `ALM_EN` writer - Alarm 2 enable
pub type AlmEnW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ALM_BUSY` reader - Alarm 2 write busy
pub type AlmBusyR = crate::BitReader;
///Field `ALM_BUSY` writer - Alarm 2 write busy
pub type AlmBusyW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ALM_DBG` reader - Alarm 2 debug enable
pub type AlmDbgR = crate::BitReader;
///Field `ALM_DBG` writer - Alarm 2 debug enable
pub type AlmDbgW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ALM_ERREN` reader - Alarm 2 error output enable
pub type AlmErrenR = crate::BitReader;
///Field `ALM_ERREN` writer - Alarm 2 error output enable
pub type AlmErrenW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ALM_ERRDBG` reader - Alarm 2 error debug enable
pub type AlmErrdbgR = crate::BitReader;
///Field `ALM_ERRDBG` writer - Alarm 2 error debug enable
pub type AlmErrdbgW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Alarm 2 enable
    #[inline(always)]
    pub fn alm_en(&self) -> AlmEnR {
        AlmEnR::new((self.bits & 1) != 0)
    }
    ///Bit 8 - Alarm 2 write busy
    #[inline(always)]
    pub fn alm_busy(&self) -> AlmBusyR {
        AlmBusyR::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 15 - Alarm 2 debug enable
    #[inline(always)]
    pub fn alm_dbg(&self) -> AlmDbgR {
        AlmDbgR::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - Alarm 2 error output enable
    #[inline(always)]
    pub fn alm_erren(&self) -> AlmErrenR {
        AlmErrenR::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 31 - Alarm 2 error debug enable
    #[inline(always)]
    pub fn alm_errdbg(&self) -> AlmErrdbgR {
        AlmErrdbgR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - Alarm 2 enable
    #[inline(always)]
    pub fn alm_en(&mut self) -> AlmEnW<'_, Almouten2Spec> {
        AlmEnW::new(self, 0)
    }
    ///Bit 8 - Alarm 2 write busy
    #[inline(always)]
    pub fn alm_busy(&mut self) -> AlmBusyW<'_, Almouten2Spec> {
        AlmBusyW::new(self, 8)
    }
    ///Bit 15 - Alarm 2 debug enable
    #[inline(always)]
    pub fn alm_dbg(&mut self) -> AlmDbgW<'_, Almouten2Spec> {
        AlmDbgW::new(self, 15)
    }
    ///Bit 16 - Alarm 2 error output enable
    #[inline(always)]
    pub fn alm_erren(&mut self) -> AlmErrenW<'_, Almouten2Spec> {
        AlmErrenW::new(self, 16)
    }
    ///Bit 31 - Alarm 2 error debug enable
    #[inline(always)]
    pub fn alm_errdbg(&mut self) -> AlmErrdbgW<'_, Almouten2Spec> {
        AlmErrdbgW::new(self, 31)
    }
}
/**Alarm 2 output enable and busy status

You can [`read`](crate::Reg::read) this register and get [`almouten2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`almouten2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct Almouten2Spec;
impl crate::RegisterSpec for Almouten2Spec {
    type Ux = u32;
}
///`read()` method returns [`almouten2::R`](R) reader structure
impl crate::Readable for Almouten2Spec {}
///`write(|w| ..)` method takes [`almouten2::W`](W) writer structure
impl crate::Writable for Almouten2Spec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets ALMOUTEN2 to value 0
impl crate::Resettable for Almouten2Spec {}
