#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - FIFO Access Port"]
    pub fifo: crate::Reg<fifo::FIFO_SPEC>,
    _reserved1: [u8; 0xfc],
    #[doc = "0x100 - FIFO size and remaining slots open values"]
    pub fifoptr: crate::Reg<fifoptr::FIFOPTR_SPEC>,
    #[doc = "0x104 - FIFO Threshold Configuration"]
    pub fifothr: crate::Reg<fifothr::FIFOTHR_SPEC>,
    #[doc = "0x108 - FIFO POP register"]
    pub fifopop: crate::Reg<fifopop::FIFOPOP_SPEC>,
    #[doc = "0x10c - FIFO PUSH register"]
    pub fifopush: crate::Reg<fifopush::FIFOPUSH_SPEC>,
    #[doc = "0x110 - FIFO Control Register"]
    pub fifoctrl: crate::Reg<fifoctrl::FIFOCTRL_SPEC>,
    #[doc = "0x114 - FIFO Pointers"]
    pub fifoloc: crate::Reg<fifoloc::FIFOLOC_SPEC>,
    _reserved7: [u8; 0xe8],
    #[doc = "0x200 - I/O Clock Configuration"]
    pub clkcfg: crate::Reg<clkcfg::CLKCFG_SPEC>,
    _reserved8: [u8; 0x08],
    #[doc = "0x20c - Command and offset Register"]
    pub cmd: crate::Reg<cmd::CMD_SPEC>,
    #[doc = "0x210 - Command Repeat Register"]
    pub cmdrpt: crate::Reg<cmdrpt::CMDRPT_SPEC>,
    #[doc = "0x214 - High order offset bytes"]
    pub offsethi: crate::Reg<offsethi::OFFSETHI_SPEC>,
    #[doc = "0x218 - Command status"]
    pub cmdstat: crate::Reg<cmdstat::CMDSTAT_SPEC>,
    _reserved12: [u8; 0x04],
    #[doc = "0x220 - IO Master Interrupts: Enable"]
    pub inten: crate::Reg<inten::INTEN_SPEC>,
    #[doc = "0x224 - IO Master Interrupts: Status"]
    pub intstat: crate::Reg<intstat::INTSTAT_SPEC>,
    #[doc = "0x228 - IO Master Interrupts: Clear"]
    pub intclr: crate::Reg<intclr::INTCLR_SPEC>,
    #[doc = "0x22c - IO Master Interrupts: Set"]
    pub intset: crate::Reg<intset::INTSET_SPEC>,
    #[doc = "0x230 - DMA Trigger Enable Register"]
    pub dmatrigen: crate::Reg<dmatrigen::DMATRIGEN_SPEC>,
    #[doc = "0x234 - DMA Trigger Status Register"]
    pub dmatrigstat: crate::Reg<dmatrigstat::DMATRIGSTAT_SPEC>,
    #[doc = "0x238 - DMA Configuration Register"]
    pub dmacfg: crate::Reg<dmacfg::DMACFG_SPEC>,
    #[doc = "0x23c - DMA Total Transfer Count"]
    pub dmatotcount: crate::Reg<dmatotcount::DMATOTCOUNT_SPEC>,
    #[doc = "0x240 - DMA Target Address Register"]
    pub dmatargaddr: crate::Reg<dmatargaddr::DMATARGADDR_SPEC>,
    #[doc = "0x244 - DMA Status Register"]
    pub dmastat: crate::Reg<dmastat::DMASTAT_SPEC>,
    #[doc = "0x248 - Command Queue Configuration Register"]
    pub cqcfg: crate::Reg<cqcfg::CQCFG_SPEC>,
    #[doc = "0x24c - CQ Target Read Address Register"]
    pub cqaddr: crate::Reg<cqaddr::CQADDR_SPEC>,
    #[doc = "0x250 - Command Queue Status Register"]
    pub cqstat: crate::Reg<cqstat::CQSTAT_SPEC>,
    #[doc = "0x254 - Command Queue Flag Register"]
    pub cqflags: crate::Reg<cqflags::CQFLAGS_SPEC>,
    #[doc = "0x258 - Command Queue Flag Set/Clear Register"]
    pub cqsetclear: crate::Reg<cqsetclear::CQSETCLEAR_SPEC>,
    #[doc = "0x25c - Command Queue Pause Enable Register"]
    pub cqpauseen: crate::Reg<cqpauseen::CQPAUSEEN_SPEC>,
    #[doc = "0x260 - IOM Command Queue current index value . Compared to the CQENDIDX reg contents to generate the IDXEQ Pause event for command queue"]
    pub cqcuridx: crate::Reg<cqcuridx::CQCURIDX_SPEC>,
    #[doc = "0x264 - IOM Command Queue current index value . Compared to the CQCURIDX reg contents to generate the IDXEQ Pause event for command queue"]
    pub cqendidx: crate::Reg<cqendidx::CQENDIDX_SPEC>,
    #[doc = "0x268 - IOM Module Status Register"]
    pub status: crate::Reg<status::STATUS_SPEC>,
    _reserved31: [u8; 0x94],
    #[doc = "0x300 - SPI module master configuration"]
    pub mspicfg: crate::Reg<mspicfg::MSPICFG_SPEC>,
    #[doc = "0x304 - BLE Core Control"]
    pub blecfg: crate::Reg<blecfg::BLECFG_SPEC>,
    #[doc = "0x308 - BLE Power command interface"]
    pub pwrcmd: crate::Reg<pwrcmd::PWRCMD_SPEC>,
    #[doc = "0x30c - BLE Core status"]
    pub bstatus: crate::Reg<bstatus::BSTATUS_SPEC>,
    _reserved35: [u8; 0x0100],
    #[doc = "0x410 - BLEIF Master Debug Register"]
    pub bledbg: crate::Reg<bledbg::BLEDBG_SPEC>,
}
#[doc = "FIFO register accessor: an alias for `Reg<FIFO_SPEC>`"]
pub type FIFO = crate::Reg<fifo::FIFO_SPEC>;
#[doc = "FIFO Access Port"]
pub mod fifo;
#[doc = "FIFOPTR register accessor: an alias for `Reg<FIFOPTR_SPEC>`"]
pub type FIFOPTR = crate::Reg<fifoptr::FIFOPTR_SPEC>;
#[doc = "FIFO size and remaining slots open values"]
pub mod fifoptr;
#[doc = "FIFOTHR register accessor: an alias for `Reg<FIFOTHR_SPEC>`"]
pub type FIFOTHR = crate::Reg<fifothr::FIFOTHR_SPEC>;
#[doc = "FIFO Threshold Configuration"]
pub mod fifothr;
#[doc = "FIFOPOP register accessor: an alias for `Reg<FIFOPOP_SPEC>`"]
pub type FIFOPOP = crate::Reg<fifopop::FIFOPOP_SPEC>;
#[doc = "FIFO POP register"]
pub mod fifopop;
#[doc = "FIFOPUSH register accessor: an alias for `Reg<FIFOPUSH_SPEC>`"]
pub type FIFOPUSH = crate::Reg<fifopush::FIFOPUSH_SPEC>;
#[doc = "FIFO PUSH register"]
pub mod fifopush;
#[doc = "FIFOCTRL register accessor: an alias for `Reg<FIFOCTRL_SPEC>`"]
pub type FIFOCTRL = crate::Reg<fifoctrl::FIFOCTRL_SPEC>;
#[doc = "FIFO Control Register"]
pub mod fifoctrl;
#[doc = "FIFOLOC register accessor: an alias for `Reg<FIFOLOC_SPEC>`"]
pub type FIFOLOC = crate::Reg<fifoloc::FIFOLOC_SPEC>;
#[doc = "FIFO Pointers"]
pub mod fifoloc;
#[doc = "CLKCFG register accessor: an alias for `Reg<CLKCFG_SPEC>`"]
pub type CLKCFG = crate::Reg<clkcfg::CLKCFG_SPEC>;
#[doc = "I/O Clock Configuration"]
pub mod clkcfg;
#[doc = "CMD register accessor: an alias for `Reg<CMD_SPEC>`"]
pub type CMD = crate::Reg<cmd::CMD_SPEC>;
#[doc = "Command and offset Register"]
pub mod cmd;
#[doc = "CMDRPT register accessor: an alias for `Reg<CMDRPT_SPEC>`"]
pub type CMDRPT = crate::Reg<cmdrpt::CMDRPT_SPEC>;
#[doc = "Command Repeat Register"]
pub mod cmdrpt;
#[doc = "OFFSETHI register accessor: an alias for `Reg<OFFSETHI_SPEC>`"]
pub type OFFSETHI = crate::Reg<offsethi::OFFSETHI_SPEC>;
#[doc = "High order offset bytes"]
pub mod offsethi;
#[doc = "CMDSTAT register accessor: an alias for `Reg<CMDSTAT_SPEC>`"]
pub type CMDSTAT = crate::Reg<cmdstat::CMDSTAT_SPEC>;
#[doc = "Command status"]
pub mod cmdstat;
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
#[doc = "CQCFG register accessor: an alias for `Reg<CQCFG_SPEC>`"]
pub type CQCFG = crate::Reg<cqcfg::CQCFG_SPEC>;
#[doc = "Command Queue Configuration Register"]
pub mod cqcfg;
#[doc = "CQADDR register accessor: an alias for `Reg<CQADDR_SPEC>`"]
pub type CQADDR = crate::Reg<cqaddr::CQADDR_SPEC>;
#[doc = "CQ Target Read Address Register"]
pub mod cqaddr;
#[doc = "CQSTAT register accessor: an alias for `Reg<CQSTAT_SPEC>`"]
pub type CQSTAT = crate::Reg<cqstat::CQSTAT_SPEC>;
#[doc = "Command Queue Status Register"]
pub mod cqstat;
#[doc = "CQFLAGS register accessor: an alias for `Reg<CQFLAGS_SPEC>`"]
pub type CQFLAGS = crate::Reg<cqflags::CQFLAGS_SPEC>;
#[doc = "Command Queue Flag Register"]
pub mod cqflags;
#[doc = "CQSETCLEAR register accessor: an alias for `Reg<CQSETCLEAR_SPEC>`"]
pub type CQSETCLEAR = crate::Reg<cqsetclear::CQSETCLEAR_SPEC>;
#[doc = "Command Queue Flag Set/Clear Register"]
pub mod cqsetclear;
#[doc = "CQPAUSEEN register accessor: an alias for `Reg<CQPAUSEEN_SPEC>`"]
pub type CQPAUSEEN = crate::Reg<cqpauseen::CQPAUSEEN_SPEC>;
#[doc = "Command Queue Pause Enable Register"]
pub mod cqpauseen;
#[doc = "CQCURIDX register accessor: an alias for `Reg<CQCURIDX_SPEC>`"]
pub type CQCURIDX = crate::Reg<cqcuridx::CQCURIDX_SPEC>;
#[doc = "IOM Command Queue current index value . Compared to the CQENDIDX reg contents to generate the IDXEQ Pause event for command queue"]
pub mod cqcuridx;
#[doc = "CQENDIDX register accessor: an alias for `Reg<CQENDIDX_SPEC>`"]
pub type CQENDIDX = crate::Reg<cqendidx::CQENDIDX_SPEC>;
#[doc = "IOM Command Queue current index value . Compared to the CQCURIDX reg contents to generate the IDXEQ Pause event for command queue"]
pub mod cqendidx;
#[doc = "STATUS register accessor: an alias for `Reg<STATUS_SPEC>`"]
pub type STATUS = crate::Reg<status::STATUS_SPEC>;
#[doc = "IOM Module Status Register"]
pub mod status;
#[doc = "MSPICFG register accessor: an alias for `Reg<MSPICFG_SPEC>`"]
pub type MSPICFG = crate::Reg<mspicfg::MSPICFG_SPEC>;
#[doc = "SPI module master configuration"]
pub mod mspicfg;
#[doc = "BLECFG register accessor: an alias for `Reg<BLECFG_SPEC>`"]
pub type BLECFG = crate::Reg<blecfg::BLECFG_SPEC>;
#[doc = "BLE Core Control"]
pub mod blecfg;
#[doc = "PWRCMD register accessor: an alias for `Reg<PWRCMD_SPEC>`"]
pub type PWRCMD = crate::Reg<pwrcmd::PWRCMD_SPEC>;
#[doc = "BLE Power command interface"]
pub mod pwrcmd;
#[doc = "BSTATUS register accessor: an alias for `Reg<BSTATUS_SPEC>`"]
pub type BSTATUS = crate::Reg<bstatus::BSTATUS_SPEC>;
#[doc = "BLE Core status"]
pub mod bstatus;
#[doc = "BLEDBG register accessor: an alias for `Reg<BLEDBG_SPEC>`"]
pub type BLEDBG = crate::Reg<bledbg::BLEDBG_SPEC>;
#[doc = "BLEIF Master Debug Register"]
pub mod bledbg;
