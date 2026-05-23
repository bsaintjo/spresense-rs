///Register `WDOGLOCK` reader
pub type R = crate::R<WdoglockSpec>;
///Register `WDOGLOCK` writer
pub type W = crate::W<WdoglockSpec>;
///Field `ACCESS` reader - Register write-access state (0 = unlocked, 1 = locked)
pub type AccessR = crate::BitReader;
///Field `ACCESS` writer - Register write-access state (0 = unlocked, 1 = locked)
pub type AccessW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Register write-access state (0 = unlocked, 1 = locked)
    #[inline(always)]
    pub fn access(&self) -> AccessR {
        AccessR::new((self.bits & 1) != 0)
    }
}
impl W {
    ///Bit 0 - Register write-access state (0 = unlocked, 1 = locked)
    #[inline(always)]
    pub fn access(&mut self) -> AccessW<'_, WdoglockSpec> {
        AccessW::new(self, 0)
    }
}
/**Watchdog lock register (write 0x1ACCE551 to unlock)

You can [`read`](crate::Reg::read) this register and get [`wdoglock::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wdoglock::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct WdoglockSpec;
impl crate::RegisterSpec for WdoglockSpec {
    type Ux = u32;
}
///`read()` method returns [`wdoglock::R`](R) reader structure
impl crate::Readable for WdoglockSpec {}
///`write(|w| ..)` method takes [`wdoglock::W`](W) writer structure
impl crate::Writable for WdoglockSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets WDOGLOCK to value 0
impl crate::Resettable for WdoglockSpec {}
