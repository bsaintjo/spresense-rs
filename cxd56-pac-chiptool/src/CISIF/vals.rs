#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum hpol {
    ///Hsync low active.
    lowactive = 0x0,
    ///Hsync high active.
    highactive = 0x01,
}
impl hpol {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> hpol {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for hpol {
    #[inline(always)]
    fn from(val: u8) -> hpol {
        hpol::from_bits(val)
    }
}
impl From<hpol> for u8 {
    #[inline(always)]
    fn from(val: hpol) -> u8 {
        hpol::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum vpol {
    ///Vsync low active.
    lowactive = 0x0,
    ///Vsync high active.
    highactive = 0x01,
}
impl vpol {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> vpol {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for vpol {
    #[inline(always)]
    fn from(val: u8) -> vpol {
        vpol::from_bits(val)
    }
}
impl From<vpol> for u8 {
    #[inline(always)]
    fn from(val: vpol) -> u8 {
        vpol::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum yc_order {
    ///Y/Cb/Y/Cr.
    YCbYCr = 0x0,
    ///Y/Cr/Y/Cb.
    YCrYCb = 0x01,
    ///Cb/Y/Cr/Y.
    CbYCrY = 0x02,
    ///Cr/Y/Cb/Y.
    CrYCbY = 0x03,
}
impl yc_order {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> yc_order {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for yc_order {
    #[inline(always)]
    fn from(val: u8) -> yc_order {
        yc_order::from_bits(val)
    }
}
impl From<yc_order> for u8 {
    #[inline(always)]
    fn from(val: yc_order) -> u8 {
        yc_order::to_bits(val)
    }
}
