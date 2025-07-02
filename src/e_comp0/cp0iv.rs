#[doc = "Register `CP0IV` reader"]
pub type R = crate::R<Cp0ivSpec>;
#[doc = "Register `CP0IV` writer"]
pub type W = crate::W<Cp0ivSpec>;
#[doc = "Comparator interrupt vector word register\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum Cpiv {
    #[doc = "0: No interrupt pending"]
    None = 0,
    #[doc = "2: CPIFG"]
    Cpifg = 2,
    #[doc = "4: CPIIFG"]
    Cpiifg = 4,
}
impl From<Cpiv> for u16 {
    #[inline(always)]
    fn from(variant: Cpiv) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Cpiv {
    type Ux = u16;
}
impl crate::IsEnum for Cpiv {}
#[doc = "Field `CPIV` reader - Comparator interrupt vector word register"]
pub type CpivR = crate::FieldReader<Cpiv>;
impl CpivR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Cpiv> {
        match self.bits {
            0 => Some(Cpiv::None),
            2 => Some(Cpiv::Cpifg),
            4 => Some(Cpiv::Cpiifg),
            _ => None,
        }
    }
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == Cpiv::None
    }
    #[doc = "CPIFG"]
    #[inline(always)]
    pub fn is_cpifg(&self) -> bool {
        *self == Cpiv::Cpifg
    }
    #[doc = "CPIIFG"]
    #[inline(always)]
    pub fn is_cpiifg(&self) -> bool {
        *self == Cpiv::Cpiifg
    }
}
impl R {
    #[doc = "Bits 0:15 - Comparator interrupt vector word register"]
    #[inline(always)]
    pub fn cpiv(&self) -> CpivR {
        CpivR::new(self.bits)
    }
}
impl W {}
#[doc = "Comparator Interrupt Vector Word Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cp0iv::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cp0iv::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cp0ivSpec;
impl crate::RegisterSpec for Cp0ivSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`cp0iv::R`](R) reader structure"]
impl crate::Readable for Cp0ivSpec {}
#[doc = "`write(|w| ..)` method takes [`cp0iv::W`](W) writer structure"]
impl crate::Writable for Cp0ivSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CP0IV to value 0"]
impl crate::Resettable for Cp0ivSpec {}
