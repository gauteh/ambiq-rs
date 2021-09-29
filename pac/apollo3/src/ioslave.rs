#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x0100],
    #[doc = "0x100 - Current FIFO Pointer"]
    pub fifoptr: crate::Reg<fifoptr::FIFOPTR_SPEC>,
    #[doc = "0x104 - FIFO Configuration"]
    pub fifocfg: crate::Reg<fifocfg::FIFOCFG_SPEC>,
    #[doc = "0x108 - FIFO Threshold Configuration"]
    pub fifothr: crate::Reg<fifothr::FIFOTHR_SPEC>,
    #[doc = "0x10c - FIFO Update Status"]
    pub fupd: crate::Reg<fupd::FUPD_SPEC>,
    #[doc = "0x110 - Overall FIFO Counter"]
    pub fifoctr: crate::Reg<fifoctr::FIFOCTR_SPEC>,
    #[doc = "0x114 - Overall FIFO Counter Increment"]
    pub fifoinc: crate::Reg<fifoinc::FIFOINC_SPEC>,
    #[doc = "0x118 - I/O Slave Configuration"]
    pub cfg: crate::Reg<cfg::CFG_SPEC>,
    #[doc = "0x11c - I/O Slave Interrupt Priority Encode"]
    pub prenc: crate::Reg<prenc::PRENC_SPEC>,
    #[doc = "0x120 - I/O Interrupt Control"]
    pub iointctl: crate::Reg<iointctl::IOINTCTL_SPEC>,
    #[doc = "0x124 - General Address Data"]
    pub genadd: crate::Reg<genadd::GENADD_SPEC>,
    _reserved10: [u8; 0xd8],
    #[doc = "0x200 - IO Slave Interrupts: Enable"]
    pub inten: crate::Reg<inten::INTEN_SPEC>,
    #[doc = "0x204 - IO Slave Interrupts: Status"]
    pub intstat: crate::Reg<intstat::INTSTAT_SPEC>,
    #[doc = "0x208 - IO Slave Interrupts: Clear"]
    pub intclr: crate::Reg<intclr::INTCLR_SPEC>,
    #[doc = "0x20c - IO Slave Interrupts: Set"]
    pub intset: crate::Reg<intset::INTSET_SPEC>,
    #[doc = "0x210 - Register Access Interrupts: Enable"]
    pub regaccinten: crate::Reg<regaccinten::REGACCINTEN_SPEC>,
    #[doc = "0x214 - Register Access Interrupts: Status"]
    pub regaccintstat: crate::Reg<regaccintstat::REGACCINTSTAT_SPEC>,
    #[doc = "0x218 - Register Access Interrupts: Clear"]
    pub regaccintclr: crate::Reg<regaccintclr::REGACCINTCLR_SPEC>,
    #[doc = "0x21c - Register Access Interrupts: Set"]
    pub regaccintset: crate::Reg<regaccintset::REGACCINTSET_SPEC>,
}
#[doc = "FIFOPTR register accessor: an alias for `Reg<FIFOPTR_SPEC>`"]
pub type FIFOPTR = crate::Reg<fifoptr::FIFOPTR_SPEC>;
#[doc = "Current FIFO Pointer"]
pub mod fifoptr;
#[doc = "FIFOCFG register accessor: an alias for `Reg<FIFOCFG_SPEC>`"]
pub type FIFOCFG = crate::Reg<fifocfg::FIFOCFG_SPEC>;
#[doc = "FIFO Configuration"]
pub mod fifocfg;
#[doc = "FIFOTHR register accessor: an alias for `Reg<FIFOTHR_SPEC>`"]
pub type FIFOTHR = crate::Reg<fifothr::FIFOTHR_SPEC>;
#[doc = "FIFO Threshold Configuration"]
pub mod fifothr;
#[doc = "FUPD register accessor: an alias for `Reg<FUPD_SPEC>`"]
pub type FUPD = crate::Reg<fupd::FUPD_SPEC>;
#[doc = "FIFO Update Status"]
pub mod fupd;
#[doc = "FIFOCTR register accessor: an alias for `Reg<FIFOCTR_SPEC>`"]
pub type FIFOCTR = crate::Reg<fifoctr::FIFOCTR_SPEC>;
#[doc = "Overall FIFO Counter"]
pub mod fifoctr;
#[doc = "FIFOINC register accessor: an alias for `Reg<FIFOINC_SPEC>`"]
pub type FIFOINC = crate::Reg<fifoinc::FIFOINC_SPEC>;
#[doc = "Overall FIFO Counter Increment"]
pub mod fifoinc;
#[doc = "CFG register accessor: an alias for `Reg<CFG_SPEC>`"]
pub type CFG = crate::Reg<cfg::CFG_SPEC>;
#[doc = "I/O Slave Configuration"]
pub mod cfg;
#[doc = "PRENC register accessor: an alias for `Reg<PRENC_SPEC>`"]
pub type PRENC = crate::Reg<prenc::PRENC_SPEC>;
#[doc = "I/O Slave Interrupt Priority Encode"]
pub mod prenc;
#[doc = "IOINTCTL register accessor: an alias for `Reg<IOINTCTL_SPEC>`"]
pub type IOINTCTL = crate::Reg<iointctl::IOINTCTL_SPEC>;
#[doc = "I/O Interrupt Control"]
pub mod iointctl;
#[doc = "GENADD register accessor: an alias for `Reg<GENADD_SPEC>`"]
pub type GENADD = crate::Reg<genadd::GENADD_SPEC>;
#[doc = "General Address Data"]
pub mod genadd;
#[doc = "INTEN register accessor: an alias for `Reg<INTEN_SPEC>`"]
pub type INTEN = crate::Reg<inten::INTEN_SPEC>;
#[doc = "IO Slave Interrupts: Enable"]
pub mod inten;
#[doc = "INTSTAT register accessor: an alias for `Reg<INTSTAT_SPEC>`"]
pub type INTSTAT = crate::Reg<intstat::INTSTAT_SPEC>;
#[doc = "IO Slave Interrupts: Status"]
pub mod intstat;
#[doc = "INTCLR register accessor: an alias for `Reg<INTCLR_SPEC>`"]
pub type INTCLR = crate::Reg<intclr::INTCLR_SPEC>;
#[doc = "IO Slave Interrupts: Clear"]
pub mod intclr;
#[doc = "INTSET register accessor: an alias for `Reg<INTSET_SPEC>`"]
pub type INTSET = crate::Reg<intset::INTSET_SPEC>;
#[doc = "IO Slave Interrupts: Set"]
pub mod intset;
#[doc = "REGACCINTEN register accessor: an alias for `Reg<REGACCINTEN_SPEC>`"]
pub type REGACCINTEN = crate::Reg<regaccinten::REGACCINTEN_SPEC>;
#[doc = "Register Access Interrupts: Enable"]
pub mod regaccinten;
#[doc = "REGACCINTSTAT register accessor: an alias for `Reg<REGACCINTSTAT_SPEC>`"]
pub type REGACCINTSTAT = crate::Reg<regaccintstat::REGACCINTSTAT_SPEC>;
#[doc = "Register Access Interrupts: Status"]
pub mod regaccintstat;
#[doc = "REGACCINTCLR register accessor: an alias for `Reg<REGACCINTCLR_SPEC>`"]
pub type REGACCINTCLR = crate::Reg<regaccintclr::REGACCINTCLR_SPEC>;
#[doc = "Register Access Interrupts: Clear"]
pub mod regaccintclr;
#[doc = "REGACCINTSET register accessor: an alias for `Reg<REGACCINTSET_SPEC>`"]
pub type REGACCINTSET = crate::Reg<regaccintset::REGACCINTSET_SPEC>;
#[doc = "Register Access Interrupts: Set"]
pub mod regaccintset;
