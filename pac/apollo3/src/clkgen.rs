#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - XT Oscillator Control"]
    pub calxt: crate::Reg<calxt::CALXT_SPEC>,
    #[doc = "0x04 - RC Oscillator Control"]
    pub calrc: crate::Reg<calrc::CALRC_SPEC>,
    #[doc = "0x08 - Autocalibration Counter"]
    pub acalctr: crate::Reg<acalctr::ACALCTR_SPEC>,
    #[doc = "0x0c - Oscillator Control"]
    pub octrl: crate::Reg<octrl::OCTRL_SPEC>,
    #[doc = "0x10 - CLKOUT Frequency Select"]
    pub clkout: crate::Reg<clkout::CLKOUT_SPEC>,
    #[doc = "0x14 - Key Register for Clock Control Register"]
    pub clkkey: crate::Reg<clkkey::CLKKEY_SPEC>,
    #[doc = "0x18 - HFRC Clock Control"]
    pub cctrl: crate::Reg<cctrl::CCTRL_SPEC>,
    #[doc = "0x1c - Clock Generator Status"]
    pub status: crate::Reg<status::STATUS_SPEC>,
    #[doc = "0x20 - HFRC Adjustment"]
    pub hfadj: crate::Reg<hfadj::HFADJ_SPEC>,
    _reserved9: [u8; 0x04],
    #[doc = "0x28 - Clock Enable Status"]
    pub clockenstat: crate::Reg<clockenstat::CLOCKENSTAT_SPEC>,
    #[doc = "0x2c - Clock Enable Status"]
    pub clocken2stat: crate::Reg<clocken2stat::CLOCKEN2STAT_SPEC>,
    #[doc = "0x30 - Clock Enable Status"]
    pub clocken3stat: crate::Reg<clocken3stat::CLOCKEN3STAT_SPEC>,
    #[doc = "0x34 - HFRC Frequency Control register"]
    pub freqctrl: crate::Reg<freqctrl::FREQCTRL_SPEC>,
    _reserved13: [u8; 0x04],
    #[doc = "0x3c - BLE BUCK TON ADJUST"]
    pub blebucktonadj: crate::Reg<blebucktonadj::BLEBUCKTONADJ_SPEC>,
    _reserved14: [u8; 0xc0],
    #[doc = "0x100 - CLKGEN Interrupt Register: Enable"]
    pub intrpten: crate::Reg<intrpten::INTRPTEN_SPEC>,
    #[doc = "0x104 - CLKGEN Interrupt Register: Status"]
    pub intrptstat: crate::Reg<intrptstat::INTRPTSTAT_SPEC>,
    #[doc = "0x108 - CLKGEN Interrupt Register: Clear"]
    pub intrptclr: crate::Reg<intrptclr::INTRPTCLR_SPEC>,
    #[doc = "0x10c - CLKGEN Interrupt Register: Set"]
    pub intrptset: crate::Reg<intrptset::INTRPTSET_SPEC>,
}
#[doc = "CALXT register accessor: an alias for `Reg<CALXT_SPEC>`"]
pub type CALXT = crate::Reg<calxt::CALXT_SPEC>;
#[doc = "XT Oscillator Control"]
pub mod calxt;
#[doc = "CALRC register accessor: an alias for `Reg<CALRC_SPEC>`"]
pub type CALRC = crate::Reg<calrc::CALRC_SPEC>;
#[doc = "RC Oscillator Control"]
pub mod calrc;
#[doc = "ACALCTR register accessor: an alias for `Reg<ACALCTR_SPEC>`"]
pub type ACALCTR = crate::Reg<acalctr::ACALCTR_SPEC>;
#[doc = "Autocalibration Counter"]
pub mod acalctr;
#[doc = "OCTRL register accessor: an alias for `Reg<OCTRL_SPEC>`"]
pub type OCTRL = crate::Reg<octrl::OCTRL_SPEC>;
#[doc = "Oscillator Control"]
pub mod octrl;
#[doc = "CLKOUT register accessor: an alias for `Reg<CLKOUT_SPEC>`"]
pub type CLKOUT = crate::Reg<clkout::CLKOUT_SPEC>;
#[doc = "CLKOUT Frequency Select"]
pub mod clkout;
#[doc = "CLKKEY register accessor: an alias for `Reg<CLKKEY_SPEC>`"]
pub type CLKKEY = crate::Reg<clkkey::CLKKEY_SPEC>;
#[doc = "Key Register for Clock Control Register"]
pub mod clkkey;
#[doc = "CCTRL register accessor: an alias for `Reg<CCTRL_SPEC>`"]
pub type CCTRL = crate::Reg<cctrl::CCTRL_SPEC>;
#[doc = "HFRC Clock Control"]
pub mod cctrl;
#[doc = "STATUS register accessor: an alias for `Reg<STATUS_SPEC>`"]
pub type STATUS = crate::Reg<status::STATUS_SPEC>;
#[doc = "Clock Generator Status"]
pub mod status;
#[doc = "HFADJ register accessor: an alias for `Reg<HFADJ_SPEC>`"]
pub type HFADJ = crate::Reg<hfadj::HFADJ_SPEC>;
#[doc = "HFRC Adjustment"]
pub mod hfadj;
#[doc = "CLOCKENSTAT register accessor: an alias for `Reg<CLOCKENSTAT_SPEC>`"]
pub type CLOCKENSTAT = crate::Reg<clockenstat::CLOCKENSTAT_SPEC>;
#[doc = "Clock Enable Status"]
pub mod clockenstat;
#[doc = "CLOCKEN2STAT register accessor: an alias for `Reg<CLOCKEN2STAT_SPEC>`"]
pub type CLOCKEN2STAT = crate::Reg<clocken2stat::CLOCKEN2STAT_SPEC>;
#[doc = "Clock Enable Status"]
pub mod clocken2stat;
#[doc = "CLOCKEN3STAT register accessor: an alias for `Reg<CLOCKEN3STAT_SPEC>`"]
pub type CLOCKEN3STAT = crate::Reg<clocken3stat::CLOCKEN3STAT_SPEC>;
#[doc = "Clock Enable Status"]
pub mod clocken3stat;
#[doc = "FREQCTRL register accessor: an alias for `Reg<FREQCTRL_SPEC>`"]
pub type FREQCTRL = crate::Reg<freqctrl::FREQCTRL_SPEC>;
#[doc = "HFRC Frequency Control register"]
pub mod freqctrl;
#[doc = "BLEBUCKTONADJ register accessor: an alias for `Reg<BLEBUCKTONADJ_SPEC>`"]
pub type BLEBUCKTONADJ = crate::Reg<blebucktonadj::BLEBUCKTONADJ_SPEC>;
#[doc = "BLE BUCK TON ADJUST"]
pub mod blebucktonadj;
#[doc = "INTRPTEN register accessor: an alias for `Reg<INTRPTEN_SPEC>`"]
pub type INTRPTEN = crate::Reg<intrpten::INTRPTEN_SPEC>;
#[doc = "CLKGEN Interrupt Register: Enable"]
pub mod intrpten;
#[doc = "INTRPTSTAT register accessor: an alias for `Reg<INTRPTSTAT_SPEC>`"]
pub type INTRPTSTAT = crate::Reg<intrptstat::INTRPTSTAT_SPEC>;
#[doc = "CLKGEN Interrupt Register: Status"]
pub mod intrptstat;
#[doc = "INTRPTCLR register accessor: an alias for `Reg<INTRPTCLR_SPEC>`"]
pub type INTRPTCLR = crate::Reg<intrptclr::INTRPTCLR_SPEC>;
#[doc = "CLKGEN Interrupt Register: Clear"]
pub mod intrptclr;
#[doc = "INTRPTSET register accessor: an alias for `Reg<INTRPTSET_SPEC>`"]
pub type INTRPTSET = crate::Reg<intrptset::INTRPTSET_SPEC>;
#[doc = "CLKGEN Interrupt Register: Set"]
pub mod intrptset;
