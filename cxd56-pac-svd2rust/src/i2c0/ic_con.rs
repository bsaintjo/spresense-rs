///Register `IC_CON` reader
pub type R = crate::R<IcConSpec>;
///Register `IC_CON` writer
pub type W = crate::W<IcConSpec>;
///Field `MASTER_MODE` reader - Master mode enable
pub type MasterModeR = crate::BitReader;
///Field `MASTER_MODE` writer - Master mode enable
pub type MasterModeW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SPEED` reader - I2C speed mode (1=SS 100k, 2=FS 400k, 3=HS 3.4M)
pub type SpeedR = crate::FieldReader;
///Field `SPEED` writer - I2C speed mode (1=SS 100k, 2=FS 400k, 3=HS 3.4M)
pub type SpeedW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `IC_10BITADDR_SLAVE` reader - 10-bit addressing for slave
pub type Ic10bitaddrSlaveR = crate::BitReader;
///Field `IC_10BITADDR_SLAVE` writer - 10-bit addressing for slave
pub type Ic10bitaddrSlaveW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IC_10BITADDR_MASTER` reader - 10-bit addressing for master transfers
pub type Ic10bitaddrMasterR = crate::BitReader;
///Field `IC_10BITADDR_MASTER` writer - 10-bit addressing for master transfers
pub type Ic10bitaddrMasterW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RESTART_EN` reader - Master restart enable
pub type RestartEnR = crate::BitReader;
///Field `RESTART_EN` writer - Master restart enable
pub type RestartEnW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SLAVE_DISABLE` reader - Slave mode disable (1 = master only)
pub type SlaveDisableR = crate::BitReader;
///Field `SLAVE_DISABLE` writer - Slave mode disable (1 = master only)
pub type SlaveDisableW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `STOP_DET_IFADDRESSED` reader - Issue STOP_DET only when addressed as slave
pub type StopDetIfaddressedR = crate::BitReader;
///Field `STOP_DET_IFADDRESSED` writer - Issue STOP_DET only when addressed as slave
pub type StopDetIfaddressedW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TX_EMPTY_CTRL` reader - Controlled TX_EMPTY interrupt generation
pub type TxEmptyCtrlR = crate::BitReader;
///Field `TX_EMPTY_CTRL` writer - Controlled TX_EMPTY interrupt generation
pub type TxEmptyCtrlW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RX_FIFO_FULL_HLD_CTRL` reader - Hold bus when RX FIFO is full
pub type RxFifoFullHldCtrlR = crate::BitReader;
///Field `RX_FIFO_FULL_HLD_CTRL` writer - Hold bus when RX FIFO is full
pub type RxFifoFullHldCtrlW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Master mode enable
    #[inline(always)]
    pub fn master_mode(&self) -> MasterModeR {
        MasterModeR::new((self.bits & 1) != 0)
    }
    ///Bits 1:2 - I2C speed mode (1=SS 100k, 2=FS 400k, 3=HS 3.4M)
    #[inline(always)]
    pub fn speed(&self) -> SpeedR {
        SpeedR::new(((self.bits >> 1) & 3) as u8)
    }
    ///Bit 3 - 10-bit addressing for slave
    #[inline(always)]
    pub fn ic_10bitaddr_slave(&self) -> Ic10bitaddrSlaveR {
        Ic10bitaddrSlaveR::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - 10-bit addressing for master transfers
    #[inline(always)]
    pub fn ic_10bitaddr_master(&self) -> Ic10bitaddrMasterR {
        Ic10bitaddrMasterR::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Master restart enable
    #[inline(always)]
    pub fn restart_en(&self) -> RestartEnR {
        RestartEnR::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Slave mode disable (1 = master only)
    #[inline(always)]
    pub fn slave_disable(&self) -> SlaveDisableR {
        SlaveDisableR::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Issue STOP_DET only when addressed as slave
    #[inline(always)]
    pub fn stop_det_ifaddressed(&self) -> StopDetIfaddressedR {
        StopDetIfaddressedR::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - Controlled TX_EMPTY interrupt generation
    #[inline(always)]
    pub fn tx_empty_ctrl(&self) -> TxEmptyCtrlR {
        TxEmptyCtrlR::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Hold bus when RX FIFO is full
    #[inline(always)]
    pub fn rx_fifo_full_hld_ctrl(&self) -> RxFifoFullHldCtrlR {
        RxFifoFullHldCtrlR::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - Master mode enable
    #[inline(always)]
    pub fn master_mode(&mut self) -> MasterModeW<'_, IcConSpec> {
        MasterModeW::new(self, 0)
    }
    ///Bits 1:2 - I2C speed mode (1=SS 100k, 2=FS 400k, 3=HS 3.4M)
    #[inline(always)]
    pub fn speed(&mut self) -> SpeedW<'_, IcConSpec> {
        SpeedW::new(self, 1)
    }
    ///Bit 3 - 10-bit addressing for slave
    #[inline(always)]
    pub fn ic_10bitaddr_slave(&mut self) -> Ic10bitaddrSlaveW<'_, IcConSpec> {
        Ic10bitaddrSlaveW::new(self, 3)
    }
    ///Bit 4 - 10-bit addressing for master transfers
    #[inline(always)]
    pub fn ic_10bitaddr_master(&mut self) -> Ic10bitaddrMasterW<'_, IcConSpec> {
        Ic10bitaddrMasterW::new(self, 4)
    }
    ///Bit 5 - Master restart enable
    #[inline(always)]
    pub fn restart_en(&mut self) -> RestartEnW<'_, IcConSpec> {
        RestartEnW::new(self, 5)
    }
    ///Bit 6 - Slave mode disable (1 = master only)
    #[inline(always)]
    pub fn slave_disable(&mut self) -> SlaveDisableW<'_, IcConSpec> {
        SlaveDisableW::new(self, 6)
    }
    ///Bit 7 - Issue STOP_DET only when addressed as slave
    #[inline(always)]
    pub fn stop_det_ifaddressed(&mut self) -> StopDetIfaddressedW<'_, IcConSpec> {
        StopDetIfaddressedW::new(self, 7)
    }
    ///Bit 8 - Controlled TX_EMPTY interrupt generation
    #[inline(always)]
    pub fn tx_empty_ctrl(&mut self) -> TxEmptyCtrlW<'_, IcConSpec> {
        TxEmptyCtrlW::new(self, 8)
    }
    ///Bit 9 - Hold bus when RX FIFO is full
    #[inline(always)]
    pub fn rx_fifo_full_hld_ctrl(&mut self) -> RxFifoFullHldCtrlW<'_, IcConSpec> {
        RxFifoFullHldCtrlW::new(self, 9)
    }
}
/**I2C control register

You can [`read`](crate::Reg::read) this register and get [`ic_con::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ic_con::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct IcConSpec;
impl crate::RegisterSpec for IcConSpec {
    type Ux = u32;
}
///`read()` method returns [`ic_con::R`](R) reader structure
impl crate::Readable for IcConSpec {}
///`write(|w| ..)` method takes [`ic_con::W`](W) writer structure
impl crate::Writable for IcConSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets IC_CON to value 0x7f
impl crate::Resettable for IcConSpec {
    const RESET_VALUE: u32 = 0x7f;
}
