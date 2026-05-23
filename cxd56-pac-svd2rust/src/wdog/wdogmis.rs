///Register `WDOGMIS` reader
pub type R = crate::R<WdogmisSpec>;
///Field `INT` reader - Masked interrupt status (RAWINT AND INTEN)
pub type IntR = crate::BitReader;
impl R {
    ///Bit 0 - Masked interrupt status (RAWINT AND INTEN)
    #[inline(always)]
    pub fn int(&self) -> IntR {
        IntR::new((self.bits & 1) != 0)
    }
}
/**Watchdog masked interrupt status

You can [`read`](crate::Reg::read) this register and get [`wdogmis::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct WdogmisSpec;
impl crate::RegisterSpec for WdogmisSpec {
    type Ux = u32;
}
///`read()` method returns [`wdogmis::R`](R) reader structure
impl crate::Readable for WdogmisSpec {}
///`reset()` method sets WDOGMIS to value 0
impl crate::Resettable for WdogmisSpec {}
