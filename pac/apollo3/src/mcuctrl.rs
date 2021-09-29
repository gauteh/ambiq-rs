#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Chip Information Register"]
    pub chippn: crate::Reg<chippn::CHIPPN_SPEC>,
    #[doc = "0x04 - Unique Chip ID 0"]
    pub chipid0: crate::Reg<chipid0::CHIPID0_SPEC>,
    #[doc = "0x08 - Unique Chip ID 1"]
    pub chipid1: crate::Reg<chipid1::CHIPID1_SPEC>,
    #[doc = "0x0c - Chip Revision"]
    pub chiprev: crate::Reg<chiprev::CHIPREV_SPEC>,
    #[doc = "0x10 - Unique Vendor ID"]
    pub vendorid: crate::Reg<vendorid::VENDORID_SPEC>,
    #[doc = "0x14 - Unique Chip SKU"]
    pub sku: crate::Reg<sku::SKU_SPEC>,
    #[doc = "0x18 - Feature Enable on Burst and BLE"]
    pub featureenable: crate::Reg<featureenable::FEATUREENABLE_SPEC>,
    _reserved7: [u8; 0x04],
    #[doc = "0x20 - Debugger Control"]
    pub debugger: crate::Reg<debugger::DEBUGGER_SPEC>,
    _reserved8: [u8; 0xdc],
    #[doc = "0x100 - BOD control Register"]
    pub bodctrl: crate::Reg<bodctrl::BODCTRL_SPEC>,
    #[doc = "0x104 - ADC Power Up Delay Control"]
    pub adcpwrdly: crate::Reg<adcpwrdly::ADCPWRDLY_SPEC>,
    _reserved10: [u8; 0x04],
    #[doc = "0x10c - ADC Calibration Control"]
    pub adccal: crate::Reg<adccal::ADCCAL_SPEC>,
    #[doc = "0x110 - ADC Battery Load Enable"]
    pub adcbattload: crate::Reg<adcbattload::ADCBATTLOAD_SPEC>,
    _reserved12: [u8; 0x04],
    #[doc = "0x118 - ADC Trims"]
    pub adctrim: crate::Reg<adctrim::ADCTRIM_SPEC>,
    #[doc = "0x11c - ADC Referece Keeper and Comparator Control"]
    pub adcrefcomp: crate::Reg<adcrefcomp::ADCREFCOMP_SPEC>,
    #[doc = "0x120 - XTAL Oscillator Control"]
    pub xtalctrl: crate::Reg<xtalctrl::XTALCTRL_SPEC>,
    #[doc = "0x124 - XTAL Oscillator General Control"]
    pub xtalgenctrl: crate::Reg<xtalgenctrl::XTALGENCTRL_SPEC>,
    _reserved16: [u8; 0x70],
    #[doc = "0x198 - Miscellaneous control register."]
    pub miscctrl: crate::Reg<miscctrl::MISCCTRL_SPEC>,
    _reserved17: [u8; 0x04],
    #[doc = "0x1a0 - Bootloader and secure boot functions"]
    pub bootloader: crate::Reg<bootloader::BOOTLOADER_SPEC>,
    #[doc = "0x1a4 - Register to indicate whether the shadow registers have been successfully loaded from the Flash Information Space."]
    pub shadowvalid: crate::Reg<shadowvalid::SHADOWVALID_SPEC>,
    _reserved19: [u8; 0x08],
    #[doc = "0x1b0 - Scratch register that is not reset by any reset"]
    pub scratch0: crate::Reg<scratch0::SCRATCH0_SPEC>,
    #[doc = "0x1b4 - Scratch register that is not reset by any reset"]
    pub scratch1: crate::Reg<scratch1::SCRATCH1_SPEC>,
    _reserved21: [u8; 0x08],
    #[doc = "0x1c0 - ICODE bus address which was present when a bus fault occurred."]
    pub icodefaultaddr: crate::Reg<icodefaultaddr::ICODEFAULTADDR_SPEC>,
    #[doc = "0x1c4 - DCODE bus address which was present when a bus fault occurred."]
    pub dcodefaultaddr: crate::Reg<dcodefaultaddr::DCODEFAULTADDR_SPEC>,
    #[doc = "0x1c8 - System bus address which was present when a bus fault occurred."]
    pub sysfaultaddr: crate::Reg<sysfaultaddr::SYSFAULTADDR_SPEC>,
    #[doc = "0x1cc - Reflects the status of the bus decoders' fault detection. Any write to this register will clear all of the status bits within the register."]
    pub faultstatus: crate::Reg<faultstatus::FAULTSTATUS_SPEC>,
    #[doc = "0x1d0 - Enable the fault capture registers"]
    pub faultcaptureen: crate::Reg<faultcaptureen::FAULTCAPTUREEN_SPEC>,
    _reserved26: [u8; 0x2c],
    #[doc = "0x200 - Read-only debug register 1"]
    pub dbgr1: crate::Reg<dbgr1::DBGR1_SPEC>,
    #[doc = "0x204 - Read-only debug register 2"]
    pub dbgr2: crate::Reg<dbgr2::DBGR2_SPEC>,
    _reserved28: [u8; 0x18],
    #[doc = "0x220 - Control bit to enable/disable the PMU"]
    pub pmuenable: crate::Reg<pmuenable::PMUENABLE_SPEC>,
    _reserved29: [u8; 0x2c],
    #[doc = "0x250 - TPIU Control Register. Determines the clock enable and frequency for the M4's TPIU interface."]
    pub tpiuctrl: crate::Reg<tpiuctrl::TPIUCTRL_SPEC>,
    _reserved30: [u8; 0x10],
    #[doc = "0x264 - OTA (Over the Air) Update Pointer/Status. Reset only by POA"]
    pub otapointer: crate::Reg<otapointer::OTAPOINTER_SPEC>,
    _reserved31: [u8; 0x18],
    #[doc = "0x280 - DMA Control Register. Determines misc settings for DMA operation"]
    pub apbdmactrl: crate::Reg<apbdmactrl::APBDMACTRL_SPEC>,
    #[doc = "0x284 - SRAM Controller mode bits"]
    pub srammode: crate::Reg<srammode::SRAMMODE_SPEC>,
    _reserved33: [u8; 0xc0],
    #[doc = "0x348 - Key Register to enable the use of external clock selects via the EXTCLKSEL reg"]
    pub kextclksel: crate::Reg<kextclksel::KEXTCLKSEL_SPEC>,
    _reserved34: [u8; 0x10],
    #[doc = "0x35c - SIMO Buck Control Reg1"]
    pub simobuck4: crate::Reg<simobuck4::SIMOBUCK4_SPEC>,
    _reserved35: [u8; 0x08],
    #[doc = "0x368 - BLEBUCK2 Control Reg"]
    pub blebuck2: crate::Reg<blebuck2::BLEBUCK2_SPEC>,
    _reserved36: [u8; 0x34],
    #[doc = "0x3a0 - Flash Write Protection Bits"]
    pub flashwprot0: crate::Reg<flashwprot0::FLASHWPROT0_SPEC>,
    #[doc = "0x3a4 - Flash Write Protection Bits"]
    pub flashwprot1: crate::Reg<flashwprot1::FLASHWPROT1_SPEC>,
    _reserved38: [u8; 0x08],
    #[doc = "0x3b0 - Flash Read Protection Bits"]
    pub flashrprot0: crate::Reg<flashrprot0::FLASHRPROT0_SPEC>,
    #[doc = "0x3b4 - Flash Read Protection Bits"]
    pub flashrprot1: crate::Reg<flashrprot1::FLASHRPROT1_SPEC>,
    _reserved40: [u8; 0x08],
    #[doc = "0x3c0 - SRAM write-protection bits."]
    pub dmasramwriteprotect0: crate::Reg<dmasramwriteprotect0::DMASRAMWRITEPROTECT0_SPEC>,
    #[doc = "0x3c4 - SRAM write-protection bits."]
    pub dmasramwriteprotect1: crate::Reg<dmasramwriteprotect1::DMASRAMWRITEPROTECT1_SPEC>,
    _reserved42: [u8; 0x08],
    #[doc = "0x3d0 - SRAM read-protection bits."]
    pub dmasramreadprotect0: crate::Reg<dmasramreadprotect0::DMASRAMREADPROTECT0_SPEC>,
    #[doc = "0x3d4 - SRAM read-protection bits."]
    pub dmasramreadprotect1: crate::Reg<dmasramreadprotect1::DMASRAMREADPROTECT1_SPEC>,
}
#[doc = "CHIPPN register accessor: an alias for `Reg<CHIPPN_SPEC>`"]
pub type CHIPPN = crate::Reg<chippn::CHIPPN_SPEC>;
#[doc = "Chip Information Register"]
pub mod chippn;
#[doc = "CHIPID0 register accessor: an alias for `Reg<CHIPID0_SPEC>`"]
pub type CHIPID0 = crate::Reg<chipid0::CHIPID0_SPEC>;
#[doc = "Unique Chip ID 0"]
pub mod chipid0;
#[doc = "CHIPID1 register accessor: an alias for `Reg<CHIPID1_SPEC>`"]
pub type CHIPID1 = crate::Reg<chipid1::CHIPID1_SPEC>;
#[doc = "Unique Chip ID 1"]
pub mod chipid1;
#[doc = "CHIPREV register accessor: an alias for `Reg<CHIPREV_SPEC>`"]
pub type CHIPREV = crate::Reg<chiprev::CHIPREV_SPEC>;
#[doc = "Chip Revision"]
pub mod chiprev;
#[doc = "VENDORID register accessor: an alias for `Reg<VENDORID_SPEC>`"]
pub type VENDORID = crate::Reg<vendorid::VENDORID_SPEC>;
#[doc = "Unique Vendor ID"]
pub mod vendorid;
#[doc = "SKU register accessor: an alias for `Reg<SKU_SPEC>`"]
pub type SKU = crate::Reg<sku::SKU_SPEC>;
#[doc = "Unique Chip SKU"]
pub mod sku;
#[doc = "FEATUREENABLE register accessor: an alias for `Reg<FEATUREENABLE_SPEC>`"]
pub type FEATUREENABLE = crate::Reg<featureenable::FEATUREENABLE_SPEC>;
#[doc = "Feature Enable on Burst and BLE"]
pub mod featureenable;
#[doc = "DEBUGGER register accessor: an alias for `Reg<DEBUGGER_SPEC>`"]
pub type DEBUGGER = crate::Reg<debugger::DEBUGGER_SPEC>;
#[doc = "Debugger Control"]
pub mod debugger;
#[doc = "BODCTRL register accessor: an alias for `Reg<BODCTRL_SPEC>`"]
pub type BODCTRL = crate::Reg<bodctrl::BODCTRL_SPEC>;
#[doc = "BOD control Register"]
pub mod bodctrl;
#[doc = "ADCPWRDLY register accessor: an alias for `Reg<ADCPWRDLY_SPEC>`"]
pub type ADCPWRDLY = crate::Reg<adcpwrdly::ADCPWRDLY_SPEC>;
#[doc = "ADC Power Up Delay Control"]
pub mod adcpwrdly;
#[doc = "ADCCAL register accessor: an alias for `Reg<ADCCAL_SPEC>`"]
pub type ADCCAL = crate::Reg<adccal::ADCCAL_SPEC>;
#[doc = "ADC Calibration Control"]
pub mod adccal;
#[doc = "ADCBATTLOAD register accessor: an alias for `Reg<ADCBATTLOAD_SPEC>`"]
pub type ADCBATTLOAD = crate::Reg<adcbattload::ADCBATTLOAD_SPEC>;
#[doc = "ADC Battery Load Enable"]
pub mod adcbattload;
#[doc = "ADCTRIM register accessor: an alias for `Reg<ADCTRIM_SPEC>`"]
pub type ADCTRIM = crate::Reg<adctrim::ADCTRIM_SPEC>;
#[doc = "ADC Trims"]
pub mod adctrim;
#[doc = "ADCREFCOMP register accessor: an alias for `Reg<ADCREFCOMP_SPEC>`"]
pub type ADCREFCOMP = crate::Reg<adcrefcomp::ADCREFCOMP_SPEC>;
#[doc = "ADC Referece Keeper and Comparator Control"]
pub mod adcrefcomp;
#[doc = "XTALCTRL register accessor: an alias for `Reg<XTALCTRL_SPEC>`"]
pub type XTALCTRL = crate::Reg<xtalctrl::XTALCTRL_SPEC>;
#[doc = "XTAL Oscillator Control"]
pub mod xtalctrl;
#[doc = "XTALGENCTRL register accessor: an alias for `Reg<XTALGENCTRL_SPEC>`"]
pub type XTALGENCTRL = crate::Reg<xtalgenctrl::XTALGENCTRL_SPEC>;
#[doc = "XTAL Oscillator General Control"]
pub mod xtalgenctrl;
#[doc = "MISCCTRL register accessor: an alias for `Reg<MISCCTRL_SPEC>`"]
pub type MISCCTRL = crate::Reg<miscctrl::MISCCTRL_SPEC>;
#[doc = "Miscellaneous control register."]
pub mod miscctrl;
#[doc = "BOOTLOADER register accessor: an alias for `Reg<BOOTLOADER_SPEC>`"]
pub type BOOTLOADER = crate::Reg<bootloader::BOOTLOADER_SPEC>;
#[doc = "Bootloader and secure boot functions"]
pub mod bootloader;
#[doc = "SHADOWVALID register accessor: an alias for `Reg<SHADOWVALID_SPEC>`"]
pub type SHADOWVALID = crate::Reg<shadowvalid::SHADOWVALID_SPEC>;
#[doc = "Register to indicate whether the shadow registers have been successfully loaded from the Flash Information Space."]
pub mod shadowvalid;
#[doc = "SCRATCH0 register accessor: an alias for `Reg<SCRATCH0_SPEC>`"]
pub type SCRATCH0 = crate::Reg<scratch0::SCRATCH0_SPEC>;
#[doc = "Scratch register that is not reset by any reset"]
pub mod scratch0;
#[doc = "SCRATCH1 register accessor: an alias for `Reg<SCRATCH1_SPEC>`"]
pub type SCRATCH1 = crate::Reg<scratch1::SCRATCH1_SPEC>;
#[doc = "Scratch register that is not reset by any reset"]
pub mod scratch1;
#[doc = "ICODEFAULTADDR register accessor: an alias for `Reg<ICODEFAULTADDR_SPEC>`"]
pub type ICODEFAULTADDR = crate::Reg<icodefaultaddr::ICODEFAULTADDR_SPEC>;
#[doc = "ICODE bus address which was present when a bus fault occurred."]
pub mod icodefaultaddr;
#[doc = "DCODEFAULTADDR register accessor: an alias for `Reg<DCODEFAULTADDR_SPEC>`"]
pub type DCODEFAULTADDR = crate::Reg<dcodefaultaddr::DCODEFAULTADDR_SPEC>;
#[doc = "DCODE bus address which was present when a bus fault occurred."]
pub mod dcodefaultaddr;
#[doc = "SYSFAULTADDR register accessor: an alias for `Reg<SYSFAULTADDR_SPEC>`"]
pub type SYSFAULTADDR = crate::Reg<sysfaultaddr::SYSFAULTADDR_SPEC>;
#[doc = "System bus address which was present when a bus fault occurred."]
pub mod sysfaultaddr;
#[doc = "FAULTSTATUS register accessor: an alias for `Reg<FAULTSTATUS_SPEC>`"]
pub type FAULTSTATUS = crate::Reg<faultstatus::FAULTSTATUS_SPEC>;
#[doc = "Reflects the status of the bus decoders' fault detection. Any write to this register will clear all of the status bits within the register."]
pub mod faultstatus;
#[doc = "FAULTCAPTUREEN register accessor: an alias for `Reg<FAULTCAPTUREEN_SPEC>`"]
pub type FAULTCAPTUREEN = crate::Reg<faultcaptureen::FAULTCAPTUREEN_SPEC>;
#[doc = "Enable the fault capture registers"]
pub mod faultcaptureen;
#[doc = "DBGR1 register accessor: an alias for `Reg<DBGR1_SPEC>`"]
pub type DBGR1 = crate::Reg<dbgr1::DBGR1_SPEC>;
#[doc = "Read-only debug register 1"]
pub mod dbgr1;
#[doc = "DBGR2 register accessor: an alias for `Reg<DBGR2_SPEC>`"]
pub type DBGR2 = crate::Reg<dbgr2::DBGR2_SPEC>;
#[doc = "Read-only debug register 2"]
pub mod dbgr2;
#[doc = "PMUENABLE register accessor: an alias for `Reg<PMUENABLE_SPEC>`"]
pub type PMUENABLE = crate::Reg<pmuenable::PMUENABLE_SPEC>;
#[doc = "Control bit to enable/disable the PMU"]
pub mod pmuenable;
#[doc = "TPIUCTRL register accessor: an alias for `Reg<TPIUCTRL_SPEC>`"]
pub type TPIUCTRL = crate::Reg<tpiuctrl::TPIUCTRL_SPEC>;
#[doc = "TPIU Control Register. Determines the clock enable and frequency for the M4's TPIU interface."]
pub mod tpiuctrl;
#[doc = "OTAPOINTER register accessor: an alias for `Reg<OTAPOINTER_SPEC>`"]
pub type OTAPOINTER = crate::Reg<otapointer::OTAPOINTER_SPEC>;
#[doc = "OTA (Over the Air) Update Pointer/Status. Reset only by POA"]
pub mod otapointer;
#[doc = "APBDMACTRL register accessor: an alias for `Reg<APBDMACTRL_SPEC>`"]
pub type APBDMACTRL = crate::Reg<apbdmactrl::APBDMACTRL_SPEC>;
#[doc = "DMA Control Register. Determines misc settings for DMA operation"]
pub mod apbdmactrl;
#[doc = "SRAMMODE register accessor: an alias for `Reg<SRAMMODE_SPEC>`"]
pub type SRAMMODE = crate::Reg<srammode::SRAMMODE_SPEC>;
#[doc = "SRAM Controller mode bits"]
pub mod srammode;
#[doc = "KEXTCLKSEL register accessor: an alias for `Reg<KEXTCLKSEL_SPEC>`"]
pub type KEXTCLKSEL = crate::Reg<kextclksel::KEXTCLKSEL_SPEC>;
#[doc = "Key Register to enable the use of external clock selects via the EXTCLKSEL reg"]
pub mod kextclksel;
#[doc = "SIMOBUCK4 register accessor: an alias for `Reg<SIMOBUCK4_SPEC>`"]
pub type SIMOBUCK4 = crate::Reg<simobuck4::SIMOBUCK4_SPEC>;
#[doc = "SIMO Buck Control Reg1"]
pub mod simobuck4;
#[doc = "BLEBUCK2 register accessor: an alias for `Reg<BLEBUCK2_SPEC>`"]
pub type BLEBUCK2 = crate::Reg<blebuck2::BLEBUCK2_SPEC>;
#[doc = "BLEBUCK2 Control Reg"]
pub mod blebuck2;
#[doc = "FLASHWPROT0 register accessor: an alias for `Reg<FLASHWPROT0_SPEC>`"]
pub type FLASHWPROT0 = crate::Reg<flashwprot0::FLASHWPROT0_SPEC>;
#[doc = "Flash Write Protection Bits"]
pub mod flashwprot0;
#[doc = "FLASHWPROT1 register accessor: an alias for `Reg<FLASHWPROT1_SPEC>`"]
pub type FLASHWPROT1 = crate::Reg<flashwprot1::FLASHWPROT1_SPEC>;
#[doc = "Flash Write Protection Bits"]
pub mod flashwprot1;
#[doc = "FLASHRPROT0 register accessor: an alias for `Reg<FLASHRPROT0_SPEC>`"]
pub type FLASHRPROT0 = crate::Reg<flashrprot0::FLASHRPROT0_SPEC>;
#[doc = "Flash Read Protection Bits"]
pub mod flashrprot0;
#[doc = "FLASHRPROT1 register accessor: an alias for `Reg<FLASHRPROT1_SPEC>`"]
pub type FLASHRPROT1 = crate::Reg<flashrprot1::FLASHRPROT1_SPEC>;
#[doc = "Flash Read Protection Bits"]
pub mod flashrprot1;
#[doc = "DMASRAMWRITEPROTECT0 register accessor: an alias for `Reg<DMASRAMWRITEPROTECT0_SPEC>`"]
pub type DMASRAMWRITEPROTECT0 = crate::Reg<dmasramwriteprotect0::DMASRAMWRITEPROTECT0_SPEC>;
#[doc = "SRAM write-protection bits."]
pub mod dmasramwriteprotect0;
#[doc = "DMASRAMWRITEPROTECT1 register accessor: an alias for `Reg<DMASRAMWRITEPROTECT1_SPEC>`"]
pub type DMASRAMWRITEPROTECT1 = crate::Reg<dmasramwriteprotect1::DMASRAMWRITEPROTECT1_SPEC>;
#[doc = "SRAM write-protection bits."]
pub mod dmasramwriteprotect1;
#[doc = "DMASRAMREADPROTECT0 register accessor: an alias for `Reg<DMASRAMREADPROTECT0_SPEC>`"]
pub type DMASRAMREADPROTECT0 = crate::Reg<dmasramreadprotect0::DMASRAMREADPROTECT0_SPEC>;
#[doc = "SRAM read-protection bits."]
pub mod dmasramreadprotect0;
#[doc = "DMASRAMREADPROTECT1 register accessor: an alias for `Reg<DMASRAMREADPROTECT1_SPEC>`"]
pub type DMASRAMREADPROTECT1 = crate::Reg<dmasramreadprotect1::DMASRAMREADPROTECT1_SPEC>;
#[doc = "SRAM read-protection bits."]
pub mod dmasramreadprotect1;
