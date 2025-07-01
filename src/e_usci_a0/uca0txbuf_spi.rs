#[doc = "Register `UCA0TXBUF_SPI` reader"]
pub type R = crate::R<Uca0txbufSpiSpec>;
#[doc = "Register `UCA0TXBUF_SPI` writer"]
pub type W = crate::W<Uca0txbufSpiSpec>;
#[doc = "Field `UCTXBUF` reader - Transmit data buffer"]
pub type UctxbufR = crate::FieldReader;
#[doc = "Field `UCTXBUF` writer - Transmit data buffer"]
pub type UctxbufW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Transmit data buffer"]
    #[inline(always)]
    pub fn uctxbuf(&self) -> UctxbufR {
        UctxbufR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Transmit data buffer"]
    #[inline(always)]
    pub fn uctxbuf(&mut self) -> UctxbufW<Uca0txbufSpiSpec> {
        UctxbufW::new(self, 0)
    }
}
#[doc = "eUSCI_Ax Transmit Buffer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`uca0txbuf_spi::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uca0txbuf_spi::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Uca0txbufSpiSpec;
impl crate::RegisterSpec for Uca0txbufSpiSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`uca0txbuf_spi::R`](R) reader structure"]
impl crate::Readable for Uca0txbufSpiSpec {}
#[doc = "`write(|w| ..)` method takes [`uca0txbuf_spi::W`](W) writer structure"]
impl crate::Writable for Uca0txbufSpiSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UCA0TXBUF_SPI to value 0"]
impl crate::Resettable for Uca0txbufSpiSpec {}
