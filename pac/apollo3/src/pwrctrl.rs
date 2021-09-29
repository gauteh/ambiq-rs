#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Voltage Regulator Select Register"]
    pub supplysrc: crate::Reg<supplysrc::SUPPLYSRC_SPEC>,
    #[doc = "0x04 - Voltage Regulators status"]
    pub supplystatus: crate::Reg<supplystatus::SUPPLYSTATUS_SPEC>,
    #[doc = "0x08 - Device Power Enables"]
    pub devpwren: crate::Reg<devpwren::DEVPWREN_SPEC>,
    #[doc = "0x0c - Powerdown SRAM banks in Deep Sleep mode"]
    pub mempwdinsleep: crate::Reg<mempwdinsleep::MEMPWDINSLEEP_SPEC>,
    #[doc = "0x10 - Enables individual banks of the MEMORY array"]
    pub mempwren: crate::Reg<mempwren::MEMPWREN_SPEC>,
    #[doc = "0x14 - Mem Power ON Status"]
    pub mempwrstatus: crate::Reg<mempwrstatus::MEMPWRSTATUS_SPEC>,
    #[doc = "0x18 - Device Power ON Status"]
    pub devpwrstatus: crate::Reg<devpwrstatus::DEVPWRSTATUS_SPEC>,
    #[doc = "0x1c - SRAM Control register"]
    pub sramctrl: crate::Reg<sramctrl::SRAMCTRL_SPEC>,
    #[doc = "0x20 - Power Status Register for ADC Block"]
    pub adcstatus: crate::Reg<adcstatus::ADCSTATUS_SPEC>,
    #[doc = "0x24 - Power Optimization Control Bits"]
    pub misc: crate::Reg<misc::MISC_SPEC>,
    #[doc = "0x28 - Event enable register to control which DEVPWRSTATUS bits are routed to event input of CPU."]
    pub devpwreventen: crate::Reg<devpwreventen::DEVPWREVENTEN_SPEC>,
    #[doc = "0x2c - Event enable register to control which MEMPWRSTATUS bits are routed to event input of CPU."]
    pub mempwreventen: crate::Reg<mempwreventen::MEMPWREVENTEN_SPEC>,
}
#[doc = "SUPPLYSRC register accessor: an alias for `Reg<SUPPLYSRC_SPEC>`"]
pub type SUPPLYSRC = crate::Reg<supplysrc::SUPPLYSRC_SPEC>;
#[doc = "Voltage Regulator Select Register"]
pub mod supplysrc;
#[doc = "SUPPLYSTATUS register accessor: an alias for `Reg<SUPPLYSTATUS_SPEC>`"]
pub type SUPPLYSTATUS = crate::Reg<supplystatus::SUPPLYSTATUS_SPEC>;
#[doc = "Voltage Regulators status"]
pub mod supplystatus;
#[doc = "DEVPWREN register accessor: an alias for `Reg<DEVPWREN_SPEC>`"]
pub type DEVPWREN = crate::Reg<devpwren::DEVPWREN_SPEC>;
#[doc = "Device Power Enables"]
pub mod devpwren;
#[doc = "MEMPWDINSLEEP register accessor: an alias for `Reg<MEMPWDINSLEEP_SPEC>`"]
pub type MEMPWDINSLEEP = crate::Reg<mempwdinsleep::MEMPWDINSLEEP_SPEC>;
#[doc = "Powerdown SRAM banks in Deep Sleep mode"]
pub mod mempwdinsleep;
#[doc = "MEMPWREN register accessor: an alias for `Reg<MEMPWREN_SPEC>`"]
pub type MEMPWREN = crate::Reg<mempwren::MEMPWREN_SPEC>;
#[doc = "Enables individual banks of the MEMORY array"]
pub mod mempwren;
#[doc = "MEMPWRSTATUS register accessor: an alias for `Reg<MEMPWRSTATUS_SPEC>`"]
pub type MEMPWRSTATUS = crate::Reg<mempwrstatus::MEMPWRSTATUS_SPEC>;
#[doc = "Mem Power ON Status"]
pub mod mempwrstatus;
#[doc = "DEVPWRSTATUS register accessor: an alias for `Reg<DEVPWRSTATUS_SPEC>`"]
pub type DEVPWRSTATUS = crate::Reg<devpwrstatus::DEVPWRSTATUS_SPEC>;
#[doc = "Device Power ON Status"]
pub mod devpwrstatus;
#[doc = "SRAMCTRL register accessor: an alias for `Reg<SRAMCTRL_SPEC>`"]
pub type SRAMCTRL = crate::Reg<sramctrl::SRAMCTRL_SPEC>;
#[doc = "SRAM Control register"]
pub mod sramctrl;
#[doc = "ADCSTATUS register accessor: an alias for `Reg<ADCSTATUS_SPEC>`"]
pub type ADCSTATUS = crate::Reg<adcstatus::ADCSTATUS_SPEC>;
#[doc = "Power Status Register for ADC Block"]
pub mod adcstatus;
#[doc = "MISC register accessor: an alias for `Reg<MISC_SPEC>`"]
pub type MISC = crate::Reg<misc::MISC_SPEC>;
#[doc = "Power Optimization Control Bits"]
pub mod misc;
#[doc = "DEVPWREVENTEN register accessor: an alias for `Reg<DEVPWREVENTEN_SPEC>`"]
pub type DEVPWREVENTEN = crate::Reg<devpwreventen::DEVPWREVENTEN_SPEC>;
#[doc = "Event enable register to control which DEVPWRSTATUS bits are routed to event input of CPU."]
pub mod devpwreventen;
#[doc = "MEMPWREVENTEN register accessor: an alias for `Reg<MEMPWREVENTEN_SPEC>`"]
pub type MEMPWREVENTEN = crate::Reg<mempwreventen::MEMPWREVENTEN_SPEC>;
#[doc = "Event enable register to control which MEMPWRSTATUS bits are routed to event input of CPU."]
pub mod mempwreventen;
