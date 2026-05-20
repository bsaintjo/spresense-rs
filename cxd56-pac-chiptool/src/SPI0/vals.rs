#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum BSY {
    ///SSP is idle.
    idle = 0x0,
    ///SSP is currently transmitting and/or receiving a frame or the transmit FIFO is not empty.
    busy = 0x01,
}
impl BSY {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> BSY {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for BSY {
    #[inline(always)]
    fn from(val: u8) -> BSY {
        BSY::from_bits(val)
    }
}
impl From<BSY> for u8 {
    #[inline(always)]
    fn from(val: BSY) -> u8 {
        BSY::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CS_MODE {
    ///CS controled by controller.
    auto = 0x0,
    ///CS controled by CS register.
    manual = 0x01,
}
impl CS_MODE {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CS_MODE {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CS_MODE {
    #[inline(always)]
    fn from(val: u8) -> CS_MODE {
        CS_MODE::from_bits(val)
    }
}
impl From<CS_MODE> for u8 {
    #[inline(always)]
    fn from(val: CS_MODE) -> u8 {
        CS_MODE::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DSS {
    _RESERVED_0 = 0x0,
    _RESERVED_1 = 0x01,
    _RESERVED_2 = 0x02,
    ///4bit data.
    N4bit = 0x03,
    ///5bit data.
    N5bit = 0x04,
    ///6bit data.
    N6bit = 0x05,
    ///7bit data.
    N7bit = 0x06,
    ///8bit data.
    N8bit = 0x07,
    ///9bit data.
    N9bit = 0x08,
    ///10bit data.
    N10bit = 0x09,
    ///11bit data.
    N11bit = 0x0a,
    ///12bit data.
    N12bit = 0x0b,
    ///13bit data.
    N13bit = 0x0c,
    ///14bit data.
    N14bit = 0x0d,
    ///15bit data.
    N15bit = 0x0e,
    ///16bit data.
    N16bit = 0x0f,
}
impl DSS {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DSS {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DSS {
    #[inline(always)]
    fn from(val: u8) -> DSS {
        DSS::from_bits(val)
    }
}
impl From<DSS> for u8 {
    #[inline(always)]
    fn from(val: DSS) -> u8 {
        DSS::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FRF {
    ///Motorola SPI frame format.
    Motorola = 0x0,
    ///TI synchronous serial frame format.
    TI = 0x01,
    ///National Microwire frame format.
    NM = 0x02,
    _RESERVED_3 = 0x03,
}
impl FRF {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FRF {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FRF {
    #[inline(always)]
    fn from(val: u8) -> FRF {
        FRF::from_bits(val)
    }
}
impl From<FRF> for u8 {
    #[inline(always)]
    fn from(val: FRF) -> u8 {
        FRF::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum LBM {
    ///Normal serial port operation enabled.
    disabled = 0x0,
    ///Output of transmit serial shifter is connected to input of recieve serial shifter internally.
    enabled = 0x01,
}
impl LBM {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> LBM {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for LBM {
    #[inline(always)]
    fn from(val: u8) -> LBM {
        LBM::from_bits(val)
    }
}
impl From<LBM> for u8 {
    #[inline(always)]
    fn from(val: LBM) -> u8 {
        LBM::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MS {
    ///Master mode.
    master = 0x0,
    ///Slave mode.
    slave = 0x01,
}
impl MS {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MS {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MS {
    #[inline(always)]
    fn from(val: u8) -> MS {
        MS::from_bits(val)
    }
}
impl From<MS> for u8 {
    #[inline(always)]
    fn from(val: MS) -> u8 {
        MS::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RFF {
    ///Receive FIFO is not full.
    notfull = 0x0,
    ///Recieve FIFO is full.
    full = 0x01,
}
impl RFF {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RFF {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RFF {
    #[inline(always)]
    fn from(val: u8) -> RFF {
        RFF::from_bits(val)
    }
}
impl From<RFF> for u8 {
    #[inline(always)]
    fn from(val: RFF) -> u8 {
        RFF::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RNE {
    ///Receive FIFO is empty.
    empty = 0x0,
    ///Receive FIFO is not empty.
    notempty = 0x01,
}
impl RNE {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RNE {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RNE {
    #[inline(always)]
    fn from(val: u8) -> RNE {
        RNE::from_bits(val)
    }
}
impl From<RNE> for u8 {
    #[inline(always)]
    fn from(val: RNE) -> u8 {
        RNE::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RORIM {
    ///Masked.
    masked = 0x0,
    ///Not masked.
    notmasked = 0x01,
}
impl RORIM {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RORIM {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RORIM {
    #[inline(always)]
    fn from(val: u8) -> RORIM {
        RORIM::from_bits(val)
    }
}
impl From<RORIM> for u8 {
    #[inline(always)]
    fn from(val: RORIM) -> u8 {
        RORIM::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RORMIS {
    ///Masked.
    masked = 0x0,
    ///Not masked.
    notmasked = 0x01,
}
impl RORMIS {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RORMIS {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RORMIS {
    #[inline(always)]
    fn from(val: u8) -> RORMIS {
        RORMIS::from_bits(val)
    }
}
impl From<RORMIS> for u8 {
    #[inline(always)]
    fn from(val: RORMIS) -> u8 {
        RORMIS::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RORRIS {
    ///Masked.
    masked = 0x0,
    ///Not masked.
    notmasked = 0x01,
}
impl RORRIS {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RORRIS {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RORRIS {
    #[inline(always)]
    fn from(val: u8) -> RORRIS {
        RORRIS::from_bits(val)
    }
}
impl From<RORRIS> for u8 {
    #[inline(always)]
    fn from(val: RORRIS) -> u8 {
        RORRIS::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RTIM {
    ///Masked.
    masked = 0x0,
    ///Not masked.
    notmasked = 0x01,
}
impl RTIM {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RTIM {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RTIM {
    #[inline(always)]
    fn from(val: u8) -> RTIM {
        RTIM::from_bits(val)
    }
}
impl From<RTIM> for u8 {
    #[inline(always)]
    fn from(val: RTIM) -> u8 {
        RTIM::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RTMIS {
    ///Masked.
    masked = 0x0,
    ///Not masked.
    notmasked = 0x01,
}
impl RTMIS {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RTMIS {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RTMIS {
    #[inline(always)]
    fn from(val: u8) -> RTMIS {
        RTMIS::from_bits(val)
    }
}
impl From<RTMIS> for u8 {
    #[inline(always)]
    fn from(val: RTMIS) -> u8 {
        RTMIS::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RTRIS {
    ///Masked.
    masked = 0x0,
    ///Not masked.
    notmasked = 0x01,
}
impl RTRIS {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RTRIS {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RTRIS {
    #[inline(always)]
    fn from(val: u8) -> RTRIS {
        RTRIS::from_bits(val)
    }
}
impl From<RTRIS> for u8 {
    #[inline(always)]
    fn from(val: RTRIS) -> u8 {
        RTRIS::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RXDMAE {
    ///Disabled.
    disabled = 0x0,
    ///Enabled.
    enabled = 0x01,
}
impl RXDMAE {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RXDMAE {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RXDMAE {
    #[inline(always)]
    fn from(val: u8) -> RXDMAE {
        RXDMAE::from_bits(val)
    }
}
impl From<RXDMAE> for u8 {
    #[inline(always)]
    fn from(val: RXDMAE) -> u8 {
        RXDMAE::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RXIM {
    ///Masked.
    masked = 0x0,
    ///Not masked.
    notmasked = 0x01,
}
impl RXIM {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RXIM {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RXIM {
    #[inline(always)]
    fn from(val: u8) -> RXIM {
        RXIM::from_bits(val)
    }
}
impl From<RXIM> for u8 {
    #[inline(always)]
    fn from(val: RXIM) -> u8 {
        RXIM::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RXMIS {
    ///Masked.
    masked = 0x0,
    ///Not masked.
    notmasked = 0x01,
}
impl RXMIS {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RXMIS {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RXMIS {
    #[inline(always)]
    fn from(val: u8) -> RXMIS {
        RXMIS::from_bits(val)
    }
}
impl From<RXMIS> for u8 {
    #[inline(always)]
    fn from(val: RXMIS) -> u8 {
        RXMIS::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RXRIS {
    ///Masked.
    masked = 0x0,
    ///Not masked.
    notmasked = 0x01,
}
impl RXRIS {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RXRIS {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RXRIS {
    #[inline(always)]
    fn from(val: u8) -> RXRIS {
        RXRIS::from_bits(val)
    }
}
impl From<RXRIS> for u8 {
    #[inline(always)]
    fn from(val: RXRIS) -> u8 {
        RXRIS::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SLAVE_TYPE {
    ///Select CS0.
    CS0 = 0x0,
    ///Select CS1.
    CS1 = 0x01,
    ///Select CS2.
    CS2 = 0x02,
    _RESERVED_3 = 0x03,
}
impl SLAVE_TYPE {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SLAVE_TYPE {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SLAVE_TYPE {
    #[inline(always)]
    fn from(val: u8) -> SLAVE_TYPE {
        SLAVE_TYPE::from_bits(val)
    }
}
impl From<SLAVE_TYPE> for u8 {
    #[inline(always)]
    fn from(val: SLAVE_TYPE) -> u8 {
        SLAVE_TYPE::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SOD {
    ///SSP can drive the SSPTXD output in slave mode.
    enable = 0x0,
    ///SSP must not drive the SSPTXD output in slave mode.
    disable = 0x01,
}
impl SOD {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SOD {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SOD {
    #[inline(always)]
    fn from(val: u8) -> SOD {
        SOD::from_bits(val)
    }
}
impl From<SOD> for u8 {
    #[inline(always)]
    fn from(val: SOD) -> u8 {
        SOD::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SSE {
    ///SSP operation disabled.
    disabled = 0x0,
    ///SSP operation enabled.
    enabled = 0x01,
}
impl SSE {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SSE {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SSE {
    #[inline(always)]
    fn from(val: u8) -> SSE {
        SSE::from_bits(val)
    }
}
impl From<SSE> for u8 {
    #[inline(always)]
    fn from(val: SSE) -> u8 {
        SSE::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SSP_CS {
    ///Output low to the CS.
    low = 0x0,
    ///Output high to the CS.
    high = 0x01,
}
impl SSP_CS {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SSP_CS {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SSP_CS {
    #[inline(always)]
    fn from(val: u8) -> SSP_CS {
        SSP_CS::from_bits(val)
    }
}
impl From<SSP_CS> for u8 {
    #[inline(always)]
    fn from(val: SSP_CS) -> u8 {
        SSP_CS::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum TFE {
    ///Transmit FIFO is not empty.
    notempty = 0x0,
    ///Transmit FIFO is empty.
    empty = 0x01,
}
impl TFE {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TFE {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TFE {
    #[inline(always)]
    fn from(val: u8) -> TFE {
        TFE::from_bits(val)
    }
}
impl From<TFE> for u8 {
    #[inline(always)]
    fn from(val: TFE) -> u8 {
        TFE::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum TNF {
    ///Transmit FIFO is full.
    full = 0x0,
    ///Transmit FIFO is not full.
    notfull = 0x01,
}
impl TNF {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TNF {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TNF {
    #[inline(always)]
    fn from(val: u8) -> TNF {
        TNF::from_bits(val)
    }
}
impl From<TNF> for u8 {
    #[inline(always)]
    fn from(val: TNF) -> u8 {
        TNF::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum TXDMAE {
    ///Disabled.
    disabled = 0x0,
    ///Enabled.
    enabled = 0x01,
}
impl TXDMAE {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TXDMAE {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TXDMAE {
    #[inline(always)]
    fn from(val: u8) -> TXDMAE {
        TXDMAE::from_bits(val)
    }
}
impl From<TXDMAE> for u8 {
    #[inline(always)]
    fn from(val: TXDMAE) -> u8 {
        TXDMAE::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum TXIM {
    ///Masked.
    masked = 0x0,
    ///Not masked.
    notmasked = 0x01,
}
impl TXIM {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TXIM {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TXIM {
    #[inline(always)]
    fn from(val: u8) -> TXIM {
        TXIM::from_bits(val)
    }
}
impl From<TXIM> for u8 {
    #[inline(always)]
    fn from(val: TXIM) -> u8 {
        TXIM::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum TXMIS {
    ///Masked.
    masked = 0x0,
    ///Not masked.
    notmasked = 0x01,
}
impl TXMIS {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TXMIS {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TXMIS {
    #[inline(always)]
    fn from(val: u8) -> TXMIS {
        TXMIS::from_bits(val)
    }
}
impl From<TXMIS> for u8 {
    #[inline(always)]
    fn from(val: TXMIS) -> u8 {
        TXMIS::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum TXRIS {
    ///Masked.
    masked = 0x0,
    ///Not masked.
    notmasked = 0x01,
}
impl TXRIS {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TXRIS {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TXRIS {
    #[inline(always)]
    fn from(val: u8) -> TXRIS {
        TXRIS::from_bits(val)
    }
}
impl From<TXRIS> for u8 {
    #[inline(always)]
    fn from(val: TXRIS) -> u8 {
        TXRIS::to_bits(val)
    }
}
