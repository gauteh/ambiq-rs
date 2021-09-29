#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - ISO7816 interrupt status"]
    pub sr: crate::Reg<sr::SR_SPEC>,
    _reserved1: [u8; 0x0c],
    #[doc = "0x10 - ISO7816 data"]
    pub dr: crate::Reg<dr::DR_SPEC>,
    _reserved2: [u8; 0x0c],
    #[doc = "0x20 - ISO7816 interrupt status 1"]
    pub sr1: crate::Reg<sr1::SR1_SPEC>,
    _reserved3: [u8; 0x14],
    #[doc = "0x38 - ISO7816 resent count inquiry"]
    pub retxcntrmi: crate::Reg<retxcntrmi::RETXCNTRMI_SPEC>,
    _reserved4: [u8; 0xc4],
    #[doc = "0x100 - Clock Control"]
    pub clkctrl: crate::Reg<clkctrl::CLKCTRL_SPEC>,
}
#[doc = "SR register accessor: an alias for `Reg<SR_SPEC>`"]
pub type SR = crate::Reg<sr::SR_SPEC>;
#[doc = "ISO7816 interrupt status"]
pub mod sr;
#[doc = "DR register accessor: an alias for `Reg<DR_SPEC>`"]
pub type DR = crate::Reg<dr::DR_SPEC>;
#[doc = "ISO7816 data"]
pub mod dr;
#[doc = "SR1 register accessor: an alias for `Reg<SR1_SPEC>`"]
pub type SR1 = crate::Reg<sr1::SR1_SPEC>;
#[doc = "ISO7816 interrupt status 1"]
pub mod sr1;
#[doc = "RETXCNTRMI register accessor: an alias for `Reg<RETXCNTRMI_SPEC>`"]
pub type RETXCNTRMI = crate::Reg<retxcntrmi::RETXCNTRMI_SPEC>;
#[doc = "ISO7816 resent count inquiry"]
pub mod retxcntrmi;
#[doc = "CLKCTRL register accessor: an alias for `Reg<CLKCTRL_SPEC>`"]
pub type CLKCTRL = crate::Reg<clkctrl::CLKCTRL_SPEC>;
#[doc = "Clock Control"]
pub mod clkctrl;
