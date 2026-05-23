///Register `WDOGRIS` reader
pub type R = crate::R<WdogrisSpec>;
///Field `RAWINT` reader - Raw interrupt status
pub type RawintR = crate::BitReader;
impl R {
    ///Bit 0 - Raw interrupt status
    #[inline(always)]
    pub fn rawint(&self) -> RawintR {
        RawintR::new((self.bits & 1) != 0)
    }
}
/**Watchdog raw interrupt status

You can [`read`](crate::Reg::read) this register and get [`wdogris::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct WdogrisSpec;
impl crate::RegisterSpec for WdogrisSpec {
    type Ux = u32;
}
///`read()` method returns [`wdogris::R`](R) reader structure
impl crate::Readable for WdogrisSpec {}
///`reset()` method sets WDOGRIS to value 0
impl crate::Resettable for WdogrisSpec {}
