#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control Register"]
    pub ctrl: crate::Reg<ctrl::CTRL_SPEC>,
    _reserved1: [u8; 0x0c],
    #[doc = "0x10 - Source Addresss"]
    pub srcaddr: crate::Reg<srcaddr::SRCADDR_SPEC>,
    _reserved2: [u8; 0x0c],
    #[doc = "0x20 - Length"]
    pub len: crate::Reg<len::LEN_SPEC>,
    _reserved3: [u8; 0x0c],
    #[doc = "0x30 - CRC Seed/Result Register"]
    pub result: crate::Reg<result::RESULT_SPEC>,
    _reserved4: [u8; 0x44],
    #[doc = "0x78 - LOCK Control Register"]
    pub lockctrl: crate::Reg<lockctrl::LOCKCTRL_SPEC>,
    #[doc = "0x7c - LOCK Status Register"]
    pub lockstat: crate::Reg<lockstat::LOCKSTAT_SPEC>,
    #[doc = "0x80 - Key0 Register"]
    pub key0: crate::Reg<key0::KEY0_SPEC>,
    #[doc = "0x84 - Key1 Register"]
    pub key1: crate::Reg<key1::KEY1_SPEC>,
    #[doc = "0x88 - Key2 Register"]
    pub key2: crate::Reg<key2::KEY2_SPEC>,
    #[doc = "0x8c - Key3 Register"]
    pub key3: crate::Reg<key3::KEY3_SPEC>,
}
#[doc = "CTRL register accessor: an alias for `Reg<CTRL_SPEC>`"]
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
#[doc = "Control Register"]
pub mod ctrl;
#[doc = "SRCADDR register accessor: an alias for `Reg<SRCADDR_SPEC>`"]
pub type SRCADDR = crate::Reg<srcaddr::SRCADDR_SPEC>;
#[doc = "Source Addresss"]
pub mod srcaddr;
#[doc = "LEN register accessor: an alias for `Reg<LEN_SPEC>`"]
pub type LEN = crate::Reg<len::LEN_SPEC>;
#[doc = "Length"]
pub mod len;
#[doc = "RESULT register accessor: an alias for `Reg<RESULT_SPEC>`"]
pub type RESULT = crate::Reg<result::RESULT_SPEC>;
#[doc = "CRC Seed/Result Register"]
pub mod result;
#[doc = "LOCKCTRL register accessor: an alias for `Reg<LOCKCTRL_SPEC>`"]
pub type LOCKCTRL = crate::Reg<lockctrl::LOCKCTRL_SPEC>;
#[doc = "LOCK Control Register"]
pub mod lockctrl;
#[doc = "LOCKSTAT register accessor: an alias for `Reg<LOCKSTAT_SPEC>`"]
pub type LOCKSTAT = crate::Reg<lockstat::LOCKSTAT_SPEC>;
#[doc = "LOCK Status Register"]
pub mod lockstat;
#[doc = "KEY0 register accessor: an alias for `Reg<KEY0_SPEC>`"]
pub type KEY0 = crate::Reg<key0::KEY0_SPEC>;
#[doc = "Key0 Register"]
pub mod key0;
#[doc = "KEY1 register accessor: an alias for `Reg<KEY1_SPEC>`"]
pub type KEY1 = crate::Reg<key1::KEY1_SPEC>;
#[doc = "Key1 Register"]
pub mod key1;
#[doc = "KEY2 register accessor: an alias for `Reg<KEY2_SPEC>`"]
pub type KEY2 = crate::Reg<key2::KEY2_SPEC>;
#[doc = "Key2 Register"]
pub mod key2;
#[doc = "KEY3 register accessor: an alias for `Reg<KEY3_SPEC>`"]
pub type KEY3 = crate::Reg<key3::KEY3_SPEC>;
#[doc = "Key3 Register"]
pub mod key3;
