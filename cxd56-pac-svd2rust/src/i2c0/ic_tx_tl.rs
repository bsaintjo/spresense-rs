///Register `IC_TX_TL` reader
pub type R = crate::R<IcTxTlSpec>;
///Register `IC_TX_TL` writer
pub type W = crate::W<IcTxTlSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
/**I2C transmit FIFO threshold register

You can [`read`](crate::Reg::read) this register and get [`ic_tx_tl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ic_tx_tl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct IcTxTlSpec;
impl crate::RegisterSpec for IcTxTlSpec {
    type Ux = u32;
}
///`read()` method returns [`ic_tx_tl::R`](R) reader structure
impl crate::Readable for IcTxTlSpec {}
///`write(|w| ..)` method takes [`ic_tx_tl::W`](W) writer structure
impl crate::Writable for IcTxTlSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets IC_TX_TL to value 0
impl crate::Resettable for IcTxTlSpec {}
