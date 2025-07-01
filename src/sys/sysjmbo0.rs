#[doc = "Register `SYSJMBO0` reader"]
pub type R = crate::R<Sysjmbo0Spec>;
#[doc = "Register `SYSJMBO0` writer"]
pub type W = crate::W<Sysjmbo0Spec>;
#[doc = "Field `MSGLO` reader - JTAG mailbox outgoing message low byte"]
pub type MsgloR = crate::FieldReader;
#[doc = "Field `MSGLO` writer - JTAG mailbox outgoing message low byte"]
pub type MsgloW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `MSGHI` reader - JTAG mailbox outgoing message high byte"]
pub type MsghiR = crate::FieldReader;
#[doc = "Field `MSGHI` writer - JTAG mailbox outgoing message high byte"]
pub type MsghiW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - JTAG mailbox outgoing message low byte"]
    #[inline(always)]
    pub fn msglo(&self) -> MsgloR {
        MsgloR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - JTAG mailbox outgoing message high byte"]
    #[inline(always)]
    pub fn msghi(&self) -> MsghiR {
        MsghiR::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - JTAG mailbox outgoing message low byte"]
    #[inline(always)]
    pub fn msglo(&mut self) -> MsgloW<Sysjmbo0Spec> {
        MsgloW::new(self, 0)
    }
    #[doc = "Bits 8:15 - JTAG mailbox outgoing message high byte"]
    #[inline(always)]
    pub fn msghi(&mut self) -> MsghiW<Sysjmbo0Spec> {
        MsghiW::new(self, 8)
    }
}
#[doc = "JTAG Mailbox Output 0\n\nYou can [`read`](crate::Reg::read) this register and get [`sysjmbo0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sysjmbo0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Sysjmbo0Spec;
impl crate::RegisterSpec for Sysjmbo0Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`sysjmbo0::R`](R) reader structure"]
impl crate::Readable for Sysjmbo0Spec {}
#[doc = "`write(|w| ..)` method takes [`sysjmbo0::W`](W) writer structure"]
impl crate::Writable for Sysjmbo0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SYSJMBO0 to value 0"]
impl crate::Resettable for Sysjmbo0Spec {}
