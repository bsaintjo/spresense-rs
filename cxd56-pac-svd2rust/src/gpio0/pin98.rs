///Register `PIN98` reader
pub type R = crate::R<Pin98Spec>;
///Register `PIN98` writer
pub type W = crate::W<Pin98Spec>;
///Field `IN` reader - Sampled pin level (read)
pub type InR = crate::BitReader;
///Field `IN` writer - Sampled pin level (read)
pub type InW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OUT` reader - Output data
pub type OutR = crate::BitReader;
///Field `OUT` writer - Output data
pub type OutW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DIR` reader - Output enable, active-low (0 = drive output, 1 = high-Z input)
pub type DirR = crate::BitReader;
///Field `DIR` writer - Output enable, active-low (0 = drive output, 1 = high-Z input)
pub type DirW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Sampled pin level (read)
    #[inline(always)]
    pub fn in_(&self) -> InR {
        InR::new((self.bits & 1) != 0)
    }
    ///Bit 8 - Output data
    #[inline(always)]
    pub fn out(&self) -> OutR {
        OutR::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 16 - Output enable, active-low (0 = drive output, 1 = high-Z input)
    #[inline(always)]
    pub fn dir(&self) -> DirR {
        DirR::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - Sampled pin level (read)
    #[inline(always)]
    pub fn in_(&mut self) -> InW<'_, Pin98Spec> {
        InW::new(self, 0)
    }
    ///Bit 8 - Output data
    #[inline(always)]
    pub fn out(&mut self) -> OutW<'_, Pin98Spec> {
        OutW::new(self, 8)
    }
    ///Bit 16 - Output enable, active-low (0 = drive output, 1 = high-Z input)
    #[inline(always)]
    pub fn dir(&mut self) -> DirW<'_, Pin98Spec> {
        DirW::new(self, 16)
    }
}
/**GPIO APP pin 98 — I2S1_LRCK / LED1 on Spresense main board

You can [`read`](crate::Reg::read) this register and get [`pin98::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pin98::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct Pin98Spec;
impl crate::RegisterSpec for Pin98Spec {
    type Ux = u32;
}
///`read()` method returns [`pin98::R`](R) reader structure
impl crate::Readable for Pin98Spec {}
///`write(|w| ..)` method takes [`pin98::W`](W) writer structure
impl crate::Writable for Pin98Spec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets PIN98 to value 0x0001_0000
impl crate::Resettable for Pin98Spec {
    const RESET_VALUE: u32 = 0x0001_0000;
}
