#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Configuration Register"]
    pub cfg: crate::Reg<cfg::CFG_SPEC>,
    #[doc = "0x04 - Status Register"]
    pub stat: crate::Reg<stat::STAT_SPEC>,
    #[doc = "0x08 - Key Register for Powering Down the Voltage Comparator"]
    pub pwdkey: crate::Reg<pwdkey::PWDKEY_SPEC>,
    _reserved3: [u8; 0x01f4],
    #[doc = "0x200 - Voltage Comparator Interrupt registers: Enable"]
    pub inten: crate::Reg<inten::INTEN_SPEC>,
    #[doc = "0x204 - Voltage Comparator Interrupt registers: Status"]
    pub intstat: crate::Reg<intstat::INTSTAT_SPEC>,
    #[doc = "0x208 - Voltage Comparator Interrupt registers: Clear"]
    pub intclr: crate::Reg<intclr::INTCLR_SPEC>,
    #[doc = "0x20c - Voltage Comparator Interrupt registers: Set"]
    pub intset: crate::Reg<intset::INTSET_SPEC>,
}
#[doc = "CFG register accessor: an alias for `Reg<CFG_SPEC>`"]
pub type CFG = crate::Reg<cfg::CFG_SPEC>;
#[doc = "Configuration Register"]
pub mod cfg;
#[doc = "STAT register accessor: an alias for `Reg<STAT_SPEC>`"]
pub type STAT = crate::Reg<stat::STAT_SPEC>;
#[doc = "Status Register"]
pub mod stat;
#[doc = "PWDKEY register accessor: an alias for `Reg<PWDKEY_SPEC>`"]
pub type PWDKEY = crate::Reg<pwdkey::PWDKEY_SPEC>;
#[doc = "Key Register for Powering Down the Voltage Comparator"]
pub mod pwdkey;
#[doc = "INTEN register accessor: an alias for `Reg<INTEN_SPEC>`"]
pub type INTEN = crate::Reg<inten::INTEN_SPEC>;
#[doc = "Voltage Comparator Interrupt registers: Enable"]
pub mod inten;
#[doc = "INTSTAT register accessor: an alias for `Reg<INTSTAT_SPEC>`"]
pub type INTSTAT = crate::Reg<intstat::INTSTAT_SPEC>;
#[doc = "Voltage Comparator Interrupt registers: Status"]
pub mod intstat;
#[doc = "INTCLR register accessor: an alias for `Reg<INTCLR_SPEC>`"]
pub type INTCLR = crate::Reg<intclr::INTCLR_SPEC>;
#[doc = "Voltage Comparator Interrupt registers: Clear"]
pub mod intclr;
#[doc = "INTSET register accessor: an alias for `Reg<INTSET_SPEC>`"]
pub type INTSET = crate::Reg<intset::INTSET_SPEC>;
#[doc = "Voltage Comparator Interrupt registers: Set"]
pub mod intset;
