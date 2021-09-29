#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Configuration Register"]
    pub cfg: crate::Reg<cfg::CFG_SPEC>,
    #[doc = "0x04 - ADC Power Status"]
    pub stat: crate::Reg<stat::STAT_SPEC>,
    #[doc = "0x08 - Software trigger"]
    pub swt: crate::Reg<swt::SWT_SPEC>,
    #[doc = "0x0c - Slot 0 Configuration Register"]
    pub sl0cfg: crate::Reg<sl0cfg::SL0CFG_SPEC>,
    #[doc = "0x10 - Slot 1 Configuration Register"]
    pub sl1cfg: crate::Reg<sl1cfg::SL1CFG_SPEC>,
    #[doc = "0x14 - Slot 2 Configuration Register"]
    pub sl2cfg: crate::Reg<sl2cfg::SL2CFG_SPEC>,
    #[doc = "0x18 - Slot 3 Configuration Register"]
    pub sl3cfg: crate::Reg<sl3cfg::SL3CFG_SPEC>,
    #[doc = "0x1c - Slot 4 Configuration Register"]
    pub sl4cfg: crate::Reg<sl4cfg::SL4CFG_SPEC>,
    #[doc = "0x20 - Slot 5 Configuration Register"]
    pub sl5cfg: crate::Reg<sl5cfg::SL5CFG_SPEC>,
    #[doc = "0x24 - Slot 6 Configuration Register"]
    pub sl6cfg: crate::Reg<sl6cfg::SL6CFG_SPEC>,
    #[doc = "0x28 - Slot 7 Configuration Register"]
    pub sl7cfg: crate::Reg<sl7cfg::SL7CFG_SPEC>,
    #[doc = "0x2c - Window Comparator Upper Limits Register"]
    pub wulim: crate::Reg<wulim::WULIM_SPEC>,
    #[doc = "0x30 - Window Comparator Lower Limits Register"]
    pub wllim: crate::Reg<wllim::WLLIM_SPEC>,
    #[doc = "0x34 - Scale Window Comparator Limits"]
    pub scwlim: crate::Reg<scwlim::SCWLIM_SPEC>,
    #[doc = "0x38 - FIFO Data and Valid Count Register"]
    pub fifo: crate::Reg<fifo::FIFO_SPEC>,
    #[doc = "0x3c - FIFO Data and Valid Count Register"]
    pub fifopr: crate::Reg<fifopr::FIFOPR_SPEC>,
    _reserved16: [u8; 0x01c0],
    #[doc = "0x200 - ADC Interrupt registers: Enable"]
    pub inten: crate::Reg<inten::INTEN_SPEC>,
    #[doc = "0x204 - ADC Interrupt registers: Status"]
    pub intstat: crate::Reg<intstat::INTSTAT_SPEC>,
    #[doc = "0x208 - ADC Interrupt registers: Clear"]
    pub intclr: crate::Reg<intclr::INTCLR_SPEC>,
    #[doc = "0x20c - ADC Interrupt registers: Set"]
    pub intset: crate::Reg<intset::INTSET_SPEC>,
    _reserved20: [u8; 0x30],
    #[doc = "0x240 - DMA Trigger Enable Register"]
    pub dmatrigen: crate::Reg<dmatrigen::DMATRIGEN_SPEC>,
    #[doc = "0x244 - DMA Trigger Status Register"]
    pub dmatrigstat: crate::Reg<dmatrigstat::DMATRIGSTAT_SPEC>,
    _reserved22: [u8; 0x38],
    #[doc = "0x280 - DMA Configuration Register"]
    pub dmacfg: crate::Reg<dmacfg::DMACFG_SPEC>,
    _reserved23: [u8; 0x04],
    #[doc = "0x288 - DMA Total Transfer Count"]
    pub dmatotcount: crate::Reg<dmatotcount::DMATOTCOUNT_SPEC>,
    #[doc = "0x28c - DMA Target Address Register"]
    pub dmatargaddr: crate::Reg<dmatargaddr::DMATARGADDR_SPEC>,
    #[doc = "0x290 - DMA Status Register"]
    pub dmastat: crate::Reg<dmastat::DMASTAT_SPEC>,
}
#[doc = "CFG register accessor: an alias for `Reg<CFG_SPEC>`"]
pub type CFG = crate::Reg<cfg::CFG_SPEC>;
#[doc = "Configuration Register"]
pub mod cfg;
#[doc = "STAT register accessor: an alias for `Reg<STAT_SPEC>`"]
pub type STAT = crate::Reg<stat::STAT_SPEC>;
#[doc = "ADC Power Status"]
pub mod stat;
#[doc = "SWT register accessor: an alias for `Reg<SWT_SPEC>`"]
pub type SWT = crate::Reg<swt::SWT_SPEC>;
#[doc = "Software trigger"]
pub mod swt;
#[doc = "SL0CFG register accessor: an alias for `Reg<SL0CFG_SPEC>`"]
pub type SL0CFG = crate::Reg<sl0cfg::SL0CFG_SPEC>;
#[doc = "Slot 0 Configuration Register"]
pub mod sl0cfg;
#[doc = "SL1CFG register accessor: an alias for `Reg<SL1CFG_SPEC>`"]
pub type SL1CFG = crate::Reg<sl1cfg::SL1CFG_SPEC>;
#[doc = "Slot 1 Configuration Register"]
pub mod sl1cfg;
#[doc = "SL2CFG register accessor: an alias for `Reg<SL2CFG_SPEC>`"]
pub type SL2CFG = crate::Reg<sl2cfg::SL2CFG_SPEC>;
#[doc = "Slot 2 Configuration Register"]
pub mod sl2cfg;
#[doc = "SL3CFG register accessor: an alias for `Reg<SL3CFG_SPEC>`"]
pub type SL3CFG = crate::Reg<sl3cfg::SL3CFG_SPEC>;
#[doc = "Slot 3 Configuration Register"]
pub mod sl3cfg;
#[doc = "SL4CFG register accessor: an alias for `Reg<SL4CFG_SPEC>`"]
pub type SL4CFG = crate::Reg<sl4cfg::SL4CFG_SPEC>;
#[doc = "Slot 4 Configuration Register"]
pub mod sl4cfg;
#[doc = "SL5CFG register accessor: an alias for `Reg<SL5CFG_SPEC>`"]
pub type SL5CFG = crate::Reg<sl5cfg::SL5CFG_SPEC>;
#[doc = "Slot 5 Configuration Register"]
pub mod sl5cfg;
#[doc = "SL6CFG register accessor: an alias for `Reg<SL6CFG_SPEC>`"]
pub type SL6CFG = crate::Reg<sl6cfg::SL6CFG_SPEC>;
#[doc = "Slot 6 Configuration Register"]
pub mod sl6cfg;
#[doc = "SL7CFG register accessor: an alias for `Reg<SL7CFG_SPEC>`"]
pub type SL7CFG = crate::Reg<sl7cfg::SL7CFG_SPEC>;
#[doc = "Slot 7 Configuration Register"]
pub mod sl7cfg;
#[doc = "WULIM register accessor: an alias for `Reg<WULIM_SPEC>`"]
pub type WULIM = crate::Reg<wulim::WULIM_SPEC>;
#[doc = "Window Comparator Upper Limits Register"]
pub mod wulim;
#[doc = "WLLIM register accessor: an alias for `Reg<WLLIM_SPEC>`"]
pub type WLLIM = crate::Reg<wllim::WLLIM_SPEC>;
#[doc = "Window Comparator Lower Limits Register"]
pub mod wllim;
#[doc = "SCWLIM register accessor: an alias for `Reg<SCWLIM_SPEC>`"]
pub type SCWLIM = crate::Reg<scwlim::SCWLIM_SPEC>;
#[doc = "Scale Window Comparator Limits"]
pub mod scwlim;
#[doc = "FIFO register accessor: an alias for `Reg<FIFO_SPEC>`"]
pub type FIFO = crate::Reg<fifo::FIFO_SPEC>;
#[doc = "FIFO Data and Valid Count Register"]
pub mod fifo;
#[doc = "FIFOPR register accessor: an alias for `Reg<FIFOPR_SPEC>`"]
pub type FIFOPR = crate::Reg<fifopr::FIFOPR_SPEC>;
#[doc = "FIFO Data and Valid Count Register"]
pub mod fifopr;
#[doc = "INTEN register accessor: an alias for `Reg<INTEN_SPEC>`"]
pub type INTEN = crate::Reg<inten::INTEN_SPEC>;
#[doc = "ADC Interrupt registers: Enable"]
pub mod inten;
#[doc = "INTSTAT register accessor: an alias for `Reg<INTSTAT_SPEC>`"]
pub type INTSTAT = crate::Reg<intstat::INTSTAT_SPEC>;
#[doc = "ADC Interrupt registers: Status"]
pub mod intstat;
#[doc = "INTCLR register accessor: an alias for `Reg<INTCLR_SPEC>`"]
pub type INTCLR = crate::Reg<intclr::INTCLR_SPEC>;
#[doc = "ADC Interrupt registers: Clear"]
pub mod intclr;
#[doc = "INTSET register accessor: an alias for `Reg<INTSET_SPEC>`"]
pub type INTSET = crate::Reg<intset::INTSET_SPEC>;
#[doc = "ADC Interrupt registers: Set"]
pub mod intset;
#[doc = "DMATRIGEN register accessor: an alias for `Reg<DMATRIGEN_SPEC>`"]
pub type DMATRIGEN = crate::Reg<dmatrigen::DMATRIGEN_SPEC>;
#[doc = "DMA Trigger Enable Register"]
pub mod dmatrigen;
#[doc = "DMATRIGSTAT register accessor: an alias for `Reg<DMATRIGSTAT_SPEC>`"]
pub type DMATRIGSTAT = crate::Reg<dmatrigstat::DMATRIGSTAT_SPEC>;
#[doc = "DMA Trigger Status Register"]
pub mod dmatrigstat;
#[doc = "DMACFG register accessor: an alias for `Reg<DMACFG_SPEC>`"]
pub type DMACFG = crate::Reg<dmacfg::DMACFG_SPEC>;
#[doc = "DMA Configuration Register"]
pub mod dmacfg;
#[doc = "DMATOTCOUNT register accessor: an alias for `Reg<DMATOTCOUNT_SPEC>`"]
pub type DMATOTCOUNT = crate::Reg<dmatotcount::DMATOTCOUNT_SPEC>;
#[doc = "DMA Total Transfer Count"]
pub mod dmatotcount;
#[doc = "DMATARGADDR register accessor: an alias for `Reg<DMATARGADDR_SPEC>`"]
pub type DMATARGADDR = crate::Reg<dmatargaddr::DMATARGADDR_SPEC>;
#[doc = "DMA Target Address Register"]
pub mod dmatargaddr;
#[doc = "DMASTAT register accessor: an alias for `Reg<DMASTAT_SPEC>`"]
pub type DMASTAT = crate::Reg<dmastat::DMASTAT_SPEC>;
#[doc = "DMA Status Register"]
pub mod dmastat;
