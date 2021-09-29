#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - PDM Configuration Register"]
    pub pcfg: crate::Reg<pcfg::PCFG_SPEC>,
    #[doc = "0x04 - Voice Configuration Register"]
    pub vcfg: crate::Reg<vcfg::VCFG_SPEC>,
    #[doc = "0x08 - Voice Status Register"]
    pub voicestat: crate::Reg<voicestat::VOICESTAT_SPEC>,
    #[doc = "0x0c - FIFO Read"]
    pub fiforead: crate::Reg<fiforead::FIFOREAD_SPEC>,
    #[doc = "0x10 - FIFO Flush"]
    pub fifoflush: crate::Reg<fifoflush::FIFOFLUSH_SPEC>,
    #[doc = "0x14 - FIFO Threshold"]
    pub fifothr: crate::Reg<fifothr::FIFOTHR_SPEC>,
    _reserved6: [u8; 0x01e8],
    #[doc = "0x200 - IO Master Interrupts: Enable"]
    pub inten: crate::Reg<inten::INTEN_SPEC>,
    #[doc = "0x204 - IO Master Interrupts: Status"]
    pub intstat: crate::Reg<intstat::INTSTAT_SPEC>,
    #[doc = "0x208 - IO Master Interrupts: Clear"]
    pub intclr: crate::Reg<intclr::INTCLR_SPEC>,
    #[doc = "0x20c - IO Master Interrupts: Set"]
    pub intset: crate::Reg<intset::INTSET_SPEC>,
    _reserved10: [u8; 0x30],
    #[doc = "0x240 - DMA Trigger Enable Register"]
    pub dmatrigen: crate::Reg<dmatrigen::DMATRIGEN_SPEC>,
    #[doc = "0x244 - DMA Trigger Status Register"]
    pub dmatrigstat: crate::Reg<dmatrigstat::DMATRIGSTAT_SPEC>,
    _reserved12: [u8; 0x38],
    #[doc = "0x280 - DMA Configuration Register"]
    pub dmacfg: crate::Reg<dmacfg::DMACFG_SPEC>,
    _reserved13: [u8; 0x04],
    #[doc = "0x288 - DMA Total Transfer Count"]
    pub dmatotcount: crate::Reg<dmatotcount::DMATOTCOUNT_SPEC>,
    #[doc = "0x28c - DMA Target Address Register"]
    pub dmatargaddr: crate::Reg<dmatargaddr::DMATARGADDR_SPEC>,
    #[doc = "0x290 - DMA Status Register"]
    pub dmastat: crate::Reg<dmastat::DMASTAT_SPEC>,
}
#[doc = "PCFG register accessor: an alias for `Reg<PCFG_SPEC>`"]
pub type PCFG = crate::Reg<pcfg::PCFG_SPEC>;
#[doc = "PDM Configuration Register"]
pub mod pcfg;
#[doc = "VCFG register accessor: an alias for `Reg<VCFG_SPEC>`"]
pub type VCFG = crate::Reg<vcfg::VCFG_SPEC>;
#[doc = "Voice Configuration Register"]
pub mod vcfg;
#[doc = "VOICESTAT register accessor: an alias for `Reg<VOICESTAT_SPEC>`"]
pub type VOICESTAT = crate::Reg<voicestat::VOICESTAT_SPEC>;
#[doc = "Voice Status Register"]
pub mod voicestat;
#[doc = "FIFOREAD register accessor: an alias for `Reg<FIFOREAD_SPEC>`"]
pub type FIFOREAD = crate::Reg<fiforead::FIFOREAD_SPEC>;
#[doc = "FIFO Read"]
pub mod fiforead;
#[doc = "FIFOFLUSH register accessor: an alias for `Reg<FIFOFLUSH_SPEC>`"]
pub type FIFOFLUSH = crate::Reg<fifoflush::FIFOFLUSH_SPEC>;
#[doc = "FIFO Flush"]
pub mod fifoflush;
#[doc = "FIFOTHR register accessor: an alias for `Reg<FIFOTHR_SPEC>`"]
pub type FIFOTHR = crate::Reg<fifothr::FIFOTHR_SPEC>;
#[doc = "FIFO Threshold"]
pub mod fifothr;
#[doc = "INTEN register accessor: an alias for `Reg<INTEN_SPEC>`"]
pub type INTEN = crate::Reg<inten::INTEN_SPEC>;
#[doc = "IO Master Interrupts: Enable"]
pub mod inten;
#[doc = "INTSTAT register accessor: an alias for `Reg<INTSTAT_SPEC>`"]
pub type INTSTAT = crate::Reg<intstat::INTSTAT_SPEC>;
#[doc = "IO Master Interrupts: Status"]
pub mod intstat;
#[doc = "INTCLR register accessor: an alias for `Reg<INTCLR_SPEC>`"]
pub type INTCLR = crate::Reg<intclr::INTCLR_SPEC>;
#[doc = "IO Master Interrupts: Clear"]
pub mod intclr;
#[doc = "INTSET register accessor: an alias for `Reg<INTSET_SPEC>`"]
pub type INTSET = crate::Reg<intset::INTSET_SPEC>;
#[doc = "IO Master Interrupts: Set"]
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
