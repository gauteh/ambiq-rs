#[doc = "Register `CFG` reader"]
pub struct R(crate::R<CFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CFG` writer"]
pub struct W(crate::W<CFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<CFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Serial clock polarity.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CPOL_A {
    #[doc = "0: Clock inactive state is low. value."]
    LOW = 0,
    #[doc = "1: Clock inactive state is high. value."]
    HIGH = 1,
}
impl From<CPOL_A> for bool {
    #[inline(always)]
    fn from(variant: CPOL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CPOL` reader - Serial clock polarity."]
pub struct CPOL_R(crate::FieldReader<bool, CPOL_A>);
impl CPOL_R {
    pub(crate) fn new(bits: bool) -> Self {
        CPOL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CPOL_A {
        match self.bits {
            false => CPOL_A::LOW,
            true => CPOL_A::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        **self == CPOL_A::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        **self == CPOL_A::HIGH
    }
}
impl core::ops::Deref for CPOL_R {
    type Target = crate::FieldReader<bool, CPOL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CPOL` writer - Serial clock polarity."]
pub struct CPOL_W<'a> {
    w: &'a mut W,
}
impl<'a> CPOL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CPOL_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clock inactive state is low. value."]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(CPOL_A::LOW)
    }
    #[doc = "Clock inactive state is high. value."]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(CPOL_A::HIGH)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 17)) | ((value as u32 & 0x01) << 17);
        self.w
    }
}
#[doc = "Serial clock phase.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CPHA_A {
    #[doc = "0: Clock toggles in middle of data bit. value."]
    MIDDLE = 0,
    #[doc = "1: Clock toggles at start of data bit. value."]
    START = 1,
}
impl From<CPHA_A> for bool {
    #[inline(always)]
    fn from(variant: CPHA_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CPHA` reader - Serial clock phase."]
pub struct CPHA_R(crate::FieldReader<bool, CPHA_A>);
impl CPHA_R {
    pub(crate) fn new(bits: bool) -> Self {
        CPHA_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CPHA_A {
        match self.bits {
            false => CPHA_A::MIDDLE,
            true => CPHA_A::START,
        }
    }
    #[doc = "Checks if the value of the field is `MIDDLE`"]
    #[inline(always)]
    pub fn is_middle(&self) -> bool {
        **self == CPHA_A::MIDDLE
    }
    #[doc = "Checks if the value of the field is `START`"]
    #[inline(always)]
    pub fn is_start(&self) -> bool {
        **self == CPHA_A::START
    }
}
impl core::ops::Deref for CPHA_R {
    type Target = crate::FieldReader<bool, CPHA_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CPHA` writer - Serial clock phase."]
pub struct CPHA_W<'a> {
    w: &'a mut W,
}
impl<'a> CPHA_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CPHA_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clock toggles in middle of data bit. value."]
    #[inline(always)]
    pub fn middle(self) -> &'a mut W {
        self.variant(CPHA_A::MIDDLE)
    }
    #[doc = "Clock toggles at start of data bit. value."]
    #[inline(always)]
    pub fn start(self) -> &'a mut W {
        self.variant(CPHA_A::START)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 16)) | ((value as u32 & 0x01) << 16);
        self.w
    }
}
#[doc = "Field `TURNAROUND` reader - Number of turnaound cycles (for TX->RX transitions). Qualified by ENTURN or XIPENTURN bit field."]
pub struct TURNAROUND_R(crate::FieldReader<u8, u8>);
impl TURNAROUND_R {
    pub(crate) fn new(bits: u8) -> Self {
        TURNAROUND_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TURNAROUND_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TURNAROUND` writer - Number of turnaound cycles (for TX->RX transitions). Qualified by ENTURN or XIPENTURN bit field."]
pub struct TURNAROUND_W<'a> {
    w: &'a mut W,
}
impl<'a> TURNAROUND_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 8)) | ((value as u32 & 0x3f) << 8);
        self.w
    }
}
#[doc = "Field `SEPIO` reader - Separate IO configuration. This bit should be set when the target device has separate MOSI and MISO pins. Respective IN/OUT bits below should be set to map pins."]
pub struct SEPIO_R(crate::FieldReader<bool, bool>);
impl SEPIO_R {
    pub(crate) fn new(bits: bool) -> Self {
        SEPIO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SEPIO_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SEPIO` writer - Separate IO configuration. This bit should be set when the target device has separate MOSI and MISO pins. Respective IN/OUT bits below should be set to map pins."]
pub struct SEPIO_W<'a> {
    w: &'a mut W,
}
impl<'a> SEPIO_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 7)) | ((value as u32 & 0x01) << 7);
        self.w
    }
}
#[doc = "Field `ISIZE` reader - Instruction Size enum name = I8 value = 0x0 desc = Instruction is 1 byte enum name = I16 value = 0x1 desc = Instruction is 2 bytes"]
pub struct ISIZE_R(crate::FieldReader<bool, bool>);
impl ISIZE_R {
    pub(crate) fn new(bits: bool) -> Self {
        ISIZE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ISIZE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ISIZE` writer - Instruction Size enum name = I8 value = 0x0 desc = Instruction is 1 byte enum name = I16 value = 0x1 desc = Instruction is 2 bytes"]
pub struct ISIZE_W<'a> {
    w: &'a mut W,
}
impl<'a> ISIZE_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 6)) | ((value as u32 & 0x01) << 6);
        self.w
    }
}
#[doc = "Field `ASIZE` reader - Address Size. Address bytes to send from ADDR register name = A1 value = 0x0 desc = Send one address byte enum name = A2 value = 0x1 desc = Send two address bytes enum name = A3 value = 0x2 desc = Send three address bytes enum name = A4 value = 0x3 desc = Send four address bytes"]
pub struct ASIZE_R(crate::FieldReader<u8, u8>);
impl ASIZE_R {
    pub(crate) fn new(bits: u8) -> Self {
        ASIZE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ASIZE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ASIZE` writer - Address Size. Address bytes to send from ADDR register name = A1 value = 0x0 desc = Send one address byte enum name = A2 value = 0x1 desc = Send two address bytes enum name = A3 value = 0x2 desc = Send three address bytes enum name = A4 value = 0x3 desc = Send four address bytes"]
pub struct ASIZE_W<'a> {
    w: &'a mut W,
}
impl<'a> ASIZE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | ((value as u32 & 0x03) << 4);
        self.w
    }
}
#[doc = "Flash configuration for XIP and AUTO DMA operations. Controls value for SER (Slave Enable) for XIP operations and address generation for DMA/XIP modes. Also used to configure SPIFRF (frame format).\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum DEVCFG_A {
    #[doc = "1: Single bit SPI flash on chip select 0 value."]
    SERIAL0 = 1,
    #[doc = "2: Single bit SPI flash on chip select 1 value."]
    SERIAL1 = 2,
    #[doc = "5: Dual SPI flash on chip select 0 value."]
    DUAL0 = 5,
    #[doc = "6: Dual bit SPI flash on chip select 1 value."]
    DUAL1 = 6,
    #[doc = "9: Quad SPI flash on chip select 0 value."]
    QUAD0 = 9,
    #[doc = "10: Quad SPI flash on chip select 1 value."]
    QUAD1 = 10,
    #[doc = "13: Octal SPI flash on chip select 0 value."]
    OCTAL0 = 13,
    #[doc = "14: Octal SPI flash on chip select 1 value."]
    OCTAL1 = 14,
    #[doc = "15: Dual Quad SPI flash on chip selects 0/1. value."]
    QUADPAIRED = 15,
    #[doc = "3: Dual Quad SPI flash on chip selects 0/1, but transmit in serial mode for initialization operations value."]
    QUADPAIRED_SERIAL = 3,
}
impl From<DEVCFG_A> for u8 {
    #[inline(always)]
    fn from(variant: DEVCFG_A) -> Self {
        variant as _
    }
}
#[doc = "Field `DEVCFG` reader - Flash configuration for XIP and AUTO DMA operations. Controls value for SER (Slave Enable) for XIP operations and address generation for DMA/XIP modes. Also used to configure SPIFRF (frame format)."]
pub struct DEVCFG_R(crate::FieldReader<u8, DEVCFG_A>);
impl DEVCFG_R {
    pub(crate) fn new(bits: u8) -> Self {
        DEVCFG_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<DEVCFG_A> {
        match self.bits {
            1 => Some(DEVCFG_A::SERIAL0),
            2 => Some(DEVCFG_A::SERIAL1),
            5 => Some(DEVCFG_A::DUAL0),
            6 => Some(DEVCFG_A::DUAL1),
            9 => Some(DEVCFG_A::QUAD0),
            10 => Some(DEVCFG_A::QUAD1),
            13 => Some(DEVCFG_A::OCTAL0),
            14 => Some(DEVCFG_A::OCTAL1),
            15 => Some(DEVCFG_A::QUADPAIRED),
            3 => Some(DEVCFG_A::QUADPAIRED_SERIAL),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `SERIAL0`"]
    #[inline(always)]
    pub fn is_serial0(&self) -> bool {
        **self == DEVCFG_A::SERIAL0
    }
    #[doc = "Checks if the value of the field is `SERIAL1`"]
    #[inline(always)]
    pub fn is_serial1(&self) -> bool {
        **self == DEVCFG_A::SERIAL1
    }
    #[doc = "Checks if the value of the field is `DUAL0`"]
    #[inline(always)]
    pub fn is_dual0(&self) -> bool {
        **self == DEVCFG_A::DUAL0
    }
    #[doc = "Checks if the value of the field is `DUAL1`"]
    #[inline(always)]
    pub fn is_dual1(&self) -> bool {
        **self == DEVCFG_A::DUAL1
    }
    #[doc = "Checks if the value of the field is `QUAD0`"]
    #[inline(always)]
    pub fn is_quad0(&self) -> bool {
        **self == DEVCFG_A::QUAD0
    }
    #[doc = "Checks if the value of the field is `QUAD1`"]
    #[inline(always)]
    pub fn is_quad1(&self) -> bool {
        **self == DEVCFG_A::QUAD1
    }
    #[doc = "Checks if the value of the field is `OCTAL0`"]
    #[inline(always)]
    pub fn is_octal0(&self) -> bool {
        **self == DEVCFG_A::OCTAL0
    }
    #[doc = "Checks if the value of the field is `OCTAL1`"]
    #[inline(always)]
    pub fn is_octal1(&self) -> bool {
        **self == DEVCFG_A::OCTAL1
    }
    #[doc = "Checks if the value of the field is `QUADPAIRED`"]
    #[inline(always)]
    pub fn is_quadpaired(&self) -> bool {
        **self == DEVCFG_A::QUADPAIRED
    }
    #[doc = "Checks if the value of the field is `QUADPAIRED_SERIAL`"]
    #[inline(always)]
    pub fn is_quadpaired_serial(&self) -> bool {
        **self == DEVCFG_A::QUADPAIRED_SERIAL
    }
}
impl core::ops::Deref for DEVCFG_R {
    type Target = crate::FieldReader<u8, DEVCFG_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DEVCFG` writer - Flash configuration for XIP and AUTO DMA operations. Controls value for SER (Slave Enable) for XIP operations and address generation for DMA/XIP modes. Also used to configure SPIFRF (frame format)."]
pub struct DEVCFG_W<'a> {
    w: &'a mut W,
}
impl<'a> DEVCFG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DEVCFG_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Single bit SPI flash on chip select 0 value."]
    #[inline(always)]
    pub fn serial0(self) -> &'a mut W {
        self.variant(DEVCFG_A::SERIAL0)
    }
    #[doc = "Single bit SPI flash on chip select 1 value."]
    #[inline(always)]
    pub fn serial1(self) -> &'a mut W {
        self.variant(DEVCFG_A::SERIAL1)
    }
    #[doc = "Dual SPI flash on chip select 0 value."]
    #[inline(always)]
    pub fn dual0(self) -> &'a mut W {
        self.variant(DEVCFG_A::DUAL0)
    }
    #[doc = "Dual bit SPI flash on chip select 1 value."]
    #[inline(always)]
    pub fn dual1(self) -> &'a mut W {
        self.variant(DEVCFG_A::DUAL1)
    }
    #[doc = "Quad SPI flash on chip select 0 value."]
    #[inline(always)]
    pub fn quad0(self) -> &'a mut W {
        self.variant(DEVCFG_A::QUAD0)
    }
    #[doc = "Quad SPI flash on chip select 1 value."]
    #[inline(always)]
    pub fn quad1(self) -> &'a mut W {
        self.variant(DEVCFG_A::QUAD1)
    }
    #[doc = "Octal SPI flash on chip select 0 value."]
    #[inline(always)]
    pub fn octal0(self) -> &'a mut W {
        self.variant(DEVCFG_A::OCTAL0)
    }
    #[doc = "Octal SPI flash on chip select 1 value."]
    #[inline(always)]
    pub fn octal1(self) -> &'a mut W {
        self.variant(DEVCFG_A::OCTAL1)
    }
    #[doc = "Dual Quad SPI flash on chip selects 0/1. value."]
    #[inline(always)]
    pub fn quadpaired(self) -> &'a mut W {
        self.variant(DEVCFG_A::QUADPAIRED)
    }
    #[doc = "Dual Quad SPI flash on chip selects 0/1, but transmit in serial mode for initialization operations value."]
    #[inline(always)]
    pub fn quadpaired_serial(self) -> &'a mut W {
        self.variant(DEVCFG_A::QUADPAIRED_SERIAL)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | (value as u32 & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bit 17 - Serial clock polarity."]
    #[inline(always)]
    pub fn cpol(&self) -> CPOL_R {
        CPOL_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Serial clock phase."]
    #[inline(always)]
    pub fn cpha(&self) -> CPHA_R {
        CPHA_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bits 8:13 - Number of turnaound cycles (for TX->RX transitions). Qualified by ENTURN or XIPENTURN bit field."]
    #[inline(always)]
    pub fn turnaround(&self) -> TURNAROUND_R {
        TURNAROUND_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bit 7 - Separate IO configuration. This bit should be set when the target device has separate MOSI and MISO pins. Respective IN/OUT bits below should be set to map pins."]
    #[inline(always)]
    pub fn sepio(&self) -> SEPIO_R {
        SEPIO_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Instruction Size enum name = I8 value = 0x0 desc = Instruction is 1 byte enum name = I16 value = 0x1 desc = Instruction is 2 bytes"]
    #[inline(always)]
    pub fn isize(&self) -> ISIZE_R {
        ISIZE_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bits 4:5 - Address Size. Address bytes to send from ADDR register name = A1 value = 0x0 desc = Send one address byte enum name = A2 value = 0x1 desc = Send two address bytes enum name = A3 value = 0x2 desc = Send three address bytes enum name = A4 value = 0x3 desc = Send four address bytes"]
    #[inline(always)]
    pub fn asize(&self) -> ASIZE_R {
        ASIZE_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 0:3 - Flash configuration for XIP and AUTO DMA operations. Controls value for SER (Slave Enable) for XIP operations and address generation for DMA/XIP modes. Also used to configure SPIFRF (frame format)."]
    #[inline(always)]
    pub fn devcfg(&self) -> DEVCFG_R {
        DEVCFG_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 17 - Serial clock polarity."]
    #[inline(always)]
    pub fn cpol(&mut self) -> CPOL_W {
        CPOL_W { w: self }
    }
    #[doc = "Bit 16 - Serial clock phase."]
    #[inline(always)]
    pub fn cpha(&mut self) -> CPHA_W {
        CPHA_W { w: self }
    }
    #[doc = "Bits 8:13 - Number of turnaound cycles (for TX->RX transitions). Qualified by ENTURN or XIPENTURN bit field."]
    #[inline(always)]
    pub fn turnaround(&mut self) -> TURNAROUND_W {
        TURNAROUND_W { w: self }
    }
    #[doc = "Bit 7 - Separate IO configuration. This bit should be set when the target device has separate MOSI and MISO pins. Respective IN/OUT bits below should be set to map pins."]
    #[inline(always)]
    pub fn sepio(&mut self) -> SEPIO_W {
        SEPIO_W { w: self }
    }
    #[doc = "Bit 6 - Instruction Size enum name = I8 value = 0x0 desc = Instruction is 1 byte enum name = I16 value = 0x1 desc = Instruction is 2 bytes"]
    #[inline(always)]
    pub fn isize(&mut self) -> ISIZE_W {
        ISIZE_W { w: self }
    }
    #[doc = "Bits 4:5 - Address Size. Address bytes to send from ADDR register name = A1 value = 0x0 desc = Send one address byte enum name = A2 value = 0x1 desc = Send two address bytes enum name = A3 value = 0x2 desc = Send three address bytes enum name = A4 value = 0x3 desc = Send four address bytes"]
    #[inline(always)]
    pub fn asize(&mut self) -> ASIZE_W {
        ASIZE_W { w: self }
    }
    #[doc = "Bits 0:3 - Flash configuration for XIP and AUTO DMA operations. Controls value for SER (Slave Enable) for XIP operations and address generation for DMA/XIP modes. Also used to configure SPIFRF (frame format)."]
    #[inline(always)]
    pub fn devcfg(&mut self) -> DEVCFG_W {
        DEVCFG_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "MSPI Transfer Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CFG_SPEC;
impl crate::RegisterSpec for CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cfg::R](R) reader structure"]
impl crate::Readable for CFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cfg::W](W) writer structure"]
impl crate::Writable for CFG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CFG to value 0x01"]
impl crate::Resettable for CFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x01
    }
}
