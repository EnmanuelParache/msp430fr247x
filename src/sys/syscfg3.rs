#[doc = "Register `SYSCFG3` reader"]
pub type R = crate::R<Syscfg3Spec>;
#[doc = "Register `SYSCFG3` writer"]
pub type W = crate::W<Syscfg3Spec>;
#[doc = "eUSCI_A0 remapping source selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Uscia0rmp {
    #[doc = "0: Default function. See the device-specific data sheet for details."]
    Uscia0rmp0 = 0,
    #[doc = "1: Remapped function. See the device-specific data sheet for details."]
    Uscia0rmp1 = 1,
}
impl From<Uscia0rmp> for bool {
    #[inline(always)]
    fn from(variant: Uscia0rmp) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USCIA0RMP` reader - eUSCI_A0 remapping source selection"]
pub type Uscia0rmpR = crate::BitReader<Uscia0rmp>;
impl Uscia0rmpR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Uscia0rmp {
        match self.bits {
            false => Uscia0rmp::Uscia0rmp0,
            true => Uscia0rmp::Uscia0rmp1,
        }
    }
    #[doc = "Default function. See the device-specific data sheet for details."]
    #[inline(always)]
    pub fn is_uscia0rmp_0(&self) -> bool {
        *self == Uscia0rmp::Uscia0rmp0
    }
    #[doc = "Remapped function. See the device-specific data sheet for details."]
    #[inline(always)]
    pub fn is_uscia0rmp_1(&self) -> bool {
        *self == Uscia0rmp::Uscia0rmp1
    }
}
#[doc = "Field `USCIA0RMP` writer - eUSCI_A0 remapping source selection"]
pub type Uscia0rmpW<'a, REG> = crate::BitWriter<'a, REG, Uscia0rmp>;
impl<'a, REG> Uscia0rmpW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Default function. See the device-specific data sheet for details."]
    #[inline(always)]
    pub fn uscia0rmp_0(self) -> &'a mut crate::W<REG> {
        self.variant(Uscia0rmp::Uscia0rmp0)
    }
    #[doc = "Remapped function. See the device-specific data sheet for details."]
    #[inline(always)]
    pub fn uscia0rmp_1(self) -> &'a mut crate::W<REG> {
        self.variant(Uscia0rmp::Uscia0rmp1)
    }
}
#[doc = "Timer2_A3 remapping source selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ta2rmp {
    #[doc = "0: Default function. See the device-specific data sheet for details."]
    Ta2rmp0 = 0,
    #[doc = "1: Remapped function. See the device-specific data sheet for details."]
    Ta2rmp1 = 1,
}
impl From<Ta2rmp> for bool {
    #[inline(always)]
    fn from(variant: Ta2rmp) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TA2RMP` reader - Timer2_A3 remapping source selection"]
pub type Ta2rmpR = crate::BitReader<Ta2rmp>;
impl Ta2rmpR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ta2rmp {
        match self.bits {
            false => Ta2rmp::Ta2rmp0,
            true => Ta2rmp::Ta2rmp1,
        }
    }
    #[doc = "Default function. See the device-specific data sheet for details."]
    #[inline(always)]
    pub fn is_ta2rmp_0(&self) -> bool {
        *self == Ta2rmp::Ta2rmp0
    }
    #[doc = "Remapped function. See the device-specific data sheet for details."]
    #[inline(always)]
    pub fn is_ta2rmp_1(&self) -> bool {
        *self == Ta2rmp::Ta2rmp1
    }
}
#[doc = "Field `TA2RMP` writer - Timer2_A3 remapping source selection"]
pub type Ta2rmpW<'a, REG> = crate::BitWriter<'a, REG, Ta2rmp>;
impl<'a, REG> Ta2rmpW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Default function. See the device-specific data sheet for details."]
    #[inline(always)]
    pub fn ta2rmp_0(self) -> &'a mut crate::W<REG> {
        self.variant(Ta2rmp::Ta2rmp0)
    }
    #[doc = "Remapped function. See the device-specific data sheet for details."]
    #[inline(always)]
    pub fn ta2rmp_1(self) -> &'a mut crate::W<REG> {
        self.variant(Ta2rmp::Ta2rmp1)
    }
}
#[doc = "Timer3_A3 remapping source selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ta3rmp {
    #[doc = "0: Default function. See the device-specific data sheet for details."]
    Ta3rmp0 = 0,
    #[doc = "1: Remapped function. See the device-specific data sheet for details."]
    Ta3rmp1 = 1,
}
impl From<Ta3rmp> for bool {
    #[inline(always)]
    fn from(variant: Ta3rmp) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TA3RMP` reader - Timer3_A3 remapping source selection"]
pub type Ta3rmpR = crate::BitReader<Ta3rmp>;
impl Ta3rmpR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ta3rmp {
        match self.bits {
            false => Ta3rmp::Ta3rmp0,
            true => Ta3rmp::Ta3rmp1,
        }
    }
    #[doc = "Default function. See the device-specific data sheet for details."]
    #[inline(always)]
    pub fn is_ta3rmp_0(&self) -> bool {
        *self == Ta3rmp::Ta3rmp0
    }
    #[doc = "Remapped function. See the device-specific data sheet for details."]
    #[inline(always)]
    pub fn is_ta3rmp_1(&self) -> bool {
        *self == Ta3rmp::Ta3rmp1
    }
}
#[doc = "Field `TA3RMP` writer - Timer3_A3 remapping source selection"]
pub type Ta3rmpW<'a, REG> = crate::BitWriter<'a, REG, Ta3rmp>;
impl<'a, REG> Ta3rmpW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Default function. See the device-specific data sheet for details."]
    #[inline(always)]
    pub fn ta3rmp_0(self) -> &'a mut crate::W<REG> {
        self.variant(Ta3rmp::Ta3rmp0)
    }
    #[doc = "Remapped function. See the device-specific data sheet for details."]
    #[inline(always)]
    pub fn ta3rmp_1(self) -> &'a mut crate::W<REG> {
        self.variant(Ta3rmp::Ta3rmp1)
    }
}
#[doc = "eUSCI_B1 remapping source selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Uscib1rmp {
    #[doc = "0: Default function. See the device-specific data sheet for details."]
    Uscib1rmp0 = 0,
    #[doc = "1: Remapped function. See the device-specific data sheet for details."]
    Uscib1rmp1 = 1,
}
impl From<Uscib1rmp> for bool {
    #[inline(always)]
    fn from(variant: Uscib1rmp) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USCIB1RMP` reader - eUSCI_B1 remapping source selection"]
pub type Uscib1rmpR = crate::BitReader<Uscib1rmp>;
impl Uscib1rmpR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Uscib1rmp {
        match self.bits {
            false => Uscib1rmp::Uscib1rmp0,
            true => Uscib1rmp::Uscib1rmp1,
        }
    }
    #[doc = "Default function. See the device-specific data sheet for details."]
    #[inline(always)]
    pub fn is_uscib1rmp_0(&self) -> bool {
        *self == Uscib1rmp::Uscib1rmp0
    }
    #[doc = "Remapped function. See the device-specific data sheet for details."]
    #[inline(always)]
    pub fn is_uscib1rmp_1(&self) -> bool {
        *self == Uscib1rmp::Uscib1rmp1
    }
}
#[doc = "Field `USCIB1RMP` writer - eUSCI_B1 remapping source selection"]
pub type Uscib1rmpW<'a, REG> = crate::BitWriter<'a, REG, Uscib1rmp>;
impl<'a, REG> Uscib1rmpW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Default function. See the device-specific data sheet for details."]
    #[inline(always)]
    pub fn uscib1rmp_0(self) -> &'a mut crate::W<REG> {
        self.variant(Uscib1rmp::Uscib1rmp0)
    }
    #[doc = "Remapped function. See the device-specific data sheet for details."]
    #[inline(always)]
    pub fn uscib1rmp_1(self) -> &'a mut crate::W<REG> {
        self.variant(Uscib1rmp::Uscib1rmp1)
    }
}
impl R {
    #[doc = "Bit 0 - eUSCI_A0 remapping source selection"]
    #[inline(always)]
    pub fn uscia0rmp(&self) -> Uscia0rmpR {
        Uscia0rmpR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - Timer2_A3 remapping source selection"]
    #[inline(always)]
    pub fn ta2rmp(&self) -> Ta2rmpR {
        Ta2rmpR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Timer3_A3 remapping source selection"]
    #[inline(always)]
    pub fn ta3rmp(&self) -> Ta3rmpR {
        Ta3rmpR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - eUSCI_B1 remapping source selection"]
    #[inline(always)]
    pub fn uscib1rmp(&self) -> Uscib1rmpR {
        Uscib1rmpR::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - eUSCI_A0 remapping source selection"]
    #[inline(always)]
    pub fn uscia0rmp(&mut self) -> Uscia0rmpW<'_, Syscfg3Spec> {
        Uscia0rmpW::new(self, 0)
    }
    #[doc = "Bit 2 - Timer2_A3 remapping source selection"]
    #[inline(always)]
    pub fn ta2rmp(&mut self) -> Ta2rmpW<'_, Syscfg3Spec> {
        Ta2rmpW::new(self, 2)
    }
    #[doc = "Bit 3 - Timer3_A3 remapping source selection"]
    #[inline(always)]
    pub fn ta3rmp(&mut self) -> Ta3rmpW<'_, Syscfg3Spec> {
        Ta3rmpW::new(self, 3)
    }
    #[doc = "Bit 4 - eUSCI_B1 remapping source selection"]
    #[inline(always)]
    pub fn uscib1rmp(&mut self) -> Uscib1rmpW<'_, Syscfg3Spec> {
        Uscib1rmpW::new(self, 4)
    }
}
#[doc = "System Configuration 3\n\nYou can [`read`](crate::Reg::read) this register and get [`syscfg3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`syscfg3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Syscfg3Spec;
impl crate::RegisterSpec for Syscfg3Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`syscfg3::R`](R) reader structure"]
impl crate::Readable for Syscfg3Spec {}
#[doc = "`write(|w| ..)` method takes [`syscfg3::W`](W) writer structure"]
impl crate::Writable for Syscfg3Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SYSCFG3 to value 0"]
impl crate::Resettable for Syscfg3Spec {}
