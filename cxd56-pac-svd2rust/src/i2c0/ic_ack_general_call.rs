///Register `IC_ACK_GENERAL_CALL` reader
pub type R = crate::R<IcAckGeneralCallSpec>;
///Register `IC_ACK_GENERAL_CALL` writer
pub type W = crate::W<IcAckGeneralCallSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
/**I2C ACK general call register

You can [`read`](crate::Reg::read) this register and get [`ic_ack_general_call::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ic_ack_general_call::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct IcAckGeneralCallSpec;
impl crate::RegisterSpec for IcAckGeneralCallSpec {
    type Ux = u32;
}
///`read()` method returns [`ic_ack_general_call::R`](R) reader structure
impl crate::Readable for IcAckGeneralCallSpec {}
///`write(|w| ..)` method takes [`ic_ack_general_call::W`](W) writer structure
impl crate::Writable for IcAckGeneralCallSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets IC_ACK_GENERAL_CALL to value 0x01
impl crate::Resettable for IcAckGeneralCallSpec {
    const RESET_VALUE: u32 = 0x01;
}
