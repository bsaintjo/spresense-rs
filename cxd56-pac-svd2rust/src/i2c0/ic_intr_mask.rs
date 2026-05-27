///Register `IC_INTR_MASK` reader
pub type R = crate::R<IcIntrMaskSpec>;
///Register `IC_INTR_MASK` writer
pub type W = crate::W<IcIntrMaskSpec>;
///Field `RX_UNDER` reader - Mask RX_UNDER interrupt
pub type RxUnderR = crate::BitReader;
///Field `RX_UNDER` writer - Mask RX_UNDER interrupt
pub type RxUnderW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RX_OVER` reader - Mask RX_OVER interrupt
pub type RxOverR = crate::BitReader;
///Field `RX_OVER` writer - Mask RX_OVER interrupt
pub type RxOverW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RX_FULL` reader - Mask RX_FULL interrupt
pub type RxFullR = crate::BitReader;
///Field `RX_FULL` writer - Mask RX_FULL interrupt
pub type RxFullW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TX_OVER` reader - Mask TX_OVER interrupt
pub type TxOverR = crate::BitReader;
///Field `TX_OVER` writer - Mask TX_OVER interrupt
pub type TxOverW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TX_EMPTY` reader - Mask TX_EMPTY interrupt
pub type TxEmptyR = crate::BitReader;
///Field `TX_EMPTY` writer - Mask TX_EMPTY interrupt
pub type TxEmptyW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RD_REQ` reader - Mask RD_REQ interrupt
pub type RdReqR = crate::BitReader;
///Field `RD_REQ` writer - Mask RD_REQ interrupt
pub type RdReqW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TX_ABRT` reader - Mask TX_ABRT interrupt
pub type TxAbrtR = crate::BitReader;
///Field `TX_ABRT` writer - Mask TX_ABRT interrupt
pub type TxAbrtW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RX_DONE` reader - Mask RX_DONE interrupt
pub type RxDoneR = crate::BitReader;
///Field `RX_DONE` writer - Mask RX_DONE interrupt
pub type RxDoneW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ACTIVITY` reader - Mask ACTIVITY interrupt
pub type ActivityR = crate::BitReader;
///Field `ACTIVITY` writer - Mask ACTIVITY interrupt
pub type ActivityW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `STOP_DET` reader - Mask STOP_DET interrupt
pub type StopDetR = crate::BitReader;
///Field `STOP_DET` writer - Mask STOP_DET interrupt
pub type StopDetW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `START_DET` reader - Mask START_DET interrupt
pub type StartDetR = crate::BitReader;
///Field `START_DET` writer - Mask START_DET interrupt
pub type StartDetW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `GEN_CALL` reader - Mask GEN_CALL interrupt
pub type GenCallR = crate::BitReader;
///Field `GEN_CALL` writer - Mask GEN_CALL interrupt
pub type GenCallW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RESTART_DET` reader - Mask RESTART_DET interrupt
pub type RestartDetR = crate::BitReader;
///Field `RESTART_DET` writer - Mask RESTART_DET interrupt
pub type RestartDetW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MST_ON_HOLD` reader - Mask MST_ON_HOLD interrupt
pub type MstOnHoldR = crate::BitReader;
///Field `MST_ON_HOLD` writer - Mask MST_ON_HOLD interrupt
pub type MstOnHoldW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Mask RX_UNDER interrupt
    #[inline(always)]
    pub fn rx_under(&self) -> RxUnderR {
        RxUnderR::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Mask RX_OVER interrupt
    #[inline(always)]
    pub fn rx_over(&self) -> RxOverR {
        RxOverR::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Mask RX_FULL interrupt
    #[inline(always)]
    pub fn rx_full(&self) -> RxFullR {
        RxFullR::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Mask TX_OVER interrupt
    #[inline(always)]
    pub fn tx_over(&self) -> TxOverR {
        TxOverR::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Mask TX_EMPTY interrupt
    #[inline(always)]
    pub fn tx_empty(&self) -> TxEmptyR {
        TxEmptyR::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Mask RD_REQ interrupt
    #[inline(always)]
    pub fn rd_req(&self) -> RdReqR {
        RdReqR::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Mask TX_ABRT interrupt
    #[inline(always)]
    pub fn tx_abrt(&self) -> TxAbrtR {
        TxAbrtR::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Mask RX_DONE interrupt
    #[inline(always)]
    pub fn rx_done(&self) -> RxDoneR {
        RxDoneR::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - Mask ACTIVITY interrupt
    #[inline(always)]
    pub fn activity(&self) -> ActivityR {
        ActivityR::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Mask STOP_DET interrupt
    #[inline(always)]
    pub fn stop_det(&self) -> StopDetR {
        StopDetR::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Mask START_DET interrupt
    #[inline(always)]
    pub fn start_det(&self) -> StartDetR {
        StartDetR::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Mask GEN_CALL interrupt
    #[inline(always)]
    pub fn gen_call(&self) -> GenCallR {
        GenCallR::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - Mask RESTART_DET interrupt
    #[inline(always)]
    pub fn restart_det(&self) -> RestartDetR {
        RestartDetR::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - Mask MST_ON_HOLD interrupt
    #[inline(always)]
    pub fn mst_on_hold(&self) -> MstOnHoldR {
        MstOnHoldR::new(((self.bits >> 13) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - Mask RX_UNDER interrupt
    #[inline(always)]
    pub fn rx_under(&mut self) -> RxUnderW<'_, IcIntrMaskSpec> {
        RxUnderW::new(self, 0)
    }
    ///Bit 1 - Mask RX_OVER interrupt
    #[inline(always)]
    pub fn rx_over(&mut self) -> RxOverW<'_, IcIntrMaskSpec> {
        RxOverW::new(self, 1)
    }
    ///Bit 2 - Mask RX_FULL interrupt
    #[inline(always)]
    pub fn rx_full(&mut self) -> RxFullW<'_, IcIntrMaskSpec> {
        RxFullW::new(self, 2)
    }
    ///Bit 3 - Mask TX_OVER interrupt
    #[inline(always)]
    pub fn tx_over(&mut self) -> TxOverW<'_, IcIntrMaskSpec> {
        TxOverW::new(self, 3)
    }
    ///Bit 4 - Mask TX_EMPTY interrupt
    #[inline(always)]
    pub fn tx_empty(&mut self) -> TxEmptyW<'_, IcIntrMaskSpec> {
        TxEmptyW::new(self, 4)
    }
    ///Bit 5 - Mask RD_REQ interrupt
    #[inline(always)]
    pub fn rd_req(&mut self) -> RdReqW<'_, IcIntrMaskSpec> {
        RdReqW::new(self, 5)
    }
    ///Bit 6 - Mask TX_ABRT interrupt
    #[inline(always)]
    pub fn tx_abrt(&mut self) -> TxAbrtW<'_, IcIntrMaskSpec> {
        TxAbrtW::new(self, 6)
    }
    ///Bit 7 - Mask RX_DONE interrupt
    #[inline(always)]
    pub fn rx_done(&mut self) -> RxDoneW<'_, IcIntrMaskSpec> {
        RxDoneW::new(self, 7)
    }
    ///Bit 8 - Mask ACTIVITY interrupt
    #[inline(always)]
    pub fn activity(&mut self) -> ActivityW<'_, IcIntrMaskSpec> {
        ActivityW::new(self, 8)
    }
    ///Bit 9 - Mask STOP_DET interrupt
    #[inline(always)]
    pub fn stop_det(&mut self) -> StopDetW<'_, IcIntrMaskSpec> {
        StopDetW::new(self, 9)
    }
    ///Bit 10 - Mask START_DET interrupt
    #[inline(always)]
    pub fn start_det(&mut self) -> StartDetW<'_, IcIntrMaskSpec> {
        StartDetW::new(self, 10)
    }
    ///Bit 11 - Mask GEN_CALL interrupt
    #[inline(always)]
    pub fn gen_call(&mut self) -> GenCallW<'_, IcIntrMaskSpec> {
        GenCallW::new(self, 11)
    }
    ///Bit 12 - Mask RESTART_DET interrupt
    #[inline(always)]
    pub fn restart_det(&mut self) -> RestartDetW<'_, IcIntrMaskSpec> {
        RestartDetW::new(self, 12)
    }
    ///Bit 13 - Mask MST_ON_HOLD interrupt
    #[inline(always)]
    pub fn mst_on_hold(&mut self) -> MstOnHoldW<'_, IcIntrMaskSpec> {
        MstOnHoldW::new(self, 13)
    }
}
/**I2C interrupt mask register (1 = unmasked)

You can [`read`](crate::Reg::read) this register and get [`ic_intr_mask::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ic_intr_mask::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct IcIntrMaskSpec;
impl crate::RegisterSpec for IcIntrMaskSpec {
    type Ux = u32;
}
///`read()` method returns [`ic_intr_mask::R`](R) reader structure
impl crate::Readable for IcIntrMaskSpec {}
///`write(|w| ..)` method takes [`ic_intr_mask::W`](W) writer structure
impl crate::Writable for IcIntrMaskSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets IC_INTR_MASK to value 0x08ff
impl crate::Resettable for IcIntrMaskSpec {
    const RESET_VALUE: u32 = 0x08ff;
}
