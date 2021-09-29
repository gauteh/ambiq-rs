#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Pad Configuration Register A (Pads 0-3)"]
    pub padrega: crate::Reg<padrega::PADREGA_SPEC>,
    #[doc = "0x04 - Pad Configuration Register B (Pads 4-7)"]
    pub padregb: crate::Reg<padregb::PADREGB_SPEC>,
    #[doc = "0x08 - Pad Configuration Register C (Pads 8-11)"]
    pub padregc: crate::Reg<padregc::PADREGC_SPEC>,
    #[doc = "0x0c - Pad Configuration Register D (Pads 12-15)"]
    pub padregd: crate::Reg<padregd::PADREGD_SPEC>,
    #[doc = "0x10 - Pad Configuration Register E (Pads 16-19)"]
    pub padrege: crate::Reg<padrege::PADREGE_SPEC>,
    #[doc = "0x14 - Pad Configuration Register F (Pads 20-23)"]
    pub padregf: crate::Reg<padregf::PADREGF_SPEC>,
    #[doc = "0x18 - Pad Configuration Register G (Pads 24-27)"]
    pub padregg: crate::Reg<padregg::PADREGG_SPEC>,
    #[doc = "0x1c - Pad Configuration Register H (Pads 28-31)"]
    pub padregh: crate::Reg<padregh::PADREGH_SPEC>,
    #[doc = "0x20 - Pad Configuration Register I (Pads 32-25)"]
    pub padregi: crate::Reg<padregi::PADREGI_SPEC>,
    #[doc = "0x24 - Pad Configuration Register J (Pads 36-39)"]
    pub padregj: crate::Reg<padregj::PADREGJ_SPEC>,
    #[doc = "0x28 - Pad Configuration Register K (Pads 40-43)"]
    pub padregk: crate::Reg<padregk::PADREGK_SPEC>,
    #[doc = "0x2c - Pad Configuration Register L (Pads 44-47)"]
    pub padregl: crate::Reg<padregl::PADREGL_SPEC>,
    #[doc = "0x30 - Pad Configuration Register M (Pads 47-48)"]
    pub padregm: crate::Reg<padregm::PADREGM_SPEC>,
    _reserved13: [u8; 0x0c],
    #[doc = "0x40 - GPIO Configuration Register A (Pads 0-7)"]
    pub cfga: crate::Reg<cfga::CFGA_SPEC>,
    #[doc = "0x44 - GPIO Configuration Register B (Pads 8-15)"]
    pub cfgb: crate::Reg<cfgb::CFGB_SPEC>,
    #[doc = "0x48 - GPIO Configuration Register C (Pads 16-23)"]
    pub cfgc: crate::Reg<cfgc::CFGC_SPEC>,
    #[doc = "0x4c - GPIO Configuration Register D (Pads 24-31)"]
    pub cfgd: crate::Reg<cfgd::CFGD_SPEC>,
    #[doc = "0x50 - GPIO Configuration Register E (Pads 32-39)"]
    pub cfge: crate::Reg<cfge::CFGE_SPEC>,
    #[doc = "0x54 - GPIO Configuration Register F (Pads 40 -47)"]
    pub cfgf: crate::Reg<cfgf::CFGF_SPEC>,
    #[doc = "0x58 - GPIO Configuration Register G (Pads 48-49)"]
    pub cfgg: crate::Reg<cfgg::CFGG_SPEC>,
    _reserved20: [u8; 0x04],
    #[doc = "0x60 - Key Register for all pad configuration registers"]
    pub padkey: crate::Reg<padkey::PADKEY_SPEC>,
    _reserved21: [u8; 0x1c],
    #[doc = "0x80 - GPIO Input Register A"]
    pub rda: crate::Reg<rda::RDA_SPEC>,
    #[doc = "0x84 - GPIO Input Register B"]
    pub rdb: crate::Reg<rdb::RDB_SPEC>,
    #[doc = "0x88 - GPIO Output Register A"]
    pub wta: crate::Reg<wta::WTA_SPEC>,
    #[doc = "0x8c - GPIO Output Register B"]
    pub wtb: crate::Reg<wtb::WTB_SPEC>,
    #[doc = "0x90 - GPIO Output Register A Set"]
    pub wtsa: crate::Reg<wtsa::WTSA_SPEC>,
    #[doc = "0x94 - GPIO Output Register B Set"]
    pub wtsb: crate::Reg<wtsb::WTSB_SPEC>,
    #[doc = "0x98 - GPIO Output Register A Clear"]
    pub wtca: crate::Reg<wtca::WTCA_SPEC>,
    #[doc = "0x9c - GPIO Output Register B Clear"]
    pub wtcb: crate::Reg<wtcb::WTCB_SPEC>,
    #[doc = "0xa0 - GPIO Enable Register A"]
    pub ena: crate::Reg<ena::ENA_SPEC>,
    #[doc = "0xa4 - GPIO Enable Register B"]
    pub enb: crate::Reg<enb::ENB_SPEC>,
    #[doc = "0xa8 - GPIO Enable Register A Set"]
    pub ensa: crate::Reg<ensa::ENSA_SPEC>,
    #[doc = "0xac - GPIO Enable Register B Set"]
    pub ensb: crate::Reg<ensb::ENSB_SPEC>,
    _reserved33: [u8; 0x04],
    #[doc = "0xb4 - GPIO Enable Register A Clear"]
    pub enca: crate::Reg<enca::ENCA_SPEC>,
    #[doc = "0xb8 - GPIO Enable Register B Clear"]
    pub encb: crate::Reg<encb::ENCB_SPEC>,
    #[doc = "0xbc - STIMER Capture Control"]
    pub stmrcap: crate::Reg<stmrcap::STMRCAP_SPEC>,
    #[doc = "0xc0 - IOM0 Flow Control IRQ Select"]
    pub iom0irq: crate::Reg<iom0irq::IOM0IRQ_SPEC>,
    #[doc = "0xc4 - IOM1 Flow Control IRQ Select"]
    pub iom1irq: crate::Reg<iom1irq::IOM1IRQ_SPEC>,
    #[doc = "0xc8 - IOM2 Flow Control IRQ Select"]
    pub iom2irq: crate::Reg<iom2irq::IOM2IRQ_SPEC>,
    #[doc = "0xcc - IOM3 Flow Control IRQ Select"]
    pub iom3irq: crate::Reg<iom3irq::IOM3IRQ_SPEC>,
    #[doc = "0xd0 - IOM4 Flow Control IRQ Select"]
    pub iom4irq: crate::Reg<iom4irq::IOM4IRQ_SPEC>,
    #[doc = "0xd4 - IOM5 Flow Control IRQ Select"]
    pub iom5irq: crate::Reg<iom5irq::IOM5IRQ_SPEC>,
    #[doc = "0xd8 - BLEIF Flow Control IRQ Select"]
    pub bleifirq: crate::Reg<bleifirq::BLEIFIRQ_SPEC>,
    #[doc = "0xdc - GPIO Observation Mode Sample register"]
    pub gpioobs: crate::Reg<gpioobs::GPIOOBS_SPEC>,
    #[doc = "0xe0 - Alternate Pad Configuration reg0 (Pads 3,2,1,0)"]
    pub altpadcfga: crate::Reg<altpadcfga::ALTPADCFGA_SPEC>,
    #[doc = "0xe4 - Alternate Pad Configuration reg1 (Pads 7,6,5,4)"]
    pub altpadcfgb: crate::Reg<altpadcfgb::ALTPADCFGB_SPEC>,
    #[doc = "0xe8 - Alternate Pad Configuration reg2 (Pads 11,10,9,8)"]
    pub altpadcfgc: crate::Reg<altpadcfgc::ALTPADCFGC_SPEC>,
    #[doc = "0xec - Alternate Pad Configuration reg3 (Pads 15,14,13,12)"]
    pub altpadcfgd: crate::Reg<altpadcfgd::ALTPADCFGD_SPEC>,
    #[doc = "0xf0 - Alternate Pad Configuration reg4 (Pads 19,18,17,16)"]
    pub altpadcfge: crate::Reg<altpadcfge::ALTPADCFGE_SPEC>,
    #[doc = "0xf4 - Alternate Pad Configuration reg5 (Pads 23,22,21,20)"]
    pub altpadcfgf: crate::Reg<altpadcfgf::ALTPADCFGF_SPEC>,
    #[doc = "0xf8 - Alternate Pad Configuration reg6 (Pads 27,26,25,24)"]
    pub altpadcfgg: crate::Reg<altpadcfgg::ALTPADCFGG_SPEC>,
    #[doc = "0xfc - Alternate Pad Configuration reg7 (Pads 31,30,29,28)"]
    pub altpadcfgh: crate::Reg<altpadcfgh::ALTPADCFGH_SPEC>,
    #[doc = "0x100 - Alternate Pad Configuration reg8 (Pads 35,34,33,32)"]
    pub altpadcfgi: crate::Reg<altpadcfgi::ALTPADCFGI_SPEC>,
    #[doc = "0x104 - Alternate Pad Configuration reg9 (Pads 39,38,37,36)"]
    pub altpadcfgj: crate::Reg<altpadcfgj::ALTPADCFGJ_SPEC>,
    #[doc = "0x108 - Alternate Pad Configuration reg10 (Pads 43,42,41,40)"]
    pub altpadcfgk: crate::Reg<altpadcfgk::ALTPADCFGK_SPEC>,
    #[doc = "0x10c - Alternate Pad Configuration reg11 (Pads 47,46,45,44)"]
    pub altpadcfgl: crate::Reg<altpadcfgl::ALTPADCFGL_SPEC>,
    #[doc = "0x110 - Alternate Pad Configuration reg12 (Pads 49,48)"]
    pub altpadcfgm: crate::Reg<altpadcfgm::ALTPADCFGM_SPEC>,
    #[doc = "0x114 - SCARD Card Detect select"]
    pub scdet: crate::Reg<scdet::SCDET_SPEC>,
    #[doc = "0x118 - Counter/Timer Enable Config"]
    pub ctencfg: crate::Reg<ctencfg::CTENCFG_SPEC>,
    _reserved59: [u8; 0xe4],
    #[doc = "0x200 - GPIO Interrupt Registers 31-0: Enable"]
    pub int0en: crate::Reg<int0en::INT0EN_SPEC>,
    #[doc = "0x204 - GPIO Interrupt Registers 31-0: Status"]
    pub int0stat: crate::Reg<int0stat::INT0STAT_SPEC>,
    #[doc = "0x208 - GPIO Interrupt Registers 31-0: Clear"]
    pub int0clr: crate::Reg<int0clr::INT0CLR_SPEC>,
    #[doc = "0x20c - GPIO Interrupt Registers 31-0: Set"]
    pub int0set: crate::Reg<int0set::INT0SET_SPEC>,
    #[doc = "0x210 - GPIO Interrupt Registers 49-32: Enable"]
    pub int1en: crate::Reg<int1en::INT1EN_SPEC>,
    #[doc = "0x214 - GPIO Interrupt Registers 49-32: Status"]
    pub int1stat: crate::Reg<int1stat::INT1STAT_SPEC>,
    #[doc = "0x218 - GPIO Interrupt Registers 49-32: Clear"]
    pub int1clr: crate::Reg<int1clr::INT1CLR_SPEC>,
    #[doc = "0x21c - GPIO Interrupt Registers 49-32: Set"]
    pub int1set: crate::Reg<int1set::INT1SET_SPEC>,
}
#[doc = "PADREGA register accessor: an alias for `Reg<PADREGA_SPEC>`"]
pub type PADREGA = crate::Reg<padrega::PADREGA_SPEC>;
#[doc = "Pad Configuration Register A (Pads 0-3)"]
pub mod padrega;
#[doc = "PADREGB register accessor: an alias for `Reg<PADREGB_SPEC>`"]
pub type PADREGB = crate::Reg<padregb::PADREGB_SPEC>;
#[doc = "Pad Configuration Register B (Pads 4-7)"]
pub mod padregb;
#[doc = "PADREGC register accessor: an alias for `Reg<PADREGC_SPEC>`"]
pub type PADREGC = crate::Reg<padregc::PADREGC_SPEC>;
#[doc = "Pad Configuration Register C (Pads 8-11)"]
pub mod padregc;
#[doc = "PADREGD register accessor: an alias for `Reg<PADREGD_SPEC>`"]
pub type PADREGD = crate::Reg<padregd::PADREGD_SPEC>;
#[doc = "Pad Configuration Register D (Pads 12-15)"]
pub mod padregd;
#[doc = "PADREGE register accessor: an alias for `Reg<PADREGE_SPEC>`"]
pub type PADREGE = crate::Reg<padrege::PADREGE_SPEC>;
#[doc = "Pad Configuration Register E (Pads 16-19)"]
pub mod padrege;
#[doc = "PADREGF register accessor: an alias for `Reg<PADREGF_SPEC>`"]
pub type PADREGF = crate::Reg<padregf::PADREGF_SPEC>;
#[doc = "Pad Configuration Register F (Pads 20-23)"]
pub mod padregf;
#[doc = "PADREGG register accessor: an alias for `Reg<PADREGG_SPEC>`"]
pub type PADREGG = crate::Reg<padregg::PADREGG_SPEC>;
#[doc = "Pad Configuration Register G (Pads 24-27)"]
pub mod padregg;
#[doc = "PADREGH register accessor: an alias for `Reg<PADREGH_SPEC>`"]
pub type PADREGH = crate::Reg<padregh::PADREGH_SPEC>;
#[doc = "Pad Configuration Register H (Pads 28-31)"]
pub mod padregh;
#[doc = "PADREGI register accessor: an alias for `Reg<PADREGI_SPEC>`"]
pub type PADREGI = crate::Reg<padregi::PADREGI_SPEC>;
#[doc = "Pad Configuration Register I (Pads 32-25)"]
pub mod padregi;
#[doc = "PADREGJ register accessor: an alias for `Reg<PADREGJ_SPEC>`"]
pub type PADREGJ = crate::Reg<padregj::PADREGJ_SPEC>;
#[doc = "Pad Configuration Register J (Pads 36-39)"]
pub mod padregj;
#[doc = "PADREGK register accessor: an alias for `Reg<PADREGK_SPEC>`"]
pub type PADREGK = crate::Reg<padregk::PADREGK_SPEC>;
#[doc = "Pad Configuration Register K (Pads 40-43)"]
pub mod padregk;
#[doc = "PADREGL register accessor: an alias for `Reg<PADREGL_SPEC>`"]
pub type PADREGL = crate::Reg<padregl::PADREGL_SPEC>;
#[doc = "Pad Configuration Register L (Pads 44-47)"]
pub mod padregl;
#[doc = "PADREGM register accessor: an alias for `Reg<PADREGM_SPEC>`"]
pub type PADREGM = crate::Reg<padregm::PADREGM_SPEC>;
#[doc = "Pad Configuration Register M (Pads 47-48)"]
pub mod padregm;
#[doc = "CFGA register accessor: an alias for `Reg<CFGA_SPEC>`"]
pub type CFGA = crate::Reg<cfga::CFGA_SPEC>;
#[doc = "GPIO Configuration Register A (Pads 0-7)"]
pub mod cfga;
#[doc = "CFGB register accessor: an alias for `Reg<CFGB_SPEC>`"]
pub type CFGB = crate::Reg<cfgb::CFGB_SPEC>;
#[doc = "GPIO Configuration Register B (Pads 8-15)"]
pub mod cfgb;
#[doc = "CFGC register accessor: an alias for `Reg<CFGC_SPEC>`"]
pub type CFGC = crate::Reg<cfgc::CFGC_SPEC>;
#[doc = "GPIO Configuration Register C (Pads 16-23)"]
pub mod cfgc;
#[doc = "CFGD register accessor: an alias for `Reg<CFGD_SPEC>`"]
pub type CFGD = crate::Reg<cfgd::CFGD_SPEC>;
#[doc = "GPIO Configuration Register D (Pads 24-31)"]
pub mod cfgd;
#[doc = "CFGE register accessor: an alias for `Reg<CFGE_SPEC>`"]
pub type CFGE = crate::Reg<cfge::CFGE_SPEC>;
#[doc = "GPIO Configuration Register E (Pads 32-39)"]
pub mod cfge;
#[doc = "CFGF register accessor: an alias for `Reg<CFGF_SPEC>`"]
pub type CFGF = crate::Reg<cfgf::CFGF_SPEC>;
#[doc = "GPIO Configuration Register F (Pads 40 -47)"]
pub mod cfgf;
#[doc = "CFGG register accessor: an alias for `Reg<CFGG_SPEC>`"]
pub type CFGG = crate::Reg<cfgg::CFGG_SPEC>;
#[doc = "GPIO Configuration Register G (Pads 48-49)"]
pub mod cfgg;
#[doc = "PADKEY register accessor: an alias for `Reg<PADKEY_SPEC>`"]
pub type PADKEY = crate::Reg<padkey::PADKEY_SPEC>;
#[doc = "Key Register for all pad configuration registers"]
pub mod padkey;
#[doc = "RDA register accessor: an alias for `Reg<RDA_SPEC>`"]
pub type RDA = crate::Reg<rda::RDA_SPEC>;
#[doc = "GPIO Input Register A"]
pub mod rda;
#[doc = "RDB register accessor: an alias for `Reg<RDB_SPEC>`"]
pub type RDB = crate::Reg<rdb::RDB_SPEC>;
#[doc = "GPIO Input Register B"]
pub mod rdb;
#[doc = "WTA register accessor: an alias for `Reg<WTA_SPEC>`"]
pub type WTA = crate::Reg<wta::WTA_SPEC>;
#[doc = "GPIO Output Register A"]
pub mod wta;
#[doc = "WTB register accessor: an alias for `Reg<WTB_SPEC>`"]
pub type WTB = crate::Reg<wtb::WTB_SPEC>;
#[doc = "GPIO Output Register B"]
pub mod wtb;
#[doc = "WTSA register accessor: an alias for `Reg<WTSA_SPEC>`"]
pub type WTSA = crate::Reg<wtsa::WTSA_SPEC>;
#[doc = "GPIO Output Register A Set"]
pub mod wtsa;
#[doc = "WTSB register accessor: an alias for `Reg<WTSB_SPEC>`"]
pub type WTSB = crate::Reg<wtsb::WTSB_SPEC>;
#[doc = "GPIO Output Register B Set"]
pub mod wtsb;
#[doc = "WTCA register accessor: an alias for `Reg<WTCA_SPEC>`"]
pub type WTCA = crate::Reg<wtca::WTCA_SPEC>;
#[doc = "GPIO Output Register A Clear"]
pub mod wtca;
#[doc = "WTCB register accessor: an alias for `Reg<WTCB_SPEC>`"]
pub type WTCB = crate::Reg<wtcb::WTCB_SPEC>;
#[doc = "GPIO Output Register B Clear"]
pub mod wtcb;
#[doc = "ENA register accessor: an alias for `Reg<ENA_SPEC>`"]
pub type ENA = crate::Reg<ena::ENA_SPEC>;
#[doc = "GPIO Enable Register A"]
pub mod ena;
#[doc = "ENB register accessor: an alias for `Reg<ENB_SPEC>`"]
pub type ENB = crate::Reg<enb::ENB_SPEC>;
#[doc = "GPIO Enable Register B"]
pub mod enb;
#[doc = "ENSA register accessor: an alias for `Reg<ENSA_SPEC>`"]
pub type ENSA = crate::Reg<ensa::ENSA_SPEC>;
#[doc = "GPIO Enable Register A Set"]
pub mod ensa;
#[doc = "ENSB register accessor: an alias for `Reg<ENSB_SPEC>`"]
pub type ENSB = crate::Reg<ensb::ENSB_SPEC>;
#[doc = "GPIO Enable Register B Set"]
pub mod ensb;
#[doc = "ENCA register accessor: an alias for `Reg<ENCA_SPEC>`"]
pub type ENCA = crate::Reg<enca::ENCA_SPEC>;
#[doc = "GPIO Enable Register A Clear"]
pub mod enca;
#[doc = "ENCB register accessor: an alias for `Reg<ENCB_SPEC>`"]
pub type ENCB = crate::Reg<encb::ENCB_SPEC>;
#[doc = "GPIO Enable Register B Clear"]
pub mod encb;
#[doc = "STMRCAP register accessor: an alias for `Reg<STMRCAP_SPEC>`"]
pub type STMRCAP = crate::Reg<stmrcap::STMRCAP_SPEC>;
#[doc = "STIMER Capture Control"]
pub mod stmrcap;
#[doc = "IOM0IRQ register accessor: an alias for `Reg<IOM0IRQ_SPEC>`"]
pub type IOM0IRQ = crate::Reg<iom0irq::IOM0IRQ_SPEC>;
#[doc = "IOM0 Flow Control IRQ Select"]
pub mod iom0irq;
#[doc = "IOM1IRQ register accessor: an alias for `Reg<IOM1IRQ_SPEC>`"]
pub type IOM1IRQ = crate::Reg<iom1irq::IOM1IRQ_SPEC>;
#[doc = "IOM1 Flow Control IRQ Select"]
pub mod iom1irq;
#[doc = "IOM2IRQ register accessor: an alias for `Reg<IOM2IRQ_SPEC>`"]
pub type IOM2IRQ = crate::Reg<iom2irq::IOM2IRQ_SPEC>;
#[doc = "IOM2 Flow Control IRQ Select"]
pub mod iom2irq;
#[doc = "IOM3IRQ register accessor: an alias for `Reg<IOM3IRQ_SPEC>`"]
pub type IOM3IRQ = crate::Reg<iom3irq::IOM3IRQ_SPEC>;
#[doc = "IOM3 Flow Control IRQ Select"]
pub mod iom3irq;
#[doc = "IOM4IRQ register accessor: an alias for `Reg<IOM4IRQ_SPEC>`"]
pub type IOM4IRQ = crate::Reg<iom4irq::IOM4IRQ_SPEC>;
#[doc = "IOM4 Flow Control IRQ Select"]
pub mod iom4irq;
#[doc = "IOM5IRQ register accessor: an alias for `Reg<IOM5IRQ_SPEC>`"]
pub type IOM5IRQ = crate::Reg<iom5irq::IOM5IRQ_SPEC>;
#[doc = "IOM5 Flow Control IRQ Select"]
pub mod iom5irq;
#[doc = "BLEIFIRQ register accessor: an alias for `Reg<BLEIFIRQ_SPEC>`"]
pub type BLEIFIRQ = crate::Reg<bleifirq::BLEIFIRQ_SPEC>;
#[doc = "BLEIF Flow Control IRQ Select"]
pub mod bleifirq;
#[doc = "GPIOOBS register accessor: an alias for `Reg<GPIOOBS_SPEC>`"]
pub type GPIOOBS = crate::Reg<gpioobs::GPIOOBS_SPEC>;
#[doc = "GPIO Observation Mode Sample register"]
pub mod gpioobs;
#[doc = "ALTPADCFGA register accessor: an alias for `Reg<ALTPADCFGA_SPEC>`"]
pub type ALTPADCFGA = crate::Reg<altpadcfga::ALTPADCFGA_SPEC>;
#[doc = "Alternate Pad Configuration reg0 (Pads 3,2,1,0)"]
pub mod altpadcfga;
#[doc = "ALTPADCFGB register accessor: an alias for `Reg<ALTPADCFGB_SPEC>`"]
pub type ALTPADCFGB = crate::Reg<altpadcfgb::ALTPADCFGB_SPEC>;
#[doc = "Alternate Pad Configuration reg1 (Pads 7,6,5,4)"]
pub mod altpadcfgb;
#[doc = "ALTPADCFGC register accessor: an alias for `Reg<ALTPADCFGC_SPEC>`"]
pub type ALTPADCFGC = crate::Reg<altpadcfgc::ALTPADCFGC_SPEC>;
#[doc = "Alternate Pad Configuration reg2 (Pads 11,10,9,8)"]
pub mod altpadcfgc;
#[doc = "ALTPADCFGD register accessor: an alias for `Reg<ALTPADCFGD_SPEC>`"]
pub type ALTPADCFGD = crate::Reg<altpadcfgd::ALTPADCFGD_SPEC>;
#[doc = "Alternate Pad Configuration reg3 (Pads 15,14,13,12)"]
pub mod altpadcfgd;
#[doc = "ALTPADCFGE register accessor: an alias for `Reg<ALTPADCFGE_SPEC>`"]
pub type ALTPADCFGE = crate::Reg<altpadcfge::ALTPADCFGE_SPEC>;
#[doc = "Alternate Pad Configuration reg4 (Pads 19,18,17,16)"]
pub mod altpadcfge;
#[doc = "ALTPADCFGF register accessor: an alias for `Reg<ALTPADCFGF_SPEC>`"]
pub type ALTPADCFGF = crate::Reg<altpadcfgf::ALTPADCFGF_SPEC>;
#[doc = "Alternate Pad Configuration reg5 (Pads 23,22,21,20)"]
pub mod altpadcfgf;
#[doc = "ALTPADCFGG register accessor: an alias for `Reg<ALTPADCFGG_SPEC>`"]
pub type ALTPADCFGG = crate::Reg<altpadcfgg::ALTPADCFGG_SPEC>;
#[doc = "Alternate Pad Configuration reg6 (Pads 27,26,25,24)"]
pub mod altpadcfgg;
#[doc = "ALTPADCFGH register accessor: an alias for `Reg<ALTPADCFGH_SPEC>`"]
pub type ALTPADCFGH = crate::Reg<altpadcfgh::ALTPADCFGH_SPEC>;
#[doc = "Alternate Pad Configuration reg7 (Pads 31,30,29,28)"]
pub mod altpadcfgh;
#[doc = "ALTPADCFGI register accessor: an alias for `Reg<ALTPADCFGI_SPEC>`"]
pub type ALTPADCFGI = crate::Reg<altpadcfgi::ALTPADCFGI_SPEC>;
#[doc = "Alternate Pad Configuration reg8 (Pads 35,34,33,32)"]
pub mod altpadcfgi;
#[doc = "ALTPADCFGJ register accessor: an alias for `Reg<ALTPADCFGJ_SPEC>`"]
pub type ALTPADCFGJ = crate::Reg<altpadcfgj::ALTPADCFGJ_SPEC>;
#[doc = "Alternate Pad Configuration reg9 (Pads 39,38,37,36)"]
pub mod altpadcfgj;
#[doc = "ALTPADCFGK register accessor: an alias for `Reg<ALTPADCFGK_SPEC>`"]
pub type ALTPADCFGK = crate::Reg<altpadcfgk::ALTPADCFGK_SPEC>;
#[doc = "Alternate Pad Configuration reg10 (Pads 43,42,41,40)"]
pub mod altpadcfgk;
#[doc = "ALTPADCFGL register accessor: an alias for `Reg<ALTPADCFGL_SPEC>`"]
pub type ALTPADCFGL = crate::Reg<altpadcfgl::ALTPADCFGL_SPEC>;
#[doc = "Alternate Pad Configuration reg11 (Pads 47,46,45,44)"]
pub mod altpadcfgl;
#[doc = "ALTPADCFGM register accessor: an alias for `Reg<ALTPADCFGM_SPEC>`"]
pub type ALTPADCFGM = crate::Reg<altpadcfgm::ALTPADCFGM_SPEC>;
#[doc = "Alternate Pad Configuration reg12 (Pads 49,48)"]
pub mod altpadcfgm;
#[doc = "SCDET register accessor: an alias for `Reg<SCDET_SPEC>`"]
pub type SCDET = crate::Reg<scdet::SCDET_SPEC>;
#[doc = "SCARD Card Detect select"]
pub mod scdet;
#[doc = "CTENCFG register accessor: an alias for `Reg<CTENCFG_SPEC>`"]
pub type CTENCFG = crate::Reg<ctencfg::CTENCFG_SPEC>;
#[doc = "Counter/Timer Enable Config"]
pub mod ctencfg;
#[doc = "INT0EN register accessor: an alias for `Reg<INT0EN_SPEC>`"]
pub type INT0EN = crate::Reg<int0en::INT0EN_SPEC>;
#[doc = "GPIO Interrupt Registers 31-0: Enable"]
pub mod int0en;
#[doc = "INT0STAT register accessor: an alias for `Reg<INT0STAT_SPEC>`"]
pub type INT0STAT = crate::Reg<int0stat::INT0STAT_SPEC>;
#[doc = "GPIO Interrupt Registers 31-0: Status"]
pub mod int0stat;
#[doc = "INT0CLR register accessor: an alias for `Reg<INT0CLR_SPEC>`"]
pub type INT0CLR = crate::Reg<int0clr::INT0CLR_SPEC>;
#[doc = "GPIO Interrupt Registers 31-0: Clear"]
pub mod int0clr;
#[doc = "INT0SET register accessor: an alias for `Reg<INT0SET_SPEC>`"]
pub type INT0SET = crate::Reg<int0set::INT0SET_SPEC>;
#[doc = "GPIO Interrupt Registers 31-0: Set"]
pub mod int0set;
#[doc = "INT1EN register accessor: an alias for `Reg<INT1EN_SPEC>`"]
pub type INT1EN = crate::Reg<int1en::INT1EN_SPEC>;
#[doc = "GPIO Interrupt Registers 49-32: Enable"]
pub mod int1en;
#[doc = "INT1STAT register accessor: an alias for `Reg<INT1STAT_SPEC>`"]
pub type INT1STAT = crate::Reg<int1stat::INT1STAT_SPEC>;
#[doc = "GPIO Interrupt Registers 49-32: Status"]
pub mod int1stat;
#[doc = "INT1CLR register accessor: an alias for `Reg<INT1CLR_SPEC>`"]
pub type INT1CLR = crate::Reg<int1clr::INT1CLR_SPEC>;
#[doc = "GPIO Interrupt Registers 49-32: Clear"]
pub mod int1clr;
#[doc = "INT1SET register accessor: an alias for `Reg<INT1SET_SPEC>`"]
pub type INT1SET = crate::Reg<int1set::INT1SET_SPEC>;
#[doc = "GPIO Interrupt Registers 49-32: Set"]
pub mod int1set;
