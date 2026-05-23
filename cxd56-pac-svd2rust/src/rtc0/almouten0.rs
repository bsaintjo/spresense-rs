///Register `ALMOUTEN0` reader
pub type R = crate::R<Almouten0Spec>;
///Register `ALMOUTEN0` writer
pub type W = crate::W<Almouten0Spec>;
///Field `ALM_EN` reader - Alarm 0 enable
pub type AlmEnR = crate::BitReader;
///Field `ALM_EN` writer - Alarm 0 enable
pub type AlmEnW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ALM_BUSY` reader - Alarm 0 write busy
pub type AlmBusyR = crate::BitReader;
///Field `ALM_BUSY` writer - Alarm 0 write busy
pub type AlmBusyW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ALM_DBG` reader - Alarm 0 debug enable
pub type AlmDbgR = crate::BitReader;
///Field `ALM_DBG` writer - Alarm 0 debug enable
pub type AlmDbgW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ALM_ERREN` reader - Alarm 0 error output enable
pub type AlmErrenR = crate::BitReader;
///Field `ALM_ERREN` writer - Alarm 0 error output enable
pub type AlmErrenW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ALM_ERRDBG` reader - Alarm 0 error debug enable
pub type AlmErrdbgR = crate::BitReader;
///Field `ALM_ERRDBG` writer - Alarm 0 error debug enable
pub type AlmErrdbgW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Alarm 0 enable
    #[inline(always)]
    pub fn alm_en(&self) -> AlmEnR {
        AlmEnR::new((self.bits & 1) != 0)
    }
    ///Bit 8 - Alarm 0 write busy
    #[inline(always)]
    pub fn alm_busy(&self) -> AlmBusyR {
        AlmBusyR::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 15 - Alarm 0 debug enable
    #[inline(always)]
    pub fn alm_dbg(&self) -> AlmDbgR {
        AlmDbgR::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - Alarm 0 error output enable
    #[inline(always)]
    pub fn alm_erren(&self) -> AlmErrenR {
        AlmErrenR::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 31 - Alarm 0 error debug enable
    #[inline(always)]
    pub fn alm_errdbg(&self) -> AlmErrdbgR {
        AlmErrdbgR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - Alarm 0 enable
    #[inline(always)]
    pub fn alm_en(&mut self) -> AlmEnW<'_, Almouten0Spec> {
        AlmEnW::new(self, 0)
    }
    ///Bit 8 - Alarm 0 write busy
    #[inline(always)]
    pub fn alm_busy(&mut self) -> AlmBusyW<'_, Almouten0Spec> {
        AlmBusyW::new(self, 8)
    }
    ///Bit 15 - Alarm 0 debug enable
    #[inline(always)]
    pub fn alm_dbg(&mut self) -> AlmDbgW<'_, Almouten0Spec> {
        AlmDbgW::new(self, 15)
    }
    ///Bit 16 - Alarm 0 error output enable
    #[inline(always)]
    pub fn alm_erren(&mut self) -> AlmErrenW<'_, Almouten0Spec> {
        AlmErrenW::new(self, 16)
    }
    ///Bit 31 - Alarm 0 error debug enable
    #[inline(always)]
    pub fn alm_errdbg(&mut self) -> AlmErrdbgW<'_, Almouten0Spec> {
        AlmErrdbgW::new(self, 31)
    }
}
/**Alarm 0 output enable and busy status

You can [`read`](crate::Reg::read) this register and get [`almouten0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`almouten0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct Almouten0Spec;
impl crate::RegisterSpec for Almouten0Spec {
    type Ux = u32;
}
///`read()` method returns [`almouten0::R`](R) reader structure
impl crate::Readable for Almouten0Spec {}
///`write(|w| ..)` method takes [`almouten0::W`](W) writer structure
impl crate::Writable for Almouten0Spec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets ALMOUTEN0 to value 0
impl crate::Resettable for Almouten0Spec {}
