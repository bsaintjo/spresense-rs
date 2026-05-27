///Register `IOCSYS_IOMD1` reader
pub type R = crate::R<IocsysIomd1Spec>;
///Register `IOCSYS_IOMD1` writer
pub type W = crate::W<IocsysIomd1Spec>;
///Field `I2C0` reader - Mode select for I2C0_BCK / I2C0_BDT (I2C0 = Func1)
pub type I2c0R = crate::FieldReader;
///Field `I2C0` writer - Mode select for I2C0_BCK / I2C0_BDT (I2C0 = Func1)
pub type I2c0W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    ///Bits 18:19 - Mode select for I2C0_BCK / I2C0_BDT (I2C0 = Func1)
    #[inline(always)]
    pub fn i2c0(&self) -> I2c0R {
        I2c0R::new(((self.bits >> 18) & 3) as u8)
    }
}
impl W {
    ///Bits 18:19 - Mode select for I2C0_BCK / I2C0_BDT (I2C0 = Func1)
    #[inline(always)]
    pub fn i2c0(&mut self) -> I2c0W<'_, IocsysIomd1Spec> {
        I2c0W::new(self, 18)
    }
}
/**SYSIOP IO-cell mode-mux register 1

You can [`read`](crate::Reg::read) this register and get [`iocsys_iomd1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iocsys_iomd1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct IocsysIomd1Spec;
impl crate::RegisterSpec for IocsysIomd1Spec {
    type Ux = u32;
}
///`read()` method returns [`iocsys_iomd1::R`](R) reader structure
impl crate::Readable for IocsysIomd1Spec {}
///`write(|w| ..)` method takes [`iocsys_iomd1::W`](W) writer structure
impl crate::Writable for IocsysIomd1Spec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets IOCSYS_IOMD1 to value 0
impl crate::Resettable for IocsysIomd1Spec {}
