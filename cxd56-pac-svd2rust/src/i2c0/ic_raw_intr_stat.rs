///Register `IC_RAW_INTR_STAT` reader
pub type R = crate::R<IcRawIntrStatSpec>;
///Field `RX_UNDER` reader - RX FIFO underflow (raw)
pub type RxUnderR = crate::BitReader;
///Field `RX_OVER` reader - RX FIFO overflow (raw)
pub type RxOverR = crate::BitReader;
///Field `RX_FULL` reader - RX FIFO at threshold (raw)
pub type RxFullR = crate::BitReader;
///Field `TX_OVER` reader - TX FIFO overflow (raw)
pub type TxOverR = crate::BitReader;
///Field `TX_EMPTY` reader - TX FIFO empty (raw)
pub type TxEmptyR = crate::BitReader;
///Field `RD_REQ` reader - Read request (raw)
pub type RdReqR = crate::BitReader;
///Field `TX_ABRT` reader - Transmit abort (raw)
pub type TxAbrtR = crate::BitReader;
///Field `RX_DONE` reader - RX done (raw)
pub type RxDoneR = crate::BitReader;
///Field `ACTIVITY` reader - I2C activity (raw)
pub type ActivityR = crate::BitReader;
///Field `STOP_DET` reader - STOP detected (raw)
pub type StopDetR = crate::BitReader;
///Field `START_DET` reader - START detected (raw)
pub type StartDetR = crate::BitReader;
///Field `GEN_CALL` reader - General call received (raw)
pub type GenCallR = crate::BitReader;
///Field `RESTART_DET` reader - RESTART detected (raw)
pub type RestartDetR = crate::BitReader;
///Field `MST_ON_HOLD` reader - Master on hold (raw)
pub type MstOnHoldR = crate::BitReader;
impl R {
    ///Bit 0 - RX FIFO underflow (raw)
    #[inline(always)]
    pub fn rx_under(&self) -> RxUnderR {
        RxUnderR::new((self.bits & 1) != 0)
    }
    ///Bit 1 - RX FIFO overflow (raw)
    #[inline(always)]
    pub fn rx_over(&self) -> RxOverR {
        RxOverR::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - RX FIFO at threshold (raw)
    #[inline(always)]
    pub fn rx_full(&self) -> RxFullR {
        RxFullR::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - TX FIFO overflow (raw)
    #[inline(always)]
    pub fn tx_over(&self) -> TxOverR {
        TxOverR::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - TX FIFO empty (raw)
    #[inline(always)]
    pub fn tx_empty(&self) -> TxEmptyR {
        TxEmptyR::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Read request (raw)
    #[inline(always)]
    pub fn rd_req(&self) -> RdReqR {
        RdReqR::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Transmit abort (raw)
    #[inline(always)]
    pub fn tx_abrt(&self) -> TxAbrtR {
        TxAbrtR::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - RX done (raw)
    #[inline(always)]
    pub fn rx_done(&self) -> RxDoneR {
        RxDoneR::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - I2C activity (raw)
    #[inline(always)]
    pub fn activity(&self) -> ActivityR {
        ActivityR::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - STOP detected (raw)
    #[inline(always)]
    pub fn stop_det(&self) -> StopDetR {
        StopDetR::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - START detected (raw)
    #[inline(always)]
    pub fn start_det(&self) -> StartDetR {
        StartDetR::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - General call received (raw)
    #[inline(always)]
    pub fn gen_call(&self) -> GenCallR {
        GenCallR::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - RESTART detected (raw)
    #[inline(always)]
    pub fn restart_det(&self) -> RestartDetR {
        RestartDetR::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - Master on hold (raw)
    #[inline(always)]
    pub fn mst_on_hold(&self) -> MstOnHoldR {
        MstOnHoldR::new(((self.bits >> 13) & 1) != 0)
    }
}
/**I2C raw interrupt status register (read-only, unmasked)

You can [`read`](crate::Reg::read) this register and get [`ic_raw_intr_stat::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct IcRawIntrStatSpec;
impl crate::RegisterSpec for IcRawIntrStatSpec {
    type Ux = u32;
}
///`read()` method returns [`ic_raw_intr_stat::R`](R) reader structure
impl crate::Readable for IcRawIntrStatSpec {}
///`reset()` method sets IC_RAW_INTR_STAT to value 0
impl crate::Resettable for IcRawIntrStatSpec {}
