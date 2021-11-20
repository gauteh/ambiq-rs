use cty::*;

#[repr(C)]
struct SECTOR_INFO  {
  SectorSize: u32,       // Sector Size in bytes
  SectorStartAddr: u32,  // Start address of the sector area (relative to the "BaseAddr" of the flash)
}

#[repr(C)]
pub struct FlashDevice {
   AlgoVer: u16,       // Algo version number
   Name: [u8; 128],     // Flash device name. NEVER change the size of this array!
   Type: u16,          // Flash device type
   BaseAddr: u32,      // Flash base address
   TotalSize: u32,     // Total flash device size in Bytes (256 KB)
   PageSize: u32,      // Page Size (number of bytes that will be passed to ProgramPage(). MinAlig is 8 byte
   Reserved: u32,      // Reserved, should be 0
   ErasedVal: u8,     // Flash erased value
   TimeoutProg: u32,   // Program page timeout in ms
   TimeoutErase: u32,  // Erase sector timeout in ms
   SectorInfo: [SECTOR_INFO; 4], // Flash sector layout definition. May be adapted up to 512 entries
}

static FlashDevice: FlashDevice = FlashDevice {
  AlgoVer: 0,
  Name: 

};

