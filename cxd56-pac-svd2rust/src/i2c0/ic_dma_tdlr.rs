///Register `IC_DMA_TDLR` reader
pub type R = crate::R<IcDmaTdlrSpec>;
///Register `IC_DMA_TDLR` writer
pub type W = crate::W<IcDmaTdlrSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
/**I2C DMA transmit data level register

You can [`read`](crate::Reg::read) this register and get [`ic_dma_tdlr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ic_dma_tdlr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct IcDmaTdlrSpec;
impl crate::RegisterSpec for IcDmaTdlrSpec {
    type Ux = u32;
}
///`read()` method returns [`ic_dma_tdlr::R`](R) reader structure
impl crate::Readable for IcDmaTdlrSpec {}
///`write(|w| ..)` method takes [`ic_dma_tdlr::W`](W) writer structure
impl crate::Writable for IcDmaTdlrSpec {
    type Safety = crate::Unsafe;
}
///`reset()` method sets IC_DMA_TDLR to value 0
impl crate::Resettable for IcDmaTdlrSpec {}
