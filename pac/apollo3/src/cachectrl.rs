#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Flash Cache Control Register"]
    pub cachecfg: crate::Reg<cachecfg::CACHECFG_SPEC>,
    #[doc = "0x04 - Flash Control Register"]
    pub flashcfg: crate::Reg<flashcfg::FLASHCFG_SPEC>,
    #[doc = "0x08 - Cache Control"]
    pub ctrl: crate::Reg<ctrl::CTRL_SPEC>,
    _reserved3: [u8; 0x04],
    #[doc = "0x10 - Flash Cache Noncachable Region 0 Start"]
    pub ncr0start: crate::Reg<ncr0start::NCR0START_SPEC>,
    #[doc = "0x14 - Flash Cache Noncachable Region 0 End"]
    pub ncr0end: crate::Reg<ncr0end::NCR0END_SPEC>,
    #[doc = "0x18 - Flash Cache Noncachable Region 1 Start"]
    pub ncr1start: crate::Reg<ncr1start::NCR1START_SPEC>,
    #[doc = "0x1c - Flash Cache Noncachable Region 1 End"]
    pub ncr1end: crate::Reg<ncr1end::NCR1END_SPEC>,
    _reserved7: [u8; 0x20],
    #[doc = "0x40 - Data Cache Total Accesses"]
    pub dmon0: crate::Reg<dmon0::DMON0_SPEC>,
    #[doc = "0x44 - Data Cache Tag Lookups"]
    pub dmon1: crate::Reg<dmon1::DMON1_SPEC>,
    #[doc = "0x48 - Data Cache Hits"]
    pub dmon2: crate::Reg<dmon2::DMON2_SPEC>,
    #[doc = "0x4c - Data Cache Line Hits"]
    pub dmon3: crate::Reg<dmon3::DMON3_SPEC>,
    #[doc = "0x50 - Instruction Cache Total Accesses"]
    pub imon0: crate::Reg<imon0::IMON0_SPEC>,
    #[doc = "0x54 - Instruction Cache Tag Lookups"]
    pub imon1: crate::Reg<imon1::IMON1_SPEC>,
    #[doc = "0x58 - Instruction Cache Hits"]
    pub imon2: crate::Reg<imon2::IMON2_SPEC>,
    #[doc = "0x5c - Instruction Cache Line Hits"]
    pub imon3: crate::Reg<imon3::IMON3_SPEC>,
}
#[doc = "CACHECFG register accessor: an alias for `Reg<CACHECFG_SPEC>`"]
pub type CACHECFG = crate::Reg<cachecfg::CACHECFG_SPEC>;
#[doc = "Flash Cache Control Register"]
pub mod cachecfg;
#[doc = "FLASHCFG register accessor: an alias for `Reg<FLASHCFG_SPEC>`"]
pub type FLASHCFG = crate::Reg<flashcfg::FLASHCFG_SPEC>;
#[doc = "Flash Control Register"]
pub mod flashcfg;
#[doc = "CTRL register accessor: an alias for `Reg<CTRL_SPEC>`"]
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
#[doc = "Cache Control"]
pub mod ctrl;
#[doc = "NCR0START register accessor: an alias for `Reg<NCR0START_SPEC>`"]
pub type NCR0START = crate::Reg<ncr0start::NCR0START_SPEC>;
#[doc = "Flash Cache Noncachable Region 0 Start"]
pub mod ncr0start;
#[doc = "NCR0END register accessor: an alias for `Reg<NCR0END_SPEC>`"]
pub type NCR0END = crate::Reg<ncr0end::NCR0END_SPEC>;
#[doc = "Flash Cache Noncachable Region 0 End"]
pub mod ncr0end;
#[doc = "NCR1START register accessor: an alias for `Reg<NCR1START_SPEC>`"]
pub type NCR1START = crate::Reg<ncr1start::NCR1START_SPEC>;
#[doc = "Flash Cache Noncachable Region 1 Start"]
pub mod ncr1start;
#[doc = "NCR1END register accessor: an alias for `Reg<NCR1END_SPEC>`"]
pub type NCR1END = crate::Reg<ncr1end::NCR1END_SPEC>;
#[doc = "Flash Cache Noncachable Region 1 End"]
pub mod ncr1end;
#[doc = "DMON0 register accessor: an alias for `Reg<DMON0_SPEC>`"]
pub type DMON0 = crate::Reg<dmon0::DMON0_SPEC>;
#[doc = "Data Cache Total Accesses"]
pub mod dmon0;
#[doc = "DMON1 register accessor: an alias for `Reg<DMON1_SPEC>`"]
pub type DMON1 = crate::Reg<dmon1::DMON1_SPEC>;
#[doc = "Data Cache Tag Lookups"]
pub mod dmon1;
#[doc = "DMON2 register accessor: an alias for `Reg<DMON2_SPEC>`"]
pub type DMON2 = crate::Reg<dmon2::DMON2_SPEC>;
#[doc = "Data Cache Hits"]
pub mod dmon2;
#[doc = "DMON3 register accessor: an alias for `Reg<DMON3_SPEC>`"]
pub type DMON3 = crate::Reg<dmon3::DMON3_SPEC>;
#[doc = "Data Cache Line Hits"]
pub mod dmon3;
#[doc = "IMON0 register accessor: an alias for `Reg<IMON0_SPEC>`"]
pub type IMON0 = crate::Reg<imon0::IMON0_SPEC>;
#[doc = "Instruction Cache Total Accesses"]
pub mod imon0;
#[doc = "IMON1 register accessor: an alias for `Reg<IMON1_SPEC>`"]
pub type IMON1 = crate::Reg<imon1::IMON1_SPEC>;
#[doc = "Instruction Cache Tag Lookups"]
pub mod imon1;
#[doc = "IMON2 register accessor: an alias for `Reg<IMON2_SPEC>`"]
pub type IMON2 = crate::Reg<imon2::IMON2_SPEC>;
#[doc = "Instruction Cache Hits"]
pub mod imon2;
#[doc = "IMON3 register accessor: an alias for `Reg<IMON3_SPEC>`"]
pub type IMON3 = crate::Reg<imon3::IMON3_SPEC>;
#[doc = "Instruction Cache Line Hits"]
pub mod imon3;
