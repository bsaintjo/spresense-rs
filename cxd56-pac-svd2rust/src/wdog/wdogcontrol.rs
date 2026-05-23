///Register `WDOGCONTROL` reader
pub type R = crate::R<WdogcontrolSpec>;
///Register `WDOGCONTROL` writer
pub type W = crate::W<WdogcontrolSpec>;
///Field `INTEN` reader - Interrupt output enable (starts the counter)
pub type IntenR = crate::BitReader;
///Field `INTEN` writer - Interrupt output enable (starts the counter)
pub type IntenW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RESEN` reader - Reset output enable (enables chip reset on second underflow)
pub type ResenR = crate::BitReader;
///Field `RESEN` writer - Reset output enable (enables chip reset on second underflow)
pub type ResenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Interrupt output enable (starts the counter)
    #[inline(always)]
    pub fn inten(&self) -> IntenR {
        IntenR::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Reset output enable (enables chip reset on second underflow)
    #[inline(always)]
    pub fn resen(&self) -> ResenR {
        ResenR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - Interrupt output enable (starts the counter)
    #[inline(always)]
    pub fn inten(&mut self) -> IntenW<'_, WdogcontrolSpec> {
        IntenW::new(self, 0)
    }
    ///Bit 1 - Reset output enable (enables chip reset on second underflow)
    #[inline(always)]
    pub fn resen(&mut self) -> ResenW<'_, WdogcontrolSpec> {
        ResenW::new(self, 1)
    }
}
/**Watchdog control register

You can [`read`](crate::Reg::read) this register and get [`wdogcontrol::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wdogcontrol::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct WdogcontrolSpec;
impl crate::RegisterSpec for WdogcontrolSpec {
    type Ux = u32;
}
///`read()` method returns [`wdogcontrol::R`](R) reader structure
impl crate::Readable for WdogcontrolSpec {}
///`write(|w| ..)` method takes [`wdogcontrol::W`](W) writer structure
impl crate::Writable for WdogcontrolSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets WDOGCONTROL to value 0
impl crate::Resettable for WdogcontrolSpec {}
