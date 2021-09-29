#[doc = "Register `PADREGC` reader"]
pub struct R(crate::R<PADREGC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PADREGC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PADREGC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PADREGC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PADREGC` writer"]
pub struct W(crate::W<PADREGC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PADREGC_SPEC>;
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
impl From<crate::W<PADREGC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PADREGC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Pad 11 function select\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PAD11FNCSEL_A {
    #[doc = "0: Configure as the analog input for ADC single ended input 2 value."]
    ADCSE2 = 0,
    #[doc = "1: IOM/MSPI nCE group 11 value."]
    NCE11 = 1,
    #[doc = "2: CTIMER connection 31 value."]
    CT31 = 2,
    #[doc = "3: Configure as GPIO11 value."]
    GPIO11 = 3,
    #[doc = "4: Configure as the IOSLAVE interrupt out signal value."]
    SLINT = 4,
    #[doc = "5: Configure as the UART1 CTS input signal value."]
    UA1CTS = 5,
    #[doc = "6: Configure as the UART0 RX input signal value."]
    UART0RX = 6,
    #[doc = "7: Configure as the PDM Data input signal value."]
    PDM_DATA = 7,
}
impl From<PAD11FNCSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: PAD11FNCSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `PAD11FNCSEL` reader - Pad 11 function select"]
pub struct PAD11FNCSEL_R(crate::FieldReader<u8, PAD11FNCSEL_A>);
impl PAD11FNCSEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        PAD11FNCSEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAD11FNCSEL_A {
        match self.bits {
            0 => PAD11FNCSEL_A::ADCSE2,
            1 => PAD11FNCSEL_A::NCE11,
            2 => PAD11FNCSEL_A::CT31,
            3 => PAD11FNCSEL_A::GPIO11,
            4 => PAD11FNCSEL_A::SLINT,
            5 => PAD11FNCSEL_A::UA1CTS,
            6 => PAD11FNCSEL_A::UART0RX,
            7 => PAD11FNCSEL_A::PDM_DATA,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ADCSE2`"]
    #[inline(always)]
    pub fn is_adcse2(&self) -> bool {
        **self == PAD11FNCSEL_A::ADCSE2
    }
    #[doc = "Checks if the value of the field is `NCE11`"]
    #[inline(always)]
    pub fn is_nce11(&self) -> bool {
        **self == PAD11FNCSEL_A::NCE11
    }
    #[doc = "Checks if the value of the field is `CT31`"]
    #[inline(always)]
    pub fn is_ct31(&self) -> bool {
        **self == PAD11FNCSEL_A::CT31
    }
    #[doc = "Checks if the value of the field is `GPIO11`"]
    #[inline(always)]
    pub fn is_gpio11(&self) -> bool {
        **self == PAD11FNCSEL_A::GPIO11
    }
    #[doc = "Checks if the value of the field is `SLINT`"]
    #[inline(always)]
    pub fn is_slint(&self) -> bool {
        **self == PAD11FNCSEL_A::SLINT
    }
    #[doc = "Checks if the value of the field is `UA1CTS`"]
    #[inline(always)]
    pub fn is_ua1cts(&self) -> bool {
        **self == PAD11FNCSEL_A::UA1CTS
    }
    #[doc = "Checks if the value of the field is `UART0RX`"]
    #[inline(always)]
    pub fn is_uart0rx(&self) -> bool {
        **self == PAD11FNCSEL_A::UART0RX
    }
    #[doc = "Checks if the value of the field is `PDM_DATA`"]
    #[inline(always)]
    pub fn is_pdm_data(&self) -> bool {
        **self == PAD11FNCSEL_A::PDM_DATA
    }
}
impl core::ops::Deref for PAD11FNCSEL_R {
    type Target = crate::FieldReader<u8, PAD11FNCSEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PAD11FNCSEL` writer - Pad 11 function select"]
pub struct PAD11FNCSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD11FNCSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD11FNCSEL_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Configure as the analog input for ADC single ended input 2 value."]
    #[inline(always)]
    pub fn adcse2(self) -> &'a mut W {
        self.variant(PAD11FNCSEL_A::ADCSE2)
    }
    #[doc = "IOM/MSPI nCE group 11 value."]
    #[inline(always)]
    pub fn nce11(self) -> &'a mut W {
        self.variant(PAD11FNCSEL_A::NCE11)
    }
    #[doc = "CTIMER connection 31 value."]
    #[inline(always)]
    pub fn ct31(self) -> &'a mut W {
        self.variant(PAD11FNCSEL_A::CT31)
    }
    #[doc = "Configure as GPIO11 value."]
    #[inline(always)]
    pub fn gpio11(self) -> &'a mut W {
        self.variant(PAD11FNCSEL_A::GPIO11)
    }
    #[doc = "Configure as the IOSLAVE interrupt out signal value."]
    #[inline(always)]
    pub fn slint(self) -> &'a mut W {
        self.variant(PAD11FNCSEL_A::SLINT)
    }
    #[doc = "Configure as the UART1 CTS input signal value."]
    #[inline(always)]
    pub fn ua1cts(self) -> &'a mut W {
        self.variant(PAD11FNCSEL_A::UA1CTS)
    }
    #[doc = "Configure as the UART0 RX input signal value."]
    #[inline(always)]
    pub fn uart0rx(self) -> &'a mut W {
        self.variant(PAD11FNCSEL_A::UART0RX)
    }
    #[doc = "Configure as the PDM Data input signal value."]
    #[inline(always)]
    pub fn pdm_data(self) -> &'a mut W {
        self.variant(PAD11FNCSEL_A::PDM_DATA)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 27)) | ((value as u32 & 0x07) << 27);
        self.w
    }
}
#[doc = "Pad 11 drive strength\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD11STRNG_A {
    #[doc = "0: Low drive strength value."]
    LOW = 0,
    #[doc = "1: High drive strength value."]
    HIGH = 1,
}
impl From<PAD11STRNG_A> for bool {
    #[inline(always)]
    fn from(variant: PAD11STRNG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PAD11STRNG` reader - Pad 11 drive strength"]
pub struct PAD11STRNG_R(crate::FieldReader<bool, PAD11STRNG_A>);
impl PAD11STRNG_R {
    pub(crate) fn new(bits: bool) -> Self {
        PAD11STRNG_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAD11STRNG_A {
        match self.bits {
            false => PAD11STRNG_A::LOW,
            true => PAD11STRNG_A::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        **self == PAD11STRNG_A::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        **self == PAD11STRNG_A::HIGH
    }
}
impl core::ops::Deref for PAD11STRNG_R {
    type Target = crate::FieldReader<bool, PAD11STRNG_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PAD11STRNG` writer - Pad 11 drive strength"]
pub struct PAD11STRNG_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD11STRNG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD11STRNG_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Low drive strength value."]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(PAD11STRNG_A::LOW)
    }
    #[doc = "High drive strength value."]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(PAD11STRNG_A::HIGH)
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
#[doc = "Pad 11 input enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD11INPEN_A {
    #[doc = "0: Pad input disabled value."]
    DIS = 0,
    #[doc = "1: Pad input enabled value."]
    EN = 1,
}
impl From<PAD11INPEN_A> for bool {
    #[inline(always)]
    fn from(variant: PAD11INPEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PAD11INPEN` reader - Pad 11 input enable"]
pub struct PAD11INPEN_R(crate::FieldReader<bool, PAD11INPEN_A>);
impl PAD11INPEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        PAD11INPEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAD11INPEN_A {
        match self.bits {
            false => PAD11INPEN_A::DIS,
            true => PAD11INPEN_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        **self == PAD11INPEN_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        **self == PAD11INPEN_A::EN
    }
}
impl core::ops::Deref for PAD11INPEN_R {
    type Target = crate::FieldReader<bool, PAD11INPEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PAD11INPEN` writer - Pad 11 input enable"]
pub struct PAD11INPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD11INPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD11INPEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Pad input disabled value."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(PAD11INPEN_A::DIS)
    }
    #[doc = "Pad input enabled value."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(PAD11INPEN_A::EN)
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
#[doc = "Pad 11 pullup enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD11PULL_A {
    #[doc = "0: Pullup disabled value."]
    DIS = 0,
    #[doc = "1: Pullup enabled value."]
    EN = 1,
}
impl From<PAD11PULL_A> for bool {
    #[inline(always)]
    fn from(variant: PAD11PULL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PAD11PULL` reader - Pad 11 pullup enable"]
pub struct PAD11PULL_R(crate::FieldReader<bool, PAD11PULL_A>);
impl PAD11PULL_R {
    pub(crate) fn new(bits: bool) -> Self {
        PAD11PULL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAD11PULL_A {
        match self.bits {
            false => PAD11PULL_A::DIS,
            true => PAD11PULL_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        **self == PAD11PULL_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        **self == PAD11PULL_A::EN
    }
}
impl core::ops::Deref for PAD11PULL_R {
    type Target = crate::FieldReader<bool, PAD11PULL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PAD11PULL` writer - Pad 11 pullup enable"]
pub struct PAD11PULL_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD11PULL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD11PULL_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Pullup disabled value."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(PAD11PULL_A::DIS)
    }
    #[doc = "Pullup enabled value."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(PAD11PULL_A::EN)
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
#[doc = "Pad 10 function select\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PAD10FNCSEL_A {
    #[doc = "1: Configure as the IOMSTR1 SPI MOSI signal value."]
    M1MOSI = 1,
    #[doc = "2: IOM/MSPI nCE group 10 value."]
    NCE10 = 2,
    #[doc = "3: Configure as GPIO10 value."]
    GPIO10 = 3,
    #[doc = "4: PDM serial clock out value."]
    PDMCLK = 4,
    #[doc = "5: Configure as the UART1 RTS output signal value."]
    UA1RTS = 5,
}
impl From<PAD10FNCSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: PAD10FNCSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `PAD10FNCSEL` reader - Pad 10 function select"]
pub struct PAD10FNCSEL_R(crate::FieldReader<u8, PAD10FNCSEL_A>);
impl PAD10FNCSEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        PAD10FNCSEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PAD10FNCSEL_A> {
        match self.bits {
            1 => Some(PAD10FNCSEL_A::M1MOSI),
            2 => Some(PAD10FNCSEL_A::NCE10),
            3 => Some(PAD10FNCSEL_A::GPIO10),
            4 => Some(PAD10FNCSEL_A::PDMCLK),
            5 => Some(PAD10FNCSEL_A::UA1RTS),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `M1MOSI`"]
    #[inline(always)]
    pub fn is_m1mosi(&self) -> bool {
        **self == PAD10FNCSEL_A::M1MOSI
    }
    #[doc = "Checks if the value of the field is `NCE10`"]
    #[inline(always)]
    pub fn is_nce10(&self) -> bool {
        **self == PAD10FNCSEL_A::NCE10
    }
    #[doc = "Checks if the value of the field is `GPIO10`"]
    #[inline(always)]
    pub fn is_gpio10(&self) -> bool {
        **self == PAD10FNCSEL_A::GPIO10
    }
    #[doc = "Checks if the value of the field is `PDMCLK`"]
    #[inline(always)]
    pub fn is_pdmclk(&self) -> bool {
        **self == PAD10FNCSEL_A::PDMCLK
    }
    #[doc = "Checks if the value of the field is `UA1RTS`"]
    #[inline(always)]
    pub fn is_ua1rts(&self) -> bool {
        **self == PAD10FNCSEL_A::UA1RTS
    }
}
impl core::ops::Deref for PAD10FNCSEL_R {
    type Target = crate::FieldReader<u8, PAD10FNCSEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PAD10FNCSEL` writer - Pad 10 function select"]
pub struct PAD10FNCSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD10FNCSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD10FNCSEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Configure as the IOMSTR1 SPI MOSI signal value."]
    #[inline(always)]
    pub fn m1mosi(self) -> &'a mut W {
        self.variant(PAD10FNCSEL_A::M1MOSI)
    }
    #[doc = "IOM/MSPI nCE group 10 value."]
    #[inline(always)]
    pub fn nce10(self) -> &'a mut W {
        self.variant(PAD10FNCSEL_A::NCE10)
    }
    #[doc = "Configure as GPIO10 value."]
    #[inline(always)]
    pub fn gpio10(self) -> &'a mut W {
        self.variant(PAD10FNCSEL_A::GPIO10)
    }
    #[doc = "PDM serial clock out value."]
    #[inline(always)]
    pub fn pdmclk(self) -> &'a mut W {
        self.variant(PAD10FNCSEL_A::PDMCLK)
    }
    #[doc = "Configure as the UART1 RTS output signal value."]
    #[inline(always)]
    pub fn ua1rts(self) -> &'a mut W {
        self.variant(PAD10FNCSEL_A::UA1RTS)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 19)) | ((value as u32 & 0x07) << 19);
        self.w
    }
}
#[doc = "Pad 10 drive strength\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD10STRNG_A {
    #[doc = "0: Low drive strength value."]
    LOW = 0,
    #[doc = "1: High drive strength value."]
    HIGH = 1,
}
impl From<PAD10STRNG_A> for bool {
    #[inline(always)]
    fn from(variant: PAD10STRNG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PAD10STRNG` reader - Pad 10 drive strength"]
pub struct PAD10STRNG_R(crate::FieldReader<bool, PAD10STRNG_A>);
impl PAD10STRNG_R {
    pub(crate) fn new(bits: bool) -> Self {
        PAD10STRNG_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAD10STRNG_A {
        match self.bits {
            false => PAD10STRNG_A::LOW,
            true => PAD10STRNG_A::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        **self == PAD10STRNG_A::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        **self == PAD10STRNG_A::HIGH
    }
}
impl core::ops::Deref for PAD10STRNG_R {
    type Target = crate::FieldReader<bool, PAD10STRNG_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PAD10STRNG` writer - Pad 10 drive strength"]
pub struct PAD10STRNG_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD10STRNG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD10STRNG_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Low drive strength value."]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(PAD10STRNG_A::LOW)
    }
    #[doc = "High drive strength value."]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(PAD10STRNG_A::HIGH)
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
#[doc = "Pad 10 input enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD10INPEN_A {
    #[doc = "0: Pad input disabled value."]
    DIS = 0,
    #[doc = "1: Pad input enabled value."]
    EN = 1,
}
impl From<PAD10INPEN_A> for bool {
    #[inline(always)]
    fn from(variant: PAD10INPEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PAD10INPEN` reader - Pad 10 input enable"]
pub struct PAD10INPEN_R(crate::FieldReader<bool, PAD10INPEN_A>);
impl PAD10INPEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        PAD10INPEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAD10INPEN_A {
        match self.bits {
            false => PAD10INPEN_A::DIS,
            true => PAD10INPEN_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        **self == PAD10INPEN_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        **self == PAD10INPEN_A::EN
    }
}
impl core::ops::Deref for PAD10INPEN_R {
    type Target = crate::FieldReader<bool, PAD10INPEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PAD10INPEN` writer - Pad 10 input enable"]
pub struct PAD10INPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD10INPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD10INPEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Pad input disabled value."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(PAD10INPEN_A::DIS)
    }
    #[doc = "Pad input enabled value."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(PAD10INPEN_A::EN)
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
#[doc = "Pad 10 pullup enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD10PULL_A {
    #[doc = "0: Pullup disabled value."]
    DIS = 0,
    #[doc = "1: Pullup enabled value."]
    EN = 1,
}
impl From<PAD10PULL_A> for bool {
    #[inline(always)]
    fn from(variant: PAD10PULL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PAD10PULL` reader - Pad 10 pullup enable"]
pub struct PAD10PULL_R(crate::FieldReader<bool, PAD10PULL_A>);
impl PAD10PULL_R {
    pub(crate) fn new(bits: bool) -> Self {
        PAD10PULL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAD10PULL_A {
        match self.bits {
            false => PAD10PULL_A::DIS,
            true => PAD10PULL_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        **self == PAD10PULL_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        **self == PAD10PULL_A::EN
    }
}
impl core::ops::Deref for PAD10PULL_R {
    type Target = crate::FieldReader<bool, PAD10PULL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PAD10PULL` writer - Pad 10 pullup enable"]
pub struct PAD10PULL_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD10PULL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD10PULL_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Pullup disabled value."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(PAD10PULL_A::DIS)
    }
    #[doc = "Pullup enabled value."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(PAD10PULL_A::EN)
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
#[doc = "Pad 9 pullup resistor selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PAD9RSEL_A {
    #[doc = "0: Pullup is ~1.5 KOhms value."]
    PULL1_5K = 0,
    #[doc = "1: Pullup is ~6 KOhms value."]
    PULL6K = 1,
    #[doc = "2: Pullup is ~12 KOhms value."]
    PULL12K = 2,
    #[doc = "3: Pullup is ~24 KOhms value."]
    PULL24K = 3,
}
impl From<PAD9RSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: PAD9RSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `PAD9RSEL` reader - Pad 9 pullup resistor selection"]
pub struct PAD9RSEL_R(crate::FieldReader<u8, PAD9RSEL_A>);
impl PAD9RSEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        PAD9RSEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAD9RSEL_A {
        match self.bits {
            0 => PAD9RSEL_A::PULL1_5K,
            1 => PAD9RSEL_A::PULL6K,
            2 => PAD9RSEL_A::PULL12K,
            3 => PAD9RSEL_A::PULL24K,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PULL1_5K`"]
    #[inline(always)]
    pub fn is_pull1_5k(&self) -> bool {
        **self == PAD9RSEL_A::PULL1_5K
    }
    #[doc = "Checks if the value of the field is `PULL6K`"]
    #[inline(always)]
    pub fn is_pull6k(&self) -> bool {
        **self == PAD9RSEL_A::PULL6K
    }
    #[doc = "Checks if the value of the field is `PULL12K`"]
    #[inline(always)]
    pub fn is_pull12k(&self) -> bool {
        **self == PAD9RSEL_A::PULL12K
    }
    #[doc = "Checks if the value of the field is `PULL24K`"]
    #[inline(always)]
    pub fn is_pull24k(&self) -> bool {
        **self == PAD9RSEL_A::PULL24K
    }
}
impl core::ops::Deref for PAD9RSEL_R {
    type Target = crate::FieldReader<u8, PAD9RSEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PAD9RSEL` writer - Pad 9 pullup resistor selection"]
pub struct PAD9RSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD9RSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD9RSEL_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Pullup is ~1.5 KOhms value."]
    #[inline(always)]
    pub fn pull1_5k(self) -> &'a mut W {
        self.variant(PAD9RSEL_A::PULL1_5K)
    }
    #[doc = "Pullup is ~6 KOhms value."]
    #[inline(always)]
    pub fn pull6k(self) -> &'a mut W {
        self.variant(PAD9RSEL_A::PULL6K)
    }
    #[doc = "Pullup is ~12 KOhms value."]
    #[inline(always)]
    pub fn pull12k(self) -> &'a mut W {
        self.variant(PAD9RSEL_A::PULL12K)
    }
    #[doc = "Pullup is ~24 KOhms value."]
    #[inline(always)]
    pub fn pull24k(self) -> &'a mut W {
        self.variant(PAD9RSEL_A::PULL24K)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 14)) | ((value as u32 & 0x03) << 14);
        self.w
    }
}
#[doc = "Pad 9 function select\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PAD9FNCSEL_A {
    #[doc = "0: Configure as the IOMSTR1 I2C SDA or SPI WIR3 signal value."]
    M1SDAWIR3 = 0,
    #[doc = "1: Configure as the IOMSTR1 SPI MISO signal value."]
    M1MISO = 1,
    #[doc = "2: IOM/MSPI nCE group 9 value."]
    NCE9 = 2,
    #[doc = "3: Configure as GPIO9 value."]
    GPIO9 = 3,
    #[doc = "4: SCARD data I/O connection value."]
    SCCIO = 4,
    #[doc = "6: Configure as UART1 RX input signal value."]
    UART1RX = 6,
}
impl From<PAD9FNCSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: PAD9FNCSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `PAD9FNCSEL` reader - Pad 9 function select"]
pub struct PAD9FNCSEL_R(crate::FieldReader<u8, PAD9FNCSEL_A>);
impl PAD9FNCSEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        PAD9FNCSEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PAD9FNCSEL_A> {
        match self.bits {
            0 => Some(PAD9FNCSEL_A::M1SDAWIR3),
            1 => Some(PAD9FNCSEL_A::M1MISO),
            2 => Some(PAD9FNCSEL_A::NCE9),
            3 => Some(PAD9FNCSEL_A::GPIO9),
            4 => Some(PAD9FNCSEL_A::SCCIO),
            6 => Some(PAD9FNCSEL_A::UART1RX),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `M1SDAWIR3`"]
    #[inline(always)]
    pub fn is_m1sdawir3(&self) -> bool {
        **self == PAD9FNCSEL_A::M1SDAWIR3
    }
    #[doc = "Checks if the value of the field is `M1MISO`"]
    #[inline(always)]
    pub fn is_m1miso(&self) -> bool {
        **self == PAD9FNCSEL_A::M1MISO
    }
    #[doc = "Checks if the value of the field is `NCE9`"]
    #[inline(always)]
    pub fn is_nce9(&self) -> bool {
        **self == PAD9FNCSEL_A::NCE9
    }
    #[doc = "Checks if the value of the field is `GPIO9`"]
    #[inline(always)]
    pub fn is_gpio9(&self) -> bool {
        **self == PAD9FNCSEL_A::GPIO9
    }
    #[doc = "Checks if the value of the field is `SCCIO`"]
    #[inline(always)]
    pub fn is_sccio(&self) -> bool {
        **self == PAD9FNCSEL_A::SCCIO
    }
    #[doc = "Checks if the value of the field is `UART1RX`"]
    #[inline(always)]
    pub fn is_uart1rx(&self) -> bool {
        **self == PAD9FNCSEL_A::UART1RX
    }
}
impl core::ops::Deref for PAD9FNCSEL_R {
    type Target = crate::FieldReader<u8, PAD9FNCSEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PAD9FNCSEL` writer - Pad 9 function select"]
pub struct PAD9FNCSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD9FNCSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD9FNCSEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Configure as the IOMSTR1 I2C SDA or SPI WIR3 signal value."]
    #[inline(always)]
    pub fn m1sdawir3(self) -> &'a mut W {
        self.variant(PAD9FNCSEL_A::M1SDAWIR3)
    }
    #[doc = "Configure as the IOMSTR1 SPI MISO signal value."]
    #[inline(always)]
    pub fn m1miso(self) -> &'a mut W {
        self.variant(PAD9FNCSEL_A::M1MISO)
    }
    #[doc = "IOM/MSPI nCE group 9 value."]
    #[inline(always)]
    pub fn nce9(self) -> &'a mut W {
        self.variant(PAD9FNCSEL_A::NCE9)
    }
    #[doc = "Configure as GPIO9 value."]
    #[inline(always)]
    pub fn gpio9(self) -> &'a mut W {
        self.variant(PAD9FNCSEL_A::GPIO9)
    }
    #[doc = "SCARD data I/O connection value."]
    #[inline(always)]
    pub fn sccio(self) -> &'a mut W {
        self.variant(PAD9FNCSEL_A::SCCIO)
    }
    #[doc = "Configure as UART1 RX input signal value."]
    #[inline(always)]
    pub fn uart1rx(self) -> &'a mut W {
        self.variant(PAD9FNCSEL_A::UART1RX)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 11)) | ((value as u32 & 0x07) << 11);
        self.w
    }
}
#[doc = "Pad 9 drive strength\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD9STRNG_A {
    #[doc = "0: Low drive strength value."]
    LOW = 0,
    #[doc = "1: High drive strength value."]
    HIGH = 1,
}
impl From<PAD9STRNG_A> for bool {
    #[inline(always)]
    fn from(variant: PAD9STRNG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PAD9STRNG` reader - Pad 9 drive strength"]
pub struct PAD9STRNG_R(crate::FieldReader<bool, PAD9STRNG_A>);
impl PAD9STRNG_R {
    pub(crate) fn new(bits: bool) -> Self {
        PAD9STRNG_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAD9STRNG_A {
        match self.bits {
            false => PAD9STRNG_A::LOW,
            true => PAD9STRNG_A::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        **self == PAD9STRNG_A::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        **self == PAD9STRNG_A::HIGH
    }
}
impl core::ops::Deref for PAD9STRNG_R {
    type Target = crate::FieldReader<bool, PAD9STRNG_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PAD9STRNG` writer - Pad 9 drive strength"]
pub struct PAD9STRNG_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD9STRNG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD9STRNG_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Low drive strength value."]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(PAD9STRNG_A::LOW)
    }
    #[doc = "High drive strength value."]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(PAD9STRNG_A::HIGH)
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
#[doc = "Pad 9 input enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD9INPEN_A {
    #[doc = "0: Pad input disabled value."]
    DIS = 0,
    #[doc = "1: Pad input enabled value."]
    EN = 1,
}
impl From<PAD9INPEN_A> for bool {
    #[inline(always)]
    fn from(variant: PAD9INPEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PAD9INPEN` reader - Pad 9 input enable"]
pub struct PAD9INPEN_R(crate::FieldReader<bool, PAD9INPEN_A>);
impl PAD9INPEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        PAD9INPEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAD9INPEN_A {
        match self.bits {
            false => PAD9INPEN_A::DIS,
            true => PAD9INPEN_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        **self == PAD9INPEN_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        **self == PAD9INPEN_A::EN
    }
}
impl core::ops::Deref for PAD9INPEN_R {
    type Target = crate::FieldReader<bool, PAD9INPEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PAD9INPEN` writer - Pad 9 input enable"]
pub struct PAD9INPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD9INPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD9INPEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Pad input disabled value."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(PAD9INPEN_A::DIS)
    }
    #[doc = "Pad input enabled value."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(PAD9INPEN_A::EN)
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
#[doc = "Pad 9 pullup enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD9PULL_A {
    #[doc = "0: Pullup disabled value."]
    DIS = 0,
    #[doc = "1: Pullup enabled value."]
    EN = 1,
}
impl From<PAD9PULL_A> for bool {
    #[inline(always)]
    fn from(variant: PAD9PULL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PAD9PULL` reader - Pad 9 pullup enable"]
pub struct PAD9PULL_R(crate::FieldReader<bool, PAD9PULL_A>);
impl PAD9PULL_R {
    pub(crate) fn new(bits: bool) -> Self {
        PAD9PULL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAD9PULL_A {
        match self.bits {
            false => PAD9PULL_A::DIS,
            true => PAD9PULL_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        **self == PAD9PULL_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        **self == PAD9PULL_A::EN
    }
}
impl core::ops::Deref for PAD9PULL_R {
    type Target = crate::FieldReader<bool, PAD9PULL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PAD9PULL` writer - Pad 9 pullup enable"]
pub struct PAD9PULL_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD9PULL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD9PULL_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Pullup disabled value."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(PAD9PULL_A::DIS)
    }
    #[doc = "Pullup enabled value."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(PAD9PULL_A::EN)
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
#[doc = "Pad 8 pullup resistor selection.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PAD8RSEL_A {
    #[doc = "0: Pullup is ~1.5 KOhms value."]
    PULL1_5K = 0,
    #[doc = "1: Pullup is ~6 KOhms value."]
    PULL6K = 1,
    #[doc = "2: Pullup is ~12 KOhms value."]
    PULL12K = 2,
    #[doc = "3: Pullup is ~24 KOhms value."]
    PULL24K = 3,
}
impl From<PAD8RSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: PAD8RSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `PAD8RSEL` reader - Pad 8 pullup resistor selection."]
pub struct PAD8RSEL_R(crate::FieldReader<u8, PAD8RSEL_A>);
impl PAD8RSEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        PAD8RSEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAD8RSEL_A {
        match self.bits {
            0 => PAD8RSEL_A::PULL1_5K,
            1 => PAD8RSEL_A::PULL6K,
            2 => PAD8RSEL_A::PULL12K,
            3 => PAD8RSEL_A::PULL24K,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PULL1_5K`"]
    #[inline(always)]
    pub fn is_pull1_5k(&self) -> bool {
        **self == PAD8RSEL_A::PULL1_5K
    }
    #[doc = "Checks if the value of the field is `PULL6K`"]
    #[inline(always)]
    pub fn is_pull6k(&self) -> bool {
        **self == PAD8RSEL_A::PULL6K
    }
    #[doc = "Checks if the value of the field is `PULL12K`"]
    #[inline(always)]
    pub fn is_pull12k(&self) -> bool {
        **self == PAD8RSEL_A::PULL12K
    }
    #[doc = "Checks if the value of the field is `PULL24K`"]
    #[inline(always)]
    pub fn is_pull24k(&self) -> bool {
        **self == PAD8RSEL_A::PULL24K
    }
}
impl core::ops::Deref for PAD8RSEL_R {
    type Target = crate::FieldReader<u8, PAD8RSEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PAD8RSEL` writer - Pad 8 pullup resistor selection."]
pub struct PAD8RSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD8RSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD8RSEL_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Pullup is ~1.5 KOhms value."]
    #[inline(always)]
    pub fn pull1_5k(self) -> &'a mut W {
        self.variant(PAD8RSEL_A::PULL1_5K)
    }
    #[doc = "Pullup is ~6 KOhms value."]
    #[inline(always)]
    pub fn pull6k(self) -> &'a mut W {
        self.variant(PAD8RSEL_A::PULL6K)
    }
    #[doc = "Pullup is ~12 KOhms value."]
    #[inline(always)]
    pub fn pull12k(self) -> &'a mut W {
        self.variant(PAD8RSEL_A::PULL12K)
    }
    #[doc = "Pullup is ~24 KOhms value."]
    #[inline(always)]
    pub fn pull24k(self) -> &'a mut W {
        self.variant(PAD8RSEL_A::PULL24K)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | ((value as u32 & 0x03) << 6);
        self.w
    }
}
#[doc = "Pad 8 function select\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PAD8FNCSEL_A {
    #[doc = "0: Configure as the IOMSTR1 I2C SCL signal value."]
    M1SCL = 0,
    #[doc = "1: Configure as the IOMSTR1 SPI SCK signal value."]
    M1SCK = 1,
    #[doc = "2: IOM/MSPI nCE group 8 value."]
    NCE8 = 2,
    #[doc = "3: Configure as GPIO8 value."]
    GPIO8 = 3,
    #[doc = "4: SCARD serial clock output value."]
    SCCLK = 4,
    #[doc = "6: Configure as the UART1 TX output signal value."]
    UART1TX = 6,
}
impl From<PAD8FNCSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: PAD8FNCSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `PAD8FNCSEL` reader - Pad 8 function select"]
pub struct PAD8FNCSEL_R(crate::FieldReader<u8, PAD8FNCSEL_A>);
impl PAD8FNCSEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        PAD8FNCSEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PAD8FNCSEL_A> {
        match self.bits {
            0 => Some(PAD8FNCSEL_A::M1SCL),
            1 => Some(PAD8FNCSEL_A::M1SCK),
            2 => Some(PAD8FNCSEL_A::NCE8),
            3 => Some(PAD8FNCSEL_A::GPIO8),
            4 => Some(PAD8FNCSEL_A::SCCLK),
            6 => Some(PAD8FNCSEL_A::UART1TX),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `M1SCL`"]
    #[inline(always)]
    pub fn is_m1scl(&self) -> bool {
        **self == PAD8FNCSEL_A::M1SCL
    }
    #[doc = "Checks if the value of the field is `M1SCK`"]
    #[inline(always)]
    pub fn is_m1sck(&self) -> bool {
        **self == PAD8FNCSEL_A::M1SCK
    }
    #[doc = "Checks if the value of the field is `NCE8`"]
    #[inline(always)]
    pub fn is_nce8(&self) -> bool {
        **self == PAD8FNCSEL_A::NCE8
    }
    #[doc = "Checks if the value of the field is `GPIO8`"]
    #[inline(always)]
    pub fn is_gpio8(&self) -> bool {
        **self == PAD8FNCSEL_A::GPIO8
    }
    #[doc = "Checks if the value of the field is `SCCLK`"]
    #[inline(always)]
    pub fn is_scclk(&self) -> bool {
        **self == PAD8FNCSEL_A::SCCLK
    }
    #[doc = "Checks if the value of the field is `UART1TX`"]
    #[inline(always)]
    pub fn is_uart1tx(&self) -> bool {
        **self == PAD8FNCSEL_A::UART1TX
    }
}
impl core::ops::Deref for PAD8FNCSEL_R {
    type Target = crate::FieldReader<u8, PAD8FNCSEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PAD8FNCSEL` writer - Pad 8 function select"]
pub struct PAD8FNCSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD8FNCSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD8FNCSEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Configure as the IOMSTR1 I2C SCL signal value."]
    #[inline(always)]
    pub fn m1scl(self) -> &'a mut W {
        self.variant(PAD8FNCSEL_A::M1SCL)
    }
    #[doc = "Configure as the IOMSTR1 SPI SCK signal value."]
    #[inline(always)]
    pub fn m1sck(self) -> &'a mut W {
        self.variant(PAD8FNCSEL_A::M1SCK)
    }
    #[doc = "IOM/MSPI nCE group 8 value."]
    #[inline(always)]
    pub fn nce8(self) -> &'a mut W {
        self.variant(PAD8FNCSEL_A::NCE8)
    }
    #[doc = "Configure as GPIO8 value."]
    #[inline(always)]
    pub fn gpio8(self) -> &'a mut W {
        self.variant(PAD8FNCSEL_A::GPIO8)
    }
    #[doc = "SCARD serial clock output value."]
    #[inline(always)]
    pub fn scclk(self) -> &'a mut W {
        self.variant(PAD8FNCSEL_A::SCCLK)
    }
    #[doc = "Configure as the UART1 TX output signal value."]
    #[inline(always)]
    pub fn uart1tx(self) -> &'a mut W {
        self.variant(PAD8FNCSEL_A::UART1TX)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 3)) | ((value as u32 & 0x07) << 3);
        self.w
    }
}
#[doc = "Pad 8 drive strength\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD8STRNG_A {
    #[doc = "0: Low drive strength value."]
    LOW = 0,
    #[doc = "1: High drive strength value."]
    HIGH = 1,
}
impl From<PAD8STRNG_A> for bool {
    #[inline(always)]
    fn from(variant: PAD8STRNG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PAD8STRNG` reader - Pad 8 drive strength"]
pub struct PAD8STRNG_R(crate::FieldReader<bool, PAD8STRNG_A>);
impl PAD8STRNG_R {
    pub(crate) fn new(bits: bool) -> Self {
        PAD8STRNG_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAD8STRNG_A {
        match self.bits {
            false => PAD8STRNG_A::LOW,
            true => PAD8STRNG_A::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        **self == PAD8STRNG_A::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        **self == PAD8STRNG_A::HIGH
    }
}
impl core::ops::Deref for PAD8STRNG_R {
    type Target = crate::FieldReader<bool, PAD8STRNG_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PAD8STRNG` writer - Pad 8 drive strength"]
pub struct PAD8STRNG_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD8STRNG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD8STRNG_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Low drive strength value."]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(PAD8STRNG_A::LOW)
    }
    #[doc = "High drive strength value."]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(PAD8STRNG_A::HIGH)
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
#[doc = "Pad 8 input enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD8INPEN_A {
    #[doc = "0: Pad input disabled value."]
    DIS = 0,
    #[doc = "1: Pad input enabled value."]
    EN = 1,
}
impl From<PAD8INPEN_A> for bool {
    #[inline(always)]
    fn from(variant: PAD8INPEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PAD8INPEN` reader - Pad 8 input enable"]
pub struct PAD8INPEN_R(crate::FieldReader<bool, PAD8INPEN_A>);
impl PAD8INPEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        PAD8INPEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAD8INPEN_A {
        match self.bits {
            false => PAD8INPEN_A::DIS,
            true => PAD8INPEN_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        **self == PAD8INPEN_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        **self == PAD8INPEN_A::EN
    }
}
impl core::ops::Deref for PAD8INPEN_R {
    type Target = crate::FieldReader<bool, PAD8INPEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PAD8INPEN` writer - Pad 8 input enable"]
pub struct PAD8INPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD8INPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD8INPEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Pad input disabled value."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(PAD8INPEN_A::DIS)
    }
    #[doc = "Pad input enabled value."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(PAD8INPEN_A::EN)
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
#[doc = "Pad 8 pullup enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD8PULL_A {
    #[doc = "0: Pullup disabled value."]
    DIS = 0,
    #[doc = "1: Pullup enabled value."]
    EN = 1,
}
impl From<PAD8PULL_A> for bool {
    #[inline(always)]
    fn from(variant: PAD8PULL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PAD8PULL` reader - Pad 8 pullup enable"]
pub struct PAD8PULL_R(crate::FieldReader<bool, PAD8PULL_A>);
impl PAD8PULL_R {
    pub(crate) fn new(bits: bool) -> Self {
        PAD8PULL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAD8PULL_A {
        match self.bits {
            false => PAD8PULL_A::DIS,
            true => PAD8PULL_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        **self == PAD8PULL_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        **self == PAD8PULL_A::EN
    }
}
impl core::ops::Deref for PAD8PULL_R {
    type Target = crate::FieldReader<bool, PAD8PULL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PAD8PULL` writer - Pad 8 pullup enable"]
pub struct PAD8PULL_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD8PULL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD8PULL_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Pullup disabled value."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(PAD8PULL_A::DIS)
    }
    #[doc = "Pullup enabled value."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(PAD8PULL_A::EN)
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
    #[doc = "Bits 27:29 - Pad 11 function select"]
    #[inline(always)]
    pub fn pad11fncsel(&self) -> PAD11FNCSEL_R {
        PAD11FNCSEL_R::new(((self.bits >> 27) & 0x07) as u8)
    }
    #[doc = "Bit 26 - Pad 11 drive strength"]
    #[inline(always)]
    pub fn pad11strng(&self) -> PAD11STRNG_R {
        PAD11STRNG_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Pad 11 input enable"]
    #[inline(always)]
    pub fn pad11inpen(&self) -> PAD11INPEN_R {
        PAD11INPEN_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Pad 11 pullup enable"]
    #[inline(always)]
    pub fn pad11pull(&self) -> PAD11PULL_R {
        PAD11PULL_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bits 19:21 - Pad 10 function select"]
    #[inline(always)]
    pub fn pad10fncsel(&self) -> PAD10FNCSEL_R {
        PAD10FNCSEL_R::new(((self.bits >> 19) & 0x07) as u8)
    }
    #[doc = "Bit 18 - Pad 10 drive strength"]
    #[inline(always)]
    pub fn pad10strng(&self) -> PAD10STRNG_R {
        PAD10STRNG_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Pad 10 input enable"]
    #[inline(always)]
    pub fn pad10inpen(&self) -> PAD10INPEN_R {
        PAD10INPEN_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Pad 10 pullup enable"]
    #[inline(always)]
    pub fn pad10pull(&self) -> PAD10PULL_R {
        PAD10PULL_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bits 14:15 - Pad 9 pullup resistor selection"]
    #[inline(always)]
    pub fn pad9rsel(&self) -> PAD9RSEL_R {
        PAD9RSEL_R::new(((self.bits >> 14) & 0x03) as u8)
    }
    #[doc = "Bits 11:13 - Pad 9 function select"]
    #[inline(always)]
    pub fn pad9fncsel(&self) -> PAD9FNCSEL_R {
        PAD9FNCSEL_R::new(((self.bits >> 11) & 0x07) as u8)
    }
    #[doc = "Bit 10 - Pad 9 drive strength"]
    #[inline(always)]
    pub fn pad9strng(&self) -> PAD9STRNG_R {
        PAD9STRNG_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Pad 9 input enable"]
    #[inline(always)]
    pub fn pad9inpen(&self) -> PAD9INPEN_R {
        PAD9INPEN_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Pad 9 pullup enable"]
    #[inline(always)]
    pub fn pad9pull(&self) -> PAD9PULL_R {
        PAD9PULL_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bits 6:7 - Pad 8 pullup resistor selection."]
    #[inline(always)]
    pub fn pad8rsel(&self) -> PAD8RSEL_R {
        PAD8RSEL_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bits 3:5 - Pad 8 function select"]
    #[inline(always)]
    pub fn pad8fncsel(&self) -> PAD8FNCSEL_R {
        PAD8FNCSEL_R::new(((self.bits >> 3) & 0x07) as u8)
    }
    #[doc = "Bit 2 - Pad 8 drive strength"]
    #[inline(always)]
    pub fn pad8strng(&self) -> PAD8STRNG_R {
        PAD8STRNG_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - Pad 8 input enable"]
    #[inline(always)]
    pub fn pad8inpen(&self) -> PAD8INPEN_R {
        PAD8INPEN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Pad 8 pullup enable"]
    #[inline(always)]
    pub fn pad8pull(&self) -> PAD8PULL_R {
        PAD8PULL_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 27:29 - Pad 11 function select"]
    #[inline(always)]
    pub fn pad11fncsel(&mut self) -> PAD11FNCSEL_W {
        PAD11FNCSEL_W { w: self }
    }
    #[doc = "Bit 26 - Pad 11 drive strength"]
    #[inline(always)]
    pub fn pad11strng(&mut self) -> PAD11STRNG_W {
        PAD11STRNG_W { w: self }
    }
    #[doc = "Bit 25 - Pad 11 input enable"]
    #[inline(always)]
    pub fn pad11inpen(&mut self) -> PAD11INPEN_W {
        PAD11INPEN_W { w: self }
    }
    #[doc = "Bit 24 - Pad 11 pullup enable"]
    #[inline(always)]
    pub fn pad11pull(&mut self) -> PAD11PULL_W {
        PAD11PULL_W { w: self }
    }
    #[doc = "Bits 19:21 - Pad 10 function select"]
    #[inline(always)]
    pub fn pad10fncsel(&mut self) -> PAD10FNCSEL_W {
        PAD10FNCSEL_W { w: self }
    }
    #[doc = "Bit 18 - Pad 10 drive strength"]
    #[inline(always)]
    pub fn pad10strng(&mut self) -> PAD10STRNG_W {
        PAD10STRNG_W { w: self }
    }
    #[doc = "Bit 17 - Pad 10 input enable"]
    #[inline(always)]
    pub fn pad10inpen(&mut self) -> PAD10INPEN_W {
        PAD10INPEN_W { w: self }
    }
    #[doc = "Bit 16 - Pad 10 pullup enable"]
    #[inline(always)]
    pub fn pad10pull(&mut self) -> PAD10PULL_W {
        PAD10PULL_W { w: self }
    }
    #[doc = "Bits 14:15 - Pad 9 pullup resistor selection"]
    #[inline(always)]
    pub fn pad9rsel(&mut self) -> PAD9RSEL_W {
        PAD9RSEL_W { w: self }
    }
    #[doc = "Bits 11:13 - Pad 9 function select"]
    #[inline(always)]
    pub fn pad9fncsel(&mut self) -> PAD9FNCSEL_W {
        PAD9FNCSEL_W { w: self }
    }
    #[doc = "Bit 10 - Pad 9 drive strength"]
    #[inline(always)]
    pub fn pad9strng(&mut self) -> PAD9STRNG_W {
        PAD9STRNG_W { w: self }
    }
    #[doc = "Bit 9 - Pad 9 input enable"]
    #[inline(always)]
    pub fn pad9inpen(&mut self) -> PAD9INPEN_W {
        PAD9INPEN_W { w: self }
    }
    #[doc = "Bit 8 - Pad 9 pullup enable"]
    #[inline(always)]
    pub fn pad9pull(&mut self) -> PAD9PULL_W {
        PAD9PULL_W { w: self }
    }
    #[doc = "Bits 6:7 - Pad 8 pullup resistor selection."]
    #[inline(always)]
    pub fn pad8rsel(&mut self) -> PAD8RSEL_W {
        PAD8RSEL_W { w: self }
    }
    #[doc = "Bits 3:5 - Pad 8 function select"]
    #[inline(always)]
    pub fn pad8fncsel(&mut self) -> PAD8FNCSEL_W {
        PAD8FNCSEL_W { w: self }
    }
    #[doc = "Bit 2 - Pad 8 drive strength"]
    #[inline(always)]
    pub fn pad8strng(&mut self) -> PAD8STRNG_W {
        PAD8STRNG_W { w: self }
    }
    #[doc = "Bit 1 - Pad 8 input enable"]
    #[inline(always)]
    pub fn pad8inpen(&mut self) -> PAD8INPEN_W {
        PAD8INPEN_W { w: self }
    }
    #[doc = "Bit 0 - Pad 8 pullup enable"]
    #[inline(always)]
    pub fn pad8pull(&mut self) -> PAD8PULL_W {
        PAD8PULL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Pad Configuration Register C (Pads 8-11)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [padregc](index.html) module"]
pub struct PADREGC_SPEC;
impl crate::RegisterSpec for PADREGC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [padregc::R](R) reader structure"]
impl crate::Readable for PADREGC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [padregc::W](W) writer structure"]
impl crate::Writable for PADREGC_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PADREGC to value 0x1818_1818"]
impl crate::Resettable for PADREGC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x1818_1818
    }
}
