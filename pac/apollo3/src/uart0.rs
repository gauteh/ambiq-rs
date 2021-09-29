#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - UART Data Register"]
    pub dr: crate::Reg<dr::DR_SPEC>,
    #[doc = "0x04 - UART Status Register"]
    pub rsr: crate::Reg<rsr::RSR_SPEC>,
    _reserved2: [u8; 0x10],
    #[doc = "0x18 - Flag Register"]
    pub fr: crate::Reg<fr::FR_SPEC>,
    _reserved3: [u8; 0x04],
    #[doc = "0x20 - IrDA Counter"]
    pub ilpr: crate::Reg<ilpr::ILPR_SPEC>,
    #[doc = "0x24 - Integer Baud Rate Divisor"]
    pub ibrd: crate::Reg<ibrd::IBRD_SPEC>,
    #[doc = "0x28 - Fractional Baud Rate Divisor"]
    pub fbrd: crate::Reg<fbrd::FBRD_SPEC>,
    #[doc = "0x2c - Line Control High"]
    pub lcrh: crate::Reg<lcrh::LCRH_SPEC>,
    #[doc = "0x30 - Control Register"]
    pub cr: crate::Reg<cr::CR_SPEC>,
    #[doc = "0x34 - FIFO Interrupt Level Select"]
    pub ifls: crate::Reg<ifls::IFLS_SPEC>,
    #[doc = "0x38 - Interrupt Enable"]
    pub ier: crate::Reg<ier::IER_SPEC>,
    #[doc = "0x3c - Interrupt Status"]
    pub ies: crate::Reg<ies::IES_SPEC>,
    #[doc = "0x40 - Masked Interrupt Status"]
    pub mis: crate::Reg<mis::MIS_SPEC>,
    #[doc = "0x44 - Interrupt Clear"]
    pub iec: crate::Reg<iec::IEC_SPEC>,
}
#[doc = "DR register accessor: an alias for `Reg<DR_SPEC>`"]
pub type DR = crate::Reg<dr::DR_SPEC>;
#[doc = "UART Data Register"]
pub mod dr;
#[doc = "RSR register accessor: an alias for `Reg<RSR_SPEC>`"]
pub type RSR = crate::Reg<rsr::RSR_SPEC>;
#[doc = "UART Status Register"]
pub mod rsr;
#[doc = "FR register accessor: an alias for `Reg<FR_SPEC>`"]
pub type FR = crate::Reg<fr::FR_SPEC>;
#[doc = "Flag Register"]
pub mod fr;
#[doc = "ILPR register accessor: an alias for `Reg<ILPR_SPEC>`"]
pub type ILPR = crate::Reg<ilpr::ILPR_SPEC>;
#[doc = "IrDA Counter"]
pub mod ilpr;
#[doc = "IBRD register accessor: an alias for `Reg<IBRD_SPEC>`"]
pub type IBRD = crate::Reg<ibrd::IBRD_SPEC>;
#[doc = "Integer Baud Rate Divisor"]
pub mod ibrd;
#[doc = "FBRD register accessor: an alias for `Reg<FBRD_SPEC>`"]
pub type FBRD = crate::Reg<fbrd::FBRD_SPEC>;
#[doc = "Fractional Baud Rate Divisor"]
pub mod fbrd;
#[doc = "LCRH register accessor: an alias for `Reg<LCRH_SPEC>`"]
pub type LCRH = crate::Reg<lcrh::LCRH_SPEC>;
#[doc = "Line Control High"]
pub mod lcrh;
#[doc = "CR register accessor: an alias for `Reg<CR_SPEC>`"]
pub type CR = crate::Reg<cr::CR_SPEC>;
#[doc = "Control Register"]
pub mod cr;
#[doc = "IFLS register accessor: an alias for `Reg<IFLS_SPEC>`"]
pub type IFLS = crate::Reg<ifls::IFLS_SPEC>;
#[doc = "FIFO Interrupt Level Select"]
pub mod ifls;
#[doc = "IER register accessor: an alias for `Reg<IER_SPEC>`"]
pub type IER = crate::Reg<ier::IER_SPEC>;
#[doc = "Interrupt Enable"]
pub mod ier;
#[doc = "IES register accessor: an alias for `Reg<IES_SPEC>`"]
pub type IES = crate::Reg<ies::IES_SPEC>;
#[doc = "Interrupt Status"]
pub mod ies;
#[doc = "MIS register accessor: an alias for `Reg<MIS_SPEC>`"]
pub type MIS = crate::Reg<mis::MIS_SPEC>;
#[doc = "Masked Interrupt Status"]
pub mod mis;
#[doc = "IEC register accessor: an alias for `Reg<IEC_SPEC>`"]
pub type IEC = crate::Reg<iec::IEC_SPEC>;
#[doc = "Interrupt Clear"]
pub mod iec;
