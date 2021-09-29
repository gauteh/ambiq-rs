#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - MSPI PIO Transfer Control/Status Register"]
    pub ctrl: crate::Reg<ctrl::CTRL_SPEC>,
    #[doc = "0x04 - MSPI Transfer Configuration Register"]
    pub cfg: crate::Reg<cfg::CFG_SPEC>,
    #[doc = "0x08 - MSPI Transfer Address Register"]
    pub addr: crate::Reg<addr::ADDR_SPEC>,
    #[doc = "0x0c - MSPI Transfer Instruction"]
    pub instr: crate::Reg<instr::INSTR_SPEC>,
    #[doc = "0x10 - TX Data FIFO"]
    pub txfifo: crate::Reg<txfifo::TXFIFO_SPEC>,
    #[doc = "0x14 - RX Data FIFO"]
    pub rxfifo: crate::Reg<rxfifo::RXFIFO_SPEC>,
    #[doc = "0x18 - TX FIFO Entries"]
    pub txentries: crate::Reg<txentries::TXENTRIES_SPEC>,
    #[doc = "0x1c - RX FIFO Entries"]
    pub rxentries: crate::Reg<rxentries::RXENTRIES_SPEC>,
    #[doc = "0x20 - TX/RX FIFO Threshhold Levels"]
    pub threshold: crate::Reg<threshold::THRESHOLD_SPEC>,
    _reserved9: [u8; 0xdc],
    #[doc = "0x100 - MSPI Module Configuration"]
    pub mspicfg: crate::Reg<mspicfg::MSPICFG_SPEC>,
    #[doc = "0x104 - MSPI Output Pad Configuration"]
    pub padcfg: crate::Reg<padcfg::PADCFG_SPEC>,
    #[doc = "0x108 - MSPI Output Enable Pad Configuration"]
    pub padouten: crate::Reg<padouten::PADOUTEN_SPEC>,
    #[doc = "0x10c - Configuration for XIP/DMA support of SPI flash modules."]
    pub flash: crate::Reg<flash::FLASH_SPEC>,
    _reserved13: [u8; 0x10],
    #[doc = "0x120 - External Flash Scrambling Controls"]
    pub scrambling: crate::Reg<scrambling::SCRAMBLING_SPEC>,
    _reserved14: [u8; 0xdc],
    #[doc = "0x200 - MSPI Master Interrupts: Enable"]
    pub inten: crate::Reg<inten::INTEN_SPEC>,
    #[doc = "0x204 - MSPI Master Interrupts: Status"]
    pub intstat: crate::Reg<intstat::INTSTAT_SPEC>,
    #[doc = "0x208 - MSPI Master Interrupts: Clear"]
    pub intclr: crate::Reg<intclr::INTCLR_SPEC>,
    #[doc = "0x20c - MSPI Master Interrupts: Set"]
    pub intset: crate::Reg<intset::INTSET_SPEC>,
    _reserved18: [u8; 0x40],
    #[doc = "0x250 - DMA Configuration Register"]
    pub dmacfg: crate::Reg<dmacfg::DMACFG_SPEC>,
    #[doc = "0x254 - DMA Status Register"]
    pub dmastat: crate::Reg<dmastat::DMASTAT_SPEC>,
    #[doc = "0x258 - DMA Target Address Register"]
    pub dmatargaddr: crate::Reg<dmatargaddr::DMATARGADDR_SPEC>,
    #[doc = "0x25c - DMA Device Address Register"]
    pub dmadevaddr: crate::Reg<dmadevaddr::DMADEVADDR_SPEC>,
    #[doc = "0x260 - DMA Total Transfer Count"]
    pub dmatotcount: crate::Reg<dmatotcount::DMATOTCOUNT_SPEC>,
    #[doc = "0x264 - DMA BYTE Transfer Count"]
    pub dmabcount: crate::Reg<dmabcount::DMABCOUNT_SPEC>,
    _reserved24: [u8; 0x10],
    #[doc = "0x278 - DMA Transmit Trigger Threshhold"]
    pub dmathresh: crate::Reg<dmathresh::DMATHRESH_SPEC>,
    _reserved25: [u8; 0x24],
    #[doc = "0x2a0 - Command Queue Configuration Register"]
    pub cqcfg: crate::Reg<cqcfg::CQCFG_SPEC>,
    _reserved26: [u8; 0x04],
    #[doc = "0x2a8 - CQ Target Read Address Register"]
    pub cqaddr: crate::Reg<cqaddr::CQADDR_SPEC>,
    #[doc = "0x2ac - Command Queue Status Register"]
    pub cqstat: crate::Reg<cqstat::CQSTAT_SPEC>,
    #[doc = "0x2b0 - Command Queue Flag Register"]
    pub cqflags: crate::Reg<cqflags::CQFLAGS_SPEC>,
    #[doc = "0x2b4 - Command Queue Flag Set/Clear Register"]
    pub cqsetclear: crate::Reg<cqsetclear::CQSETCLEAR_SPEC>,
    #[doc = "0x2b8 - Command Queue Pause Mask Register"]
    pub cqpause: crate::Reg<cqpause::CQPAUSE_SPEC>,
    _reserved31: [u8; 0x04],
    #[doc = "0x2c0 - Command Queue Current Index"]
    pub cqcuridx: crate::Reg<cqcuridx::CQCURIDX_SPEC>,
    #[doc = "0x2c4 - Command Queue End Index"]
    pub cqendidx: crate::Reg<cqendidx::CQENDIDX_SPEC>,
}
#[doc = "CTRL register accessor: an alias for `Reg<CTRL_SPEC>`"]
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
#[doc = "MSPI PIO Transfer Control/Status Register"]
pub mod ctrl;
#[doc = "CFG register accessor: an alias for `Reg<CFG_SPEC>`"]
pub type CFG = crate::Reg<cfg::CFG_SPEC>;
#[doc = "MSPI Transfer Configuration Register"]
pub mod cfg;
#[doc = "ADDR register accessor: an alias for `Reg<ADDR_SPEC>`"]
pub type ADDR = crate::Reg<addr::ADDR_SPEC>;
#[doc = "MSPI Transfer Address Register"]
pub mod addr;
#[doc = "INSTR register accessor: an alias for `Reg<INSTR_SPEC>`"]
pub type INSTR = crate::Reg<instr::INSTR_SPEC>;
#[doc = "MSPI Transfer Instruction"]
pub mod instr;
#[doc = "TXFIFO register accessor: an alias for `Reg<TXFIFO_SPEC>`"]
pub type TXFIFO = crate::Reg<txfifo::TXFIFO_SPEC>;
#[doc = "TX Data FIFO"]
pub mod txfifo;
#[doc = "RXFIFO register accessor: an alias for `Reg<RXFIFO_SPEC>`"]
pub type RXFIFO = crate::Reg<rxfifo::RXFIFO_SPEC>;
#[doc = "RX Data FIFO"]
pub mod rxfifo;
#[doc = "TXENTRIES register accessor: an alias for `Reg<TXENTRIES_SPEC>`"]
pub type TXENTRIES = crate::Reg<txentries::TXENTRIES_SPEC>;
#[doc = "TX FIFO Entries"]
pub mod txentries;
#[doc = "RXENTRIES register accessor: an alias for `Reg<RXENTRIES_SPEC>`"]
pub type RXENTRIES = crate::Reg<rxentries::RXENTRIES_SPEC>;
#[doc = "RX FIFO Entries"]
pub mod rxentries;
#[doc = "THRESHOLD register accessor: an alias for `Reg<THRESHOLD_SPEC>`"]
pub type THRESHOLD = crate::Reg<threshold::THRESHOLD_SPEC>;
#[doc = "TX/RX FIFO Threshhold Levels"]
pub mod threshold;
#[doc = "MSPICFG register accessor: an alias for `Reg<MSPICFG_SPEC>`"]
pub type MSPICFG = crate::Reg<mspicfg::MSPICFG_SPEC>;
#[doc = "MSPI Module Configuration"]
pub mod mspicfg;
#[doc = "PADCFG register accessor: an alias for `Reg<PADCFG_SPEC>`"]
pub type PADCFG = crate::Reg<padcfg::PADCFG_SPEC>;
#[doc = "MSPI Output Pad Configuration"]
pub mod padcfg;
#[doc = "PADOUTEN register accessor: an alias for `Reg<PADOUTEN_SPEC>`"]
pub type PADOUTEN = crate::Reg<padouten::PADOUTEN_SPEC>;
#[doc = "MSPI Output Enable Pad Configuration"]
pub mod padouten;
#[doc = "FLASH register accessor: an alias for `Reg<FLASH_SPEC>`"]
pub type FLASH = crate::Reg<flash::FLASH_SPEC>;
#[doc = "Configuration for XIP/DMA support of SPI flash modules."]
pub mod flash;
#[doc = "SCRAMBLING register accessor: an alias for `Reg<SCRAMBLING_SPEC>`"]
pub type SCRAMBLING = crate::Reg<scrambling::SCRAMBLING_SPEC>;
#[doc = "External Flash Scrambling Controls"]
pub mod scrambling;
#[doc = "INTEN register accessor: an alias for `Reg<INTEN_SPEC>`"]
pub type INTEN = crate::Reg<inten::INTEN_SPEC>;
#[doc = "MSPI Master Interrupts: Enable"]
pub mod inten;
#[doc = "INTSTAT register accessor: an alias for `Reg<INTSTAT_SPEC>`"]
pub type INTSTAT = crate::Reg<intstat::INTSTAT_SPEC>;
#[doc = "MSPI Master Interrupts: Status"]
pub mod intstat;
#[doc = "INTCLR register accessor: an alias for `Reg<INTCLR_SPEC>`"]
pub type INTCLR = crate::Reg<intclr::INTCLR_SPEC>;
#[doc = "MSPI Master Interrupts: Clear"]
pub mod intclr;
#[doc = "INTSET register accessor: an alias for `Reg<INTSET_SPEC>`"]
pub type INTSET = crate::Reg<intset::INTSET_SPEC>;
#[doc = "MSPI Master Interrupts: Set"]
pub mod intset;
#[doc = "DMACFG register accessor: an alias for `Reg<DMACFG_SPEC>`"]
pub type DMACFG = crate::Reg<dmacfg::DMACFG_SPEC>;
#[doc = "DMA Configuration Register"]
pub mod dmacfg;
#[doc = "DMASTAT register accessor: an alias for `Reg<DMASTAT_SPEC>`"]
pub type DMASTAT = crate::Reg<dmastat::DMASTAT_SPEC>;
#[doc = "DMA Status Register"]
pub mod dmastat;
#[doc = "DMATARGADDR register accessor: an alias for `Reg<DMATARGADDR_SPEC>`"]
pub type DMATARGADDR = crate::Reg<dmatargaddr::DMATARGADDR_SPEC>;
#[doc = "DMA Target Address Register"]
pub mod dmatargaddr;
#[doc = "DMADEVADDR register accessor: an alias for `Reg<DMADEVADDR_SPEC>`"]
pub type DMADEVADDR = crate::Reg<dmadevaddr::DMADEVADDR_SPEC>;
#[doc = "DMA Device Address Register"]
pub mod dmadevaddr;
#[doc = "DMATOTCOUNT register accessor: an alias for `Reg<DMATOTCOUNT_SPEC>`"]
pub type DMATOTCOUNT = crate::Reg<dmatotcount::DMATOTCOUNT_SPEC>;
#[doc = "DMA Total Transfer Count"]
pub mod dmatotcount;
#[doc = "DMABCOUNT register accessor: an alias for `Reg<DMABCOUNT_SPEC>`"]
pub type DMABCOUNT = crate::Reg<dmabcount::DMABCOUNT_SPEC>;
#[doc = "DMA BYTE Transfer Count"]
pub mod dmabcount;
#[doc = "DMATHRESH register accessor: an alias for `Reg<DMATHRESH_SPEC>`"]
pub type DMATHRESH = crate::Reg<dmathresh::DMATHRESH_SPEC>;
#[doc = "DMA Transmit Trigger Threshhold"]
pub mod dmathresh;
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
#[doc = "CQPAUSE register accessor: an alias for `Reg<CQPAUSE_SPEC>`"]
pub type CQPAUSE = crate::Reg<cqpause::CQPAUSE_SPEC>;
#[doc = "Command Queue Pause Mask Register"]
pub mod cqpause;
#[doc = "CQCURIDX register accessor: an alias for `Reg<CQCURIDX_SPEC>`"]
pub type CQCURIDX = crate::Reg<cqcuridx::CQCURIDX_SPEC>;
#[doc = "Command Queue Current Index"]
pub mod cqcuridx;
#[doc = "CQENDIDX register accessor: an alias for `Reg<CQENDIDX_SPEC>`"]
pub type CQENDIDX = crate::Reg<cqendidx::CQENDIDX_SPEC>;
#[doc = "Command Queue End Index"]
pub mod cqendidx;
