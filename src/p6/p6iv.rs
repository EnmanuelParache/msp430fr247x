#[doc = "Register `P6IV` reader"]
pub type R = crate::R<P6ivSpec>;
#[doc = "Register `P6IV` writer"]
pub type W = crate::W<P6ivSpec>;
#[doc = "Port 6 interrupt vector value\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum P6iv {
    #[doc = "0: No interrupt pending"]
    None = 0,
    #[doc = "2: Interrupt Source: Port x.0 interrupt; Interrupt Flag: PxIFG.0; Interrupt Priority: Highest"]
    Ifg0 = 2,
    #[doc = "4: Interrupt Source: Port x.1 interrupt; Interrupt Flag: PxIFG.1"]
    Ifg1 = 4,
    #[doc = "6: Interrupt Source: Port x.2 interrupt; Interrupt Flag: PxIFG.2"]
    Ifg2 = 6,
    #[doc = "8: Interrupt Source: Port x.3 interrupt; Interrupt Flag: PxIFG.3"]
    Ifg3 = 8,
    #[doc = "10: Interrupt Source: Port x.4 interrupt; Interrupt Flag: PxIFG.4"]
    Ifg4 = 10,
    #[doc = "12: Interrupt Source: Port x.5 interrupt; Interrupt Flag: PxIFG.5"]
    Ifg5 = 12,
    #[doc = "14: Interrupt Source: Port x.6 interrupt; Interrupt Flag: PxIFG.6"]
    Ifg6 = 14,
    #[doc = "16: Interrupt Source: Port x.7 interrupt; Interrupt Flag: PxIFG.7; Interrupt Priority: Lowest"]
    Ifg7 = 16,
}
impl From<P6iv> for u16 {
    #[inline(always)]
    fn from(variant: P6iv) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for P6iv {
    type Ux = u16;
}
impl crate::IsEnum for P6iv {}
#[doc = "Field `P6IV` reader - Port 6 interrupt vector value"]
pub type P6ivR = crate::FieldReader<P6iv>;
impl P6ivR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<P6iv> {
        match self.bits {
            0 => Some(P6iv::None),
            2 => Some(P6iv::Ifg0),
            4 => Some(P6iv::Ifg1),
            6 => Some(P6iv::Ifg2),
            8 => Some(P6iv::Ifg3),
            10 => Some(P6iv::Ifg4),
            12 => Some(P6iv::Ifg5),
            14 => Some(P6iv::Ifg6),
            16 => Some(P6iv::Ifg7),
            _ => None,
        }
    }
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == P6iv::None
    }
    #[doc = "Interrupt Source: Port x.0 interrupt; Interrupt Flag: PxIFG.0; Interrupt Priority: Highest"]
    #[inline(always)]
    pub fn is_ifg0(&self) -> bool {
        *self == P6iv::Ifg0
    }
    #[doc = "Interrupt Source: Port x.1 interrupt; Interrupt Flag: PxIFG.1"]
    #[inline(always)]
    pub fn is_ifg1(&self) -> bool {
        *self == P6iv::Ifg1
    }
    #[doc = "Interrupt Source: Port x.2 interrupt; Interrupt Flag: PxIFG.2"]
    #[inline(always)]
    pub fn is_ifg2(&self) -> bool {
        *self == P6iv::Ifg2
    }
    #[doc = "Interrupt Source: Port x.3 interrupt; Interrupt Flag: PxIFG.3"]
    #[inline(always)]
    pub fn is_ifg3(&self) -> bool {
        *self == P6iv::Ifg3
    }
    #[doc = "Interrupt Source: Port x.4 interrupt; Interrupt Flag: PxIFG.4"]
    #[inline(always)]
    pub fn is_ifg4(&self) -> bool {
        *self == P6iv::Ifg4
    }
    #[doc = "Interrupt Source: Port x.5 interrupt; Interrupt Flag: PxIFG.5"]
    #[inline(always)]
    pub fn is_ifg5(&self) -> bool {
        *self == P6iv::Ifg5
    }
    #[doc = "Interrupt Source: Port x.6 interrupt; Interrupt Flag: PxIFG.6"]
    #[inline(always)]
    pub fn is_ifg6(&self) -> bool {
        *self == P6iv::Ifg6
    }
    #[doc = "Interrupt Source: Port x.7 interrupt; Interrupt Flag: PxIFG.7; Interrupt Priority: Lowest"]
    #[inline(always)]
    pub fn is_ifg7(&self) -> bool {
        *self == P6iv::Ifg7
    }
}
impl R {
    #[doc = "Bits 0:15 - Port 6 interrupt vector value"]
    #[inline(always)]
    pub fn p6iv(&self) -> P6ivR {
        P6ivR::new(self.bits)
    }
}
impl W {}
#[doc = "Port 6 Interrupt Vector Register\n\nYou can [`read`](crate::Reg::read) this register and get [`p6iv::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p6iv::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct P6ivSpec;
impl crate::RegisterSpec for P6ivSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`p6iv::R`](R) reader structure"]
impl crate::Readable for P6ivSpec {}
#[doc = "`write(|w| ..)` method takes [`p6iv::W`](W) writer structure"]
impl crate::Writable for P6ivSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets P6IV to value 0"]
impl crate::Resettable for P6ivSpec {}
