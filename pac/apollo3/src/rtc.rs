#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x40],
    #[doc = "0x40 - RTC Counters Lower"]
    pub ctrlow: crate::Reg<ctrlow::CTRLOW_SPEC>,
    #[doc = "0x44 - RTC Counters Upper"]
    pub ctrup: crate::Reg<ctrup::CTRUP_SPEC>,
    #[doc = "0x48 - RTC Alarms Lower"]
    pub almlow: crate::Reg<almlow::ALMLOW_SPEC>,
    #[doc = "0x4c - RTC Alarms Upper"]
    pub almup: crate::Reg<almup::ALMUP_SPEC>,
    #[doc = "0x50 - RTC Control Register"]
    pub rtcctl: crate::Reg<rtcctl::RTCCTL_SPEC>,
    _reserved5: [u8; 0xac],
    #[doc = "0x100 - RTC Interrupt Register: Enable"]
    pub inten: crate::Reg<inten::INTEN_SPEC>,
    #[doc = "0x104 - RTC Interrupt Register: Status"]
    pub intstat: crate::Reg<intstat::INTSTAT_SPEC>,
    #[doc = "0x108 - RTC Interrupt Register: Clear"]
    pub intclr: crate::Reg<intclr::INTCLR_SPEC>,
    #[doc = "0x10c - RTC Interrupt Register: Set"]
    pub intset: crate::Reg<intset::INTSET_SPEC>,
}
#[doc = "CTRLOW register accessor: an alias for `Reg<CTRLOW_SPEC>`"]
pub type CTRLOW = crate::Reg<ctrlow::CTRLOW_SPEC>;
#[doc = "RTC Counters Lower"]
pub mod ctrlow;
#[doc = "CTRUP register accessor: an alias for `Reg<CTRUP_SPEC>`"]
pub type CTRUP = crate::Reg<ctrup::CTRUP_SPEC>;
#[doc = "RTC Counters Upper"]
pub mod ctrup;
#[doc = "ALMLOW register accessor: an alias for `Reg<ALMLOW_SPEC>`"]
pub type ALMLOW = crate::Reg<almlow::ALMLOW_SPEC>;
#[doc = "RTC Alarms Lower"]
pub mod almlow;
#[doc = "ALMUP register accessor: an alias for `Reg<ALMUP_SPEC>`"]
pub type ALMUP = crate::Reg<almup::ALMUP_SPEC>;
#[doc = "RTC Alarms Upper"]
pub mod almup;
#[doc = "RTCCTL register accessor: an alias for `Reg<RTCCTL_SPEC>`"]
pub type RTCCTL = crate::Reg<rtcctl::RTCCTL_SPEC>;
#[doc = "RTC Control Register"]
pub mod rtcctl;
#[doc = "INTEN register accessor: an alias for `Reg<INTEN_SPEC>`"]
pub type INTEN = crate::Reg<inten::INTEN_SPEC>;
#[doc = "RTC Interrupt Register: Enable"]
pub mod inten;
#[doc = "INTSTAT register accessor: an alias for `Reg<INTSTAT_SPEC>`"]
pub type INTSTAT = crate::Reg<intstat::INTSTAT_SPEC>;
#[doc = "RTC Interrupt Register: Status"]
pub mod intstat;
#[doc = "INTCLR register accessor: an alias for `Reg<INTCLR_SPEC>`"]
pub type INTCLR = crate::Reg<intclr::INTCLR_SPEC>;
#[doc = "RTC Interrupt Register: Clear"]
pub mod intclr;
#[doc = "INTSET register accessor: an alias for `Reg<INTSET_SPEC>`"]
pub type INTSET = crate::Reg<intset::INTSET_SPEC>;
#[doc = "RTC Interrupt Register: Set"]
pub mod intset;
