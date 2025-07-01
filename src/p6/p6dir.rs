#[doc = "Register `P6DIR` reader"]
pub type R = crate::R<P6dirSpec>;
#[doc = "Register `P6DIR` writer"]
pub type W = crate::W<P6dirSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Port 6 Direction\n\nYou can [`read`](crate::Reg::read) this register and get [`p6dir::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p6dir::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct P6dirSpec;
impl crate::RegisterSpec for P6dirSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`p6dir::R`](R) reader structure"]
impl crate::Readable for P6dirSpec {}
#[doc = "`write(|w| ..)` method takes [`p6dir::W`](W) writer structure"]
impl crate::Writable for P6dirSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets P6DIR to value 0"]
impl crate::Resettable for P6dirSpec {}
