#[doc = "Register `PADREGL` reader"]
pub struct R(crate::R<PADREGL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PADREGL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PADREGL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PADREGL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PADREGL` writer"]
pub struct W(crate::W<PADREGL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PADREGL_SPEC>;
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
impl From<crate::W<PADREGL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PADREGL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Pad 47 function select\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PAD47FNCSEL_A {
    #[doc = "0: Configure as the 32kHz output clock from the crystal value."]
    _32KHZXT = 0,
    #[doc = "1: IOM/MSPI nCE group 47 value."]
    NCE47 = 1,
    #[doc = "2: CTIMER connection 26 value."]
    CT26 = 2,
    #[doc = "3: Configure as GPIO47 value."]
    GPIO47 = 3,
    #[doc = "5: Configure as the IOMSTR5 SPI MOSI output signal value."]
    M5MOSI = 5,
    #[doc = "6: Configure as the UART1 RX input signal value."]
    UART1RX = 6,
}
impl From<PAD47FNCSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: PAD47FNCSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `PAD47FNCSEL` reader - Pad 47 function select"]
pub struct PAD47FNCSEL_R(crate::FieldReader<u8, PAD47FNCSEL_A>);
impl PAD47FNCSEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        PAD47FNCSEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PAD47FNCSEL_A> {
        match self.bits {
            0 => Some(PAD47FNCSEL_A::_32KHZXT),
            1 => Some(PAD47FNCSEL_A::NCE47),
            2 => Some(PAD47FNCSEL_A::CT26),
            3 => Some(PAD47FNCSEL_A::GPIO47),
            5 => Some(PAD47FNCSEL_A::M5MOSI),
            6 => Some(PAD47FNCSEL_A::UART1RX),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_32KHZXT`"]
    #[inline(always)]
    pub fn is_32k_hz_xt(&self) -> bool {
        **self == PAD47FNCSEL_A::_32KHZXT
    }
    #[doc = "Checks if the value of the field is `NCE47`"]
    #[inline(always)]
    pub fn is_nce47(&self) -> bool {
        **self == PAD47FNCSEL_A::NCE47
    }
    #[doc = "Checks if the value of the field is `CT26`"]
    #[inline(always)]
    pub fn is_ct26(&self) -> bool {
        **self == PAD47FNCSEL_A::CT26
    }
    #[doc = "Checks if the value of the field is `GPIO47`"]
    #[inline(always)]
    pub fn is_gpio47(&self) -> bool {
        **self == PAD47FNCSEL_A::GPIO47
    }
    #[doc = "Checks if the value of the field is `M5MOSI`"]
    #[inline(always)]
    pub fn is_m5mosi(&self) -> bool {
        **self == PAD47FNCSEL_A::M5MOSI
    }
    #[doc = "Checks if the value of the field is `UART1RX`"]
    #[inline(always)]
    pub fn is_uart1rx(&self) -> bool {
        **self == PAD47FNCSEL_A::UART1RX
    }
}
impl core::ops::Deref for PAD47FNCSEL_R {
    type Target = crate::FieldReader<u8, PAD47FNCSEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PAD47FNCSEL` writer - Pad 47 function select"]
pub struct PAD47FNCSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD47FNCSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD47FNCSEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Configure as the 32kHz output clock from the crystal value."]
    #[inline(always)]
    pub fn _32k_hz_xt(self) -> &'a mut W {
        self.variant(PAD47FNCSEL_A::_32KHZXT)
    }
    #[doc = "IOM/MSPI nCE group 47 value."]
    #[inline(always)]
    pub fn nce47(self) -> &'a mut W {
        self.variant(PAD47FNCSEL_A::NCE47)
    }
    #[doc = "CTIMER connection 26 value."]
    #[inline(always)]
    pub fn ct26(self) -> &'a mut W {
        self.variant(PAD47FNCSEL_A::CT26)
    }
    #[doc = "Configure as GPIO47 value."]
    #[inline(always)]
    pub fn gpio47(self) -> &'a mut W {
        self.variant(PAD47FNCSEL_A::GPIO47)
    }
    #[doc = "Configure as the IOMSTR5 SPI MOSI output signal value."]
    #[inline(always)]
    pub fn m5mosi(self) -> &'a mut W {
        self.variant(PAD47FNCSEL_A::M5MOSI)
    }
    #[doc = "Configure as the UART1 RX input signal value."]
    #[inline(always)]
    pub fn uart1rx(self) -> &'a mut W {
        self.variant(PAD47FNCSEL_A::UART1RX)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 27)) | ((value as u32 & 0x07) << 27);
        self.w
    }
}
#[doc = "Pad 47 drive strength\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD47STRNG_A {
    #[doc = "0: Low drive strength value."]
    LOW = 0,
    #[doc = "1: High drive strength value."]
    HIGH = 1,
}
impl From<PAD47STRNG_A> for bool {
    #[inline(always)]
    fn from(variant: PAD47STRNG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PAD47STRNG` reader - Pad 47 drive strength"]
pub struct PAD47STRNG_R(crate::FieldReader<bool, PAD47STRNG_A>);
impl PAD47STRNG_R {
    pub(crate) fn new(bits: bool) -> Self {
        PAD47STRNG_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAD47STRNG_A {
        match self.bits {
            false => PAD47STRNG_A::LOW,
            true => PAD47STRNG_A::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        **self == PAD47STRNG_A::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        **self == PAD47STRNG_A::HIGH
    }
}
impl core::ops::Deref for PAD47STRNG_R {
    type Target = crate::FieldReader<bool, PAD47STRNG_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PAD47STRNG` writer - Pad 47 drive strength"]
pub struct PAD47STRNG_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD47STRNG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD47STRNG_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Low drive strength value."]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(PAD47STRNG_A::LOW)
    }
    #[doc = "High drive strength value."]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(PAD47STRNG_A::HIGH)
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
        self.w.bits = (self.w.bits & !(0x01 << 26)) | ((value as u32 & 0x01) << 26);
        self.w
    }
}
#[doc = "Pad 47 input enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD47INPEN_A {
    #[doc = "0: Pad input disabled value."]
    DIS = 0,
    #[doc = "1: Pad input enabled value."]
    EN = 1,
}
impl From<PAD47INPEN_A> for bool {
    #[inline(always)]
    fn from(variant: PAD47INPEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PAD47INPEN` reader - Pad 47 input enable"]
pub struct PAD47INPEN_R(crate::FieldReader<bool, PAD47INPEN_A>);
impl PAD47INPEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        PAD47INPEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAD47INPEN_A {
        match self.bits {
            false => PAD47INPEN_A::DIS,
            true => PAD47INPEN_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        **self == PAD47INPEN_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        **self == PAD47INPEN_A::EN
    }
}
impl core::ops::Deref for PAD47INPEN_R {
    type Target = crate::FieldReader<bool, PAD47INPEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PAD47INPEN` writer - Pad 47 input enable"]
pub struct PAD47INPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD47INPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD47INPEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Pad input disabled value."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(PAD47INPEN_A::DIS)
    }
    #[doc = "Pad input enabled value."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(PAD47INPEN_A::EN)
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | ((value as u32 & 0x01) << 25);
        self.w
    }
}
#[doc = "Pad 47 pullup enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD47PULL_A {
    #[doc = "0: Pullup disabled value."]
    DIS = 0,
    #[doc = "1: Pullup enabled value."]
    EN = 1,
}
impl From<PAD47PULL_A> for bool {
    #[inline(always)]
    fn from(variant: PAD47PULL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PAD47PULL` reader - Pad 47 pullup enable"]
pub struct PAD47PULL_R(crate::FieldReader<bool, PAD47PULL_A>);
impl PAD47PULL_R {
    pub(crate) fn new(bits: bool) -> Self {
        PAD47PULL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAD47PULL_A {
        match self.bits {
            false => PAD47PULL_A::DIS,
            true => PAD47PULL_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        **self == PAD47PULL_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        **self == PAD47PULL_A::EN
    }
}
impl core::ops::Deref for PAD47PULL_R {
    type Target = crate::FieldReader<bool, PAD47PULL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PAD47PULL` writer - Pad 47 pullup enable"]
pub struct PAD47PULL_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD47PULL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD47PULL_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Pullup disabled value."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(PAD47PULL_A::DIS)
    }
    #[doc = "Pullup enabled value."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(PAD47PULL_A::EN)
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | ((value as u32 & 0x01) << 24);
        self.w
    }
}
#[doc = "Pad 46 function select\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PAD46FNCSEL_A {
    #[doc = "0: Configure as the 32kHz output clock from the crystal value."]
    _32KHZ_XT = 0,
    #[doc = "1: IOM/MSPI nCE group 46 value."]
    NCE46 = 1,
    #[doc = "2: CTIMER connection 24 value."]
    CT24 = 2,
    #[doc = "3: Configure as GPIO46 value."]
    GPIO46 = 3,
    #[doc = "4: SCARD reset output value."]
    SCCRST = 4,
    #[doc = "5: PDM serial clock output value."]
    PDMCLK = 5,
    #[doc = "6: Configure as the UART1 TX output signal value."]
    UART1TX = 6,
    #[doc = "7: Configure as the serial wire debug SWO signal value."]
    SWO = 7,
}
impl From<PAD46FNCSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: PAD46FNCSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `PAD46FNCSEL` reader - Pad 46 function select"]
pub struct PAD46FNCSEL_R(crate::FieldReader<u8, PAD46FNCSEL_A>);
impl PAD46FNCSEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        PAD46FNCSEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAD46FNCSEL_A {
        match self.bits {
            0 => PAD46FNCSEL_A::_32KHZ_XT,
            1 => PAD46FNCSEL_A::NCE46,
            2 => PAD46FNCSEL_A::CT24,
            3 => PAD46FNCSEL_A::GPIO46,
            4 => PAD46FNCSEL_A::SCCRST,
            5 => PAD46FNCSEL_A::PDMCLK,
            6 => PAD46FNCSEL_A::UART1TX,
            7 => PAD46FNCSEL_A::SWO,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_32KHZ_XT`"]
    #[inline(always)]
    pub fn is_32khz_xt(&self) -> bool {
        **self == PAD46FNCSEL_A::_32KHZ_XT
    }
    #[doc = "Checks if the value of the field is `NCE46`"]
    #[inline(always)]
    pub fn is_nce46(&self) -> bool {
        **self == PAD46FNCSEL_A::NCE46
    }
    #[doc = "Checks if the value of the field is `CT24`"]
    #[inline(always)]
    pub fn is_ct24(&self) -> bool {
        **self == PAD46FNCSEL_A::CT24
    }
    #[doc = "Checks if the value of the field is `GPIO46`"]
    #[inline(always)]
    pub fn is_gpio46(&self) -> bool {
        **self == PAD46FNCSEL_A::GPIO46
    }
    #[doc = "Checks if the value of the field is `SCCRST`"]
    #[inline(always)]
    pub fn is_sccrst(&self) -> bool {
        **self == PAD46FNCSEL_A::SCCRST
    }
    #[doc = "Checks if the value of the field is `PDMCLK`"]
    #[inline(always)]
    pub fn is_pdmclk(&self) -> bool {
        **self == PAD46FNCSEL_A::PDMCLK
    }
    #[doc = "Checks if the value of the field is `UART1TX`"]
    #[inline(always)]
    pub fn is_uart1tx(&self) -> bool {
        **self == PAD46FNCSEL_A::UART1TX
    }
    #[doc = "Checks if the value of the field is `SWO`"]
    #[inline(always)]
    pub fn is_swo(&self) -> bool {
        **self == PAD46FNCSEL_A::SWO
    }
}
impl core::ops::Deref for PAD46FNCSEL_R {
    type Target = crate::FieldReader<u8, PAD46FNCSEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PAD46FNCSEL` writer - Pad 46 function select"]
pub struct PAD46FNCSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD46FNCSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD46FNCSEL_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Configure as the 32kHz output clock from the crystal value."]
    #[inline(always)]
    pub fn _32khz_xt(self) -> &'a mut W {
        self.variant(PAD46FNCSEL_A::_32KHZ_XT)
    }
    #[doc = "IOM/MSPI nCE group 46 value."]
    #[inline(always)]
    pub fn nce46(self) -> &'a mut W {
        self.variant(PAD46FNCSEL_A::NCE46)
    }
    #[doc = "CTIMER connection 24 value."]
    #[inline(always)]
    pub fn ct24(self) -> &'a mut W {
        self.variant(PAD46FNCSEL_A::CT24)
    }
    #[doc = "Configure as GPIO46 value."]
    #[inline(always)]
    pub fn gpio46(self) -> &'a mut W {
        self.variant(PAD46FNCSEL_A::GPIO46)
    }
    #[doc = "SCARD reset output value."]
    #[inline(always)]
    pub fn sccrst(self) -> &'a mut W {
        self.variant(PAD46FNCSEL_A::SCCRST)
    }
    #[doc = "PDM serial clock output value."]
    #[inline(always)]
    pub fn pdmclk(self) -> &'a mut W {
        self.variant(PAD46FNCSEL_A::PDMCLK)
    }
    #[doc = "Configure as the UART1 TX output signal value."]
    #[inline(always)]
    pub fn uart1tx(self) -> &'a mut W {
        self.variant(PAD46FNCSEL_A::UART1TX)
    }
    #[doc = "Configure as the serial wire debug SWO signal value."]
    #[inline(always)]
    pub fn swo(self) -> &'a mut W {
        self.variant(PAD46FNCSEL_A::SWO)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 19)) | ((value as u32 & 0x07) << 19);
        self.w
    }
}
#[doc = "Pad 46 drive strength\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD46STRNG_A {
    #[doc = "0: Low drive strength value."]
    LOW = 0,
    #[doc = "1: High drive strength value."]
    HIGH = 1,
}
impl From<PAD46STRNG_A> for bool {
    #[inline(always)]
    fn from(variant: PAD46STRNG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PAD46STRNG` reader - Pad 46 drive strength"]
pub struct PAD46STRNG_R(crate::FieldReader<bool, PAD46STRNG_A>);
impl PAD46STRNG_R {
    pub(crate) fn new(bits: bool) -> Self {
        PAD46STRNG_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAD46STRNG_A {
        match self.bits {
            false => PAD46STRNG_A::LOW,
            true => PAD46STRNG_A::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        **self == PAD46STRNG_A::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        **self == PAD46STRNG_A::HIGH
    }
}
impl core::ops::Deref for PAD46STRNG_R {
    type Target = crate::FieldReader<bool, PAD46STRNG_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PAD46STRNG` writer - Pad 46 drive strength"]
pub struct PAD46STRNG_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD46STRNG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD46STRNG_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Low drive strength value."]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(PAD46STRNG_A::LOW)
    }
    #[doc = "High drive strength value."]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(PAD46STRNG_A::HIGH)
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | ((value as u32 & 0x01) << 18);
        self.w
    }
}
#[doc = "Pad 46 input enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD46INPEN_A {
    #[doc = "0: Pad input disabled value."]
    DIS = 0,
    #[doc = "1: Pad input enabled value."]
    EN = 1,
}
impl From<PAD46INPEN_A> for bool {
    #[inline(always)]
    fn from(variant: PAD46INPEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PAD46INPEN` reader - Pad 46 input enable"]
pub struct PAD46INPEN_R(crate::FieldReader<bool, PAD46INPEN_A>);
impl PAD46INPEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        PAD46INPEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAD46INPEN_A {
        match self.bits {
            false => PAD46INPEN_A::DIS,
            true => PAD46INPEN_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        **self == PAD46INPEN_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        **self == PAD46INPEN_A::EN
    }
}
impl core::ops::Deref for PAD46INPEN_R {
    type Target = crate::FieldReader<bool, PAD46INPEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PAD46INPEN` writer - Pad 46 input enable"]
pub struct PAD46INPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD46INPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD46INPEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Pad input disabled value."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(PAD46INPEN_A::DIS)
    }
    #[doc = "Pad input enabled value."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(PAD46INPEN_A::EN)
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
#[doc = "Pad 46 pullup enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD46PULL_A {
    #[doc = "0: Pullup disabled value."]
    DIS = 0,
    #[doc = "1: Pullup enabled value."]
    EN = 1,
}
impl From<PAD46PULL_A> for bool {
    #[inline(always)]
    fn from(variant: PAD46PULL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PAD46PULL` reader - Pad 46 pullup enable"]
pub struct PAD46PULL_R(crate::FieldReader<bool, PAD46PULL_A>);
impl PAD46PULL_R {
    pub(crate) fn new(bits: bool) -> Self {
        PAD46PULL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAD46PULL_A {
        match self.bits {
            false => PAD46PULL_A::DIS,
            true => PAD46PULL_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        **self == PAD46PULL_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        **self == PAD46PULL_A::EN
    }
}
impl core::ops::Deref for PAD46PULL_R {
    type Target = crate::FieldReader<bool, PAD46PULL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PAD46PULL` writer - Pad 46 pullup enable"]
pub struct PAD46PULL_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD46PULL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD46PULL_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Pullup disabled value."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(PAD46PULL_A::DIS)
    }
    #[doc = "Pullup enabled value."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(PAD46PULL_A::EN)
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
#[doc = "Pad 45 function select\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PAD45FNCSEL_A {
    #[doc = "0: Configure as the UART1 CTS input signal value."]
    UA1CTS = 0,
    #[doc = "1: IOM/MSPI nCE group 45 value."]
    NCE45 = 1,
    #[doc = "2: CTIMER connection 22 value."]
    CT22 = 2,
    #[doc = "3: Configure as GPIO45 value."]
    GPIO45 = 3,
    #[doc = "4: I2S serial data output value."]
    I2SDAT = 4,
    #[doc = "5: PDM serial data input value."]
    PDMDATA = 5,
    #[doc = "6: Configure as the SPI channel 5 nCE signal from IOMSTR5 value."]
    UART0RX = 6,
    #[doc = "7: Configure as the serial wire debug SWO signal value."]
    SWO = 7,
}
impl From<PAD45FNCSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: PAD45FNCSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `PAD45FNCSEL` reader - Pad 45 function select"]
pub struct PAD45FNCSEL_R(crate::FieldReader<u8, PAD45FNCSEL_A>);
impl PAD45FNCSEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        PAD45FNCSEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAD45FNCSEL_A {
        match self.bits {
            0 => PAD45FNCSEL_A::UA1CTS,
            1 => PAD45FNCSEL_A::NCE45,
            2 => PAD45FNCSEL_A::CT22,
            3 => PAD45FNCSEL_A::GPIO45,
            4 => PAD45FNCSEL_A::I2SDAT,
            5 => PAD45FNCSEL_A::PDMDATA,
            6 => PAD45FNCSEL_A::UART0RX,
            7 => PAD45FNCSEL_A::SWO,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `UA1CTS`"]
    #[inline(always)]
    pub fn is_ua1cts(&self) -> bool {
        **self == PAD45FNCSEL_A::UA1CTS
    }
    #[doc = "Checks if the value of the field is `NCE45`"]
    #[inline(always)]
    pub fn is_nce45(&self) -> bool {
        **self == PAD45FNCSEL_A::NCE45
    }
    #[doc = "Checks if the value of the field is `CT22`"]
    #[inline(always)]
    pub fn is_ct22(&self) -> bool {
        **self == PAD45FNCSEL_A::CT22
    }
    #[doc = "Checks if the value of the field is `GPIO45`"]
    #[inline(always)]
    pub fn is_gpio45(&self) -> bool {
        **self == PAD45FNCSEL_A::GPIO45
    }
    #[doc = "Checks if the value of the field is `I2SDAT`"]
    #[inline(always)]
    pub fn is_i2sdat(&self) -> bool {
        **self == PAD45FNCSEL_A::I2SDAT
    }
    #[doc = "Checks if the value of the field is `PDMDATA`"]
    #[inline(always)]
    pub fn is_pdmdata(&self) -> bool {
        **self == PAD45FNCSEL_A::PDMDATA
    }
    #[doc = "Checks if the value of the field is `UART0RX`"]
    #[inline(always)]
    pub fn is_uart0rx(&self) -> bool {
        **self == PAD45FNCSEL_A::UART0RX
    }
    #[doc = "Checks if the value of the field is `SWO`"]
    #[inline(always)]
    pub fn is_swo(&self) -> bool {
        **self == PAD45FNCSEL_A::SWO
    }
}
impl core::ops::Deref for PAD45FNCSEL_R {
    type Target = crate::FieldReader<u8, PAD45FNCSEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PAD45FNCSEL` writer - Pad 45 function select"]
pub struct PAD45FNCSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD45FNCSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD45FNCSEL_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Configure as the UART1 CTS input signal value."]
    #[inline(always)]
    pub fn ua1cts(self) -> &'a mut W {
        self.variant(PAD45FNCSEL_A::UA1CTS)
    }
    #[doc = "IOM/MSPI nCE group 45 value."]
    #[inline(always)]
    pub fn nce45(self) -> &'a mut W {
        self.variant(PAD45FNCSEL_A::NCE45)
    }
    #[doc = "CTIMER connection 22 value."]
    #[inline(always)]
    pub fn ct22(self) -> &'a mut W {
        self.variant(PAD45FNCSEL_A::CT22)
    }
    #[doc = "Configure as GPIO45 value."]
    #[inline(always)]
    pub fn gpio45(self) -> &'a mut W {
        self.variant(PAD45FNCSEL_A::GPIO45)
    }
    #[doc = "I2S serial data output value."]
    #[inline(always)]
    pub fn i2sdat(self) -> &'a mut W {
        self.variant(PAD45FNCSEL_A::I2SDAT)
    }
    #[doc = "PDM serial data input value."]
    #[inline(always)]
    pub fn pdmdata(self) -> &'a mut W {
        self.variant(PAD45FNCSEL_A::PDMDATA)
    }
    #[doc = "Configure as the SPI channel 5 nCE signal from IOMSTR5 value."]
    #[inline(always)]
    pub fn uart0rx(self) -> &'a mut W {
        self.variant(PAD45FNCSEL_A::UART0RX)
    }
    #[doc = "Configure as the serial wire debug SWO signal value."]
    #[inline(always)]
    pub fn swo(self) -> &'a mut W {
        self.variant(PAD45FNCSEL_A::SWO)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 11)) | ((value as u32 & 0x07) << 11);
        self.w
    }
}
#[doc = "Pad 45 drive strength\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD45STRNG_A {
    #[doc = "0: Low drive strength value."]
    LOW = 0,
    #[doc = "1: High drive strength value."]
    HIGH = 1,
}
impl From<PAD45STRNG_A> for bool {
    #[inline(always)]
    fn from(variant: PAD45STRNG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PAD45STRNG` reader - Pad 45 drive strength"]
pub struct PAD45STRNG_R(crate::FieldReader<bool, PAD45STRNG_A>);
impl PAD45STRNG_R {
    pub(crate) fn new(bits: bool) -> Self {
        PAD45STRNG_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAD45STRNG_A {
        match self.bits {
            false => PAD45STRNG_A::LOW,
            true => PAD45STRNG_A::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        **self == PAD45STRNG_A::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        **self == PAD45STRNG_A::HIGH
    }
}
impl core::ops::Deref for PAD45STRNG_R {
    type Target = crate::FieldReader<bool, PAD45STRNG_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PAD45STRNG` writer - Pad 45 drive strength"]
pub struct PAD45STRNG_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD45STRNG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD45STRNG_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Low drive strength value."]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(PAD45STRNG_A::LOW)
    }
    #[doc = "High drive strength value."]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(PAD45STRNG_A::HIGH)
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | ((value as u32 & 0x01) << 10);
        self.w
    }
}
#[doc = "Pad 45 input enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD45INPEN_A {
    #[doc = "0: Pad input disabled value."]
    DIS = 0,
    #[doc = "1: Pad input enabled value."]
    EN = 1,
}
impl From<PAD45INPEN_A> for bool {
    #[inline(always)]
    fn from(variant: PAD45INPEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PAD45INPEN` reader - Pad 45 input enable"]
pub struct PAD45INPEN_R(crate::FieldReader<bool, PAD45INPEN_A>);
impl PAD45INPEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        PAD45INPEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAD45INPEN_A {
        match self.bits {
            false => PAD45INPEN_A::DIS,
            true => PAD45INPEN_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        **self == PAD45INPEN_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        **self == PAD45INPEN_A::EN
    }
}
impl core::ops::Deref for PAD45INPEN_R {
    type Target = crate::FieldReader<bool, PAD45INPEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PAD45INPEN` writer - Pad 45 input enable"]
pub struct PAD45INPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD45INPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD45INPEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Pad input disabled value."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(PAD45INPEN_A::DIS)
    }
    #[doc = "Pad input enabled value."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(PAD45INPEN_A::EN)
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | ((value as u32 & 0x01) << 9);
        self.w
    }
}
#[doc = "Pad 45 pullup enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD45PULL_A {
    #[doc = "0: Pullup disabled value."]
    DIS = 0,
    #[doc = "1: Pullup enabled value."]
    EN = 1,
}
impl From<PAD45PULL_A> for bool {
    #[inline(always)]
    fn from(variant: PAD45PULL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PAD45PULL` reader - Pad 45 pullup enable"]
pub struct PAD45PULL_R(crate::FieldReader<bool, PAD45PULL_A>);
impl PAD45PULL_R {
    pub(crate) fn new(bits: bool) -> Self {
        PAD45PULL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAD45PULL_A {
        match self.bits {
            false => PAD45PULL_A::DIS,
            true => PAD45PULL_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        **self == PAD45PULL_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        **self == PAD45PULL_A::EN
    }
}
impl core::ops::Deref for PAD45PULL_R {
    type Target = crate::FieldReader<bool, PAD45PULL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PAD45PULL` writer - Pad 45 pullup enable"]
pub struct PAD45PULL_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD45PULL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD45PULL_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Pullup disabled value."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(PAD45PULL_A::DIS)
    }
    #[doc = "Pullup enabled value."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(PAD45PULL_A::EN)
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | ((value as u32 & 0x01) << 8);
        self.w
    }
}
#[doc = "Pad 44 function select\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PAD44FNCSEL_A {
    #[doc = "0: Configure as the UART1 RTS output signal value."]
    UA1RTS = 0,
    #[doc = "1: IOM/MSPI nCE group 44 value."]
    NCE44 = 1,
    #[doc = "2: CTIMER connection 20 value."]
    CT20 = 2,
    #[doc = "3: Configure as GPIO44 value."]
    GPIO44 = 3,
    #[doc = "5: Configure as the IOMSTR4 SPI MOSI signal value."]
    M4MOSI = 5,
    #[doc = "6: Configure as the SPI channel 6 nCE signal from IOMSTR5 value."]
    M5NCE6 = 6,
}
impl From<PAD44FNCSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: PAD44FNCSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `PAD44FNCSEL` reader - Pad 44 function select"]
pub struct PAD44FNCSEL_R(crate::FieldReader<u8, PAD44FNCSEL_A>);
impl PAD44FNCSEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        PAD44FNCSEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PAD44FNCSEL_A> {
        match self.bits {
            0 => Some(PAD44FNCSEL_A::UA1RTS),
            1 => Some(PAD44FNCSEL_A::NCE44),
            2 => Some(PAD44FNCSEL_A::CT20),
            3 => Some(PAD44FNCSEL_A::GPIO44),
            5 => Some(PAD44FNCSEL_A::M4MOSI),
            6 => Some(PAD44FNCSEL_A::M5NCE6),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `UA1RTS`"]
    #[inline(always)]
    pub fn is_ua1rts(&self) -> bool {
        **self == PAD44FNCSEL_A::UA1RTS
    }
    #[doc = "Checks if the value of the field is `NCE44`"]
    #[inline(always)]
    pub fn is_nce44(&self) -> bool {
        **self == PAD44FNCSEL_A::NCE44
    }
    #[doc = "Checks if the value of the field is `CT20`"]
    #[inline(always)]
    pub fn is_ct20(&self) -> bool {
        **self == PAD44FNCSEL_A::CT20
    }
    #[doc = "Checks if the value of the field is `GPIO44`"]
    #[inline(always)]
    pub fn is_gpio44(&self) -> bool {
        **self == PAD44FNCSEL_A::GPIO44
    }
    #[doc = "Checks if the value of the field is `M4MOSI`"]
    #[inline(always)]
    pub fn is_m4mosi(&self) -> bool {
        **self == PAD44FNCSEL_A::M4MOSI
    }
    #[doc = "Checks if the value of the field is `M5NCE6`"]
    #[inline(always)]
    pub fn is_m5n_ce6(&self) -> bool {
        **self == PAD44FNCSEL_A::M5NCE6
    }
}
impl core::ops::Deref for PAD44FNCSEL_R {
    type Target = crate::FieldReader<u8, PAD44FNCSEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PAD44FNCSEL` writer - Pad 44 function select"]
pub struct PAD44FNCSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD44FNCSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD44FNCSEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Configure as the UART1 RTS output signal value."]
    #[inline(always)]
    pub fn ua1rts(self) -> &'a mut W {
        self.variant(PAD44FNCSEL_A::UA1RTS)
    }
    #[doc = "IOM/MSPI nCE group 44 value."]
    #[inline(always)]
    pub fn nce44(self) -> &'a mut W {
        self.variant(PAD44FNCSEL_A::NCE44)
    }
    #[doc = "CTIMER connection 20 value."]
    #[inline(always)]
    pub fn ct20(self) -> &'a mut W {
        self.variant(PAD44FNCSEL_A::CT20)
    }
    #[doc = "Configure as GPIO44 value."]
    #[inline(always)]
    pub fn gpio44(self) -> &'a mut W {
        self.variant(PAD44FNCSEL_A::GPIO44)
    }
    #[doc = "Configure as the IOMSTR4 SPI MOSI signal value."]
    #[inline(always)]
    pub fn m4mosi(self) -> &'a mut W {
        self.variant(PAD44FNCSEL_A::M4MOSI)
    }
    #[doc = "Configure as the SPI channel 6 nCE signal from IOMSTR5 value."]
    #[inline(always)]
    pub fn m5n_ce6(self) -> &'a mut W {
        self.variant(PAD44FNCSEL_A::M5NCE6)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 3)) | ((value as u32 & 0x07) << 3);
        self.w
    }
}
#[doc = "Pad 44 drive strength\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD44STRNG_A {
    #[doc = "0: Low drive strength value."]
    LOW = 0,
    #[doc = "1: High drive strength value."]
    HIGH = 1,
}
impl From<PAD44STRNG_A> for bool {
    #[inline(always)]
    fn from(variant: PAD44STRNG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PAD44STRNG` reader - Pad 44 drive strength"]
pub struct PAD44STRNG_R(crate::FieldReader<bool, PAD44STRNG_A>);
impl PAD44STRNG_R {
    pub(crate) fn new(bits: bool) -> Self {
        PAD44STRNG_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAD44STRNG_A {
        match self.bits {
            false => PAD44STRNG_A::LOW,
            true => PAD44STRNG_A::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        **self == PAD44STRNG_A::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        **self == PAD44STRNG_A::HIGH
    }
}
impl core::ops::Deref for PAD44STRNG_R {
    type Target = crate::FieldReader<bool, PAD44STRNG_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PAD44STRNG` writer - Pad 44 drive strength"]
pub struct PAD44STRNG_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD44STRNG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD44STRNG_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Low drive strength value."]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(PAD44STRNG_A::LOW)
    }
    #[doc = "High drive strength value."]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(PAD44STRNG_A::HIGH)
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
        self.w
    }
}
#[doc = "Pad 44 input enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD44INPEN_A {
    #[doc = "0: Pad input disabled value."]
    DIS = 0,
    #[doc = "1: Pad input enabled value."]
    EN = 1,
}
impl From<PAD44INPEN_A> for bool {
    #[inline(always)]
    fn from(variant: PAD44INPEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PAD44INPEN` reader - Pad 44 input enable"]
pub struct PAD44INPEN_R(crate::FieldReader<bool, PAD44INPEN_A>);
impl PAD44INPEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        PAD44INPEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAD44INPEN_A {
        match self.bits {
            false => PAD44INPEN_A::DIS,
            true => PAD44INPEN_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        **self == PAD44INPEN_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        **self == PAD44INPEN_A::EN
    }
}
impl core::ops::Deref for PAD44INPEN_R {
    type Target = crate::FieldReader<bool, PAD44INPEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PAD44INPEN` writer - Pad 44 input enable"]
pub struct PAD44INPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD44INPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD44INPEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Pad input disabled value."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(PAD44INPEN_A::DIS)
    }
    #[doc = "Pad input enabled value."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(PAD44INPEN_A::EN)
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
#[doc = "Pad 44 pullup enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD44PULL_A {
    #[doc = "0: Pullup disabled value."]
    DIS = 0,
    #[doc = "1: Pullup enabled value."]
    EN = 1,
}
impl From<PAD44PULL_A> for bool {
    #[inline(always)]
    fn from(variant: PAD44PULL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PAD44PULL` reader - Pad 44 pullup enable"]
pub struct PAD44PULL_R(crate::FieldReader<bool, PAD44PULL_A>);
impl PAD44PULL_R {
    pub(crate) fn new(bits: bool) -> Self {
        PAD44PULL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAD44PULL_A {
        match self.bits {
            false => PAD44PULL_A::DIS,
            true => PAD44PULL_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        **self == PAD44PULL_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        **self == PAD44PULL_A::EN
    }
}
impl core::ops::Deref for PAD44PULL_R {
    type Target = crate::FieldReader<bool, PAD44PULL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PAD44PULL` writer - Pad 44 pullup enable"]
pub struct PAD44PULL_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD44PULL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD44PULL_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Pullup disabled value."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(PAD44PULL_A::DIS)
    }
    #[doc = "Pullup enabled value."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(PAD44PULL_A::EN)
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
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
impl R {
    #[doc = "Bits 27:29 - Pad 47 function select"]
    #[inline(always)]
    pub fn pad47fncsel(&self) -> PAD47FNCSEL_R {
        PAD47FNCSEL_R::new(((self.bits >> 27) & 0x07) as u8)
    }
    #[doc = "Bit 26 - Pad 47 drive strength"]
    #[inline(always)]
    pub fn pad47strng(&self) -> PAD47STRNG_R {
        PAD47STRNG_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Pad 47 input enable"]
    #[inline(always)]
    pub fn pad47inpen(&self) -> PAD47INPEN_R {
        PAD47INPEN_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Pad 47 pullup enable"]
    #[inline(always)]
    pub fn pad47pull(&self) -> PAD47PULL_R {
        PAD47PULL_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bits 19:21 - Pad 46 function select"]
    #[inline(always)]
    pub fn pad46fncsel(&self) -> PAD46FNCSEL_R {
        PAD46FNCSEL_R::new(((self.bits >> 19) & 0x07) as u8)
    }
    #[doc = "Bit 18 - Pad 46 drive strength"]
    #[inline(always)]
    pub fn pad46strng(&self) -> PAD46STRNG_R {
        PAD46STRNG_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Pad 46 input enable"]
    #[inline(always)]
    pub fn pad46inpen(&self) -> PAD46INPEN_R {
        PAD46INPEN_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Pad 46 pullup enable"]
    #[inline(always)]
    pub fn pad46pull(&self) -> PAD46PULL_R {
        PAD46PULL_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bits 11:13 - Pad 45 function select"]
    #[inline(always)]
    pub fn pad45fncsel(&self) -> PAD45FNCSEL_R {
        PAD45FNCSEL_R::new(((self.bits >> 11) & 0x07) as u8)
    }
    #[doc = "Bit 10 - Pad 45 drive strength"]
    #[inline(always)]
    pub fn pad45strng(&self) -> PAD45STRNG_R {
        PAD45STRNG_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Pad 45 input enable"]
    #[inline(always)]
    pub fn pad45inpen(&self) -> PAD45INPEN_R {
        PAD45INPEN_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Pad 45 pullup enable"]
    #[inline(always)]
    pub fn pad45pull(&self) -> PAD45PULL_R {
        PAD45PULL_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bits 3:5 - Pad 44 function select"]
    #[inline(always)]
    pub fn pad44fncsel(&self) -> PAD44FNCSEL_R {
        PAD44FNCSEL_R::new(((self.bits >> 3) & 0x07) as u8)
    }
    #[doc = "Bit 2 - Pad 44 drive strength"]
    #[inline(always)]
    pub fn pad44strng(&self) -> PAD44STRNG_R {
        PAD44STRNG_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - Pad 44 input enable"]
    #[inline(always)]
    pub fn pad44inpen(&self) -> PAD44INPEN_R {
        PAD44INPEN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Pad 44 pullup enable"]
    #[inline(always)]
    pub fn pad44pull(&self) -> PAD44PULL_R {
        PAD44PULL_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 27:29 - Pad 47 function select"]
    #[inline(always)]
    pub fn pad47fncsel(&mut self) -> PAD47FNCSEL_W {
        PAD47FNCSEL_W { w: self }
    }
    #[doc = "Bit 26 - Pad 47 drive strength"]
    #[inline(always)]
    pub fn pad47strng(&mut self) -> PAD47STRNG_W {
        PAD47STRNG_W { w: self }
    }
    #[doc = "Bit 25 - Pad 47 input enable"]
    #[inline(always)]
    pub fn pad47inpen(&mut self) -> PAD47INPEN_W {
        PAD47INPEN_W { w: self }
    }
    #[doc = "Bit 24 - Pad 47 pullup enable"]
    #[inline(always)]
    pub fn pad47pull(&mut self) -> PAD47PULL_W {
        PAD47PULL_W { w: self }
    }
    #[doc = "Bits 19:21 - Pad 46 function select"]
    #[inline(always)]
    pub fn pad46fncsel(&mut self) -> PAD46FNCSEL_W {
        PAD46FNCSEL_W { w: self }
    }
    #[doc = "Bit 18 - Pad 46 drive strength"]
    #[inline(always)]
    pub fn pad46strng(&mut self) -> PAD46STRNG_W {
        PAD46STRNG_W { w: self }
    }
    #[doc = "Bit 17 - Pad 46 input enable"]
    #[inline(always)]
    pub fn pad46inpen(&mut self) -> PAD46INPEN_W {
        PAD46INPEN_W { w: self }
    }
    #[doc = "Bit 16 - Pad 46 pullup enable"]
    #[inline(always)]
    pub fn pad46pull(&mut self) -> PAD46PULL_W {
        PAD46PULL_W { w: self }
    }
    #[doc = "Bits 11:13 - Pad 45 function select"]
    #[inline(always)]
    pub fn pad45fncsel(&mut self) -> PAD45FNCSEL_W {
        PAD45FNCSEL_W { w: self }
    }
    #[doc = "Bit 10 - Pad 45 drive strength"]
    #[inline(always)]
    pub fn pad45strng(&mut self) -> PAD45STRNG_W {
        PAD45STRNG_W { w: self }
    }
    #[doc = "Bit 9 - Pad 45 input enable"]
    #[inline(always)]
    pub fn pad45inpen(&mut self) -> PAD45INPEN_W {
        PAD45INPEN_W { w: self }
    }
    #[doc = "Bit 8 - Pad 45 pullup enable"]
    #[inline(always)]
    pub fn pad45pull(&mut self) -> PAD45PULL_W {
        PAD45PULL_W { w: self }
    }
    #[doc = "Bits 3:5 - Pad 44 function select"]
    #[inline(always)]
    pub fn pad44fncsel(&mut self) -> PAD44FNCSEL_W {
        PAD44FNCSEL_W { w: self }
    }
    #[doc = "Bit 2 - Pad 44 drive strength"]
    #[inline(always)]
    pub fn pad44strng(&mut self) -> PAD44STRNG_W {
        PAD44STRNG_W { w: self }
    }
    #[doc = "Bit 1 - Pad 44 input enable"]
    #[inline(always)]
    pub fn pad44inpen(&mut self) -> PAD44INPEN_W {
        PAD44INPEN_W { w: self }
    }
    #[doc = "Bit 0 - Pad 44 pullup enable"]
    #[inline(always)]
    pub fn pad44pull(&mut self) -> PAD44PULL_W {
        PAD44PULL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Pad Configuration Register L (Pads 44-47)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [padregl](index.html) module"]
pub struct PADREGL_SPEC;
impl crate::RegisterSpec for PADREGL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [padregl::R](R) reader structure"]
impl crate::Readable for PADREGL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [padregl::W](W) writer structure"]
impl crate::Writable for PADREGL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PADREGL to value 0x1818_1818"]
impl crate::Resettable for PADREGL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x1818_1818
    }
}
