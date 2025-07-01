#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    tb0ctl: Tb0ctl,
    tb0cctl0: Tb0cctl0,
    tb0cctl1: Tb0cctl1,
    tb0cctl2: Tb0cctl2,
    tb0cctl3: Tb0cctl3,
    tb0cctl4: Tb0cctl4,
    tb0cctl5: Tb0cctl5,
    tb0cctl6: Tb0cctl6,
    tb0r: Tb0r,
    tb0ccr0: Tb0ccr0,
    tb0ccr1: Tb0ccr1,
    tb0ccr2: Tb0ccr2,
    tb0ccr3: Tb0ccr3,
    tb0ccr4: Tb0ccr4,
    tb0ccr5: Tb0ccr5,
    tb0ccr6: Tb0ccr6,
    tb0ex0: Tb0ex0,
    _reserved17: [u8; 0x0c],
    tb0iv: Tb0iv,
}
impl RegisterBlock {
    #[doc = "0x00 - Timer_B Control Register"]
    #[inline(always)]
    pub const fn tb0ctl(&self) -> &Tb0ctl {
        &self.tb0ctl
    }
    #[doc = "0x02 - Timer_B Capture/Compare Control Register"]
    #[inline(always)]
    pub const fn tb0cctl0(&self) -> &Tb0cctl0 {
        &self.tb0cctl0
    }
    #[doc = "0x04 - Timer_B Capture/Compare Control Register"]
    #[inline(always)]
    pub const fn tb0cctl1(&self) -> &Tb0cctl1 {
        &self.tb0cctl1
    }
    #[doc = "0x06 - Timer_B Capture/Compare Control Register"]
    #[inline(always)]
    pub const fn tb0cctl2(&self) -> &Tb0cctl2 {
        &self.tb0cctl2
    }
    #[doc = "0x08 - Timer_B Capture/Compare Control Register"]
    #[inline(always)]
    pub const fn tb0cctl3(&self) -> &Tb0cctl3 {
        &self.tb0cctl3
    }
    #[doc = "0x0a - Timer_B Capture/Compare Control Register"]
    #[inline(always)]
    pub const fn tb0cctl4(&self) -> &Tb0cctl4 {
        &self.tb0cctl4
    }
    #[doc = "0x0c - Timer_B Capture/Compare Control Register"]
    #[inline(always)]
    pub const fn tb0cctl5(&self) -> &Tb0cctl5 {
        &self.tb0cctl5
    }
    #[doc = "0x0e - Timer_B Capture/Compare Control Register"]
    #[inline(always)]
    pub const fn tb0cctl6(&self) -> &Tb0cctl6 {
        &self.tb0cctl6
    }
    #[doc = "0x10 - Timer_B count register"]
    #[inline(always)]
    pub const fn tb0r(&self) -> &Tb0r {
        &self.tb0r
    }
    #[doc = "0x12 - Timer_B Capture/Compare Register"]
    #[inline(always)]
    pub const fn tb0ccr0(&self) -> &Tb0ccr0 {
        &self.tb0ccr0
    }
    #[doc = "0x14 - Timer_B Capture/Compare Register"]
    #[inline(always)]
    pub const fn tb0ccr1(&self) -> &Tb0ccr1 {
        &self.tb0ccr1
    }
    #[doc = "0x16 - Timer_B Capture/Compare Register"]
    #[inline(always)]
    pub const fn tb0ccr2(&self) -> &Tb0ccr2 {
        &self.tb0ccr2
    }
    #[doc = "0x18 - Timer_B Capture/Compare Register"]
    #[inline(always)]
    pub const fn tb0ccr3(&self) -> &Tb0ccr3 {
        &self.tb0ccr3
    }
    #[doc = "0x1a - Timer_B Capture/Compare Register"]
    #[inline(always)]
    pub const fn tb0ccr4(&self) -> &Tb0ccr4 {
        &self.tb0ccr4
    }
    #[doc = "0x1c - Timer_B Capture/Compare Register"]
    #[inline(always)]
    pub const fn tb0ccr5(&self) -> &Tb0ccr5 {
        &self.tb0ccr5
    }
    #[doc = "0x1e - Timer_B Capture/Compare Register"]
    #[inline(always)]
    pub const fn tb0ccr6(&self) -> &Tb0ccr6 {
        &self.tb0ccr6
    }
    #[doc = "0x20 - Timer_Bx Expansion Register 0"]
    #[inline(always)]
    pub const fn tb0ex0(&self) -> &Tb0ex0 {
        &self.tb0ex0
    }
    #[doc = "0x2e - Timer_Bx Interrupt Vector Register"]
    #[inline(always)]
    pub const fn tb0iv(&self) -> &Tb0iv {
        &self.tb0iv
    }
}
#[doc = "TB0CTL (rw) register accessor: Timer_B Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tb0ctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tb0ctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tb0ctl`] module"]
#[doc(alias = "TB0CTL")]
pub type Tb0ctl = crate::Reg<tb0ctl::Tb0ctlSpec>;
#[doc = "Timer_B Control Register"]
pub mod tb0ctl;
#[doc = "TB0CCTL0 (rw) register accessor: Timer_B Capture/Compare Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tb0cctl0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tb0cctl0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tb0cctl0`] module"]
#[doc(alias = "TB0CCTL0")]
pub type Tb0cctl0 = crate::Reg<tb0cctl0::Tb0cctl0Spec>;
#[doc = "Timer_B Capture/Compare Control Register"]
pub mod tb0cctl0;
#[doc = "TB0CCTL1 (rw) register accessor: Timer_B Capture/Compare Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tb0cctl1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tb0cctl1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tb0cctl1`] module"]
#[doc(alias = "TB0CCTL1")]
pub type Tb0cctl1 = crate::Reg<tb0cctl1::Tb0cctl1Spec>;
#[doc = "Timer_B Capture/Compare Control Register"]
pub mod tb0cctl1;
#[doc = "TB0CCTL2 (rw) register accessor: Timer_B Capture/Compare Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tb0cctl2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tb0cctl2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tb0cctl2`] module"]
#[doc(alias = "TB0CCTL2")]
pub type Tb0cctl2 = crate::Reg<tb0cctl2::Tb0cctl2Spec>;
#[doc = "Timer_B Capture/Compare Control Register"]
pub mod tb0cctl2;
#[doc = "TB0CCTL3 (rw) register accessor: Timer_B Capture/Compare Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tb0cctl3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tb0cctl3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tb0cctl3`] module"]
#[doc(alias = "TB0CCTL3")]
pub type Tb0cctl3 = crate::Reg<tb0cctl3::Tb0cctl3Spec>;
#[doc = "Timer_B Capture/Compare Control Register"]
pub mod tb0cctl3;
#[doc = "TB0CCTL4 (rw) register accessor: Timer_B Capture/Compare Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tb0cctl4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tb0cctl4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tb0cctl4`] module"]
#[doc(alias = "TB0CCTL4")]
pub type Tb0cctl4 = crate::Reg<tb0cctl4::Tb0cctl4Spec>;
#[doc = "Timer_B Capture/Compare Control Register"]
pub mod tb0cctl4;
#[doc = "TB0CCTL5 (rw) register accessor: Timer_B Capture/Compare Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tb0cctl5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tb0cctl5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tb0cctl5`] module"]
#[doc(alias = "TB0CCTL5")]
pub type Tb0cctl5 = crate::Reg<tb0cctl5::Tb0cctl5Spec>;
#[doc = "Timer_B Capture/Compare Control Register"]
pub mod tb0cctl5;
#[doc = "TB0CCTL6 (rw) register accessor: Timer_B Capture/Compare Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tb0cctl6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tb0cctl6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tb0cctl6`] module"]
#[doc(alias = "TB0CCTL6")]
pub type Tb0cctl6 = crate::Reg<tb0cctl6::Tb0cctl6Spec>;
#[doc = "Timer_B Capture/Compare Control Register"]
pub mod tb0cctl6;
#[doc = "TB0R (rw) register accessor: Timer_B count register\n\nYou can [`read`](crate::Reg::read) this register and get [`tb0r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tb0r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tb0r`] module"]
#[doc(alias = "TB0R")]
pub type Tb0r = crate::Reg<tb0r::Tb0rSpec>;
#[doc = "Timer_B count register"]
pub mod tb0r;
#[doc = "TB0CCR0 (rw) register accessor: Timer_B Capture/Compare Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tb0ccr0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tb0ccr0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tb0ccr0`] module"]
#[doc(alias = "TB0CCR0")]
pub type Tb0ccr0 = crate::Reg<tb0ccr0::Tb0ccr0Spec>;
#[doc = "Timer_B Capture/Compare Register"]
pub mod tb0ccr0;
#[doc = "TB0CCR1 (rw) register accessor: Timer_B Capture/Compare Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tb0ccr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tb0ccr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tb0ccr1`] module"]
#[doc(alias = "TB0CCR1")]
pub type Tb0ccr1 = crate::Reg<tb0ccr1::Tb0ccr1Spec>;
#[doc = "Timer_B Capture/Compare Register"]
pub mod tb0ccr1;
#[doc = "TB0CCR2 (rw) register accessor: Timer_B Capture/Compare Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tb0ccr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tb0ccr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tb0ccr2`] module"]
#[doc(alias = "TB0CCR2")]
pub type Tb0ccr2 = crate::Reg<tb0ccr2::Tb0ccr2Spec>;
#[doc = "Timer_B Capture/Compare Register"]
pub mod tb0ccr2;
#[doc = "TB0CCR3 (rw) register accessor: Timer_B Capture/Compare Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tb0ccr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tb0ccr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tb0ccr3`] module"]
#[doc(alias = "TB0CCR3")]
pub type Tb0ccr3 = crate::Reg<tb0ccr3::Tb0ccr3Spec>;
#[doc = "Timer_B Capture/Compare Register"]
pub mod tb0ccr3;
#[doc = "TB0CCR4 (rw) register accessor: Timer_B Capture/Compare Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tb0ccr4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tb0ccr4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tb0ccr4`] module"]
#[doc(alias = "TB0CCR4")]
pub type Tb0ccr4 = crate::Reg<tb0ccr4::Tb0ccr4Spec>;
#[doc = "Timer_B Capture/Compare Register"]
pub mod tb0ccr4;
#[doc = "TB0CCR5 (rw) register accessor: Timer_B Capture/Compare Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tb0ccr5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tb0ccr5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tb0ccr5`] module"]
#[doc(alias = "TB0CCR5")]
pub type Tb0ccr5 = crate::Reg<tb0ccr5::Tb0ccr5Spec>;
#[doc = "Timer_B Capture/Compare Register"]
pub mod tb0ccr5;
#[doc = "TB0CCR6 (rw) register accessor: Timer_B Capture/Compare Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tb0ccr6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tb0ccr6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tb0ccr6`] module"]
#[doc(alias = "TB0CCR6")]
pub type Tb0ccr6 = crate::Reg<tb0ccr6::Tb0ccr6Spec>;
#[doc = "Timer_B Capture/Compare Register"]
pub mod tb0ccr6;
#[doc = "TB0EX0 (rw) register accessor: Timer_Bx Expansion Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`tb0ex0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tb0ex0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tb0ex0`] module"]
#[doc(alias = "TB0EX0")]
pub type Tb0ex0 = crate::Reg<tb0ex0::Tb0ex0Spec>;
#[doc = "Timer_Bx Expansion Register 0"]
pub mod tb0ex0;
#[doc = "TB0IV (rw) register accessor: Timer_Bx Interrupt Vector Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tb0iv::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tb0iv::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tb0iv`] module"]
#[doc(alias = "TB0IV")]
pub type Tb0iv = crate::Reg<tb0iv::Tb0ivSpec>;
#[doc = "Timer_Bx Interrupt Vector Register"]
pub mod tb0iv;
