#[doc = "Register `SYSCFG2` reader"]
pub type R = crate::R<Syscfg2Spec>;
#[doc = "Register `SYSCFG2` writer"]
pub type W = crate::W<Syscfg2Spec>;
#[doc = "RTC clock selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rtccksel {
    #[doc = "0: SMCLK is selected"]
    Rtccksel0 = 0,
    #[doc = "1: ACLK is selected"]
    Rtccksel1 = 1,
}
impl From<Rtccksel> for bool {
    #[inline(always)]
    fn from(variant: Rtccksel) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RTCCKSEL` reader - RTC clock selection"]
pub type RtcckselR = crate::BitReader<Rtccksel>;
impl RtcckselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rtccksel {
        match self.bits {
            false => Rtccksel::Rtccksel0,
            true => Rtccksel::Rtccksel1,
        }
    }
    #[doc = "SMCLK is selected"]
    #[inline(always)]
    pub fn is_rtccksel_0(&self) -> bool {
        *self == Rtccksel::Rtccksel0
    }
    #[doc = "ACLK is selected"]
    #[inline(always)]
    pub fn is_rtccksel_1(&self) -> bool {
        *self == Rtccksel::Rtccksel1
    }
}
#[doc = "Field `RTCCKSEL` writer - RTC clock selection"]
pub type RtcckselW<'a, REG> = crate::BitWriter<'a, REG, Rtccksel>;
impl<'a, REG> RtcckselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SMCLK is selected"]
    #[inline(always)]
    pub fn rtccksel_0(self) -> &'a mut crate::W<REG> {
        self.variant(Rtccksel::Rtccksel0)
    }
    #[doc = "ACLK is selected"]
    #[inline(always)]
    pub fn rtccksel_1(self) -> &'a mut crate::W<REG> {
        self.variant(Rtccksel::Rtccksel1)
    }
}
#[doc = "eUSCI_B0 remapping source selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Uscib0rmp {
    #[doc = "0: Default function. See the device-specific data sheet for details."]
    Uscib0rmp0 = 0,
    #[doc = "1: Remapped function. See the device-specific data sheet for details."]
    Uscib0rmp1 = 1,
}
impl From<Uscib0rmp> for bool {
    #[inline(always)]
    fn from(variant: Uscib0rmp) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USCIB0RMP` reader - eUSCI_B0 remapping source selection"]
pub type Uscib0rmpR = crate::BitReader<Uscib0rmp>;
impl Uscib0rmpR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Uscib0rmp {
        match self.bits {
            false => Uscib0rmp::Uscib0rmp0,
            true => Uscib0rmp::Uscib0rmp1,
        }
    }
    #[doc = "Default function. See the device-specific data sheet for details."]
    #[inline(always)]
    pub fn is_uscib0rmp_0(&self) -> bool {
        *self == Uscib0rmp::Uscib0rmp0
    }
    #[doc = "Remapped function. See the device-specific data sheet for details."]
    #[inline(always)]
    pub fn is_uscib0rmp_1(&self) -> bool {
        *self == Uscib0rmp::Uscib0rmp1
    }
}
#[doc = "Field `USCIB0RMP` writer - eUSCI_B0 remapping source selection"]
pub type Uscib0rmpW<'a, REG> = crate::BitWriter<'a, REG, Uscib0rmp>;
impl<'a, REG> Uscib0rmpW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Default function. See the device-specific data sheet for details."]
    #[inline(always)]
    pub fn uscib0rmp_0(self) -> &'a mut crate::W<REG> {
        self.variant(Uscib0rmp::Uscib0rmp0)
    }
    #[doc = "Remapped function. See the device-specific data sheet for details."]
    #[inline(always)]
    pub fn uscib0rmp_1(self) -> &'a mut crate::W<REG> {
        self.variant(Uscib0rmp::Uscib0rmp1)
    }
}
#[doc = "TB0OUTH trigger source selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tb0trgsel {
    #[doc = "0: Internal source is selected"]
    Tb0trgsel0 = 0,
    #[doc = "1: External source is selected"]
    Tb0trgsel1 = 1,
}
impl From<Tb0trgsel> for bool {
    #[inline(always)]
    fn from(variant: Tb0trgsel) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TB0TRGSEL` reader - TB0OUTH trigger source selection"]
pub type Tb0trgselR = crate::BitReader<Tb0trgsel>;
impl Tb0trgselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tb0trgsel {
        match self.bits {
            false => Tb0trgsel::Tb0trgsel0,
            true => Tb0trgsel::Tb0trgsel1,
        }
    }
    #[doc = "Internal source is selected"]
    #[inline(always)]
    pub fn is_tb0trgsel_0(&self) -> bool {
        *self == Tb0trgsel::Tb0trgsel0
    }
    #[doc = "External source is selected"]
    #[inline(always)]
    pub fn is_tb0trgsel_1(&self) -> bool {
        *self == Tb0trgsel::Tb0trgsel1
    }
}
#[doc = "Field `TB0TRGSEL` writer - TB0OUTH trigger source selection"]
pub type Tb0trgselW<'a, REG> = crate::BitWriter<'a, REG, Tb0trgsel>;
impl<'a, REG> Tb0trgselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Internal source is selected"]
    #[inline(always)]
    pub fn tb0trgsel_0(self) -> &'a mut crate::W<REG> {
        self.variant(Tb0trgsel::Tb0trgsel0)
    }
    #[doc = "External source is selected"]
    #[inline(always)]
    pub fn tb0trgsel_1(self) -> &'a mut crate::W<REG> {
        self.variant(Tb0trgsel::Tb0trgsel1)
    }
}
impl R {
    #[doc = "Bit 10 - RTC clock selection"]
    #[inline(always)]
    pub fn rtccksel(&self) -> RtcckselR {
        RtcckselR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - eUSCI_B0 remapping source selection"]
    #[inline(always)]
    pub fn uscib0rmp(&self) -> Uscib0rmpR {
        Uscib0rmpR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 15 - TB0OUTH trigger source selection"]
    #[inline(always)]
    pub fn tb0trgsel(&self) -> Tb0trgselR {
        Tb0trgselR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 10 - RTC clock selection"]
    #[inline(always)]
    pub fn rtccksel(&mut self) -> RtcckselW<Syscfg2Spec> {
        RtcckselW::new(self, 10)
    }
    #[doc = "Bit 11 - eUSCI_B0 remapping source selection"]
    #[inline(always)]
    pub fn uscib0rmp(&mut self) -> Uscib0rmpW<Syscfg2Spec> {
        Uscib0rmpW::new(self, 11)
    }
    #[doc = "Bit 15 - TB0OUTH trigger source selection"]
    #[inline(always)]
    pub fn tb0trgsel(&mut self) -> Tb0trgselW<Syscfg2Spec> {
        Tb0trgselW::new(self, 15)
    }
}
#[doc = "System Configuration 2\n\nYou can [`read`](crate::Reg::read) this register and get [`syscfg2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`syscfg2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Syscfg2Spec;
impl crate::RegisterSpec for Syscfg2Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`syscfg2::R`](R) reader structure"]
impl crate::Readable for Syscfg2Spec {}
#[doc = "`write(|w| ..)` method takes [`syscfg2::W`](W) writer structure"]
impl crate::Writable for Syscfg2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SYSCFG2 to value 0"]
impl crate::Resettable for Syscfg2Spec {}
