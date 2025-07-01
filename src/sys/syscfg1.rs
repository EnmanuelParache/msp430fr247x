#[doc = "Register `SYSCFG1` reader"]
pub type R = crate::R<Syscfg1Spec>;
#[doc = "Register `SYSCFG1` writer"]
pub type W = crate::W<Syscfg1Spec>;
#[doc = "Infrared enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Iren {
    #[doc = "0: Infrared function disabled"]
    Dis = 0,
    #[doc = "1: Infrared function enabled"]
    En = 1,
}
impl From<Iren> for bool {
    #[inline(always)]
    fn from(variant: Iren) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IREN` reader - Infrared enable"]
pub type IrenR = crate::BitReader<Iren>;
impl IrenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Iren {
        match self.bits {
            false => Iren::Dis,
            true => Iren::En,
        }
    }
    #[doc = "Infrared function disabled"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Iren::Dis
    }
    #[doc = "Infrared function enabled"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == Iren::En
    }
}
#[doc = "Field `IREN` writer - Infrared enable"]
pub type IrenW<'a, REG> = crate::BitWriter<'a, REG, Iren>;
impl<'a, REG> IrenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Infrared function disabled"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Iren::Dis)
    }
    #[doc = "Infrared function enabled"]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(Iren::En)
    }
}
#[doc = "Infrared polarity select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Irpsel {
    #[doc = "0: Normal polarity"]
    Norm = 0,
    #[doc = "1: Inverted polarity"]
    Inv = 1,
}
impl From<Irpsel> for bool {
    #[inline(always)]
    fn from(variant: Irpsel) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IRPSEL` reader - Infrared polarity select"]
pub type IrpselR = crate::BitReader<Irpsel>;
impl IrpselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Irpsel {
        match self.bits {
            false => Irpsel::Norm,
            true => Irpsel::Inv,
        }
    }
    #[doc = "Normal polarity"]
    #[inline(always)]
    pub fn is_norm(&self) -> bool {
        *self == Irpsel::Norm
    }
    #[doc = "Inverted polarity"]
    #[inline(always)]
    pub fn is_inv(&self) -> bool {
        *self == Irpsel::Inv
    }
}
#[doc = "Field `IRPSEL` writer - Infrared polarity select"]
pub type IrpselW<'a, REG> = crate::BitWriter<'a, REG, Irpsel>;
impl<'a, REG> IrpselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Normal polarity"]
    #[inline(always)]
    pub fn norm(self) -> &'a mut crate::W<REG> {
        self.variant(Irpsel::Norm)
    }
    #[doc = "Inverted polarity"]
    #[inline(always)]
    pub fn inv(self) -> &'a mut crate::W<REG> {
        self.variant(Irpsel::Inv)
    }
}
#[doc = "Infrared mode select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Irmsel {
    #[doc = "0: ASK mode"]
    Ask = 0,
    #[doc = "1: FSK mode"]
    Fsk = 1,
}
impl From<Irmsel> for bool {
    #[inline(always)]
    fn from(variant: Irmsel) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IRMSEL` reader - Infrared mode select"]
pub type IrmselR = crate::BitReader<Irmsel>;
impl IrmselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Irmsel {
        match self.bits {
            false => Irmsel::Ask,
            true => Irmsel::Fsk,
        }
    }
    #[doc = "ASK mode"]
    #[inline(always)]
    pub fn is_ask(&self) -> bool {
        *self == Irmsel::Ask
    }
    #[doc = "FSK mode"]
    #[inline(always)]
    pub fn is_fsk(&self) -> bool {
        *self == Irmsel::Fsk
    }
}
#[doc = "Field `IRMSEL` writer - Infrared mode select"]
pub type IrmselW<'a, REG> = crate::BitWriter<'a, REG, Irmsel>;
impl<'a, REG> IrmselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "ASK mode"]
    #[inline(always)]
    pub fn ask(self) -> &'a mut crate::W<REG> {
        self.variant(Irmsel::Ask)
    }
    #[doc = "FSK mode"]
    #[inline(always)]
    pub fn fsk(self) -> &'a mut crate::W<REG> {
        self.variant(Irmsel::Fsk)
    }
}
#[doc = "Infrared data source select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Irdssel {
    #[doc = "0: From hardware peripherals upon device configuration"]
    Hw = 0,
    #[doc = "1: From IRDATA bit"]
    Irdata = 1,
}
impl From<Irdssel> for bool {
    #[inline(always)]
    fn from(variant: Irdssel) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IRDSSEL` reader - Infrared data source select"]
pub type IrdsselR = crate::BitReader<Irdssel>;
impl IrdsselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Irdssel {
        match self.bits {
            false => Irdssel::Hw,
            true => Irdssel::Irdata,
        }
    }
    #[doc = "From hardware peripherals upon device configuration"]
    #[inline(always)]
    pub fn is_hw(&self) -> bool {
        *self == Irdssel::Hw
    }
    #[doc = "From IRDATA bit"]
    #[inline(always)]
    pub fn is_irdata(&self) -> bool {
        *self == Irdssel::Irdata
    }
}
#[doc = "Field `IRDSSEL` writer - Infrared data source select"]
pub type IrdsselW<'a, REG> = crate::BitWriter<'a, REG, Irdssel>;
impl<'a, REG> IrdsselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "From hardware peripherals upon device configuration"]
    #[inline(always)]
    pub fn hw(self) -> &'a mut crate::W<REG> {
        self.variant(Irdssel::Hw)
    }
    #[doc = "From IRDATA bit"]
    #[inline(always)]
    pub fn irdata(self) -> &'a mut crate::W<REG> {
        self.variant(Irdssel::Irdata)
    }
}
#[doc = "Infrared data\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Irdata {
    #[doc = "0: Infrared data logic 0"]
    Low = 0,
    #[doc = "1: Infrared data logic 1"]
    High = 1,
}
impl From<Irdata> for bool {
    #[inline(always)]
    fn from(variant: Irdata) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IRDATA` reader - Infrared data"]
pub type IrdataR = crate::BitReader<Irdata>;
impl IrdataR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Irdata {
        match self.bits {
            false => Irdata::Low,
            true => Irdata::High,
        }
    }
    #[doc = "Infrared data logic 0"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == Irdata::Low
    }
    #[doc = "Infrared data logic 1"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == Irdata::High
    }
}
#[doc = "Field `IRDATA` writer - Infrared data"]
pub type IrdataW<'a, REG> = crate::BitWriter<'a, REG, Irdata>;
impl<'a, REG> IrdataW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Infrared data logic 0"]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(Irdata::Low)
    }
    #[doc = "Infrared data logic 1"]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(Irdata::High)
    }
}
#[doc = "Captivate Conversion triggered Source Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Syncsel {
    #[doc = "0: External source is selected"]
    Syncsel0 = 0,
    #[doc = "1: ADC as the source is selected"]
    Syncsel1 = 1,
    #[doc = "2: Comparator as the source is selected"]
    Syncsel2 = 2,
    #[doc = "3: Reserved"]
    Syncsel3 = 3,
}
impl From<Syncsel> for u8 {
    #[inline(always)]
    fn from(variant: Syncsel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Syncsel {
    type Ux = u8;
}
impl crate::IsEnum for Syncsel {}
#[doc = "Field `SYNCSEL` reader - Captivate Conversion triggered Source Selection"]
pub type SyncselR = crate::FieldReader<Syncsel>;
impl SyncselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Syncsel {
        match self.bits {
            0 => Syncsel::Syncsel0,
            1 => Syncsel::Syncsel1,
            2 => Syncsel::Syncsel2,
            3 => Syncsel::Syncsel3,
            _ => unreachable!(),
        }
    }
    #[doc = "External source is selected"]
    #[inline(always)]
    pub fn is_syncsel_0(&self) -> bool {
        *self == Syncsel::Syncsel0
    }
    #[doc = "ADC as the source is selected"]
    #[inline(always)]
    pub fn is_syncsel_1(&self) -> bool {
        *self == Syncsel::Syncsel1
    }
    #[doc = "Comparator as the source is selected"]
    #[inline(always)]
    pub fn is_syncsel_2(&self) -> bool {
        *self == Syncsel::Syncsel2
    }
    #[doc = "Reserved"]
    #[inline(always)]
    pub fn is_syncsel_3(&self) -> bool {
        *self == Syncsel::Syncsel3
    }
}
#[doc = "Field `SYNCSEL` writer - Captivate Conversion triggered Source Selection"]
pub type SyncselW<'a, REG> = crate::FieldWriter<'a, REG, 2, Syncsel, crate::Safe>;
impl<'a, REG> SyncselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "External source is selected"]
    #[inline(always)]
    pub fn syncsel_0(self) -> &'a mut crate::W<REG> {
        self.variant(Syncsel::Syncsel0)
    }
    #[doc = "ADC as the source is selected"]
    #[inline(always)]
    pub fn syncsel_1(self) -> &'a mut crate::W<REG> {
        self.variant(Syncsel::Syncsel1)
    }
    #[doc = "Comparator as the source is selected"]
    #[inline(always)]
    pub fn syncsel_2(self) -> &'a mut crate::W<REG> {
        self.variant(Syncsel::Syncsel2)
    }
    #[doc = "Reserved"]
    #[inline(always)]
    pub fn syncsel_3(self) -> &'a mut crate::W<REG> {
        self.variant(Syncsel::Syncsel3)
    }
}
impl R {
    #[doc = "Bit 0 - Infrared enable"]
    #[inline(always)]
    pub fn iren(&self) -> IrenR {
        IrenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Infrared polarity select"]
    #[inline(always)]
    pub fn irpsel(&self) -> IrpselR {
        IrpselR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Infrared mode select"]
    #[inline(always)]
    pub fn irmsel(&self) -> IrmselR {
        IrmselR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Infrared data source select"]
    #[inline(always)]
    pub fn irdssel(&self) -> IrdsselR {
        IrdsselR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Infrared data"]
    #[inline(always)]
    pub fn irdata(&self) -> IrdataR {
        IrdataR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 6:7 - Captivate Conversion triggered Source Selection"]
    #[inline(always)]
    pub fn syncsel(&self) -> SyncselR {
        SyncselR::new(((self.bits >> 6) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Infrared enable"]
    #[inline(always)]
    pub fn iren(&mut self) -> IrenW<Syscfg1Spec> {
        IrenW::new(self, 0)
    }
    #[doc = "Bit 1 - Infrared polarity select"]
    #[inline(always)]
    pub fn irpsel(&mut self) -> IrpselW<Syscfg1Spec> {
        IrpselW::new(self, 1)
    }
    #[doc = "Bit 2 - Infrared mode select"]
    #[inline(always)]
    pub fn irmsel(&mut self) -> IrmselW<Syscfg1Spec> {
        IrmselW::new(self, 2)
    }
    #[doc = "Bit 3 - Infrared data source select"]
    #[inline(always)]
    pub fn irdssel(&mut self) -> IrdsselW<Syscfg1Spec> {
        IrdsselW::new(self, 3)
    }
    #[doc = "Bit 4 - Infrared data"]
    #[inline(always)]
    pub fn irdata(&mut self) -> IrdataW<Syscfg1Spec> {
        IrdataW::new(self, 4)
    }
    #[doc = "Bits 6:7 - Captivate Conversion triggered Source Selection"]
    #[inline(always)]
    pub fn syncsel(&mut self) -> SyncselW<Syscfg1Spec> {
        SyncselW::new(self, 6)
    }
}
#[doc = "System Configuration 1\n\nYou can [`read`](crate::Reg::read) this register and get [`syscfg1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`syscfg1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Syscfg1Spec;
impl crate::RegisterSpec for Syscfg1Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`syscfg1::R`](R) reader structure"]
impl crate::Readable for Syscfg1Spec {}
#[doc = "`write(|w| ..)` method takes [`syscfg1::W`](W) writer structure"]
impl crate::Writable for Syscfg1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SYSCFG1 to value 0"]
impl crate::Resettable for Syscfg1Spec {}
