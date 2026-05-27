///Register `IC_DATA_CMD` reader
pub type R = crate::R<IcDataCmdSpec>;
///Register `IC_DATA_CMD` writer
pub type W = crate::W<IcDataCmdSpec>;
///Field `DAT` reader - Data byte for TX, or received byte in RX
pub type DatR = crate::FieldReader;
///Field `DAT` writer - Data byte for TX, or received byte in RX
pub type DatW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `CMD` reader - Transfer direction (0=write, 1=read)
pub type CmdR = crate::BitReader;
///Field `CMD` writer - Transfer direction (0=write, 1=read)
pub type CmdW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `STOP` reader - Issue STOP after this byte
pub type StopR = crate::BitReader;
///Field `STOP` writer - Issue STOP after this byte
pub type StopW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RESTART` reader - Issue RESTART before this byte
pub type RestartR = crate::BitReader;
///Field `RESTART` writer - Issue RESTART before this byte
pub type RestartW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FIRST_DATA_BYTE` reader - First data byte after address phase (read-only)
pub type FirstDataByteR = crate::BitReader;
///Field `FIRST_DATA_BYTE` writer - First data byte after address phase (read-only)
pub type FirstDataByteW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:7 - Data byte for TX, or received byte in RX
    #[inline(always)]
    pub fn dat(&self) -> DatR {
        DatR::new((self.bits & 0xff) as u8)
    }
    ///Bit 8 - Transfer direction (0=write, 1=read)
    #[inline(always)]
    pub fn cmd(&self) -> CmdR {
        CmdR::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Issue STOP after this byte
    #[inline(always)]
    pub fn stop(&self) -> StopR {
        StopR::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Issue RESTART before this byte
    #[inline(always)]
    pub fn restart(&self) -> RestartR {
        RestartR::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - First data byte after address phase (read-only)
    #[inline(always)]
    pub fn first_data_byte(&self) -> FirstDataByteR {
        FirstDataByteR::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    ///Bits 0:7 - Data byte for TX, or received byte in RX
    #[inline(always)]
    pub fn dat(&mut self) -> DatW<'_, IcDataCmdSpec> {
        DatW::new(self, 0)
    }
    ///Bit 8 - Transfer direction (0=write, 1=read)
    #[inline(always)]
    pub fn cmd(&mut self) -> CmdW<'_, IcDataCmdSpec> {
        CmdW::new(self, 8)
    }
    ///Bit 9 - Issue STOP after this byte
    #[inline(always)]
    pub fn stop(&mut self) -> StopW<'_, IcDataCmdSpec> {
        StopW::new(self, 9)
    }
    ///Bit 10 - Issue RESTART before this byte
    #[inline(always)]
    pub fn restart(&mut self) -> RestartW<'_, IcDataCmdSpec> {
        RestartW::new(self, 10)
    }
    ///Bit 11 - First data byte after address phase (read-only)
    #[inline(always)]
    pub fn first_data_byte(&mut self) -> FirstDataByteW<'_, IcDataCmdSpec> {
        FirstDataByteW::new(self, 11)
    }
}
/**I2C Rx/Tx data buffer and command register

You can [`read`](crate::Reg::read) this register and get [`ic_data_cmd::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ic_data_cmd::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct IcDataCmdSpec;
impl crate::RegisterSpec for IcDataCmdSpec {
    type Ux = u32;
}
///`read()` method returns [`ic_data_cmd::R`](R) reader structure
impl crate::Readable for IcDataCmdSpec {}
///`write(|w| ..)` method takes [`ic_data_cmd::W`](W) writer structure
impl crate::Writable for IcDataCmdSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets IC_DATA_CMD to value 0
impl crate::Resettable for IcDataCmdSpec {}
