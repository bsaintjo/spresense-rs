///Register `IC_DMA_CR` reader
pub type R = crate::R<IcDmaCrSpec>;
///Register `IC_DMA_CR` writer
pub type W = crate::W<IcDmaCrSpec>;
///Field `RDMAE` reader - RX DMA enable
pub type RdmaeR = crate::BitReader;
///Field `RDMAE` writer - RX DMA enable
pub type RdmaeW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TDMAE` reader - TX DMA enable
pub type TdmaeR = crate::BitReader;
///Field `TDMAE` writer - TX DMA enable
pub type TdmaeW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - RX DMA enable
    #[inline(always)]
    pub fn rdmae(&self) -> RdmaeR {
        RdmaeR::new((self.bits & 1) != 0)
    }
    ///Bit 1 - TX DMA enable
    #[inline(always)]
    pub fn tdmae(&self) -> TdmaeR {
        TdmaeR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - RX DMA enable
    #[inline(always)]
    pub fn rdmae(&mut self) -> RdmaeW<'_, IcDmaCrSpec> {
        RdmaeW::new(self, 0)
    }
    ///Bit 1 - TX DMA enable
    #[inline(always)]
    pub fn tdmae(&mut self) -> TdmaeW<'_, IcDmaCrSpec> {
        TdmaeW::new(self, 1)
    }
}
/**I2C DMA control register

You can [`read`](crate::Reg::read) this register and get [`ic_dma_cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ic_dma_cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct IcDmaCrSpec;
impl crate::RegisterSpec for IcDmaCrSpec {
    type Ux = u32;
}
///`read()` method returns [`ic_dma_cr::R`](R) reader structure
impl crate::Readable for IcDmaCrSpec {}
///`write(|w| ..)` method takes [`ic_dma_cr::W`](W) writer structure
impl crate::Writable for IcDmaCrSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets IC_DMA_CR to value 0
impl crate::Resettable for IcDmaCrSpec {}
