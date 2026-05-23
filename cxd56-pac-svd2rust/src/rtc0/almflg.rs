///Register `ALMFLG` reader
pub type R = crate::R<AlmflgSpec>;
///Field `ALM0` reader - Alarm 0 flag
pub type Alm0R = crate::BitReader;
///Field `ALM1` reader - Alarm 1 flag
pub type Alm1R = crate::BitReader;
///Field `ALM2` reader - Alarm 2 flag
pub type Alm2R = crate::BitReader;
///Field `ALM0_ERR` reader - Alarm 0 error flag
pub type Alm0ErrR = crate::BitReader;
///Field `ALM1_ERR` reader - Alarm 1 error flag
pub type Alm1ErrR = crate::BitReader;
impl R {
    ///Bit 0 - Alarm 0 flag
    #[inline(always)]
    pub fn alm0(&self) -> Alm0R {
        Alm0R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Alarm 1 flag
    #[inline(always)]
    pub fn alm1(&self) -> Alm1R {
        Alm1R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Alarm 2 flag
    #[inline(always)]
    pub fn alm2(&self) -> Alm2R {
        Alm2R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 16 - Alarm 0 error flag
    #[inline(always)]
    pub fn alm0_err(&self) -> Alm0ErrR {
        Alm0ErrR::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - Alarm 1 error flag
    #[inline(always)]
    pub fn alm1_err(&self) -> Alm1ErrR {
        Alm1ErrR::new(((self.bits >> 17) & 1) != 0)
    }
}
/**Alarm flag status (read-only)

You can [`read`](crate::Reg::read) this register and get [`almflg::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct AlmflgSpec;
impl crate::RegisterSpec for AlmflgSpec {
    type Ux = u32;
}
///`read()` method returns [`almflg::R`](R) reader structure
impl crate::Readable for AlmflgSpec {}
///`reset()` method sets ALMFLG to value 0
impl crate::Resettable for AlmflgSpec {}
