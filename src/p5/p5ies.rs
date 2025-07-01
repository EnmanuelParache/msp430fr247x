#[doc = "Register `P5IES` reader"]
pub type R = crate::R<P5iesSpec>;
#[doc = "Register `P5IES` writer"]
pub type W = crate::W<P5iesSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Port 5 Interrupt Edge Select\n\nYou can [`read`](crate::Reg::read) this register and get [`p5ies::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p5ies::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct P5iesSpec;
impl crate::RegisterSpec for P5iesSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`p5ies::R`](R) reader structure"]
impl crate::Readable for P5iesSpec {}
#[doc = "`write(|w| ..)` method takes [`p5ies::W`](W) writer structure"]
impl crate::Writable for P5iesSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets P5IES to value 0"]
impl crate::Resettable for P5iesSpec {}
