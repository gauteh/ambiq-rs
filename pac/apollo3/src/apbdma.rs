#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control Register"]
    pub bbvalue: crate::Reg<bbvalue::BBVALUE_SPEC>,
    #[doc = "0x04 - Set/Clear Register"]
    pub bbsetclear: crate::Reg<bbsetclear::BBSETCLEAR_SPEC>,
    #[doc = "0x08 - PIO Input Values"]
    pub bbinput: crate::Reg<bbinput::BBINPUT_SPEC>,
    _reserved3: [u8; 0x14],
    #[doc = "0x20 - PIO Input Values"]
    pub debugdata: crate::Reg<debugdata::DEBUGDATA_SPEC>,
    _reserved4: [u8; 0x1c],
    #[doc = "0x40 - PIO Input Values"]
    pub debug: crate::Reg<debug::DEBUG_SPEC>,
}
#[doc = "BBVALUE register accessor: an alias for `Reg<BBVALUE_SPEC>`"]
pub type BBVALUE = crate::Reg<bbvalue::BBVALUE_SPEC>;
#[doc = "Control Register"]
pub mod bbvalue;
#[doc = "BBSETCLEAR register accessor: an alias for `Reg<BBSETCLEAR_SPEC>`"]
pub type BBSETCLEAR = crate::Reg<bbsetclear::BBSETCLEAR_SPEC>;
#[doc = "Set/Clear Register"]
pub mod bbsetclear;
#[doc = "BBINPUT register accessor: an alias for `Reg<BBINPUT_SPEC>`"]
pub type BBINPUT = crate::Reg<bbinput::BBINPUT_SPEC>;
#[doc = "PIO Input Values"]
pub mod bbinput;
#[doc = "DEBUGDATA register accessor: an alias for `Reg<DEBUGDATA_SPEC>`"]
pub type DEBUGDATA = crate::Reg<debugdata::DEBUGDATA_SPEC>;
#[doc = "PIO Input Values"]
pub mod debugdata;
#[doc = "DEBUG register accessor: an alias for `Reg<DEBUG_SPEC>`"]
pub type DEBUG = crate::Reg<debug::DEBUG_SPEC>;
#[doc = "PIO Input Values"]
pub mod debug;
