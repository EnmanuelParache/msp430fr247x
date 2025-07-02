#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved0: [u8; 0x01],
    p6in: P6in,
    _reserved1: [u8; 0x01],
    p6out: P6out,
    _reserved2: [u8; 0x01],
    p6dir: P6dir,
    _reserved3: [u8; 0x01],
    p6ren: P6ren,
    _reserved4: [u8; 0x03],
    p6sel0: P6sel0,
    _reserved5: [u8; 0x01],
    p6sel1: P6sel1,
    _reserved6: [u8; 0x09],
    p6selc: P6selc,
    _reserved7: [u8; 0x01],
    p6ies: P6ies,
    _reserved8: [u8; 0x01],
    p6ie: P6ie,
    _reserved9: [u8; 0x01],
    p6ifg: P6ifg,
    p6iv: P6iv,
}
impl RegisterBlock {
    #[doc = "0x01 - Port 6 Input"]
    #[inline(always)]
    pub const fn p6in(&self) -> &P6in {
        &self.p6in
    }
    #[doc = "0x03 - Port 6 Output"]
    #[inline(always)]
    pub const fn p6out(&self) -> &P6out {
        &self.p6out
    }
    #[doc = "0x05 - Port 6 Direction"]
    #[inline(always)]
    pub const fn p6dir(&self) -> &P6dir {
        &self.p6dir
    }
    #[doc = "0x07 - Port 6 Resistor Enable"]
    #[inline(always)]
    pub const fn p6ren(&self) -> &P6ren {
        &self.p6ren
    }
    #[doc = "0x0b - Port 6 Select 0"]
    #[inline(always)]
    pub const fn p6sel0(&self) -> &P6sel0 {
        &self.p6sel0
    }
    #[doc = "0x0d - Port 6 Select 1"]
    #[inline(always)]
    pub const fn p6sel1(&self) -> &P6sel1 {
        &self.p6sel1
    }
    #[doc = "0x17 - Port 6 Complement Select"]
    #[inline(always)]
    pub const fn p6selc(&self) -> &P6selc {
        &self.p6selc
    }
    #[doc = "0x19 - Port 6 Interrupt Edge Select"]
    #[inline(always)]
    pub const fn p6ies(&self) -> &P6ies {
        &self.p6ies
    }
    #[doc = "0x1b - Port 6 Interrupt Enable"]
    #[inline(always)]
    pub const fn p6ie(&self) -> &P6ie {
        &self.p6ie
    }
    #[doc = "0x1d - Port 6 Interrupt Flag"]
    #[inline(always)]
    pub const fn p6ifg(&self) -> &P6ifg {
        &self.p6ifg
    }
    #[doc = "0x1e - Port 6 Interrupt Vector Register"]
    #[inline(always)]
    pub const fn p6iv(&self) -> &P6iv {
        &self.p6iv
    }
}
#[doc = "P6IN (rw) register accessor: Port 6 Input\n\nYou can [`read`](crate::Reg::read) this register and get [`p6in::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p6in::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p6in`] module"]
#[doc(alias = "P6IN")]
pub type P6in = crate::Reg<p6in::P6inSpec>;
#[doc = "Port 6 Input"]
pub mod p6in;
#[doc = "P6OUT (rw) register accessor: Port 6 Output\n\nYou can [`read`](crate::Reg::read) this register and get [`p6out::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p6out::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p6out`] module"]
#[doc(alias = "P6OUT")]
pub type P6out = crate::Reg<p6out::P6outSpec>;
#[doc = "Port 6 Output"]
pub mod p6out;
#[doc = "P6DIR (rw) register accessor: Port 6 Direction\n\nYou can [`read`](crate::Reg::read) this register and get [`p6dir::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p6dir::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p6dir`] module"]
#[doc(alias = "P6DIR")]
pub type P6dir = crate::Reg<p6dir::P6dirSpec>;
#[doc = "Port 6 Direction"]
pub mod p6dir;
#[doc = "P6REN (rw) register accessor: Port 6 Resistor Enable\n\nYou can [`read`](crate::Reg::read) this register and get [`p6ren::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p6ren::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p6ren`] module"]
#[doc(alias = "P6REN")]
pub type P6ren = crate::Reg<p6ren::P6renSpec>;
#[doc = "Port 6 Resistor Enable"]
pub mod p6ren;
#[doc = "P6SEL0 (rw) register accessor: Port 6 Select 0\n\nYou can [`read`](crate::Reg::read) this register and get [`p6sel0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p6sel0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p6sel0`] module"]
#[doc(alias = "P6SEL0")]
pub type P6sel0 = crate::Reg<p6sel0::P6sel0Spec>;
#[doc = "Port 6 Select 0"]
pub mod p6sel0;
#[doc = "P6SEL1 (rw) register accessor: Port 6 Select 1\n\nYou can [`read`](crate::Reg::read) this register and get [`p6sel1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p6sel1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p6sel1`] module"]
#[doc(alias = "P6SEL1")]
pub type P6sel1 = crate::Reg<p6sel1::P6sel1Spec>;
#[doc = "Port 6 Select 1"]
pub mod p6sel1;
#[doc = "P6SELC (rw) register accessor: Port 6 Complement Select\n\nYou can [`read`](crate::Reg::read) this register and get [`p6selc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p6selc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p6selc`] module"]
#[doc(alias = "P6SELC")]
pub type P6selc = crate::Reg<p6selc::P6selcSpec>;
#[doc = "Port 6 Complement Select"]
pub mod p6selc;
#[doc = "P6IES (rw) register accessor: Port 6 Interrupt Edge Select\n\nYou can [`read`](crate::Reg::read) this register and get [`p6ies::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p6ies::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p6ies`] module"]
#[doc(alias = "P6IES")]
pub type P6ies = crate::Reg<p6ies::P6iesSpec>;
#[doc = "Port 6 Interrupt Edge Select"]
pub mod p6ies;
#[doc = "P6IE (rw) register accessor: Port 6 Interrupt Enable\n\nYou can [`read`](crate::Reg::read) this register and get [`p6ie::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p6ie::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p6ie`] module"]
#[doc(alias = "P6IE")]
pub type P6ie = crate::Reg<p6ie::P6ieSpec>;
#[doc = "Port 6 Interrupt Enable"]
pub mod p6ie;
#[doc = "P6IFG (rw) register accessor: Port 6 Interrupt Flag\n\nYou can [`read`](crate::Reg::read) this register and get [`p6ifg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p6ifg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p6ifg`] module"]
#[doc(alias = "P6IFG")]
pub type P6ifg = crate::Reg<p6ifg::P6ifgSpec>;
#[doc = "Port 6 Interrupt Flag"]
pub mod p6ifg;
#[doc = "P6IV (rw) register accessor: Port 6 Interrupt Vector Register\n\nYou can [`read`](crate::Reg::read) this register and get [`p6iv::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p6iv::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p6iv`] module"]
#[doc(alias = "P6IV")]
pub type P6iv = crate::Reg<p6iv::P6ivSpec>;
#[doc = "Port 6 Interrupt Vector Register"]
pub mod p6iv;
