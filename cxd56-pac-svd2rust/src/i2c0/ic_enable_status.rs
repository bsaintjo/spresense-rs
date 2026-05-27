///Register `IC_ENABLE_STATUS` reader
pub type R = crate::R<IcEnableStatusSpec>;
///Field `IC_EN` reader - I2C enabled (clock-synced reflection of IC_ENABLE.ENABLE)
pub type IcEnR = crate::BitReader;
///Field `SLV_RX_ABORTED` reader - Slave RX aborted during disable
pub type SlvRxAbortedR = crate::BitReader;
///Field `SLV_FIFO_FLUSHED` reader - Slave FIFO flushed after controller was disabled
pub type SlvFifoFlushedR = crate::BitReader;
impl R {
    ///Bit 0 - I2C enabled (clock-synced reflection of IC_ENABLE.ENABLE)
    #[inline(always)]
    pub fn ic_en(&self) -> IcEnR {
        IcEnR::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Slave RX aborted during disable
    #[inline(always)]
    pub fn slv_rx_aborted(&self) -> SlvRxAbortedR {
        SlvRxAbortedR::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Slave FIFO flushed after controller was disabled
    #[inline(always)]
    pub fn slv_fifo_flushed(&self) -> SlvFifoFlushedR {
        SlvFifoFlushedR::new(((self.bits >> 2) & 1) != 0)
    }
}
/**I2C enable status register (read-only)

You can [`read`](crate::Reg::read) this register and get [`ic_enable_status::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct IcEnableStatusSpec;
impl crate::RegisterSpec for IcEnableStatusSpec {
    type Ux = u32;
}
///`read()` method returns [`ic_enable_status::R`](R) reader structure
impl crate::Readable for IcEnableStatusSpec {}
///`reset()` method sets IC_ENABLE_STATUS to value 0
impl crate::Resettable for IcEnableStatusSpec {}
