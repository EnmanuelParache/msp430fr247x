#[doc = "Register `TA1CCTL0` reader"]
pub struct R(crate::R<TA1CCTL0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TA1CCTL0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TA1CCTL0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TA1CCTL0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TA1CCTL0` writer"]
pub struct W(crate::W<TA1CCTL0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TA1CCTL0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<TA1CCTL0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TA1CCTL0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CCIFG` reader - Capture/compare interrupt flag"]
pub type CCIFG_R = crate::BitReader<CCIFG_A>;
#[doc = "Capture/compare interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CCIFG_A {
    #[doc = "0: No interrupt pending"]
    CCIFG_0 = 0,
    #[doc = "1: Interrupt pending"]
    CCIFG_1 = 1,
}
impl From<CCIFG_A> for bool {
    #[inline(always)]
    fn from(variant: CCIFG_A) -> Self {
        variant as u8 != 0
    }
}
impl CCIFG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CCIFG_A {
        match self.bits {
            false => CCIFG_A::CCIFG_0,
            true => CCIFG_A::CCIFG_1,
        }
    }
    #[doc = "Checks if the value of the field is `CCIFG_0`"]
    #[inline(always)]
    pub fn is_ccifg_0(&self) -> bool {
        *self == CCIFG_A::CCIFG_0
    }
    #[doc = "Checks if the value of the field is `CCIFG_1`"]
    #[inline(always)]
    pub fn is_ccifg_1(&self) -> bool {
        *self == CCIFG_A::CCIFG_1
    }
}
#[doc = "Field `CCIFG` writer - Capture/compare interrupt flag"]
pub type CCIFG_W<'a, const O: u8> = crate::BitWriter<'a, u16, TA1CCTL0_SPEC, CCIFG_A, O>;
impl<'a, const O: u8> CCIFG_W<'a, O> {
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn ccifg_0(self) -> &'a mut W {
        self.variant(CCIFG_A::CCIFG_0)
    }
    #[doc = "Interrupt pending"]
    #[inline(always)]
    pub fn ccifg_1(self) -> &'a mut W {
        self.variant(CCIFG_A::CCIFG_1)
    }
}
#[doc = "Field `COV` reader - Capture overflow"]
pub type COV_R = crate::BitReader<COV_A>;
#[doc = "Capture overflow\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum COV_A {
    #[doc = "0: No capture overflow occurred"]
    COV_0 = 0,
    #[doc = "1: Capture overflow occurred"]
    COV_1 = 1,
}
impl From<COV_A> for bool {
    #[inline(always)]
    fn from(variant: COV_A) -> Self {
        variant as u8 != 0
    }
}
impl COV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> COV_A {
        match self.bits {
            false => COV_A::COV_0,
            true => COV_A::COV_1,
        }
    }
    #[doc = "Checks if the value of the field is `COV_0`"]
    #[inline(always)]
    pub fn is_cov_0(&self) -> bool {
        *self == COV_A::COV_0
    }
    #[doc = "Checks if the value of the field is `COV_1`"]
    #[inline(always)]
    pub fn is_cov_1(&self) -> bool {
        *self == COV_A::COV_1
    }
}
#[doc = "Field `COV` writer - Capture overflow"]
pub type COV_W<'a, const O: u8> = crate::BitWriter<'a, u16, TA1CCTL0_SPEC, COV_A, O>;
impl<'a, const O: u8> COV_W<'a, O> {
    #[doc = "No capture overflow occurred"]
    #[inline(always)]
    pub fn cov_0(self) -> &'a mut W {
        self.variant(COV_A::COV_0)
    }
    #[doc = "Capture overflow occurred"]
    #[inline(always)]
    pub fn cov_1(self) -> &'a mut W {
        self.variant(COV_A::COV_1)
    }
}
#[doc = "Field `OUT` reader - Output"]
pub type OUT_R = crate::BitReader<OUT_A>;
#[doc = "Output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OUT_A {
    #[doc = "0: Output low"]
    LOW = 0,
    #[doc = "1: Output high"]
    HIGH = 1,
}
impl From<OUT_A> for bool {
    #[inline(always)]
    fn from(variant: OUT_A) -> Self {
        variant as u8 != 0
    }
}
impl OUT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OUT_A {
        match self.bits {
            false => OUT_A::LOW,
            true => OUT_A::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == OUT_A::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == OUT_A::HIGH
    }
}
#[doc = "Field `OUT` writer - Output"]
pub type OUT_W<'a, const O: u8> = crate::BitWriter<'a, u16, TA1CCTL0_SPEC, OUT_A, O>;
impl<'a, const O: u8> OUT_W<'a, O> {
    #[doc = "Output low"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(OUT_A::LOW)
    }
    #[doc = "Output high"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(OUT_A::HIGH)
    }
}
#[doc = "Field `CCI` reader - Capture/compare input"]
pub type CCI_R = crate::BitReader<bool>;
#[doc = "Field `CCIE` reader - Capture/compare interrupt enable"]
pub type CCIE_R = crate::BitReader<CCIE_A>;
#[doc = "Capture/compare interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CCIE_A {
    #[doc = "0: Interrupt disabled"]
    CCIE_0 = 0,
    #[doc = "1: Interrupt enabled"]
    CCIE_1 = 1,
}
impl From<CCIE_A> for bool {
    #[inline(always)]
    fn from(variant: CCIE_A) -> Self {
        variant as u8 != 0
    }
}
impl CCIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CCIE_A {
        match self.bits {
            false => CCIE_A::CCIE_0,
            true => CCIE_A::CCIE_1,
        }
    }
    #[doc = "Checks if the value of the field is `CCIE_0`"]
    #[inline(always)]
    pub fn is_ccie_0(&self) -> bool {
        *self == CCIE_A::CCIE_0
    }
    #[doc = "Checks if the value of the field is `CCIE_1`"]
    #[inline(always)]
    pub fn is_ccie_1(&self) -> bool {
        *self == CCIE_A::CCIE_1
    }
}
#[doc = "Field `CCIE` writer - Capture/compare interrupt enable"]
pub type CCIE_W<'a, const O: u8> = crate::BitWriter<'a, u16, TA1CCTL0_SPEC, CCIE_A, O>;
impl<'a, const O: u8> CCIE_W<'a, O> {
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn ccie_0(self) -> &'a mut W {
        self.variant(CCIE_A::CCIE_0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn ccie_1(self) -> &'a mut W {
        self.variant(CCIE_A::CCIE_1)
    }
}
#[doc = "Field `OUTMOD` reader - Output mode"]
pub type OUTMOD_R = crate::FieldReader<u8, OUTMOD_A>;
#[doc = "Output mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum OUTMOD_A {
    #[doc = "0: OUT bit value"]
    OUTMOD_0 = 0,
    #[doc = "1: Set"]
    OUTMOD_1 = 1,
    #[doc = "2: Toggle/reset"]
    OUTMOD_2 = 2,
    #[doc = "3: Set/reset"]
    OUTMOD_3 = 3,
    #[doc = "4: Toggle"]
    OUTMOD_4 = 4,
    #[doc = "5: Reset"]
    OUTMOD_5 = 5,
    #[doc = "6: Toggle/set"]
    OUTMOD_6 = 6,
    #[doc = "7: Reset/set"]
    OUTMOD_7 = 7,
}
impl From<OUTMOD_A> for u8 {
    #[inline(always)]
    fn from(variant: OUTMOD_A) -> Self {
        variant as _
    }
}
impl OUTMOD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OUTMOD_A {
        match self.bits {
            0 => OUTMOD_A::OUTMOD_0,
            1 => OUTMOD_A::OUTMOD_1,
            2 => OUTMOD_A::OUTMOD_2,
            3 => OUTMOD_A::OUTMOD_3,
            4 => OUTMOD_A::OUTMOD_4,
            5 => OUTMOD_A::OUTMOD_5,
            6 => OUTMOD_A::OUTMOD_6,
            7 => OUTMOD_A::OUTMOD_7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `OUTMOD_0`"]
    #[inline(always)]
    pub fn is_outmod_0(&self) -> bool {
        *self == OUTMOD_A::OUTMOD_0
    }
    #[doc = "Checks if the value of the field is `OUTMOD_1`"]
    #[inline(always)]
    pub fn is_outmod_1(&self) -> bool {
        *self == OUTMOD_A::OUTMOD_1
    }
    #[doc = "Checks if the value of the field is `OUTMOD_2`"]
    #[inline(always)]
    pub fn is_outmod_2(&self) -> bool {
        *self == OUTMOD_A::OUTMOD_2
    }
    #[doc = "Checks if the value of the field is `OUTMOD_3`"]
    #[inline(always)]
    pub fn is_outmod_3(&self) -> bool {
        *self == OUTMOD_A::OUTMOD_3
    }
    #[doc = "Checks if the value of the field is `OUTMOD_4`"]
    #[inline(always)]
    pub fn is_outmod_4(&self) -> bool {
        *self == OUTMOD_A::OUTMOD_4
    }
    #[doc = "Checks if the value of the field is `OUTMOD_5`"]
    #[inline(always)]
    pub fn is_outmod_5(&self) -> bool {
        *self == OUTMOD_A::OUTMOD_5
    }
    #[doc = "Checks if the value of the field is `OUTMOD_6`"]
    #[inline(always)]
    pub fn is_outmod_6(&self) -> bool {
        *self == OUTMOD_A::OUTMOD_6
    }
    #[doc = "Checks if the value of the field is `OUTMOD_7`"]
    #[inline(always)]
    pub fn is_outmod_7(&self) -> bool {
        *self == OUTMOD_A::OUTMOD_7
    }
}
#[doc = "Field `OUTMOD` writer - Output mode"]
pub type OUTMOD_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u16, TA1CCTL0_SPEC, u8, OUTMOD_A, 3, O>;
impl<'a, const O: u8> OUTMOD_W<'a, O> {
    #[doc = "OUT bit value"]
    #[inline(always)]
    pub fn outmod_0(self) -> &'a mut W {
        self.variant(OUTMOD_A::OUTMOD_0)
    }
    #[doc = "Set"]
    #[inline(always)]
    pub fn outmod_1(self) -> &'a mut W {
        self.variant(OUTMOD_A::OUTMOD_1)
    }
    #[doc = "Toggle/reset"]
    #[inline(always)]
    pub fn outmod_2(self) -> &'a mut W {
        self.variant(OUTMOD_A::OUTMOD_2)
    }
    #[doc = "Set/reset"]
    #[inline(always)]
    pub fn outmod_3(self) -> &'a mut W {
        self.variant(OUTMOD_A::OUTMOD_3)
    }
    #[doc = "Toggle"]
    #[inline(always)]
    pub fn outmod_4(self) -> &'a mut W {
        self.variant(OUTMOD_A::OUTMOD_4)
    }
    #[doc = "Reset"]
    #[inline(always)]
    pub fn outmod_5(self) -> &'a mut W {
        self.variant(OUTMOD_A::OUTMOD_5)
    }
    #[doc = "Toggle/set"]
    #[inline(always)]
    pub fn outmod_6(self) -> &'a mut W {
        self.variant(OUTMOD_A::OUTMOD_6)
    }
    #[doc = "Reset/set"]
    #[inline(always)]
    pub fn outmod_7(self) -> &'a mut W {
        self.variant(OUTMOD_A::OUTMOD_7)
    }
}
#[doc = "Field `CAP` reader - Capture mode"]
pub type CAP_R = crate::BitReader<CAP_A>;
#[doc = "Capture mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CAP_A {
    #[doc = "0: Compare mode"]
    COMPARE = 0,
    #[doc = "1: Capture mode"]
    CAPTURE = 1,
}
impl From<CAP_A> for bool {
    #[inline(always)]
    fn from(variant: CAP_A) -> Self {
        variant as u8 != 0
    }
}
impl CAP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CAP_A {
        match self.bits {
            false => CAP_A::COMPARE,
            true => CAP_A::CAPTURE,
        }
    }
    #[doc = "Checks if the value of the field is `COMPARE`"]
    #[inline(always)]
    pub fn is_compare(&self) -> bool {
        *self == CAP_A::COMPARE
    }
    #[doc = "Checks if the value of the field is `CAPTURE`"]
    #[inline(always)]
    pub fn is_capture(&self) -> bool {
        *self == CAP_A::CAPTURE
    }
}
#[doc = "Field `CAP` writer - Capture mode"]
pub type CAP_W<'a, const O: u8> = crate::BitWriter<'a, u16, TA1CCTL0_SPEC, CAP_A, O>;
impl<'a, const O: u8> CAP_W<'a, O> {
    #[doc = "Compare mode"]
    #[inline(always)]
    pub fn compare(self) -> &'a mut W {
        self.variant(CAP_A::COMPARE)
    }
    #[doc = "Capture mode"]
    #[inline(always)]
    pub fn capture(self) -> &'a mut W {
        self.variant(CAP_A::CAPTURE)
    }
}
#[doc = "Field `SCCI` reader - Synchronized capture/compare input"]
pub type SCCI_R = crate::BitReader<bool>;
#[doc = "Field `SCCI` writer - Synchronized capture/compare input"]
pub type SCCI_W<'a, const O: u8> = crate::BitWriter<'a, u16, TA1CCTL0_SPEC, bool, O>;
#[doc = "Field `SCS` reader - Synchronize capture source"]
pub type SCS_R = crate::BitReader<SCS_A>;
#[doc = "Synchronize capture source\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SCS_A {
    #[doc = "0: Asynchronous capture"]
    ASYNC = 0,
    #[doc = "1: Synchronous capture"]
    SYNC = 1,
}
impl From<SCS_A> for bool {
    #[inline(always)]
    fn from(variant: SCS_A) -> Self {
        variant as u8 != 0
    }
}
impl SCS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SCS_A {
        match self.bits {
            false => SCS_A::ASYNC,
            true => SCS_A::SYNC,
        }
    }
    #[doc = "Checks if the value of the field is `ASYNC`"]
    #[inline(always)]
    pub fn is_async(&self) -> bool {
        *self == SCS_A::ASYNC
    }
    #[doc = "Checks if the value of the field is `SYNC`"]
    #[inline(always)]
    pub fn is_sync(&self) -> bool {
        *self == SCS_A::SYNC
    }
}
#[doc = "Field `SCS` writer - Synchronize capture source"]
pub type SCS_W<'a, const O: u8> = crate::BitWriter<'a, u16, TA1CCTL0_SPEC, SCS_A, O>;
impl<'a, const O: u8> SCS_W<'a, O> {
    #[doc = "Asynchronous capture"]
    #[inline(always)]
    pub fn async_(self) -> &'a mut W {
        self.variant(SCS_A::ASYNC)
    }
    #[doc = "Synchronous capture"]
    #[inline(always)]
    pub fn sync(self) -> &'a mut W {
        self.variant(SCS_A::SYNC)
    }
}
#[doc = "Field `CCIS` reader - Capture/compare input select"]
pub type CCIS_R = crate::FieldReader<u8, CCIS_A>;
#[doc = "Capture/compare input select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CCIS_A {
    #[doc = "0: CCIxA"]
    CCIA = 0,
    #[doc = "1: CCIxB"]
    CCIB = 1,
    #[doc = "2: GND"]
    GND = 2,
    #[doc = "3: VCC"]
    VCC = 3,
}
impl From<CCIS_A> for u8 {
    #[inline(always)]
    fn from(variant: CCIS_A) -> Self {
        variant as _
    }
}
impl CCIS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CCIS_A {
        match self.bits {
            0 => CCIS_A::CCIA,
            1 => CCIS_A::CCIB,
            2 => CCIS_A::GND,
            3 => CCIS_A::VCC,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `CCIA`"]
    #[inline(always)]
    pub fn is_ccia(&self) -> bool {
        *self == CCIS_A::CCIA
    }
    #[doc = "Checks if the value of the field is `CCIB`"]
    #[inline(always)]
    pub fn is_ccib(&self) -> bool {
        *self == CCIS_A::CCIB
    }
    #[doc = "Checks if the value of the field is `GND`"]
    #[inline(always)]
    pub fn is_gnd(&self) -> bool {
        *self == CCIS_A::GND
    }
    #[doc = "Checks if the value of the field is `VCC`"]
    #[inline(always)]
    pub fn is_vcc(&self) -> bool {
        *self == CCIS_A::VCC
    }
}
#[doc = "Field `CCIS` writer - Capture/compare input select"]
pub type CCIS_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u16, TA1CCTL0_SPEC, u8, CCIS_A, 2, O>;
impl<'a, const O: u8> CCIS_W<'a, O> {
    #[doc = "CCIxA"]
    #[inline(always)]
    pub fn ccia(self) -> &'a mut W {
        self.variant(CCIS_A::CCIA)
    }
    #[doc = "CCIxB"]
    #[inline(always)]
    pub fn ccib(self) -> &'a mut W {
        self.variant(CCIS_A::CCIB)
    }
    #[doc = "GND"]
    #[inline(always)]
    pub fn gnd(self) -> &'a mut W {
        self.variant(CCIS_A::GND)
    }
    #[doc = "VCC"]
    #[inline(always)]
    pub fn vcc(self) -> &'a mut W {
        self.variant(CCIS_A::VCC)
    }
}
#[doc = "Field `CM` reader - Capture mode"]
pub type CM_R = crate::FieldReader<u8, CM_A>;
#[doc = "Capture mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CM_A {
    #[doc = "0: No capture"]
    NONE = 0,
    #[doc = "1: Capture on rising edge"]
    RISING = 1,
    #[doc = "2: Capture on falling edge"]
    FALLING = 2,
    #[doc = "3: Capture on both rising and falling edges"]
    BOTH = 3,
}
impl From<CM_A> for u8 {
    #[inline(always)]
    fn from(variant: CM_A) -> Self {
        variant as _
    }
}
impl CM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CM_A {
        match self.bits {
            0 => CM_A::NONE,
            1 => CM_A::RISING,
            2 => CM_A::FALLING,
            3 => CM_A::BOTH,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == CM_A::NONE
    }
    #[doc = "Checks if the value of the field is `RISING`"]
    #[inline(always)]
    pub fn is_rising(&self) -> bool {
        *self == CM_A::RISING
    }
    #[doc = "Checks if the value of the field is `FALLING`"]
    #[inline(always)]
    pub fn is_falling(&self) -> bool {
        *self == CM_A::FALLING
    }
    #[doc = "Checks if the value of the field is `BOTH`"]
    #[inline(always)]
    pub fn is_both(&self) -> bool {
        *self == CM_A::BOTH
    }
}
#[doc = "Field `CM` writer - Capture mode"]
pub type CM_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u16, TA1CCTL0_SPEC, u8, CM_A, 2, O>;
impl<'a, const O: u8> CM_W<'a, O> {
    #[doc = "No capture"]
    #[inline(always)]
    pub fn none(self) -> &'a mut W {
        self.variant(CM_A::NONE)
    }
    #[doc = "Capture on rising edge"]
    #[inline(always)]
    pub fn rising(self) -> &'a mut W {
        self.variant(CM_A::RISING)
    }
    #[doc = "Capture on falling edge"]
    #[inline(always)]
    pub fn falling(self) -> &'a mut W {
        self.variant(CM_A::FALLING)
    }
    #[doc = "Capture on both rising and falling edges"]
    #[inline(always)]
    pub fn both(self) -> &'a mut W {
        self.variant(CM_A::BOTH)
    }
}
impl R {
    #[doc = "Bit 0 - Capture/compare interrupt flag"]
    #[inline(always)]
    pub fn ccifg(&self) -> CCIFG_R {
        CCIFG_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Capture overflow"]
    #[inline(always)]
    pub fn cov(&self) -> COV_R {
        COV_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Output"]
    #[inline(always)]
    pub fn out(&self) -> OUT_R {
        OUT_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Capture/compare input"]
    #[inline(always)]
    pub fn cci(&self) -> CCI_R {
        CCI_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Capture/compare interrupt enable"]
    #[inline(always)]
    pub fn ccie(&self) -> CCIE_R {
        CCIE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:7 - Output mode"]
    #[inline(always)]
    pub fn outmod(&self) -> OUTMOD_R {
        OUTMOD_R::new(((self.bits >> 5) & 7) as u8)
    }
    #[doc = "Bit 8 - Capture mode"]
    #[inline(always)]
    pub fn cap(&self) -> CAP_R {
        CAP_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 10 - Synchronized capture/compare input"]
    #[inline(always)]
    pub fn scci(&self) -> SCCI_R {
        SCCI_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Synchronize capture source"]
    #[inline(always)]
    pub fn scs(&self) -> SCS_R {
        SCS_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:13 - Capture/compare input select"]
    #[inline(always)]
    pub fn ccis(&self) -> CCIS_R {
        CCIS_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - Capture mode"]
    #[inline(always)]
    pub fn cm(&self) -> CM_R {
        CM_R::new(((self.bits >> 14) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Capture/compare interrupt flag"]
    #[inline(always)]
    #[must_use]
    pub fn ccifg(&mut self) -> CCIFG_W<0> {
        CCIFG_W::new(self)
    }
    #[doc = "Bit 1 - Capture overflow"]
    #[inline(always)]
    #[must_use]
    pub fn cov(&mut self) -> COV_W<1> {
        COV_W::new(self)
    }
    #[doc = "Bit 2 - Output"]
    #[inline(always)]
    #[must_use]
    pub fn out(&mut self) -> OUT_W<2> {
        OUT_W::new(self)
    }
    #[doc = "Bit 4 - Capture/compare interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn ccie(&mut self) -> CCIE_W<4> {
        CCIE_W::new(self)
    }
    #[doc = "Bits 5:7 - Output mode"]
    #[inline(always)]
    #[must_use]
    pub fn outmod(&mut self) -> OUTMOD_W<5> {
        OUTMOD_W::new(self)
    }
    #[doc = "Bit 8 - Capture mode"]
    #[inline(always)]
    #[must_use]
    pub fn cap(&mut self) -> CAP_W<8> {
        CAP_W::new(self)
    }
    #[doc = "Bit 10 - Synchronized capture/compare input"]
    #[inline(always)]
    #[must_use]
    pub fn scci(&mut self) -> SCCI_W<10> {
        SCCI_W::new(self)
    }
    #[doc = "Bit 11 - Synchronize capture source"]
    #[inline(always)]
    #[must_use]
    pub fn scs(&mut self) -> SCS_W<11> {
        SCS_W::new(self)
    }
    #[doc = "Bits 12:13 - Capture/compare input select"]
    #[inline(always)]
    #[must_use]
    pub fn ccis(&mut self) -> CCIS_W<12> {
        CCIS_W::new(self)
    }
    #[doc = "Bits 14:15 - Capture mode"]
    #[inline(always)]
    #[must_use]
    pub fn cm(&mut self) -> CM_W<14> {
        CM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Timer_A Capture/Compare Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ta1cctl0](index.html) module"]
pub struct TA1CCTL0_SPEC;
impl crate::RegisterSpec for TA1CCTL0_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [ta1cctl0::R](R) reader structure"]
impl crate::Readable for TA1CCTL0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ta1cctl0::W](W) writer structure"]
impl crate::Writable for TA1CCTL0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TA1CCTL0 to value 0"]
impl crate::Resettable for TA1CCTL0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
