///Register `RIS` reader
pub type R = crate::R<RisSpec>;
///Field `TIMER_INTERRUPT` reader - Raw interrupt (set when counter reaches zero)
pub type TimerInterruptR = crate::BitReader;
impl R {
    ///Bit 0 - Raw interrupt (set when counter reaches zero)
    #[inline(always)]
    pub fn timer_interrupt(&self) -> TimerInterruptR {
        TimerInterruptR::new((self.bits & 1) != 0)
    }
}
/**Raw interrupt status

You can [`read`](crate::Reg::read) this register and get [`ris::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct RisSpec;
impl crate::RegisterSpec for RisSpec {
    type Ux = u32;
}
///`read()` method returns [`ris::R`](R) reader structure
impl crate::Readable for RisSpec {}
///`reset()` method sets RIS to value 0
impl crate::Resettable for RisSpec {}
