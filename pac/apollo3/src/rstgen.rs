#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Configuration Register"]
    pub cfg: crate::Reg<cfg::CFG_SPEC>,
    #[doc = "0x04 - Software POI Reset"]
    pub swpoi: crate::Reg<swpoi::SWPOI_SPEC>,
    #[doc = "0x08 - Software POR Reset"]
    pub swpor: crate::Reg<swpor::SWPOR_SPEC>,
    _reserved3: [u8; 0x08],
    #[doc = "0x14 - TPIU reset"]
    pub tpiurst: crate::Reg<tpiurst::TPIURST_SPEC>,
    _reserved4: [u8; 0x01e8],
    #[doc = "0x200 - Reset Interrupt register: Enable"]
    pub inten: crate::Reg<inten::INTEN_SPEC>,
    #[doc = "0x204 - Reset Interrupt register: Status"]
    pub intstat: crate::Reg<intstat::INTSTAT_SPEC>,
    #[doc = "0x208 - Reset Interrupt register: Clear"]
    pub intclr: crate::Reg<intclr::INTCLR_SPEC>,
    #[doc = "0x20c - Reset Interrupt register: Set"]
    pub intset: crate::Reg<intset::INTSET_SPEC>,
    _reserved8: [u8; 0x0fff_edf0],
    #[doc = "0xffff000 - Status Register (SBL)"]
    pub stat: crate::Reg<stat::STAT_SPEC>,
}
#[doc = "CFG register accessor: an alias for `Reg<CFG_SPEC>`"]
pub type CFG = crate::Reg<cfg::CFG_SPEC>;
#[doc = "Configuration Register"]
pub mod cfg;
#[doc = "SWPOI register accessor: an alias for `Reg<SWPOI_SPEC>`"]
pub type SWPOI = crate::Reg<swpoi::SWPOI_SPEC>;
#[doc = "Software POI Reset"]
pub mod swpoi;
#[doc = "SWPOR register accessor: an alias for `Reg<SWPOR_SPEC>`"]
pub type SWPOR = crate::Reg<swpor::SWPOR_SPEC>;
#[doc = "Software POR Reset"]
pub mod swpor;
#[doc = "TPIURST register accessor: an alias for `Reg<TPIURST_SPEC>`"]
pub type TPIURST = crate::Reg<tpiurst::TPIURST_SPEC>;
#[doc = "TPIU reset"]
pub mod tpiurst;
#[doc = "INTEN register accessor: an alias for `Reg<INTEN_SPEC>`"]
pub type INTEN = crate::Reg<inten::INTEN_SPEC>;
#[doc = "Reset Interrupt register: Enable"]
pub mod inten;
#[doc = "INTSTAT register accessor: an alias for `Reg<INTSTAT_SPEC>`"]
pub type INTSTAT = crate::Reg<intstat::INTSTAT_SPEC>;
#[doc = "Reset Interrupt register: Status"]
pub mod intstat;
#[doc = "INTCLR register accessor: an alias for `Reg<INTCLR_SPEC>`"]
pub type INTCLR = crate::Reg<intclr::INTCLR_SPEC>;
#[doc = "Reset Interrupt register: Clear"]
pub mod intclr;
#[doc = "INTSET register accessor: an alias for `Reg<INTSET_SPEC>`"]
pub type INTSET = crate::Reg<intset::INTSET_SPEC>;
#[doc = "Reset Interrupt register: Set"]
pub mod intset;
#[doc = "STAT register accessor: an alias for `Reg<STAT_SPEC>`"]
pub type STAT = crate::Reg<stat::STAT_SPEC>;
#[doc = "Status Register (SBL)"]
pub mod stat;
