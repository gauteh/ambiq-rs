#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Configuration Register"]
    pub cfg: crate::Reg<cfg::CFG_SPEC>,
    #[doc = "0x04 - Restart the watchdog timer."]
    pub rstrt: crate::Reg<rstrt::RSTRT_SPEC>,
    #[doc = "0x08 - Locks the WDT"]
    pub lock: crate::Reg<lock::LOCK_SPEC>,
    #[doc = "0x0c - Current Counter Value for WDT"]
    pub count: crate::Reg<count::COUNT_SPEC>,
    _reserved4: [u8; 0x01f0],
    #[doc = "0x200 - WDT Interrupt register: Enable"]
    pub inten: crate::Reg<inten::INTEN_SPEC>,
    #[doc = "0x204 - WDT Interrupt register: Status"]
    pub intstat: crate::Reg<intstat::INTSTAT_SPEC>,
    #[doc = "0x208 - WDT Interrupt register: Clear"]
    pub intclr: crate::Reg<intclr::INTCLR_SPEC>,
    #[doc = "0x20c - WDT Interrupt register: Set"]
    pub intset: crate::Reg<intset::INTSET_SPEC>,
}
#[doc = "CFG register accessor: an alias for `Reg<CFG_SPEC>`"]
pub type CFG = crate::Reg<cfg::CFG_SPEC>;
#[doc = "Configuration Register"]
pub mod cfg;
#[doc = "RSTRT register accessor: an alias for `Reg<RSTRT_SPEC>`"]
pub type RSTRT = crate::Reg<rstrt::RSTRT_SPEC>;
#[doc = "Restart the watchdog timer."]
pub mod rstrt;
#[doc = "LOCK register accessor: an alias for `Reg<LOCK_SPEC>`"]
pub type LOCK = crate::Reg<lock::LOCK_SPEC>;
#[doc = "Locks the WDT"]
pub mod lock;
#[doc = "COUNT register accessor: an alias for `Reg<COUNT_SPEC>`"]
pub type COUNT = crate::Reg<count::COUNT_SPEC>;
#[doc = "Current Counter Value for WDT"]
pub mod count;
#[doc = "INTEN register accessor: an alias for `Reg<INTEN_SPEC>`"]
pub type INTEN = crate::Reg<inten::INTEN_SPEC>;
#[doc = "WDT Interrupt register: Enable"]
pub mod inten;
#[doc = "INTSTAT register accessor: an alias for `Reg<INTSTAT_SPEC>`"]
pub type INTSTAT = crate::Reg<intstat::INTSTAT_SPEC>;
#[doc = "WDT Interrupt register: Status"]
pub mod intstat;
#[doc = "INTCLR register accessor: an alias for `Reg<INTCLR_SPEC>`"]
pub type INTCLR = crate::Reg<intclr::INTCLR_SPEC>;
#[doc = "WDT Interrupt register: Clear"]
pub mod intclr;
#[doc = "INTSET register accessor: an alias for `Reg<INTSET_SPEC>`"]
pub type INTSET = crate::Reg<intset::INTSET_SPEC>;
#[doc = "WDT Interrupt register: Set"]
pub mod intset;
