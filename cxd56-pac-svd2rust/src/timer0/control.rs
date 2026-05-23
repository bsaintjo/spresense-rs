///Register `CONTROL` reader
pub type R = crate::R<ControlSpec>;
///Register `CONTROL` writer
pub type W = crate::W<ControlSpec>;
///Field `MODE` reader - Reload behavior on zero
pub type ModeR = crate::BitReader;
///Field `MODE` writer - Reload behavior on zero
pub type ModeW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SIZE` reader - Counter width
pub type SizeR = crate::BitReader;
///Field `SIZE` writer - Counter width
pub type SizeW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DIV` reader - Input clock prescaler
pub type DivR = crate::FieldReader;
///Field `DIV` writer - Input clock prescaler
pub type DivW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `INTENABLE` reader - Interrupt enable
pub type IntenableR = crate::BitReader;
///Field `INTENABLE` writer - Interrupt enable
pub type IntenableW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PERIODIC` reader - Counter mode
pub type PeriodicR = crate::BitReader;
///Field `PERIODIC` writer - Counter mode
pub type PeriodicW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ENABLE` reader - Timer enable
pub type EnableR = crate::BitReader;
///Field `ENABLE` writer - Timer enable
pub type EnableW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Reload behavior on zero
    #[inline(always)]
    pub fn mode(&self) -> ModeR {
        ModeR::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Counter width
    #[inline(always)]
    pub fn size(&self) -> SizeR {
        SizeR::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bits 2:3 - Input clock prescaler
    #[inline(always)]
    pub fn div(&self) -> DivR {
        DivR::new(((self.bits >> 2) & 3) as u8)
    }
    ///Bit 5 - Interrupt enable
    #[inline(always)]
    pub fn intenable(&self) -> IntenableR {
        IntenableR::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Counter mode
    #[inline(always)]
    pub fn periodic(&self) -> PeriodicR {
        PeriodicR::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Timer enable
    #[inline(always)]
    pub fn enable(&self) -> EnableR {
        EnableR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - Reload behavior on zero
    #[inline(always)]
    pub fn mode(&mut self) -> ModeW<'_, ControlSpec> {
        ModeW::new(self, 0)
    }
    ///Bit 1 - Counter width
    #[inline(always)]
    pub fn size(&mut self) -> SizeW<'_, ControlSpec> {
        SizeW::new(self, 1)
    }
    ///Bits 2:3 - Input clock prescaler
    #[inline(always)]
    pub fn div(&mut self) -> DivW<'_, ControlSpec> {
        DivW::new(self, 2)
    }
    ///Bit 5 - Interrupt enable
    #[inline(always)]
    pub fn intenable(&mut self) -> IntenableW<'_, ControlSpec> {
        IntenableW::new(self, 5)
    }
    ///Bit 6 - Counter mode
    #[inline(always)]
    pub fn periodic(&mut self) -> PeriodicW<'_, ControlSpec> {
        PeriodicW::new(self, 6)
    }
    ///Bit 7 - Timer enable
    #[inline(always)]
    pub fn enable(&mut self) -> EnableW<'_, ControlSpec> {
        EnableW::new(self, 7)
    }
}
/**Timer mode, prescaler, and interrupt control

You can [`read`](crate::Reg::read) this register and get [`control::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`control::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct ControlSpec;
impl crate::RegisterSpec for ControlSpec {
    type Ux = u32;
}
///`read()` method returns [`control::R`](R) reader structure
impl crate::Readable for ControlSpec {}
///`write(|w| ..)` method takes [`control::W`](W) writer structure
impl crate::Writable for ControlSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CONTROL to value 0x20
impl crate::Resettable for ControlSpec {
    const RESET_VALUE: u32 = 0x20;
}
