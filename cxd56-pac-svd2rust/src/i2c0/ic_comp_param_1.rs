///Register `IC_COMP_PARAM_1` reader
pub type R = crate::R<IcCompParam1Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
/**Component parameter 1 (read-only, configuration encoded at synthesis)

You can [`read`](crate::Reg::read) this register and get [`ic_comp_param_1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct IcCompParam1Spec;
impl crate::RegisterSpec for IcCompParam1Spec {
    type Ux = u32;
}
///`read()` method returns [`ic_comp_param_1::R`](R) reader structure
impl crate::Readable for IcCompParam1Spec {}
///`reset()` method sets IC_COMP_PARAM_1 to value 0
impl crate::Resettable for IcCompParam1Spec {}
