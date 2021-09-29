#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Counter/Timer Register"]
    pub tmr0: crate::Reg<tmr0::TMR0_SPEC>,
    #[doc = "0x04 - Counter/Timer A0 Compare Registers"]
    pub cmpra0: crate::Reg<cmpra0::CMPRA0_SPEC>,
    #[doc = "0x08 - Counter/Timer B0 Compare Registers"]
    pub cmprb0: crate::Reg<cmprb0::CMPRB0_SPEC>,
    #[doc = "0x0c - Counter/Timer Control"]
    pub ctrl0: crate::Reg<ctrl0::CTRL0_SPEC>,
    _reserved4: [u8; 0x04],
    #[doc = "0x14 - Counter/Timer A0 Compare Registers"]
    pub cmprauxa0: crate::Reg<cmprauxa0::CMPRAUXA0_SPEC>,
    #[doc = "0x18 - Counter/Timer B0 Compare Registers"]
    pub cmprauxb0: crate::Reg<cmprauxb0::CMPRAUXB0_SPEC>,
    #[doc = "0x1c - Counter/Timer Auxiliary"]
    pub aux0: crate::Reg<aux0::AUX0_SPEC>,
    #[doc = "0x20 - Counter/Timer Register"]
    pub tmr1: crate::Reg<tmr1::TMR1_SPEC>,
    #[doc = "0x24 - Counter/Timer A1 Compare Registers"]
    pub cmpra1: crate::Reg<cmpra1::CMPRA1_SPEC>,
    #[doc = "0x28 - Counter/Timer B1 Compare Registers"]
    pub cmprb1: crate::Reg<cmprb1::CMPRB1_SPEC>,
    #[doc = "0x2c - Counter/Timer Control"]
    pub ctrl1: crate::Reg<ctrl1::CTRL1_SPEC>,
    _reserved11: [u8; 0x04],
    #[doc = "0x34 - Counter/Timer A1 Compare Registers"]
    pub cmprauxa1: crate::Reg<cmprauxa1::CMPRAUXA1_SPEC>,
    #[doc = "0x38 - Counter/Timer B1 Compare Registers"]
    pub cmprauxb1: crate::Reg<cmprauxb1::CMPRAUXB1_SPEC>,
    #[doc = "0x3c - Counter/Timer Auxiliary"]
    pub aux1: crate::Reg<aux1::AUX1_SPEC>,
    #[doc = "0x40 - Counter/Timer Register"]
    pub tmr2: crate::Reg<tmr2::TMR2_SPEC>,
    #[doc = "0x44 - Counter/Timer A2 Compare Registers"]
    pub cmpra2: crate::Reg<cmpra2::CMPRA2_SPEC>,
    #[doc = "0x48 - Counter/Timer B2 Compare Registers"]
    pub cmprb2: crate::Reg<cmprb2::CMPRB2_SPEC>,
    #[doc = "0x4c - Counter/Timer Control"]
    pub ctrl2: crate::Reg<ctrl2::CTRL2_SPEC>,
    _reserved18: [u8; 0x04],
    #[doc = "0x54 - Counter/Timer A2 Compare Registers"]
    pub cmprauxa2: crate::Reg<cmprauxa2::CMPRAUXA2_SPEC>,
    #[doc = "0x58 - Counter/Timer B2 Compare Registers"]
    pub cmprauxb2: crate::Reg<cmprauxb2::CMPRAUXB2_SPEC>,
    #[doc = "0x5c - Counter/Timer Auxiliary"]
    pub aux2: crate::Reg<aux2::AUX2_SPEC>,
    #[doc = "0x60 - Counter/Timer Register"]
    pub tmr3: crate::Reg<tmr3::TMR3_SPEC>,
    #[doc = "0x64 - Counter/Timer A3 Compare Registers"]
    pub cmpra3: crate::Reg<cmpra3::CMPRA3_SPEC>,
    #[doc = "0x68 - Counter/Timer B3 Compare Registers"]
    pub cmprb3: crate::Reg<cmprb3::CMPRB3_SPEC>,
    #[doc = "0x6c - Counter/Timer Control"]
    pub ctrl3: crate::Reg<ctrl3::CTRL3_SPEC>,
    _reserved25: [u8; 0x04],
    #[doc = "0x74 - Counter/Timer A3 Compare Registers"]
    pub cmprauxa3: crate::Reg<cmprauxa3::CMPRAUXA3_SPEC>,
    #[doc = "0x78 - Counter/Timer B3 Compare Registers"]
    pub cmprauxb3: crate::Reg<cmprauxb3::CMPRAUXB3_SPEC>,
    #[doc = "0x7c - Counter/Timer Auxiliary"]
    pub aux3: crate::Reg<aux3::AUX3_SPEC>,
    #[doc = "0x80 - Counter/Timer Register"]
    pub tmr4: crate::Reg<tmr4::TMR4_SPEC>,
    #[doc = "0x84 - Counter/Timer A4 Compare Registers"]
    pub cmpra4: crate::Reg<cmpra4::CMPRA4_SPEC>,
    #[doc = "0x88 - Counter/Timer B4 Compare Registers"]
    pub cmprb4: crate::Reg<cmprb4::CMPRB4_SPEC>,
    #[doc = "0x8c - Counter/Timer Control"]
    pub ctrl4: crate::Reg<ctrl4::CTRL4_SPEC>,
    _reserved32: [u8; 0x04],
    #[doc = "0x94 - Counter/Timer A4 Compare Registers"]
    pub cmprauxa4: crate::Reg<cmprauxa4::CMPRAUXA4_SPEC>,
    #[doc = "0x98 - Counter/Timer B4 Compare Registers"]
    pub cmprauxb4: crate::Reg<cmprauxb4::CMPRAUXB4_SPEC>,
    #[doc = "0x9c - Counter/Timer Auxiliary"]
    pub aux4: crate::Reg<aux4::AUX4_SPEC>,
    #[doc = "0xa0 - Counter/Timer Register"]
    pub tmr5: crate::Reg<tmr5::TMR5_SPEC>,
    #[doc = "0xa4 - Counter/Timer A5 Compare Registers"]
    pub cmpra5: crate::Reg<cmpra5::CMPRA5_SPEC>,
    #[doc = "0xa8 - Counter/Timer B5 Compare Registers"]
    pub cmprb5: crate::Reg<cmprb5::CMPRB5_SPEC>,
    #[doc = "0xac - Counter/Timer Control"]
    pub ctrl5: crate::Reg<ctrl5::CTRL5_SPEC>,
    _reserved39: [u8; 0x04],
    #[doc = "0xb4 - Counter/Timer A5 Compare Registers"]
    pub cmprauxa5: crate::Reg<cmprauxa5::CMPRAUXA5_SPEC>,
    #[doc = "0xb8 - Counter/Timer B5 Compare Registers"]
    pub cmprauxb5: crate::Reg<cmprauxb5::CMPRAUXB5_SPEC>,
    #[doc = "0xbc - Counter/Timer Auxiliary"]
    pub aux5: crate::Reg<aux5::AUX5_SPEC>,
    #[doc = "0xc0 - Counter/Timer Register"]
    pub tmr6: crate::Reg<tmr6::TMR6_SPEC>,
    #[doc = "0xc4 - Counter/Timer A6 Compare Registers"]
    pub cmpra6: crate::Reg<cmpra6::CMPRA6_SPEC>,
    #[doc = "0xc8 - Counter/Timer B6 Compare Registers"]
    pub cmprb6: crate::Reg<cmprb6::CMPRB6_SPEC>,
    #[doc = "0xcc - Counter/Timer Control"]
    pub ctrl6: crate::Reg<ctrl6::CTRL6_SPEC>,
    _reserved46: [u8; 0x04],
    #[doc = "0xd4 - Counter/Timer A6 Compare Registers"]
    pub cmprauxa6: crate::Reg<cmprauxa6::CMPRAUXA6_SPEC>,
    #[doc = "0xd8 - Counter/Timer B6 Compare Registers"]
    pub cmprauxb6: crate::Reg<cmprauxb6::CMPRAUXB6_SPEC>,
    #[doc = "0xdc - Counter/Timer Auxiliary"]
    pub aux6: crate::Reg<aux6::AUX6_SPEC>,
    #[doc = "0xe0 - Counter/Timer Register"]
    pub tmr7: crate::Reg<tmr7::TMR7_SPEC>,
    #[doc = "0xe4 - Counter/Timer A7 Compare Registers"]
    pub cmpra7: crate::Reg<cmpra7::CMPRA7_SPEC>,
    #[doc = "0xe8 - Counter/Timer B7 Compare Registers"]
    pub cmprb7: crate::Reg<cmprb7::CMPRB7_SPEC>,
    #[doc = "0xec - Counter/Timer Control"]
    pub ctrl7: crate::Reg<ctrl7::CTRL7_SPEC>,
    _reserved53: [u8; 0x04],
    #[doc = "0xf4 - Counter/Timer A7 Compare Registers"]
    pub cmprauxa7: crate::Reg<cmprauxa7::CMPRAUXA7_SPEC>,
    #[doc = "0xf8 - Counter/Timer B7 Compare Registers"]
    pub cmprauxb7: crate::Reg<cmprauxb7::CMPRAUXB7_SPEC>,
    #[doc = "0xfc - Counter/Timer Auxiliary"]
    pub aux7: crate::Reg<aux7::AUX7_SPEC>,
    #[doc = "0x100 - Counter/Timer Global Enable"]
    pub globen: crate::Reg<globen::GLOBEN_SPEC>,
    #[doc = "0x104 - Counter/Timer Output Config 0"]
    pub outcfg0: crate::Reg<outcfg0::OUTCFG0_SPEC>,
    #[doc = "0x108 - Counter/Timer Output Config 1"]
    pub outcfg1: crate::Reg<outcfg1::OUTCFG1_SPEC>,
    #[doc = "0x10c - Counter/Timer Output Config 2"]
    pub outcfg2: crate::Reg<outcfg2::OUTCFG2_SPEC>,
    _reserved60: [u8; 0x04],
    #[doc = "0x114 - Counter/Timer Output Config 3"]
    pub outcfg3: crate::Reg<outcfg3::OUTCFG3_SPEC>,
    #[doc = "0x118 - Counter/Timer Input Config"]
    pub incfg: crate::Reg<incfg::INCFG_SPEC>,
    _reserved62: [u8; 0x24],
    #[doc = "0x140 - Configuration Register"]
    pub stcfg: crate::Reg<stcfg::STCFG_SPEC>,
    #[doc = "0x144 - System Timer Count Register (Real Time Counter)"]
    pub sttmr: crate::Reg<sttmr::STTMR_SPEC>,
    #[doc = "0x148 - Capture Control Register"]
    pub capturecontrol: crate::Reg<capturecontrol::CAPTURECONTROL_SPEC>,
    _reserved65: [u8; 0x04],
    #[doc = "0x150 - Compare Register A"]
    pub scmpr0: crate::Reg<scmpr0::SCMPR0_SPEC>,
    #[doc = "0x154 - Compare Register B"]
    pub scmpr1: crate::Reg<scmpr1::SCMPR1_SPEC>,
    #[doc = "0x158 - Compare Register C"]
    pub scmpr2: crate::Reg<scmpr2::SCMPR2_SPEC>,
    #[doc = "0x15c - Compare Register D"]
    pub scmpr3: crate::Reg<scmpr3::SCMPR3_SPEC>,
    #[doc = "0x160 - Compare Register E"]
    pub scmpr4: crate::Reg<scmpr4::SCMPR4_SPEC>,
    #[doc = "0x164 - Compare Register F"]
    pub scmpr5: crate::Reg<scmpr5::SCMPR5_SPEC>,
    #[doc = "0x168 - Compare Register G"]
    pub scmpr6: crate::Reg<scmpr6::SCMPR6_SPEC>,
    #[doc = "0x16c - Compare Register H"]
    pub scmpr7: crate::Reg<scmpr7::SCMPR7_SPEC>,
    _reserved73: [u8; 0x70],
    #[doc = "0x1e0 - Capture Register A"]
    pub scapt0: crate::Reg<scapt0::SCAPT0_SPEC>,
    #[doc = "0x1e4 - Capture Register B"]
    pub scapt1: crate::Reg<scapt1::SCAPT1_SPEC>,
    #[doc = "0x1e8 - Capture Register C"]
    pub scapt2: crate::Reg<scapt2::SCAPT2_SPEC>,
    #[doc = "0x1ec - Capture Register D"]
    pub scapt3: crate::Reg<scapt3::SCAPT3_SPEC>,
    #[doc = "0x1f0 - System Timer NVRAM_A Register"]
    pub snvr0: crate::Reg<snvr0::SNVR0_SPEC>,
    #[doc = "0x1f4 - System Timer NVRAM_B Register"]
    pub snvr1: crate::Reg<snvr1::SNVR1_SPEC>,
    #[doc = "0x1f8 - System Timer NVRAM_C Register"]
    pub snvr2: crate::Reg<snvr2::SNVR2_SPEC>,
    #[doc = "0x1fc - System Timer NVRAM_D Register"]
    pub snvr3: crate::Reg<snvr3::SNVR3_SPEC>,
    #[doc = "0x200 - Counter/Timer Interrupts: Enable"]
    pub inten: crate::Reg<inten::INTEN_SPEC>,
    #[doc = "0x204 - Counter/Timer Interrupts: Status"]
    pub intstat: crate::Reg<intstat::INTSTAT_SPEC>,
    #[doc = "0x208 - Counter/Timer Interrupts: Clear"]
    pub intclr: crate::Reg<intclr::INTCLR_SPEC>,
    #[doc = "0x20c - Counter/Timer Interrupts: Set"]
    pub intset: crate::Reg<intset::INTSET_SPEC>,
    _reserved85: [u8; 0xf0],
    #[doc = "0x300 - STIMER Interrupt registers: Enable"]
    pub stminten: crate::Reg<stminten::STMINTEN_SPEC>,
    #[doc = "0x304 - STIMER Interrupt registers: Status"]
    pub stmintstat: crate::Reg<stmintstat::STMINTSTAT_SPEC>,
    #[doc = "0x308 - STIMER Interrupt registers: Clear"]
    pub stmintclr: crate::Reg<stmintclr::STMINTCLR_SPEC>,
    #[doc = "0x30c - STIMER Interrupt registers: Set"]
    pub stmintset: crate::Reg<stmintset::STMINTSET_SPEC>,
}
#[doc = "TMR0 register accessor: an alias for `Reg<TMR0_SPEC>`"]
pub type TMR0 = crate::Reg<tmr0::TMR0_SPEC>;
#[doc = "Counter/Timer Register"]
pub mod tmr0;
#[doc = "CMPRA0 register accessor: an alias for `Reg<CMPRA0_SPEC>`"]
pub type CMPRA0 = crate::Reg<cmpra0::CMPRA0_SPEC>;
#[doc = "Counter/Timer A0 Compare Registers"]
pub mod cmpra0;
#[doc = "CMPRB0 register accessor: an alias for `Reg<CMPRB0_SPEC>`"]
pub type CMPRB0 = crate::Reg<cmprb0::CMPRB0_SPEC>;
#[doc = "Counter/Timer B0 Compare Registers"]
pub mod cmprb0;
#[doc = "CTRL0 register accessor: an alias for `Reg<CTRL0_SPEC>`"]
pub type CTRL0 = crate::Reg<ctrl0::CTRL0_SPEC>;
#[doc = "Counter/Timer Control"]
pub mod ctrl0;
#[doc = "CMPRAUXA0 register accessor: an alias for `Reg<CMPRAUXA0_SPEC>`"]
pub type CMPRAUXA0 = crate::Reg<cmprauxa0::CMPRAUXA0_SPEC>;
#[doc = "Counter/Timer A0 Compare Registers"]
pub mod cmprauxa0;
#[doc = "CMPRAUXB0 register accessor: an alias for `Reg<CMPRAUXB0_SPEC>`"]
pub type CMPRAUXB0 = crate::Reg<cmprauxb0::CMPRAUXB0_SPEC>;
#[doc = "Counter/Timer B0 Compare Registers"]
pub mod cmprauxb0;
#[doc = "AUX0 register accessor: an alias for `Reg<AUX0_SPEC>`"]
pub type AUX0 = crate::Reg<aux0::AUX0_SPEC>;
#[doc = "Counter/Timer Auxiliary"]
pub mod aux0;
#[doc = "TMR1 register accessor: an alias for `Reg<TMR1_SPEC>`"]
pub type TMR1 = crate::Reg<tmr1::TMR1_SPEC>;
#[doc = "Counter/Timer Register"]
pub mod tmr1;
#[doc = "CMPRA1 register accessor: an alias for `Reg<CMPRA1_SPEC>`"]
pub type CMPRA1 = crate::Reg<cmpra1::CMPRA1_SPEC>;
#[doc = "Counter/Timer A1 Compare Registers"]
pub mod cmpra1;
#[doc = "CMPRB1 register accessor: an alias for `Reg<CMPRB1_SPEC>`"]
pub type CMPRB1 = crate::Reg<cmprb1::CMPRB1_SPEC>;
#[doc = "Counter/Timer B1 Compare Registers"]
pub mod cmprb1;
#[doc = "CTRL1 register accessor: an alias for `Reg<CTRL1_SPEC>`"]
pub type CTRL1 = crate::Reg<ctrl1::CTRL1_SPEC>;
#[doc = "Counter/Timer Control"]
pub mod ctrl1;
#[doc = "CMPRAUXA1 register accessor: an alias for `Reg<CMPRAUXA1_SPEC>`"]
pub type CMPRAUXA1 = crate::Reg<cmprauxa1::CMPRAUXA1_SPEC>;
#[doc = "Counter/Timer A1 Compare Registers"]
pub mod cmprauxa1;
#[doc = "CMPRAUXB1 register accessor: an alias for `Reg<CMPRAUXB1_SPEC>`"]
pub type CMPRAUXB1 = crate::Reg<cmprauxb1::CMPRAUXB1_SPEC>;
#[doc = "Counter/Timer B1 Compare Registers"]
pub mod cmprauxb1;
#[doc = "AUX1 register accessor: an alias for `Reg<AUX1_SPEC>`"]
pub type AUX1 = crate::Reg<aux1::AUX1_SPEC>;
#[doc = "Counter/Timer Auxiliary"]
pub mod aux1;
#[doc = "TMR2 register accessor: an alias for `Reg<TMR2_SPEC>`"]
pub type TMR2 = crate::Reg<tmr2::TMR2_SPEC>;
#[doc = "Counter/Timer Register"]
pub mod tmr2;
#[doc = "CMPRA2 register accessor: an alias for `Reg<CMPRA2_SPEC>`"]
pub type CMPRA2 = crate::Reg<cmpra2::CMPRA2_SPEC>;
#[doc = "Counter/Timer A2 Compare Registers"]
pub mod cmpra2;
#[doc = "CMPRB2 register accessor: an alias for `Reg<CMPRB2_SPEC>`"]
pub type CMPRB2 = crate::Reg<cmprb2::CMPRB2_SPEC>;
#[doc = "Counter/Timer B2 Compare Registers"]
pub mod cmprb2;
#[doc = "CTRL2 register accessor: an alias for `Reg<CTRL2_SPEC>`"]
pub type CTRL2 = crate::Reg<ctrl2::CTRL2_SPEC>;
#[doc = "Counter/Timer Control"]
pub mod ctrl2;
#[doc = "CMPRAUXA2 register accessor: an alias for `Reg<CMPRAUXA2_SPEC>`"]
pub type CMPRAUXA2 = crate::Reg<cmprauxa2::CMPRAUXA2_SPEC>;
#[doc = "Counter/Timer A2 Compare Registers"]
pub mod cmprauxa2;
#[doc = "CMPRAUXB2 register accessor: an alias for `Reg<CMPRAUXB2_SPEC>`"]
pub type CMPRAUXB2 = crate::Reg<cmprauxb2::CMPRAUXB2_SPEC>;
#[doc = "Counter/Timer B2 Compare Registers"]
pub mod cmprauxb2;
#[doc = "AUX2 register accessor: an alias for `Reg<AUX2_SPEC>`"]
pub type AUX2 = crate::Reg<aux2::AUX2_SPEC>;
#[doc = "Counter/Timer Auxiliary"]
pub mod aux2;
#[doc = "TMR3 register accessor: an alias for `Reg<TMR3_SPEC>`"]
pub type TMR3 = crate::Reg<tmr3::TMR3_SPEC>;
#[doc = "Counter/Timer Register"]
pub mod tmr3;
#[doc = "CMPRA3 register accessor: an alias for `Reg<CMPRA3_SPEC>`"]
pub type CMPRA3 = crate::Reg<cmpra3::CMPRA3_SPEC>;
#[doc = "Counter/Timer A3 Compare Registers"]
pub mod cmpra3;
#[doc = "CMPRB3 register accessor: an alias for `Reg<CMPRB3_SPEC>`"]
pub type CMPRB3 = crate::Reg<cmprb3::CMPRB3_SPEC>;
#[doc = "Counter/Timer B3 Compare Registers"]
pub mod cmprb3;
#[doc = "CTRL3 register accessor: an alias for `Reg<CTRL3_SPEC>`"]
pub type CTRL3 = crate::Reg<ctrl3::CTRL3_SPEC>;
#[doc = "Counter/Timer Control"]
pub mod ctrl3;
#[doc = "CMPRAUXA3 register accessor: an alias for `Reg<CMPRAUXA3_SPEC>`"]
pub type CMPRAUXA3 = crate::Reg<cmprauxa3::CMPRAUXA3_SPEC>;
#[doc = "Counter/Timer A3 Compare Registers"]
pub mod cmprauxa3;
#[doc = "CMPRAUXB3 register accessor: an alias for `Reg<CMPRAUXB3_SPEC>`"]
pub type CMPRAUXB3 = crate::Reg<cmprauxb3::CMPRAUXB3_SPEC>;
#[doc = "Counter/Timer B3 Compare Registers"]
pub mod cmprauxb3;
#[doc = "AUX3 register accessor: an alias for `Reg<AUX3_SPEC>`"]
pub type AUX3 = crate::Reg<aux3::AUX3_SPEC>;
#[doc = "Counter/Timer Auxiliary"]
pub mod aux3;
#[doc = "TMR4 register accessor: an alias for `Reg<TMR4_SPEC>`"]
pub type TMR4 = crate::Reg<tmr4::TMR4_SPEC>;
#[doc = "Counter/Timer Register"]
pub mod tmr4;
#[doc = "CMPRA4 register accessor: an alias for `Reg<CMPRA4_SPEC>`"]
pub type CMPRA4 = crate::Reg<cmpra4::CMPRA4_SPEC>;
#[doc = "Counter/Timer A4 Compare Registers"]
pub mod cmpra4;
#[doc = "CMPRB4 register accessor: an alias for `Reg<CMPRB4_SPEC>`"]
pub type CMPRB4 = crate::Reg<cmprb4::CMPRB4_SPEC>;
#[doc = "Counter/Timer B4 Compare Registers"]
pub mod cmprb4;
#[doc = "CTRL4 register accessor: an alias for `Reg<CTRL4_SPEC>`"]
pub type CTRL4 = crate::Reg<ctrl4::CTRL4_SPEC>;
#[doc = "Counter/Timer Control"]
pub mod ctrl4;
#[doc = "CMPRAUXA4 register accessor: an alias for `Reg<CMPRAUXA4_SPEC>`"]
pub type CMPRAUXA4 = crate::Reg<cmprauxa4::CMPRAUXA4_SPEC>;
#[doc = "Counter/Timer A4 Compare Registers"]
pub mod cmprauxa4;
#[doc = "CMPRAUXB4 register accessor: an alias for `Reg<CMPRAUXB4_SPEC>`"]
pub type CMPRAUXB4 = crate::Reg<cmprauxb4::CMPRAUXB4_SPEC>;
#[doc = "Counter/Timer B4 Compare Registers"]
pub mod cmprauxb4;
#[doc = "AUX4 register accessor: an alias for `Reg<AUX4_SPEC>`"]
pub type AUX4 = crate::Reg<aux4::AUX4_SPEC>;
#[doc = "Counter/Timer Auxiliary"]
pub mod aux4;
#[doc = "TMR5 register accessor: an alias for `Reg<TMR5_SPEC>`"]
pub type TMR5 = crate::Reg<tmr5::TMR5_SPEC>;
#[doc = "Counter/Timer Register"]
pub mod tmr5;
#[doc = "CMPRA5 register accessor: an alias for `Reg<CMPRA5_SPEC>`"]
pub type CMPRA5 = crate::Reg<cmpra5::CMPRA5_SPEC>;
#[doc = "Counter/Timer A5 Compare Registers"]
pub mod cmpra5;
#[doc = "CMPRB5 register accessor: an alias for `Reg<CMPRB5_SPEC>`"]
pub type CMPRB5 = crate::Reg<cmprb5::CMPRB5_SPEC>;
#[doc = "Counter/Timer B5 Compare Registers"]
pub mod cmprb5;
#[doc = "CTRL5 register accessor: an alias for `Reg<CTRL5_SPEC>`"]
pub type CTRL5 = crate::Reg<ctrl5::CTRL5_SPEC>;
#[doc = "Counter/Timer Control"]
pub mod ctrl5;
#[doc = "CMPRAUXA5 register accessor: an alias for `Reg<CMPRAUXA5_SPEC>`"]
pub type CMPRAUXA5 = crate::Reg<cmprauxa5::CMPRAUXA5_SPEC>;
#[doc = "Counter/Timer A5 Compare Registers"]
pub mod cmprauxa5;
#[doc = "CMPRAUXB5 register accessor: an alias for `Reg<CMPRAUXB5_SPEC>`"]
pub type CMPRAUXB5 = crate::Reg<cmprauxb5::CMPRAUXB5_SPEC>;
#[doc = "Counter/Timer B5 Compare Registers"]
pub mod cmprauxb5;
#[doc = "AUX5 register accessor: an alias for `Reg<AUX5_SPEC>`"]
pub type AUX5 = crate::Reg<aux5::AUX5_SPEC>;
#[doc = "Counter/Timer Auxiliary"]
pub mod aux5;
#[doc = "TMR6 register accessor: an alias for `Reg<TMR6_SPEC>`"]
pub type TMR6 = crate::Reg<tmr6::TMR6_SPEC>;
#[doc = "Counter/Timer Register"]
pub mod tmr6;
#[doc = "CMPRA6 register accessor: an alias for `Reg<CMPRA6_SPEC>`"]
pub type CMPRA6 = crate::Reg<cmpra6::CMPRA6_SPEC>;
#[doc = "Counter/Timer A6 Compare Registers"]
pub mod cmpra6;
#[doc = "CMPRB6 register accessor: an alias for `Reg<CMPRB6_SPEC>`"]
pub type CMPRB6 = crate::Reg<cmprb6::CMPRB6_SPEC>;
#[doc = "Counter/Timer B6 Compare Registers"]
pub mod cmprb6;
#[doc = "CTRL6 register accessor: an alias for `Reg<CTRL6_SPEC>`"]
pub type CTRL6 = crate::Reg<ctrl6::CTRL6_SPEC>;
#[doc = "Counter/Timer Control"]
pub mod ctrl6;
#[doc = "CMPRAUXA6 register accessor: an alias for `Reg<CMPRAUXA6_SPEC>`"]
pub type CMPRAUXA6 = crate::Reg<cmprauxa6::CMPRAUXA6_SPEC>;
#[doc = "Counter/Timer A6 Compare Registers"]
pub mod cmprauxa6;
#[doc = "CMPRAUXB6 register accessor: an alias for `Reg<CMPRAUXB6_SPEC>`"]
pub type CMPRAUXB6 = crate::Reg<cmprauxb6::CMPRAUXB6_SPEC>;
#[doc = "Counter/Timer B6 Compare Registers"]
pub mod cmprauxb6;
#[doc = "AUX6 register accessor: an alias for `Reg<AUX6_SPEC>`"]
pub type AUX6 = crate::Reg<aux6::AUX6_SPEC>;
#[doc = "Counter/Timer Auxiliary"]
pub mod aux6;
#[doc = "TMR7 register accessor: an alias for `Reg<TMR7_SPEC>`"]
pub type TMR7 = crate::Reg<tmr7::TMR7_SPEC>;
#[doc = "Counter/Timer Register"]
pub mod tmr7;
#[doc = "CMPRA7 register accessor: an alias for `Reg<CMPRA7_SPEC>`"]
pub type CMPRA7 = crate::Reg<cmpra7::CMPRA7_SPEC>;
#[doc = "Counter/Timer A7 Compare Registers"]
pub mod cmpra7;
#[doc = "CMPRB7 register accessor: an alias for `Reg<CMPRB7_SPEC>`"]
pub type CMPRB7 = crate::Reg<cmprb7::CMPRB7_SPEC>;
#[doc = "Counter/Timer B7 Compare Registers"]
pub mod cmprb7;
#[doc = "CTRL7 register accessor: an alias for `Reg<CTRL7_SPEC>`"]
pub type CTRL7 = crate::Reg<ctrl7::CTRL7_SPEC>;
#[doc = "Counter/Timer Control"]
pub mod ctrl7;
#[doc = "CMPRAUXA7 register accessor: an alias for `Reg<CMPRAUXA7_SPEC>`"]
pub type CMPRAUXA7 = crate::Reg<cmprauxa7::CMPRAUXA7_SPEC>;
#[doc = "Counter/Timer A7 Compare Registers"]
pub mod cmprauxa7;
#[doc = "CMPRAUXB7 register accessor: an alias for `Reg<CMPRAUXB7_SPEC>`"]
pub type CMPRAUXB7 = crate::Reg<cmprauxb7::CMPRAUXB7_SPEC>;
#[doc = "Counter/Timer B7 Compare Registers"]
pub mod cmprauxb7;
#[doc = "AUX7 register accessor: an alias for `Reg<AUX7_SPEC>`"]
pub type AUX7 = crate::Reg<aux7::AUX7_SPEC>;
#[doc = "Counter/Timer Auxiliary"]
pub mod aux7;
#[doc = "GLOBEN register accessor: an alias for `Reg<GLOBEN_SPEC>`"]
pub type GLOBEN = crate::Reg<globen::GLOBEN_SPEC>;
#[doc = "Counter/Timer Global Enable"]
pub mod globen;
#[doc = "OUTCFG0 register accessor: an alias for `Reg<OUTCFG0_SPEC>`"]
pub type OUTCFG0 = crate::Reg<outcfg0::OUTCFG0_SPEC>;
#[doc = "Counter/Timer Output Config 0"]
pub mod outcfg0;
#[doc = "OUTCFG1 register accessor: an alias for `Reg<OUTCFG1_SPEC>`"]
pub type OUTCFG1 = crate::Reg<outcfg1::OUTCFG1_SPEC>;
#[doc = "Counter/Timer Output Config 1"]
pub mod outcfg1;
#[doc = "OUTCFG2 register accessor: an alias for `Reg<OUTCFG2_SPEC>`"]
pub type OUTCFG2 = crate::Reg<outcfg2::OUTCFG2_SPEC>;
#[doc = "Counter/Timer Output Config 2"]
pub mod outcfg2;
#[doc = "OUTCFG3 register accessor: an alias for `Reg<OUTCFG3_SPEC>`"]
pub type OUTCFG3 = crate::Reg<outcfg3::OUTCFG3_SPEC>;
#[doc = "Counter/Timer Output Config 3"]
pub mod outcfg3;
#[doc = "INCFG register accessor: an alias for `Reg<INCFG_SPEC>`"]
pub type INCFG = crate::Reg<incfg::INCFG_SPEC>;
#[doc = "Counter/Timer Input Config"]
pub mod incfg;
#[doc = "STCFG register accessor: an alias for `Reg<STCFG_SPEC>`"]
pub type STCFG = crate::Reg<stcfg::STCFG_SPEC>;
#[doc = "Configuration Register"]
pub mod stcfg;
#[doc = "STTMR register accessor: an alias for `Reg<STTMR_SPEC>`"]
pub type STTMR = crate::Reg<sttmr::STTMR_SPEC>;
#[doc = "System Timer Count Register (Real Time Counter)"]
pub mod sttmr;
#[doc = "CAPTURECONTROL register accessor: an alias for `Reg<CAPTURECONTROL_SPEC>`"]
pub type CAPTURECONTROL = crate::Reg<capturecontrol::CAPTURECONTROL_SPEC>;
#[doc = "Capture Control Register"]
pub mod capturecontrol;
#[doc = "SCMPR0 register accessor: an alias for `Reg<SCMPR0_SPEC>`"]
pub type SCMPR0 = crate::Reg<scmpr0::SCMPR0_SPEC>;
#[doc = "Compare Register A"]
pub mod scmpr0;
#[doc = "SCMPR1 register accessor: an alias for `Reg<SCMPR1_SPEC>`"]
pub type SCMPR1 = crate::Reg<scmpr1::SCMPR1_SPEC>;
#[doc = "Compare Register B"]
pub mod scmpr1;
#[doc = "SCMPR2 register accessor: an alias for `Reg<SCMPR2_SPEC>`"]
pub type SCMPR2 = crate::Reg<scmpr2::SCMPR2_SPEC>;
#[doc = "Compare Register C"]
pub mod scmpr2;
#[doc = "SCMPR3 register accessor: an alias for `Reg<SCMPR3_SPEC>`"]
pub type SCMPR3 = crate::Reg<scmpr3::SCMPR3_SPEC>;
#[doc = "Compare Register D"]
pub mod scmpr3;
#[doc = "SCMPR4 register accessor: an alias for `Reg<SCMPR4_SPEC>`"]
pub type SCMPR4 = crate::Reg<scmpr4::SCMPR4_SPEC>;
#[doc = "Compare Register E"]
pub mod scmpr4;
#[doc = "SCMPR5 register accessor: an alias for `Reg<SCMPR5_SPEC>`"]
pub type SCMPR5 = crate::Reg<scmpr5::SCMPR5_SPEC>;
#[doc = "Compare Register F"]
pub mod scmpr5;
#[doc = "SCMPR6 register accessor: an alias for `Reg<SCMPR6_SPEC>`"]
pub type SCMPR6 = crate::Reg<scmpr6::SCMPR6_SPEC>;
#[doc = "Compare Register G"]
pub mod scmpr6;
#[doc = "SCMPR7 register accessor: an alias for `Reg<SCMPR7_SPEC>`"]
pub type SCMPR7 = crate::Reg<scmpr7::SCMPR7_SPEC>;
#[doc = "Compare Register H"]
pub mod scmpr7;
#[doc = "SCAPT0 register accessor: an alias for `Reg<SCAPT0_SPEC>`"]
pub type SCAPT0 = crate::Reg<scapt0::SCAPT0_SPEC>;
#[doc = "Capture Register A"]
pub mod scapt0;
#[doc = "SCAPT1 register accessor: an alias for `Reg<SCAPT1_SPEC>`"]
pub type SCAPT1 = crate::Reg<scapt1::SCAPT1_SPEC>;
#[doc = "Capture Register B"]
pub mod scapt1;
#[doc = "SCAPT2 register accessor: an alias for `Reg<SCAPT2_SPEC>`"]
pub type SCAPT2 = crate::Reg<scapt2::SCAPT2_SPEC>;
#[doc = "Capture Register C"]
pub mod scapt2;
#[doc = "SCAPT3 register accessor: an alias for `Reg<SCAPT3_SPEC>`"]
pub type SCAPT3 = crate::Reg<scapt3::SCAPT3_SPEC>;
#[doc = "Capture Register D"]
pub mod scapt3;
#[doc = "SNVR0 register accessor: an alias for `Reg<SNVR0_SPEC>`"]
pub type SNVR0 = crate::Reg<snvr0::SNVR0_SPEC>;
#[doc = "System Timer NVRAM_A Register"]
pub mod snvr0;
#[doc = "SNVR1 register accessor: an alias for `Reg<SNVR1_SPEC>`"]
pub type SNVR1 = crate::Reg<snvr1::SNVR1_SPEC>;
#[doc = "System Timer NVRAM_B Register"]
pub mod snvr1;
#[doc = "SNVR2 register accessor: an alias for `Reg<SNVR2_SPEC>`"]
pub type SNVR2 = crate::Reg<snvr2::SNVR2_SPEC>;
#[doc = "System Timer NVRAM_C Register"]
pub mod snvr2;
#[doc = "SNVR3 register accessor: an alias for `Reg<SNVR3_SPEC>`"]
pub type SNVR3 = crate::Reg<snvr3::SNVR3_SPEC>;
#[doc = "System Timer NVRAM_D Register"]
pub mod snvr3;
#[doc = "INTEN register accessor: an alias for `Reg<INTEN_SPEC>`"]
pub type INTEN = crate::Reg<inten::INTEN_SPEC>;
#[doc = "Counter/Timer Interrupts: Enable"]
pub mod inten;
#[doc = "INTSTAT register accessor: an alias for `Reg<INTSTAT_SPEC>`"]
pub type INTSTAT = crate::Reg<intstat::INTSTAT_SPEC>;
#[doc = "Counter/Timer Interrupts: Status"]
pub mod intstat;
#[doc = "INTCLR register accessor: an alias for `Reg<INTCLR_SPEC>`"]
pub type INTCLR = crate::Reg<intclr::INTCLR_SPEC>;
#[doc = "Counter/Timer Interrupts: Clear"]
pub mod intclr;
#[doc = "INTSET register accessor: an alias for `Reg<INTSET_SPEC>`"]
pub type INTSET = crate::Reg<intset::INTSET_SPEC>;
#[doc = "Counter/Timer Interrupts: Set"]
pub mod intset;
#[doc = "STMINTEN register accessor: an alias for `Reg<STMINTEN_SPEC>`"]
pub type STMINTEN = crate::Reg<stminten::STMINTEN_SPEC>;
#[doc = "STIMER Interrupt registers: Enable"]
pub mod stminten;
#[doc = "STMINTSTAT register accessor: an alias for `Reg<STMINTSTAT_SPEC>`"]
pub type STMINTSTAT = crate::Reg<stmintstat::STMINTSTAT_SPEC>;
#[doc = "STIMER Interrupt registers: Status"]
pub mod stmintstat;
#[doc = "STMINTCLR register accessor: an alias for `Reg<STMINTCLR_SPEC>`"]
pub type STMINTCLR = crate::Reg<stmintclr::STMINTCLR_SPEC>;
#[doc = "STIMER Interrupt registers: Clear"]
pub mod stmintclr;
#[doc = "STMINTSET register accessor: an alias for `Reg<STMINTSET_SPEC>`"]
pub type STMINTSET = crate::Reg<stmintset::STMINTSET_SPEC>;
#[doc = "STIMER Interrupt registers: Set"]
pub mod stmintset;
