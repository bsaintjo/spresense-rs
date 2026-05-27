///Register `IC_TX_ABRT_SOURCE` reader
pub type R = crate::R<IcTxAbrtSourceSpec>;
///Field `ABRT_7B_ADDR_NOACK` reader - 7-bit address not ACK'd
pub type Abrt7bAddrNoackR = crate::BitReader;
///Field `ABRT_10ADDR1_NOACK` reader - 10-bit address byte 1 not ACK'd
pub type Abrt10addr1NoackR = crate::BitReader;
///Field `ABRT_10ADDR2_NOACK` reader - 10-bit address byte 2 not ACK'd
pub type Abrt10addr2NoackR = crate::BitReader;
///Field `ABRT_TXDATA_NOACK` reader - TX data byte not ACK'd
pub type AbrtTxdataNoackR = crate::BitReader;
///Field `ABRT_GCALL_NOACK` reader - General call not ACK'd by any slave
pub type AbrtGcallNoackR = crate::BitReader;
///Field `ABRT_GCALL_READ` reader - General call with read bit set
pub type AbrtGcallReadR = crate::BitReader;
///Field `ABRT_HS_ACKDET` reader - HS master code ACK detected
pub type AbrtHsAckdetR = crate::BitReader;
///Field `ABRT_SBYTE_ACKDET` reader - START byte ACK detected
pub type AbrtSbyteAckdetR = crate::BitReader;
///Field `ABRT_HS_NORSTRT` reader - HS mode without RESTART
pub type AbrtHsNorstrtR = crate::BitReader;
///Field `ABRT_SBYTE_NORSTRT` reader - START byte without RESTART
pub type AbrtSbyteNorstrtR = crate::BitReader;
///Field `ABRT_10B_RD_NORSTRT` reader - 10-bit read without RESTART
pub type Abrt10bRdNorstrtR = crate::BitReader;
///Field `ABRT_MASTER_DIS` reader - Master disabled during transfer
pub type AbrtMasterDisR = crate::BitReader;
///Field `ABRT_ARB_LOST` reader - Arbitration lost
pub type AbrtArbLostR = crate::BitReader;
///Field `ABRT_SLVFLUSH_TXFIFO` reader - Slave flush TX FIFO on read command
pub type AbrtSlvflushTxfifoR = crate::BitReader;
///Field `ABRT_SLV_ARBLOST` reader - Slave arbitration lost
pub type AbrtSlvArblostR = crate::BitReader;
///Field `ABRT_SLVRD_INTX` reader - Slave read request in TX mode
pub type AbrtSlvrdIntxR = crate::BitReader;
///Field `ABRT_USER_ABRT` reader - User abort
pub type AbrtUserAbrtR = crate::BitReader;
impl R {
    ///Bit 0 - 7-bit address not ACK'd
    #[inline(always)]
    pub fn abrt_7b_addr_noack(&self) -> Abrt7bAddrNoackR {
        Abrt7bAddrNoackR::new((self.bits & 1) != 0)
    }
    ///Bit 1 - 10-bit address byte 1 not ACK'd
    #[inline(always)]
    pub fn abrt_10addr1_noack(&self) -> Abrt10addr1NoackR {
        Abrt10addr1NoackR::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - 10-bit address byte 2 not ACK'd
    #[inline(always)]
    pub fn abrt_10addr2_noack(&self) -> Abrt10addr2NoackR {
        Abrt10addr2NoackR::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - TX data byte not ACK'd
    #[inline(always)]
    pub fn abrt_txdata_noack(&self) -> AbrtTxdataNoackR {
        AbrtTxdataNoackR::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - General call not ACK'd by any slave
    #[inline(always)]
    pub fn abrt_gcall_noack(&self) -> AbrtGcallNoackR {
        AbrtGcallNoackR::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - General call with read bit set
    #[inline(always)]
    pub fn abrt_gcall_read(&self) -> AbrtGcallReadR {
        AbrtGcallReadR::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - HS master code ACK detected
    #[inline(always)]
    pub fn abrt_hs_ackdet(&self) -> AbrtHsAckdetR {
        AbrtHsAckdetR::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - START byte ACK detected
    #[inline(always)]
    pub fn abrt_sbyte_ackdet(&self) -> AbrtSbyteAckdetR {
        AbrtSbyteAckdetR::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - HS mode without RESTART
    #[inline(always)]
    pub fn abrt_hs_norstrt(&self) -> AbrtHsNorstrtR {
        AbrtHsNorstrtR::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - START byte without RESTART
    #[inline(always)]
    pub fn abrt_sbyte_norstrt(&self) -> AbrtSbyteNorstrtR {
        AbrtSbyteNorstrtR::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - 10-bit read without RESTART
    #[inline(always)]
    pub fn abrt_10b_rd_norstrt(&self) -> Abrt10bRdNorstrtR {
        Abrt10bRdNorstrtR::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Master disabled during transfer
    #[inline(always)]
    pub fn abrt_master_dis(&self) -> AbrtMasterDisR {
        AbrtMasterDisR::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - Arbitration lost
    #[inline(always)]
    pub fn abrt_arb_lost(&self) -> AbrtArbLostR {
        AbrtArbLostR::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - Slave flush TX FIFO on read command
    #[inline(always)]
    pub fn abrt_slvflush_txfifo(&self) -> AbrtSlvflushTxfifoR {
        AbrtSlvflushTxfifoR::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - Slave arbitration lost
    #[inline(always)]
    pub fn abrt_slv_arblost(&self) -> AbrtSlvArblostR {
        AbrtSlvArblostR::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - Slave read request in TX mode
    #[inline(always)]
    pub fn abrt_slvrd_intx(&self) -> AbrtSlvrdIntxR {
        AbrtSlvrdIntxR::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - User abort
    #[inline(always)]
    pub fn abrt_user_abrt(&self) -> AbrtUserAbrtR {
        AbrtUserAbrtR::new(((self.bits >> 16) & 1) != 0)
    }
}
/**I2C transmit abort source register (read-only)

You can [`read`](crate::Reg::read) this register and get [`ic_tx_abrt_source::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct IcTxAbrtSourceSpec;
impl crate::RegisterSpec for IcTxAbrtSourceSpec {
    type Ux = u32;
}
///`read()` method returns [`ic_tx_abrt_source::R`](R) reader structure
impl crate::Readable for IcTxAbrtSourceSpec {}
///`reset()` method sets IC_TX_ABRT_SOURCE to value 0
impl crate::Resettable for IcTxAbrtSourceSpec {}
