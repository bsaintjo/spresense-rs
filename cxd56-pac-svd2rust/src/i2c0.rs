#[repr(C)]
///Register block
pub struct RegisterBlock {
    ic_con: IcCon,
    ic_tar: IcTar,
    ic_sar: IcSar,
    ic_hs_maddr: IcHsMaddr,
    ic_data_cmd: IcDataCmd,
    ic_ss_scl_hcnt: IcSsSclHcnt,
    ic_ss_scl_lcnt: IcSsSclLcnt,
    ic_fs_scl_hcnt: IcFsSclHcnt,
    ic_fs_scl_lcnt: IcFsSclLcnt,
    ic_hs_scl_hcnt: IcHsSclHcnt,
    ic_hs_scl_lcnt: IcHsSclLcnt,
    ic_intr_stat: IcIntrStat,
    ic_intr_mask: IcIntrMask,
    ic_raw_intr_stat: IcRawIntrStat,
    ic_rx_tl: IcRxTl,
    ic_tx_tl: IcTxTl,
    ic_clr_intr: IcClrIntr,
    ic_clr_rx_under: IcClrRxUnder,
    ic_clr_rx_over: IcClrRxOver,
    ic_clr_tx_over: IcClrTxOver,
    ic_clr_rd_req: IcClrRdReq,
    ic_clr_tx_abrt: IcClrTxAbrt,
    ic_clr_rx_done: IcClrRxDone,
    ic_clr_activity: IcClrActivity,
    ic_clr_stop_det: IcClrStopDet,
    ic_clr_start_det: IcClrStartDet,
    ic_clr_gen_call: IcClrGenCall,
    ic_enable: IcEnable,
    ic_status: IcStatus,
    ic_txflr: IcTxflr,
    ic_rxflr: IcRxflr,
    ic_sda_hold: IcSdaHold,
    ic_tx_abrt_source: IcTxAbrtSource,
    ic_slv_data_nack_only: IcSlvDataNackOnly,
    ic_dma_cr: IcDmaCr,
    ic_dma_tdlr: IcDmaTdlr,
    ic_dma_rdlr: IcDmaRdlr,
    ic_sda_setup: IcSdaSetup,
    ic_ack_general_call: IcAckGeneralCall,
    ic_enable_status: IcEnableStatus,
    ic_fs_spklen: IcFsSpklen,
    ic_hs_spklen: IcHsSpklen,
    _reserved42: [u8; 0x4c],
    ic_comp_param_1: IcCompParam1,
    ic_comp_version: IcCompVersion,
    ic_comp_type: IcCompType,
}
impl RegisterBlock {
    ///0x00 - I2C control register
    #[inline(always)]
    pub const fn ic_con(&self) -> &IcCon {
        &self.ic_con
    }
    ///0x04 - I2C target address register
    #[inline(always)]
    pub const fn ic_tar(&self) -> &IcTar {
        &self.ic_tar
    }
    ///0x08 - I2C slave address register
    #[inline(always)]
    pub const fn ic_sar(&self) -> &IcSar {
        &self.ic_sar
    }
    ///0x0c - I2C high speed master mode code address register
    #[inline(always)]
    pub const fn ic_hs_maddr(&self) -> &IcHsMaddr {
        &self.ic_hs_maddr
    }
    ///0x10 - I2C Rx/Tx data buffer and command register
    #[inline(always)]
    pub const fn ic_data_cmd(&self) -> &IcDataCmd {
        &self.ic_data_cmd
    }
    ///0x14 - Standard speed SCL high period count
    #[inline(always)]
    pub const fn ic_ss_scl_hcnt(&self) -> &IcSsSclHcnt {
        &self.ic_ss_scl_hcnt
    }
    ///0x18 - Standard speed SCL low period count
    #[inline(always)]
    pub const fn ic_ss_scl_lcnt(&self) -> &IcSsSclLcnt {
        &self.ic_ss_scl_lcnt
    }
    ///0x1c - Fast speed SCL high period count
    #[inline(always)]
    pub const fn ic_fs_scl_hcnt(&self) -> &IcFsSclHcnt {
        &self.ic_fs_scl_hcnt
    }
    ///0x20 - Fast speed SCL low period count
    #[inline(always)]
    pub const fn ic_fs_scl_lcnt(&self) -> &IcFsSclLcnt {
        &self.ic_fs_scl_lcnt
    }
    ///0x24 - High speed SCL high period count
    #[inline(always)]
    pub const fn ic_hs_scl_hcnt(&self) -> &IcHsSclHcnt {
        &self.ic_hs_scl_hcnt
    }
    ///0x28 - High speed SCL low period count
    #[inline(always)]
    pub const fn ic_hs_scl_lcnt(&self) -> &IcHsSclLcnt {
        &self.ic_hs_scl_lcnt
    }
    ///0x2c - I2C interrupt status register (read-only, masked)
    #[inline(always)]
    pub const fn ic_intr_stat(&self) -> &IcIntrStat {
        &self.ic_intr_stat
    }
    ///0x30 - I2C interrupt mask register (1 = unmasked)
    #[inline(always)]
    pub const fn ic_intr_mask(&self) -> &IcIntrMask {
        &self.ic_intr_mask
    }
    ///0x34 - I2C raw interrupt status register (read-only, unmasked)
    #[inline(always)]
    pub const fn ic_raw_intr_stat(&self) -> &IcRawIntrStat {
        &self.ic_raw_intr_stat
    }
    ///0x38 - I2C receive FIFO threshold register
    #[inline(always)]
    pub const fn ic_rx_tl(&self) -> &IcRxTl {
        &self.ic_rx_tl
    }
    ///0x3c - I2C transmit FIFO threshold register
    #[inline(always)]
    pub const fn ic_tx_tl(&self) -> &IcTxTl {
        &self.ic_tx_tl
    }
    ///0x40 - Clear combined interrupt (read-to-clear)
    #[inline(always)]
    pub const fn ic_clr_intr(&self) -> &IcClrIntr {
        &self.ic_clr_intr
    }
    ///0x44 - Clear RX_UNDER interrupt (read-to-clear)
    #[inline(always)]
    pub const fn ic_clr_rx_under(&self) -> &IcClrRxUnder {
        &self.ic_clr_rx_under
    }
    ///0x48 - Clear RX_OVER interrupt (read-to-clear)
    #[inline(always)]
    pub const fn ic_clr_rx_over(&self) -> &IcClrRxOver {
        &self.ic_clr_rx_over
    }
    ///0x4c - Clear TX_OVER interrupt (read-to-clear)
    #[inline(always)]
    pub const fn ic_clr_tx_over(&self) -> &IcClrTxOver {
        &self.ic_clr_tx_over
    }
    ///0x50 - Clear RD_REQ interrupt (read-to-clear)
    #[inline(always)]
    pub const fn ic_clr_rd_req(&self) -> &IcClrRdReq {
        &self.ic_clr_rd_req
    }
    ///0x54 - Clear TX_ABRT interrupt (read-to-clear)
    #[inline(always)]
    pub const fn ic_clr_tx_abrt(&self) -> &IcClrTxAbrt {
        &self.ic_clr_tx_abrt
    }
    ///0x58 - Clear RX_DONE interrupt (read-to-clear)
    #[inline(always)]
    pub const fn ic_clr_rx_done(&self) -> &IcClrRxDone {
        &self.ic_clr_rx_done
    }
    ///0x5c - Clear ACTIVITY interrupt (read-to-clear)
    #[inline(always)]
    pub const fn ic_clr_activity(&self) -> &IcClrActivity {
        &self.ic_clr_activity
    }
    ///0x60 - Clear STOP_DET interrupt (read-to-clear)
    #[inline(always)]
    pub const fn ic_clr_stop_det(&self) -> &IcClrStopDet {
        &self.ic_clr_stop_det
    }
    ///0x64 - Clear START_DET interrupt (read-to-clear)
    #[inline(always)]
    pub const fn ic_clr_start_det(&self) -> &IcClrStartDet {
        &self.ic_clr_start_det
    }
    ///0x68 - Clear GEN_CALL interrupt (read-to-clear)
    #[inline(always)]
    pub const fn ic_clr_gen_call(&self) -> &IcClrGenCall {
        &self.ic_clr_gen_call
    }
    ///0x6c - I2C enable register
    #[inline(always)]
    pub const fn ic_enable(&self) -> &IcEnable {
        &self.ic_enable
    }
    ///0x70 - I2C status register (read-only)
    #[inline(always)]
    pub const fn ic_status(&self) -> &IcStatus {
        &self.ic_status
    }
    ///0x74 - I2C transmit FIFO level (read-only)
    #[inline(always)]
    pub const fn ic_txflr(&self) -> &IcTxflr {
        &self.ic_txflr
    }
    ///0x78 - I2C receive FIFO level (read-only)
    #[inline(always)]
    pub const fn ic_rxflr(&self) -> &IcRxflr {
        &self.ic_rxflr
    }
    ///0x7c - I2C SDA hold time length register
    #[inline(always)]
    pub const fn ic_sda_hold(&self) -> &IcSdaHold {
        &self.ic_sda_hold
    }
    ///0x80 - I2C transmit abort source register (read-only)
    #[inline(always)]
    pub const fn ic_tx_abrt_source(&self) -> &IcTxAbrtSource {
        &self.ic_tx_abrt_source
    }
    ///0x84 - Generate NACK for data bytes in slave mode
    #[inline(always)]
    pub const fn ic_slv_data_nack_only(&self) -> &IcSlvDataNackOnly {
        &self.ic_slv_data_nack_only
    }
    ///0x88 - I2C DMA control register
    #[inline(always)]
    pub const fn ic_dma_cr(&self) -> &IcDmaCr {
        &self.ic_dma_cr
    }
    ///0x8c - I2C DMA transmit data level register
    #[inline(always)]
    pub const fn ic_dma_tdlr(&self) -> &IcDmaTdlr {
        &self.ic_dma_tdlr
    }
    ///0x90 - I2C DMA receive data level register
    #[inline(always)]
    pub const fn ic_dma_rdlr(&self) -> &IcDmaRdlr {
        &self.ic_dma_rdlr
    }
    ///0x94 - I2C SDA setup time register
    #[inline(always)]
    pub const fn ic_sda_setup(&self) -> &IcSdaSetup {
        &self.ic_sda_setup
    }
    ///0x98 - I2C ACK general call register
    #[inline(always)]
    pub const fn ic_ack_general_call(&self) -> &IcAckGeneralCall {
        &self.ic_ack_general_call
    }
    ///0x9c - I2C enable status register (read-only)
    #[inline(always)]
    pub const fn ic_enable_status(&self) -> &IcEnableStatus {
        &self.ic_enable_status
    }
    ///0xa0 - I2C SS/FS spike suppression limit register
    #[inline(always)]
    pub const fn ic_fs_spklen(&self) -> &IcFsSpklen {
        &self.ic_fs_spklen
    }
    ///0xa4 - I2C HS spike suppression limit register
    #[inline(always)]
    pub const fn ic_hs_spklen(&self) -> &IcHsSpklen {
        &self.ic_hs_spklen
    }
    ///0xf4 - Component parameter 1 (read-only, configuration encoded at synthesis)
    #[inline(always)]
    pub const fn ic_comp_param_1(&self) -> &IcCompParam1 {
        &self.ic_comp_param_1
    }
    ///0xf8 - Component version register (read-only)
    #[inline(always)]
    pub const fn ic_comp_version(&self) -> &IcCompVersion {
        &self.ic_comp_version
    }
    ///0xfc - Component type register (read-only, 0x44570140 = DW_apb_i2c)
    #[inline(always)]
    pub const fn ic_comp_type(&self) -> &IcCompType {
        &self.ic_comp_type
    }
}
/**IC_CON (rw) register accessor: I2C control register

You can [`read`](crate::Reg::read) this register and get [`ic_con::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ic_con::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@ic_con`] module*/
#[doc(alias = "IC_CON")]
pub type IcCon = crate::Reg<ic_con::IcConSpec>;
///I2C control register
pub mod ic_con;
/**IC_TAR (rw) register accessor: I2C target address register

You can [`read`](crate::Reg::read) this register and get [`ic_tar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ic_tar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@ic_tar`] module*/
#[doc(alias = "IC_TAR")]
pub type IcTar = crate::Reg<ic_tar::IcTarSpec>;
///I2C target address register
pub mod ic_tar;
/**IC_SAR (rw) register accessor: I2C slave address register

You can [`read`](crate::Reg::read) this register and get [`ic_sar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ic_sar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@ic_sar`] module*/
#[doc(alias = "IC_SAR")]
pub type IcSar = crate::Reg<ic_sar::IcSarSpec>;
///I2C slave address register
pub mod ic_sar;
/**IC_HS_MADDR (rw) register accessor: I2C high speed master mode code address register

You can [`read`](crate::Reg::read) this register and get [`ic_hs_maddr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ic_hs_maddr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@ic_hs_maddr`] module*/
#[doc(alias = "IC_HS_MADDR")]
pub type IcHsMaddr = crate::Reg<ic_hs_maddr::IcHsMaddrSpec>;
///I2C high speed master mode code address register
pub mod ic_hs_maddr;
/**IC_DATA_CMD (rw) register accessor: I2C Rx/Tx data buffer and command register

You can [`read`](crate::Reg::read) this register and get [`ic_data_cmd::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ic_data_cmd::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@ic_data_cmd`] module*/
#[doc(alias = "IC_DATA_CMD")]
pub type IcDataCmd = crate::Reg<ic_data_cmd::IcDataCmdSpec>;
///I2C Rx/Tx data buffer and command register
pub mod ic_data_cmd;
/**IC_SS_SCL_HCNT (rw) register accessor: Standard speed SCL high period count

You can [`read`](crate::Reg::read) this register and get [`ic_ss_scl_hcnt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ic_ss_scl_hcnt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@ic_ss_scl_hcnt`] module*/
#[doc(alias = "IC_SS_SCL_HCNT")]
pub type IcSsSclHcnt = crate::Reg<ic_ss_scl_hcnt::IcSsSclHcntSpec>;
///Standard speed SCL high period count
pub mod ic_ss_scl_hcnt;
/**IC_SS_SCL_LCNT (rw) register accessor: Standard speed SCL low period count

You can [`read`](crate::Reg::read) this register and get [`ic_ss_scl_lcnt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ic_ss_scl_lcnt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@ic_ss_scl_lcnt`] module*/
#[doc(alias = "IC_SS_SCL_LCNT")]
pub type IcSsSclLcnt = crate::Reg<ic_ss_scl_lcnt::IcSsSclLcntSpec>;
///Standard speed SCL low period count
pub mod ic_ss_scl_lcnt;
/**IC_FS_SCL_HCNT (rw) register accessor: Fast speed SCL high period count

You can [`read`](crate::Reg::read) this register and get [`ic_fs_scl_hcnt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ic_fs_scl_hcnt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@ic_fs_scl_hcnt`] module*/
#[doc(alias = "IC_FS_SCL_HCNT")]
pub type IcFsSclHcnt = crate::Reg<ic_fs_scl_hcnt::IcFsSclHcntSpec>;
///Fast speed SCL high period count
pub mod ic_fs_scl_hcnt;
/**IC_FS_SCL_LCNT (rw) register accessor: Fast speed SCL low period count

You can [`read`](crate::Reg::read) this register and get [`ic_fs_scl_lcnt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ic_fs_scl_lcnt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@ic_fs_scl_lcnt`] module*/
#[doc(alias = "IC_FS_SCL_LCNT")]
pub type IcFsSclLcnt = crate::Reg<ic_fs_scl_lcnt::IcFsSclLcntSpec>;
///Fast speed SCL low period count
pub mod ic_fs_scl_lcnt;
/**IC_HS_SCL_HCNT (rw) register accessor: High speed SCL high period count

You can [`read`](crate::Reg::read) this register and get [`ic_hs_scl_hcnt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ic_hs_scl_hcnt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@ic_hs_scl_hcnt`] module*/
#[doc(alias = "IC_HS_SCL_HCNT")]
pub type IcHsSclHcnt = crate::Reg<ic_hs_scl_hcnt::IcHsSclHcntSpec>;
///High speed SCL high period count
pub mod ic_hs_scl_hcnt;
/**IC_HS_SCL_LCNT (rw) register accessor: High speed SCL low period count

You can [`read`](crate::Reg::read) this register and get [`ic_hs_scl_lcnt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ic_hs_scl_lcnt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@ic_hs_scl_lcnt`] module*/
#[doc(alias = "IC_HS_SCL_LCNT")]
pub type IcHsSclLcnt = crate::Reg<ic_hs_scl_lcnt::IcHsSclLcntSpec>;
///High speed SCL low period count
pub mod ic_hs_scl_lcnt;
/**IC_INTR_STAT (r) register accessor: I2C interrupt status register (read-only, masked)

You can [`read`](crate::Reg::read) this register and get [`ic_intr_stat::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@ic_intr_stat`] module*/
#[doc(alias = "IC_INTR_STAT")]
pub type IcIntrStat = crate::Reg<ic_intr_stat::IcIntrStatSpec>;
///I2C interrupt status register (read-only, masked)
pub mod ic_intr_stat;
/**IC_INTR_MASK (rw) register accessor: I2C interrupt mask register (1 = unmasked)

You can [`read`](crate::Reg::read) this register and get [`ic_intr_mask::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ic_intr_mask::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@ic_intr_mask`] module*/
#[doc(alias = "IC_INTR_MASK")]
pub type IcIntrMask = crate::Reg<ic_intr_mask::IcIntrMaskSpec>;
///I2C interrupt mask register (1 = unmasked)
pub mod ic_intr_mask;
/**IC_RAW_INTR_STAT (r) register accessor: I2C raw interrupt status register (read-only, unmasked)

You can [`read`](crate::Reg::read) this register and get [`ic_raw_intr_stat::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@ic_raw_intr_stat`] module*/
#[doc(alias = "IC_RAW_INTR_STAT")]
pub type IcRawIntrStat = crate::Reg<ic_raw_intr_stat::IcRawIntrStatSpec>;
///I2C raw interrupt status register (read-only, unmasked)
pub mod ic_raw_intr_stat;
/**IC_RX_TL (rw) register accessor: I2C receive FIFO threshold register

You can [`read`](crate::Reg::read) this register and get [`ic_rx_tl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ic_rx_tl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@ic_rx_tl`] module*/
#[doc(alias = "IC_RX_TL")]
pub type IcRxTl = crate::Reg<ic_rx_tl::IcRxTlSpec>;
///I2C receive FIFO threshold register
pub mod ic_rx_tl;
/**IC_TX_TL (rw) register accessor: I2C transmit FIFO threshold register

You can [`read`](crate::Reg::read) this register and get [`ic_tx_tl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ic_tx_tl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@ic_tx_tl`] module*/
#[doc(alias = "IC_TX_TL")]
pub type IcTxTl = crate::Reg<ic_tx_tl::IcTxTlSpec>;
///I2C transmit FIFO threshold register
pub mod ic_tx_tl;
/**IC_CLR_INTR (r) register accessor: Clear combined interrupt (read-to-clear)

You can [`read`](crate::Reg::read) this register and get [`ic_clr_intr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@ic_clr_intr`] module*/
#[doc(alias = "IC_CLR_INTR")]
pub type IcClrIntr = crate::Reg<ic_clr_intr::IcClrIntrSpec>;
///Clear combined interrupt (read-to-clear)
pub mod ic_clr_intr;
/**IC_CLR_RX_UNDER (r) register accessor: Clear RX_UNDER interrupt (read-to-clear)

You can [`read`](crate::Reg::read) this register and get [`ic_clr_rx_under::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@ic_clr_rx_under`] module*/
#[doc(alias = "IC_CLR_RX_UNDER")]
pub type IcClrRxUnder = crate::Reg<ic_clr_rx_under::IcClrRxUnderSpec>;
///Clear RX_UNDER interrupt (read-to-clear)
pub mod ic_clr_rx_under;
/**IC_CLR_RX_OVER (r) register accessor: Clear RX_OVER interrupt (read-to-clear)

You can [`read`](crate::Reg::read) this register and get [`ic_clr_rx_over::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@ic_clr_rx_over`] module*/
#[doc(alias = "IC_CLR_RX_OVER")]
pub type IcClrRxOver = crate::Reg<ic_clr_rx_over::IcClrRxOverSpec>;
///Clear RX_OVER interrupt (read-to-clear)
pub mod ic_clr_rx_over;
/**IC_CLR_TX_OVER (r) register accessor: Clear TX_OVER interrupt (read-to-clear)

You can [`read`](crate::Reg::read) this register and get [`ic_clr_tx_over::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@ic_clr_tx_over`] module*/
#[doc(alias = "IC_CLR_TX_OVER")]
pub type IcClrTxOver = crate::Reg<ic_clr_tx_over::IcClrTxOverSpec>;
///Clear TX_OVER interrupt (read-to-clear)
pub mod ic_clr_tx_over;
/**IC_CLR_RD_REQ (r) register accessor: Clear RD_REQ interrupt (read-to-clear)

You can [`read`](crate::Reg::read) this register and get [`ic_clr_rd_req::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@ic_clr_rd_req`] module*/
#[doc(alias = "IC_CLR_RD_REQ")]
pub type IcClrRdReq = crate::Reg<ic_clr_rd_req::IcClrRdReqSpec>;
///Clear RD_REQ interrupt (read-to-clear)
pub mod ic_clr_rd_req;
/**IC_CLR_TX_ABRT (r) register accessor: Clear TX_ABRT interrupt (read-to-clear)

You can [`read`](crate::Reg::read) this register and get [`ic_clr_tx_abrt::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@ic_clr_tx_abrt`] module*/
#[doc(alias = "IC_CLR_TX_ABRT")]
pub type IcClrTxAbrt = crate::Reg<ic_clr_tx_abrt::IcClrTxAbrtSpec>;
///Clear TX_ABRT interrupt (read-to-clear)
pub mod ic_clr_tx_abrt;
/**IC_CLR_RX_DONE (r) register accessor: Clear RX_DONE interrupt (read-to-clear)

You can [`read`](crate::Reg::read) this register and get [`ic_clr_rx_done::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@ic_clr_rx_done`] module*/
#[doc(alias = "IC_CLR_RX_DONE")]
pub type IcClrRxDone = crate::Reg<ic_clr_rx_done::IcClrRxDoneSpec>;
///Clear RX_DONE interrupt (read-to-clear)
pub mod ic_clr_rx_done;
/**IC_CLR_ACTIVITY (r) register accessor: Clear ACTIVITY interrupt (read-to-clear)

You can [`read`](crate::Reg::read) this register and get [`ic_clr_activity::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@ic_clr_activity`] module*/
#[doc(alias = "IC_CLR_ACTIVITY")]
pub type IcClrActivity = crate::Reg<ic_clr_activity::IcClrActivitySpec>;
///Clear ACTIVITY interrupt (read-to-clear)
pub mod ic_clr_activity;
/**IC_CLR_STOP_DET (r) register accessor: Clear STOP_DET interrupt (read-to-clear)

You can [`read`](crate::Reg::read) this register and get [`ic_clr_stop_det::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@ic_clr_stop_det`] module*/
#[doc(alias = "IC_CLR_STOP_DET")]
pub type IcClrStopDet = crate::Reg<ic_clr_stop_det::IcClrStopDetSpec>;
///Clear STOP_DET interrupt (read-to-clear)
pub mod ic_clr_stop_det;
/**IC_CLR_START_DET (r) register accessor: Clear START_DET interrupt (read-to-clear)

You can [`read`](crate::Reg::read) this register and get [`ic_clr_start_det::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@ic_clr_start_det`] module*/
#[doc(alias = "IC_CLR_START_DET")]
pub type IcClrStartDet = crate::Reg<ic_clr_start_det::IcClrStartDetSpec>;
///Clear START_DET interrupt (read-to-clear)
pub mod ic_clr_start_det;
/**IC_CLR_GEN_CALL (r) register accessor: Clear GEN_CALL interrupt (read-to-clear)

You can [`read`](crate::Reg::read) this register and get [`ic_clr_gen_call::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@ic_clr_gen_call`] module*/
#[doc(alias = "IC_CLR_GEN_CALL")]
pub type IcClrGenCall = crate::Reg<ic_clr_gen_call::IcClrGenCallSpec>;
///Clear GEN_CALL interrupt (read-to-clear)
pub mod ic_clr_gen_call;
/**IC_ENABLE (rw) register accessor: I2C enable register

You can [`read`](crate::Reg::read) this register and get [`ic_enable::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ic_enable::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@ic_enable`] module*/
#[doc(alias = "IC_ENABLE")]
pub type IcEnable = crate::Reg<ic_enable::IcEnableSpec>;
///I2C enable register
pub mod ic_enable;
/**IC_STATUS (r) register accessor: I2C status register (read-only)

You can [`read`](crate::Reg::read) this register and get [`ic_status::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@ic_status`] module*/
#[doc(alias = "IC_STATUS")]
pub type IcStatus = crate::Reg<ic_status::IcStatusSpec>;
///I2C status register (read-only)
pub mod ic_status;
/**IC_TXFLR (r) register accessor: I2C transmit FIFO level (read-only)

You can [`read`](crate::Reg::read) this register and get [`ic_txflr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@ic_txflr`] module*/
#[doc(alias = "IC_TXFLR")]
pub type IcTxflr = crate::Reg<ic_txflr::IcTxflrSpec>;
///I2C transmit FIFO level (read-only)
pub mod ic_txflr;
/**IC_RXFLR (r) register accessor: I2C receive FIFO level (read-only)

You can [`read`](crate::Reg::read) this register and get [`ic_rxflr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@ic_rxflr`] module*/
#[doc(alias = "IC_RXFLR")]
pub type IcRxflr = crate::Reg<ic_rxflr::IcRxflrSpec>;
///I2C receive FIFO level (read-only)
pub mod ic_rxflr;
/**IC_SDA_HOLD (rw) register accessor: I2C SDA hold time length register

You can [`read`](crate::Reg::read) this register and get [`ic_sda_hold::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ic_sda_hold::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@ic_sda_hold`] module*/
#[doc(alias = "IC_SDA_HOLD")]
pub type IcSdaHold = crate::Reg<ic_sda_hold::IcSdaHoldSpec>;
///I2C SDA hold time length register
pub mod ic_sda_hold;
/**IC_TX_ABRT_SOURCE (r) register accessor: I2C transmit abort source register (read-only)

You can [`read`](crate::Reg::read) this register and get [`ic_tx_abrt_source::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@ic_tx_abrt_source`] module*/
#[doc(alias = "IC_TX_ABRT_SOURCE")]
pub type IcTxAbrtSource = crate::Reg<ic_tx_abrt_source::IcTxAbrtSourceSpec>;
///I2C transmit abort source register (read-only)
pub mod ic_tx_abrt_source;
/**IC_SLV_DATA_NACK_ONLY (rw) register accessor: Generate NACK for data bytes in slave mode

You can [`read`](crate::Reg::read) this register and get [`ic_slv_data_nack_only::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ic_slv_data_nack_only::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@ic_slv_data_nack_only`] module*/
#[doc(alias = "IC_SLV_DATA_NACK_ONLY")]
pub type IcSlvDataNackOnly = crate::Reg<ic_slv_data_nack_only::IcSlvDataNackOnlySpec>;
///Generate NACK for data bytes in slave mode
pub mod ic_slv_data_nack_only;
/**IC_DMA_CR (rw) register accessor: I2C DMA control register

You can [`read`](crate::Reg::read) this register and get [`ic_dma_cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ic_dma_cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@ic_dma_cr`] module*/
#[doc(alias = "IC_DMA_CR")]
pub type IcDmaCr = crate::Reg<ic_dma_cr::IcDmaCrSpec>;
///I2C DMA control register
pub mod ic_dma_cr;
/**IC_DMA_TDLR (rw) register accessor: I2C DMA transmit data level register

You can [`read`](crate::Reg::read) this register and get [`ic_dma_tdlr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ic_dma_tdlr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@ic_dma_tdlr`] module*/
#[doc(alias = "IC_DMA_TDLR")]
pub type IcDmaTdlr = crate::Reg<ic_dma_tdlr::IcDmaTdlrSpec>;
///I2C DMA transmit data level register
pub mod ic_dma_tdlr;
/**IC_DMA_RDLR (rw) register accessor: I2C DMA receive data level register

You can [`read`](crate::Reg::read) this register and get [`ic_dma_rdlr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ic_dma_rdlr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@ic_dma_rdlr`] module*/
#[doc(alias = "IC_DMA_RDLR")]
pub type IcDmaRdlr = crate::Reg<ic_dma_rdlr::IcDmaRdlrSpec>;
///I2C DMA receive data level register
pub mod ic_dma_rdlr;
/**IC_SDA_SETUP (rw) register accessor: I2C SDA setup time register

You can [`read`](crate::Reg::read) this register and get [`ic_sda_setup::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ic_sda_setup::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@ic_sda_setup`] module*/
#[doc(alias = "IC_SDA_SETUP")]
pub type IcSdaSetup = crate::Reg<ic_sda_setup::IcSdaSetupSpec>;
///I2C SDA setup time register
pub mod ic_sda_setup;
/**IC_ACK_GENERAL_CALL (rw) register accessor: I2C ACK general call register

You can [`read`](crate::Reg::read) this register and get [`ic_ack_general_call::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ic_ack_general_call::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@ic_ack_general_call`] module*/
#[doc(alias = "IC_ACK_GENERAL_CALL")]
pub type IcAckGeneralCall = crate::Reg<ic_ack_general_call::IcAckGeneralCallSpec>;
///I2C ACK general call register
pub mod ic_ack_general_call;
/**IC_ENABLE_STATUS (r) register accessor: I2C enable status register (read-only)

You can [`read`](crate::Reg::read) this register and get [`ic_enable_status::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@ic_enable_status`] module*/
#[doc(alias = "IC_ENABLE_STATUS")]
pub type IcEnableStatus = crate::Reg<ic_enable_status::IcEnableStatusSpec>;
///I2C enable status register (read-only)
pub mod ic_enable_status;
/**IC_FS_SPKLEN (rw) register accessor: I2C SS/FS spike suppression limit register

You can [`read`](crate::Reg::read) this register and get [`ic_fs_spklen::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ic_fs_spklen::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@ic_fs_spklen`] module*/
#[doc(alias = "IC_FS_SPKLEN")]
pub type IcFsSpklen = crate::Reg<ic_fs_spklen::IcFsSpklenSpec>;
///I2C SS/FS spike suppression limit register
pub mod ic_fs_spklen;
/**IC_HS_SPKLEN (rw) register accessor: I2C HS spike suppression limit register

You can [`read`](crate::Reg::read) this register and get [`ic_hs_spklen::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ic_hs_spklen::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@ic_hs_spklen`] module*/
#[doc(alias = "IC_HS_SPKLEN")]
pub type IcHsSpklen = crate::Reg<ic_hs_spklen::IcHsSpklenSpec>;
///I2C HS spike suppression limit register
pub mod ic_hs_spklen;
/**IC_COMP_PARAM_1 (r) register accessor: Component parameter 1 (read-only, configuration encoded at synthesis)

You can [`read`](crate::Reg::read) this register and get [`ic_comp_param_1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@ic_comp_param_1`] module*/
#[doc(alias = "IC_COMP_PARAM_1")]
pub type IcCompParam1 = crate::Reg<ic_comp_param_1::IcCompParam1Spec>;
///Component parameter 1 (read-only, configuration encoded at synthesis)
pub mod ic_comp_param_1;
/**IC_COMP_VERSION (r) register accessor: Component version register (read-only)

You can [`read`](crate::Reg::read) this register and get [`ic_comp_version::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@ic_comp_version`] module*/
#[doc(alias = "IC_COMP_VERSION")]
pub type IcCompVersion = crate::Reg<ic_comp_version::IcCompVersionSpec>;
///Component version register (read-only)
pub mod ic_comp_version;
/**IC_COMP_TYPE (r) register accessor: Component type register (read-only, 0x44570140 = DW_apb_i2c)

You can [`read`](crate::Reg::read) this register and get [`ic_comp_type::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@ic_comp_type`] module*/
#[doc(alias = "IC_COMP_TYPE")]
pub type IcCompType = crate::Reg<ic_comp_type::IcCompTypeSpec>;
///Component type register (read-only, 0x44570140 = DW_apb_i2c)
pub mod ic_comp_type;
