///Register `IC_STATUS` reader
pub type R = crate::R<IcStatusSpec>;
///Field `ACTIVITY` reader - I2C activity
pub type ActivityR = crate::BitReader;
///Field `TFNF` reader - TX FIFO not full
pub type TfnfR = crate::BitReader;
///Field `TFE` reader - TX FIFO empty
pub type TfeR = crate::BitReader;
///Field `RFNE` reader - RX FIFO not empty
pub type RfneR = crate::BitReader;
///Field `RFF` reader - RX FIFO full
pub type RffR = crate::BitReader;
///Field `MST_ACTIVITY` reader - Master FSM activity
pub type MstActivityR = crate::BitReader;
///Field `SLV_ACTIVITY` reader - Slave FSM activity
pub type SlvActivityR = crate::BitReader;
impl R {
    ///Bit 0 - I2C activity
    #[inline(always)]
    pub fn activity(&self) -> ActivityR {
        ActivityR::new((self.bits & 1) != 0)
    }
    ///Bit 1 - TX FIFO not full
    #[inline(always)]
    pub fn tfnf(&self) -> TfnfR {
        TfnfR::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - TX FIFO empty
    #[inline(always)]
    pub fn tfe(&self) -> TfeR {
        TfeR::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - RX FIFO not empty
    #[inline(always)]
    pub fn rfne(&self) -> RfneR {
        RfneR::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - RX FIFO full
    #[inline(always)]
    pub fn rff(&self) -> RffR {
        RffR::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Master FSM activity
    #[inline(always)]
    pub fn mst_activity(&self) -> MstActivityR {
        MstActivityR::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Slave FSM activity
    #[inline(always)]
    pub fn slv_activity(&self) -> SlvActivityR {
        SlvActivityR::new(((self.bits >> 6) & 1) != 0)
    }
}
/**I2C status register (read-only)

You can [`read`](crate::Reg::read) this register and get [`ic_status::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct IcStatusSpec;
impl crate::RegisterSpec for IcStatusSpec {
    type Ux = u32;
}
///`read()` method returns [`ic_status::R`](R) reader structure
impl crate::Readable for IcStatusSpec {}
///`reset()` method sets IC_STATUS to value 0x06
impl crate::Resettable for IcStatusSpec {
    const RESET_VALUE: u32 = 0x06;
}
