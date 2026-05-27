///Register `IC_ENABLE` reader
pub type R = crate::R<IcEnableSpec>;
///Register `IC_ENABLE` writer
pub type W = crate::W<IcEnableSpec>;
///Field `ENABLE` reader - I2C controller enable
pub type EnableR = crate::BitReader;
///Field `ENABLE` writer - I2C controller enable
pub type EnableW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - I2C controller enable
    #[inline(always)]
    pub fn enable(&self) -> EnableR {
        EnableR::new((self.bits & 1) != 0)
    }
}
impl W {
    ///Bit 0 - I2C controller enable
    #[inline(always)]
    pub fn enable(&mut self) -> EnableW<'_, IcEnableSpec> {
        EnableW::new(self, 0)
    }
}
/**I2C enable register

You can [`read`](crate::Reg::read) this register and get [`ic_enable::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ic_enable::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct IcEnableSpec;
impl crate::RegisterSpec for IcEnableSpec {
    type Ux = u32;
}
///`read()` method returns [`ic_enable::R`](R) reader structure
impl crate::Readable for IcEnableSpec {}
///`write(|w| ..)` method takes [`ic_enable::W`](W) writer structure
impl crate::Writable for IcEnableSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets IC_ENABLE to value 0
impl crate::Resettable for IcEnableSpec {}
