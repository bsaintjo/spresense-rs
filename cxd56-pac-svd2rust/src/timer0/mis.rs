///Register `MIS` reader
pub type R = crate::R<MisSpec>;
///Field `TIMER_INTERRUPT` reader - Masked interrupt (RIS AND INTENABLE)
pub type TimerInterruptR = crate::BitReader;
impl R {
    ///Bit 0 - Masked interrupt (RIS AND INTENABLE)
    #[inline(always)]
    pub fn timer_interrupt(&self) -> TimerInterruptR {
        TimerInterruptR::new((self.bits & 1) != 0)
    }
}
/**Masked interrupt status

You can [`read`](crate::Reg::read) this register and get [`mis::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct MisSpec;
impl crate::RegisterSpec for MisSpec {
    type Ux = u32;
}
///`read()` method returns [`mis::R`](R) reader structure
impl crate::Readable for MisSpec {}
///`reset()` method sets MIS to value 0
impl crate::Resettable for MisSpec {}
