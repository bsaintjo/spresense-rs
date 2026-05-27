///Register `IC_CLR_RD_REQ` reader
pub type R = crate::R<IcClrRdReqSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
/**Clear RD_REQ interrupt (read-to-clear)

You can [`read`](crate::Reg::read) this register and get [`ic_clr_rd_req::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct IcClrRdReqSpec;
impl crate::RegisterSpec for IcClrRdReqSpec {
    type Ux = u32;
}
///`read()` method returns [`ic_clr_rd_req::R`](R) reader structure
impl crate::Readable for IcClrRdReqSpec {}
///`reset()` method sets IC_CLR_RD_REQ to value 0
impl crate::Resettable for IcClrRdReqSpec {}
