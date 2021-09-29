#[doc = "Register `PADREGB` reader"]
pub struct R(crate::R<PADREGB_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PADREGB_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PADREGB_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PADREGB_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PADREGB` writer"]
pub struct W(crate::W<PADREGB_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PADREGB_SPEC>;
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
impl From<crate::W<PADREGB_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PADREGB_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Pad 7 function select\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PAD7FNCSEL_A {
    #[doc = "0: IOM/MSPI nCE group 7 value."]
    NCE7 = 0,
    #[doc = "1: Configure as the IOMSTR0 SPI MOSI signal value."]
    M0MOSI = 1,
    #[doc = "2: Configure as the CLKOUT signal value."]
    CLKOUT = 2,
    #[doc = "3: Configure as GPIO7 value."]
    GPIO7 = 3,
    #[doc = "4: Configure as the ADC Trigger 0 signal value."]
    TRIG0 = 4,
    #[doc = "5: Configure as the UART0 TX output signal value."]
    UART0TX = 5,
    #[doc = "7: CTIMER connection 19 value."]
    CT19 = 7,
}
impl From<PAD7FNCSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: PAD7FNCSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `PAD7FNCSEL` reader - Pad 7 function select"]
pub struct PAD7FNCSEL_R(crate::FieldReader<u8, PAD7FNCSEL_A>);
impl PAD7FNCSEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        PAD7FNCSEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PAD7FNCSEL_A> {
        match self.bits {
            0 => Some(PAD7FNCSEL_A::NCE7),
            1 => Some(PAD7FNCSEL_A::M0MOSI),
            2 => Some(PAD7FNCSEL_A::CLKOUT),
            3 => Some(PAD7FNCSEL_A::GPIO7),
            4 => Some(PAD7FNCSEL_A::TRIG0),
            5 => Some(PAD7FNCSEL_A::UART0TX),
            7 => Some(PAD7FNCSEL_A::CT19),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `NCE7`"]
    #[inline(always)]
    pub fn is_nce7(&self) -> bool {
        **self == PAD7FNCSEL_A::NCE7
    }
    #[doc = "Checks if the value of the field is `M0MOSI`"]
    #[inline(always)]
    pub fn is_m0mosi(&self) -> bool {
        **self == PAD7FNCSEL_A::M0MOSI
    }
    #[doc = "Checks if the value of the field is `CLKOUT`"]
    #[inline(always)]
    pub fn is_clkout(&self) -> bool {
        **self == PAD7FNCSEL_A::CLKOUT
    }
    #[doc = "Checks if the value of the field is `GPIO7`"]
    #[inline(always)]
    pub fn is_gpio7(&self) -> bool {
        **self == PAD7FNCSEL_A::GPIO7
    }
    #[doc = "Checks if the value of the field is `TRIG0`"]
    #[inline(always)]
    pub fn is_trig0(&self) -> bool {
        **self == PAD7FNCSEL_A::TRIG0
    }
    #[doc = "Checks if the value of the field is `UART0TX`"]
    #[inline(always)]
    pub fn is_uart0tx(&self) -> bool {
        **self == PAD7FNCSEL_A::UART0TX
    }
    #[doc = "Checks if the value of the field is `CT19`"]
    #[inline(always)]
    pub fn is_ct19(&self) -> bool {
        **self == PAD7FNCSEL_A::CT19
    }
}
impl core::ops::Deref for PAD7FNCSEL_R {
    type Target = crate::FieldReader<u8, PAD7FNCSEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PAD7FNCSEL` writer - Pad 7 function select"]
pub struct PAD7FNCSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD7FNCSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD7FNCSEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "IOM/MSPI nCE group 7 value."]
    #[inline(always)]
    pub fn nce7(self) -> &'a mut W {
        self.variant(PAD7FNCSEL_A::NCE7)
    }
    #[doc = "Configure as the IOMSTR0 SPI MOSI signal value."]
    #[inline(always)]
    pub fn m0mosi(self) -> &'a mut W {
        self.variant(PAD7FNCSEL_A::M0MOSI)
    }
    #[doc = "Configure as the CLKOUT signal value."]
    #[inline(always)]
    pub fn clkout(self) -> &'a mut W {
        self.variant(PAD7FNCSEL_A::CLKOUT)
    }
    #[doc = "Configure as GPIO7 value."]
    #[inline(always)]
    pub fn gpio7(self) -> &'a mut W {
        self.variant(PAD7FNCSEL_A::GPIO7)
    }
    #[doc = "Configure as the ADC Trigger 0 signal value."]
    #[inline(always)]
    pub fn trig0(self) -> &'a mut W {
        self.variant(PAD7FNCSEL_A::TRIG0)
    }
    #[doc = "Configure as the UART0 TX output signal value."]
    #[inline(always)]
    pub fn uart0tx(self) -> &'a mut W {
        self.variant(PAD7FNCSEL_A::UART0TX)
    }
    #[doc = "CTIMER connection 19 value."]
    #[inline(always)]
    pub fn ct19(self) -> &'a mut W {
        self.variant(PAD7FNCSEL_A::CT19)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 27)) | ((value as u32 & 0x07) << 27);
        self.w
    }
}
#[doc = "Pad 7 drive strength\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD7STRNG_A {
    #[doc = "0: Low drive strength value."]
    LOW = 0,
    #[doc = "1: High drive strength value."]
    HIGH = 1,
}
impl From<PAD7STRNG_A> for bool {
    #[inline(always)]
    fn from(variant: PAD7STRNG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PAD7STRNG` reader - Pad 7 drive strength"]
pub struct PAD7STRNG_R(crate::FieldReader<bool, PAD7STRNG_A>);
impl PAD7STRNG_R {
    pub(crate) fn new(bits: bool) -> Self {
        PAD7STRNG_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAD7STRNG_A {
        match self.bits {
            false => PAD7STRNG_A::LOW,
            true => PAD7STRNG_A::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        **self == PAD7STRNG_A::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        **self == PAD7STRNG_A::HIGH
    }
}
impl core::ops::Deref for PAD7STRNG_R {
    type Target = crate::FieldReader<bool, PAD7STRNG_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PAD7STRNG` writer - Pad 7 drive strength"]
pub struct PAD7STRNG_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD7STRNG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD7STRNG_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Low drive strength value."]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(PAD7STRNG_A::LOW)
    }
    #[doc = "High drive strength value."]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(PAD7STRNG_A::HIGH)
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
#[doc = "Pad 7 input enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD7INPEN_A {
    #[doc = "0: Pad input disabled value."]
    DIS = 0,
    #[doc = "1: Pad input enabled value."]
    EN = 1,
}
impl From<PAD7INPEN_A> for bool {
    #[inline(always)]
    fn from(variant: PAD7INPEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PAD7INPEN` reader - Pad 7 input enable"]
pub struct PAD7INPEN_R(crate::FieldReader<bool, PAD7INPEN_A>);
impl PAD7INPEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        PAD7INPEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAD7INPEN_A {
        match self.bits {
            false => PAD7INPEN_A::DIS,
            true => PAD7INPEN_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        **self == PAD7INPEN_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        **self == PAD7INPEN_A::EN
    }
}
impl core::ops::Deref for PAD7INPEN_R {
    type Target = crate::FieldReader<bool, PAD7INPEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PAD7INPEN` writer - Pad 7 input enable"]
pub struct PAD7INPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD7INPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD7INPEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Pad input disabled value."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(PAD7INPEN_A::DIS)
    }
    #[doc = "Pad input enabled value."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(PAD7INPEN_A::EN)
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
#[doc = "Pad 7 pullup enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD7PULL_A {
    #[doc = "0: Pullup disabled value."]
    DIS = 0,
    #[doc = "1: Pullup enabled value."]
    EN = 1,
}
impl From<PAD7PULL_A> for bool {
    #[inline(always)]
    fn from(variant: PAD7PULL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PAD7PULL` reader - Pad 7 pullup enable"]
pub struct PAD7PULL_R(crate::FieldReader<bool, PAD7PULL_A>);
impl PAD7PULL_R {
    pub(crate) fn new(bits: bool) -> Self {
        PAD7PULL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAD7PULL_A {
        match self.bits {
            false => PAD7PULL_A::DIS,
            true => PAD7PULL_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        **self == PAD7PULL_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        **self == PAD7PULL_A::EN
    }
}
impl core::ops::Deref for PAD7PULL_R {
    type Target = crate::FieldReader<bool, PAD7PULL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PAD7PULL` writer - Pad 7 pullup enable"]
pub struct PAD7PULL_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD7PULL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD7PULL_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Pullup disabled value."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(PAD7PULL_A::DIS)
    }
    #[doc = "Pullup enabled value."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(PAD7PULL_A::EN)
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
#[doc = "Pad 6 pullup resistor selection.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PAD6RSEL_A {
    #[doc = "0: Pullup is ~1.5 KOhms value."]
    PULL1_5K = 0,
    #[doc = "1: Pullup is ~6 KOhms value."]
    PULL6K = 1,
    #[doc = "2: Pullup is ~12 KOhms value."]
    PULL12K = 2,
    #[doc = "3: Pullup is ~24 KOhms value."]
    PULL24K = 3,
}
impl From<PAD6RSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: PAD6RSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `PAD6RSEL` reader - Pad 6 pullup resistor selection."]
pub struct PAD6RSEL_R(crate::FieldReader<u8, PAD6RSEL_A>);
impl PAD6RSEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        PAD6RSEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAD6RSEL_A {
        match self.bits {
            0 => PAD6RSEL_A::PULL1_5K,
            1 => PAD6RSEL_A::PULL6K,
            2 => PAD6RSEL_A::PULL12K,
            3 => PAD6RSEL_A::PULL24K,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PULL1_5K`"]
    #[inline(always)]
    pub fn is_pull1_5k(&self) -> bool {
        **self == PAD6RSEL_A::PULL1_5K
    }
    #[doc = "Checks if the value of the field is `PULL6K`"]
    #[inline(always)]
    pub fn is_pull6k(&self) -> bool {
        **self == PAD6RSEL_A::PULL6K
    }
    #[doc = "Checks if the value of the field is `PULL12K`"]
    #[inline(always)]
    pub fn is_pull12k(&self) -> bool {
        **self == PAD6RSEL_A::PULL12K
    }
    #[doc = "Checks if the value of the field is `PULL24K`"]
    #[inline(always)]
    pub fn is_pull24k(&self) -> bool {
        **self == PAD6RSEL_A::PULL24K
    }
}
impl core::ops::Deref for PAD6RSEL_R {
    type Target = crate::FieldReader<u8, PAD6RSEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PAD6RSEL` writer - Pad 6 pullup resistor selection."]
pub struct PAD6RSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD6RSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD6RSEL_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Pullup is ~1.5 KOhms value."]
    #[inline(always)]
    pub fn pull1_5k(self) -> &'a mut W {
        self.variant(PAD6RSEL_A::PULL1_5K)
    }
    #[doc = "Pullup is ~6 KOhms value."]
    #[inline(always)]
    pub fn pull6k(self) -> &'a mut W {
        self.variant(PAD6RSEL_A::PULL6K)
    }
    #[doc = "Pullup is ~12 KOhms value."]
    #[inline(always)]
    pub fn pull12k(self) -> &'a mut W {
        self.variant(PAD6RSEL_A::PULL12K)
    }
    #[doc = "Pullup is ~24 KOhms value."]
    #[inline(always)]
    pub fn pull24k(self) -> &'a mut W {
        self.variant(PAD6RSEL_A::PULL24K)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 22)) | ((value as u32 & 0x03) << 22);
        self.w
    }
}
#[doc = "Pad 6 function select\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PAD6FNCSEL_A {
    #[doc = "0: Configure as the IOMSTR0 I2C SDA or SPI WIR3 signal value."]
    M0SDAWIR3 = 0,
    #[doc = "1: Configure as the IOMSTR0 SPI MISO signal value."]
    M0MISO = 1,
    #[doc = "2: Configure as the UART0 CTS input signal value."]
    UA0CTS = 2,
    #[doc = "3: Configure as GPIO6 value."]
    GPIO6 = 3,
    #[doc = "5: CTIMER connection 10 value."]
    CT10 = 5,
    #[doc = "7: Configure as the PDM I2S Data output signal value."]
    I2S_DAT = 7,
}
impl From<PAD6FNCSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: PAD6FNCSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `PAD6FNCSEL` reader - Pad 6 function select"]
pub struct PAD6FNCSEL_R(crate::FieldReader<u8, PAD6FNCSEL_A>);
impl PAD6FNCSEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        PAD6FNCSEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PAD6FNCSEL_A> {
        match self.bits {
            0 => Some(PAD6FNCSEL_A::M0SDAWIR3),
            1 => Some(PAD6FNCSEL_A::M0MISO),
            2 => Some(PAD6FNCSEL_A::UA0CTS),
            3 => Some(PAD6FNCSEL_A::GPIO6),
            5 => Some(PAD6FNCSEL_A::CT10),
            7 => Some(PAD6FNCSEL_A::I2S_DAT),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `M0SDAWIR3`"]
    #[inline(always)]
    pub fn is_m0sdawir3(&self) -> bool {
        **self == PAD6FNCSEL_A::M0SDAWIR3
    }
    #[doc = "Checks if the value of the field is `M0MISO`"]
    #[inline(always)]
    pub fn is_m0miso(&self) -> bool {
        **self == PAD6FNCSEL_A::M0MISO
    }
    #[doc = "Checks if the value of the field is `UA0CTS`"]
    #[inline(always)]
    pub fn is_ua0cts(&self) -> bool {
        **self == PAD6FNCSEL_A::UA0CTS
    }
    #[doc = "Checks if the value of the field is `GPIO6`"]
    #[inline(always)]
    pub fn is_gpio6(&self) -> bool {
        **self == PAD6FNCSEL_A::GPIO6
    }
    #[doc = "Checks if the value of the field is `CT10`"]
    #[inline(always)]
    pub fn is_ct10(&self) -> bool {
        **self == PAD6FNCSEL_A::CT10
    }
    #[doc = "Checks if the value of the field is `I2S_DAT`"]
    #[inline(always)]
    pub fn is_i2s_dat(&self) -> bool {
        **self == PAD6FNCSEL_A::I2S_DAT
    }
}
impl core::ops::Deref for PAD6FNCSEL_R {
    type Target = crate::FieldReader<u8, PAD6FNCSEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PAD6FNCSEL` writer - Pad 6 function select"]
pub struct PAD6FNCSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD6FNCSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD6FNCSEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Configure as the IOMSTR0 I2C SDA or SPI WIR3 signal value."]
    #[inline(always)]
    pub fn m0sdawir3(self) -> &'a mut W {
        self.variant(PAD6FNCSEL_A::M0SDAWIR3)
    }
    #[doc = "Configure as the IOMSTR0 SPI MISO signal value."]
    #[inline(always)]
    pub fn m0miso(self) -> &'a mut W {
        self.variant(PAD6FNCSEL_A::M0MISO)
    }
    #[doc = "Configure as the UART0 CTS input signal value."]
    #[inline(always)]
    pub fn ua0cts(self) -> &'a mut W {
        self.variant(PAD6FNCSEL_A::UA0CTS)
    }
    #[doc = "Configure as GPIO6 value."]
    #[inline(always)]
    pub fn gpio6(self) -> &'a mut W {
        self.variant(PAD6FNCSEL_A::GPIO6)
    }
    #[doc = "CTIMER connection 10 value."]
    #[inline(always)]
    pub fn ct10(self) -> &'a mut W {
        self.variant(PAD6FNCSEL_A::CT10)
    }
    #[doc = "Configure as the PDM I2S Data output signal value."]
    #[inline(always)]
    pub fn i2s_dat(self) -> &'a mut W {
        self.variant(PAD6FNCSEL_A::I2S_DAT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 19)) | ((value as u32 & 0x07) << 19);
        self.w
    }
}
#[doc = "Pad 6 drive strength\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD6STRNG_A {
    #[doc = "0: Low drive strength value."]
    LOW = 0,
    #[doc = "1: High drive strength value."]
    HIGH = 1,
}
impl From<PAD6STRNG_A> for bool {
    #[inline(always)]
    fn from(variant: PAD6STRNG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PAD6STRNG` reader - Pad 6 drive strength"]
pub struct PAD6STRNG_R(crate::FieldReader<bool, PAD6STRNG_A>);
impl PAD6STRNG_R {
    pub(crate) fn new(bits: bool) -> Self {
        PAD6STRNG_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAD6STRNG_A {
        match self.bits {
            false => PAD6STRNG_A::LOW,
            true => PAD6STRNG_A::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        **self == PAD6STRNG_A::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        **self == PAD6STRNG_A::HIGH
    }
}
impl core::ops::Deref for PAD6STRNG_R {
    type Target = crate::FieldReader<bool, PAD6STRNG_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PAD6STRNG` writer - Pad 6 drive strength"]
pub struct PAD6STRNG_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD6STRNG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD6STRNG_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Low drive strength value."]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(PAD6STRNG_A::LOW)
    }
    #[doc = "High drive strength value."]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(PAD6STRNG_A::HIGH)
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
#[doc = "Pad 6 input enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD6INPEN_A {
    #[doc = "0: Pad input disabled value."]
    DIS = 0,
    #[doc = "1: Pad input enabled value."]
    EN = 1,
}
impl From<PAD6INPEN_A> for bool {
    #[inline(always)]
    fn from(variant: PAD6INPEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PAD6INPEN` reader - Pad 6 input enable"]
pub struct PAD6INPEN_R(crate::FieldReader<bool, PAD6INPEN_A>);
impl PAD6INPEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        PAD6INPEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAD6INPEN_A {
        match self.bits {
            false => PAD6INPEN_A::DIS,
            true => PAD6INPEN_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        **self == PAD6INPEN_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        **self == PAD6INPEN_A::EN
    }
}
impl core::ops::Deref for PAD6INPEN_R {
    type Target = crate::FieldReader<bool, PAD6INPEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PAD6INPEN` writer - Pad 6 input enable"]
pub struct PAD6INPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD6INPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD6INPEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Pad input disabled value."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(PAD6INPEN_A::DIS)
    }
    #[doc = "Pad input enabled value."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(PAD6INPEN_A::EN)
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
#[doc = "Pad 6 pullup enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD6PULL_A {
    #[doc = "0: Pullup disabled value."]
    DIS = 0,
    #[doc = "1: Pullup enabled value."]
    EN = 1,
}
impl From<PAD6PULL_A> for bool {
    #[inline(always)]
    fn from(variant: PAD6PULL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PAD6PULL` reader - Pad 6 pullup enable"]
pub struct PAD6PULL_R(crate::FieldReader<bool, PAD6PULL_A>);
impl PAD6PULL_R {
    pub(crate) fn new(bits: bool) -> Self {
        PAD6PULL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAD6PULL_A {
        match self.bits {
            false => PAD6PULL_A::DIS,
            true => PAD6PULL_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        **self == PAD6PULL_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        **self == PAD6PULL_A::EN
    }
}
impl core::ops::Deref for PAD6PULL_R {
    type Target = crate::FieldReader<bool, PAD6PULL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PAD6PULL` writer - Pad 6 pullup enable"]
pub struct PAD6PULL_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD6PULL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD6PULL_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Pullup disabled value."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(PAD6PULL_A::DIS)
    }
    #[doc = "Pullup enabled value."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(PAD6PULL_A::EN)
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
#[doc = "Pad 5 pullup resistor selection.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PAD5RSEL_A {
    #[doc = "0: Pullup is ~1.5 KOhms value."]
    PULL1_5K = 0,
    #[doc = "1: Pullup is ~6 KOhms value."]
    PULL6K = 1,
    #[doc = "2: Pullup is ~12 KOhms value."]
    PULL12K = 2,
    #[doc = "3: Pullup is ~24 KOhms value."]
    PULL24K = 3,
}
impl From<PAD5RSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: PAD5RSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `PAD5RSEL` reader - Pad 5 pullup resistor selection."]
pub struct PAD5RSEL_R(crate::FieldReader<u8, PAD5RSEL_A>);
impl PAD5RSEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        PAD5RSEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAD5RSEL_A {
        match self.bits {
            0 => PAD5RSEL_A::PULL1_5K,
            1 => PAD5RSEL_A::PULL6K,
            2 => PAD5RSEL_A::PULL12K,
            3 => PAD5RSEL_A::PULL24K,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PULL1_5K`"]
    #[inline(always)]
    pub fn is_pull1_5k(&self) -> bool {
        **self == PAD5RSEL_A::PULL1_5K
    }
    #[doc = "Checks if the value of the field is `PULL6K`"]
    #[inline(always)]
    pub fn is_pull6k(&self) -> bool {
        **self == PAD5RSEL_A::PULL6K
    }
    #[doc = "Checks if the value of the field is `PULL12K`"]
    #[inline(always)]
    pub fn is_pull12k(&self) -> bool {
        **self == PAD5RSEL_A::PULL12K
    }
    #[doc = "Checks if the value of the field is `PULL24K`"]
    #[inline(always)]
    pub fn is_pull24k(&self) -> bool {
        **self == PAD5RSEL_A::PULL24K
    }
}
impl core::ops::Deref for PAD5RSEL_R {
    type Target = crate::FieldReader<u8, PAD5RSEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PAD5RSEL` writer - Pad 5 pullup resistor selection."]
pub struct PAD5RSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD5RSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD5RSEL_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Pullup is ~1.5 KOhms value."]
    #[inline(always)]
    pub fn pull1_5k(self) -> &'a mut W {
        self.variant(PAD5RSEL_A::PULL1_5K)
    }
    #[doc = "Pullup is ~6 KOhms value."]
    #[inline(always)]
    pub fn pull6k(self) -> &'a mut W {
        self.variant(PAD5RSEL_A::PULL6K)
    }
    #[doc = "Pullup is ~12 KOhms value."]
    #[inline(always)]
    pub fn pull12k(self) -> &'a mut W {
        self.variant(PAD5RSEL_A::PULL12K)
    }
    #[doc = "Pullup is ~24 KOhms value."]
    #[inline(always)]
    pub fn pull24k(self) -> &'a mut W {
        self.variant(PAD5RSEL_A::PULL24K)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 14)) | ((value as u32 & 0x03) << 14);
        self.w
    }
}
#[doc = "Pad 5 function select\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PAD5FNCSEL_A {
    #[doc = "0: Configure as the IOMSTR0 I2C SCL signal value."]
    M0SCL = 0,
    #[doc = "1: Configure as the IOMSTR0 SPI SCK signal value."]
    M0SCK = 1,
    #[doc = "2: Configure as the UART0 RTS signal output value."]
    UA0RTS = 2,
    #[doc = "3: Configure as GPIO5 value."]
    GPIO5 = 3,
    #[doc = "5: Configure as the External HFA input clock value."]
    EXTHFA = 5,
    #[doc = "7: CTIMER connection 8 value."]
    CT8 = 7,
}
impl From<PAD5FNCSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: PAD5FNCSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `PAD5FNCSEL` reader - Pad 5 function select"]
pub struct PAD5FNCSEL_R(crate::FieldReader<u8, PAD5FNCSEL_A>);
impl PAD5FNCSEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        PAD5FNCSEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PAD5FNCSEL_A> {
        match self.bits {
            0 => Some(PAD5FNCSEL_A::M0SCL),
            1 => Some(PAD5FNCSEL_A::M0SCK),
            2 => Some(PAD5FNCSEL_A::UA0RTS),
            3 => Some(PAD5FNCSEL_A::GPIO5),
            5 => Some(PAD5FNCSEL_A::EXTHFA),
            7 => Some(PAD5FNCSEL_A::CT8),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `M0SCL`"]
    #[inline(always)]
    pub fn is_m0scl(&self) -> bool {
        **self == PAD5FNCSEL_A::M0SCL
    }
    #[doc = "Checks if the value of the field is `M0SCK`"]
    #[inline(always)]
    pub fn is_m0sck(&self) -> bool {
        **self == PAD5FNCSEL_A::M0SCK
    }
    #[doc = "Checks if the value of the field is `UA0RTS`"]
    #[inline(always)]
    pub fn is_ua0rts(&self) -> bool {
        **self == PAD5FNCSEL_A::UA0RTS
    }
    #[doc = "Checks if the value of the field is `GPIO5`"]
    #[inline(always)]
    pub fn is_gpio5(&self) -> bool {
        **self == PAD5FNCSEL_A::GPIO5
    }
    #[doc = "Checks if the value of the field is `EXTHFA`"]
    #[inline(always)]
    pub fn is_exthfa(&self) -> bool {
        **self == PAD5FNCSEL_A::EXTHFA
    }
    #[doc = "Checks if the value of the field is `CT8`"]
    #[inline(always)]
    pub fn is_ct8(&self) -> bool {
        **self == PAD5FNCSEL_A::CT8
    }
}
impl core::ops::Deref for PAD5FNCSEL_R {
    type Target = crate::FieldReader<u8, PAD5FNCSEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PAD5FNCSEL` writer - Pad 5 function select"]
pub struct PAD5FNCSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD5FNCSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD5FNCSEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Configure as the IOMSTR0 I2C SCL signal value."]
    #[inline(always)]
    pub fn m0scl(self) -> &'a mut W {
        self.variant(PAD5FNCSEL_A::M0SCL)
    }
    #[doc = "Configure as the IOMSTR0 SPI SCK signal value."]
    #[inline(always)]
    pub fn m0sck(self) -> &'a mut W {
        self.variant(PAD5FNCSEL_A::M0SCK)
    }
    #[doc = "Configure as the UART0 RTS signal output value."]
    #[inline(always)]
    pub fn ua0rts(self) -> &'a mut W {
        self.variant(PAD5FNCSEL_A::UA0RTS)
    }
    #[doc = "Configure as GPIO5 value."]
    #[inline(always)]
    pub fn gpio5(self) -> &'a mut W {
        self.variant(PAD5FNCSEL_A::GPIO5)
    }
    #[doc = "Configure as the External HFA input clock value."]
    #[inline(always)]
    pub fn exthfa(self) -> &'a mut W {
        self.variant(PAD5FNCSEL_A::EXTHFA)
    }
    #[doc = "CTIMER connection 8 value."]
    #[inline(always)]
    pub fn ct8(self) -> &'a mut W {
        self.variant(PAD5FNCSEL_A::CT8)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 11)) | ((value as u32 & 0x07) << 11);
        self.w
    }
}
#[doc = "Pad 5 drive strength\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD5STRNG_A {
    #[doc = "0: Low drive strength value."]
    LOW = 0,
    #[doc = "1: High drive strength value."]
    HIGH = 1,
}
impl From<PAD5STRNG_A> for bool {
    #[inline(always)]
    fn from(variant: PAD5STRNG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PAD5STRNG` reader - Pad 5 drive strength"]
pub struct PAD5STRNG_R(crate::FieldReader<bool, PAD5STRNG_A>);
impl PAD5STRNG_R {
    pub(crate) fn new(bits: bool) -> Self {
        PAD5STRNG_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAD5STRNG_A {
        match self.bits {
            false => PAD5STRNG_A::LOW,
            true => PAD5STRNG_A::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        **self == PAD5STRNG_A::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        **self == PAD5STRNG_A::HIGH
    }
}
impl core::ops::Deref for PAD5STRNG_R {
    type Target = crate::FieldReader<bool, PAD5STRNG_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PAD5STRNG` writer - Pad 5 drive strength"]
pub struct PAD5STRNG_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD5STRNG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD5STRNG_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Low drive strength value."]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(PAD5STRNG_A::LOW)
    }
    #[doc = "High drive strength value."]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(PAD5STRNG_A::HIGH)
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
#[doc = "Pad 5 input enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD5INPEN_A {
    #[doc = "0: Pad input disabled value."]
    DIS = 0,
    #[doc = "1: Pad input enabled value."]
    EN = 1,
}
impl From<PAD5INPEN_A> for bool {
    #[inline(always)]
    fn from(variant: PAD5INPEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PAD5INPEN` reader - Pad 5 input enable"]
pub struct PAD5INPEN_R(crate::FieldReader<bool, PAD5INPEN_A>);
impl PAD5INPEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        PAD5INPEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAD5INPEN_A {
        match self.bits {
            false => PAD5INPEN_A::DIS,
            true => PAD5INPEN_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        **self == PAD5INPEN_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        **self == PAD5INPEN_A::EN
    }
}
impl core::ops::Deref for PAD5INPEN_R {
    type Target = crate::FieldReader<bool, PAD5INPEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PAD5INPEN` writer - Pad 5 input enable"]
pub struct PAD5INPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD5INPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD5INPEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Pad input disabled value."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(PAD5INPEN_A::DIS)
    }
    #[doc = "Pad input enabled value."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(PAD5INPEN_A::EN)
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
#[doc = "Pad 5 pullup enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD5PULL_A {
    #[doc = "0: Pullup disabled value."]
    DIS = 0,
    #[doc = "1: Pullup enabled value."]
    EN = 1,
}
impl From<PAD5PULL_A> for bool {
    #[inline(always)]
    fn from(variant: PAD5PULL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PAD5PULL` reader - Pad 5 pullup enable"]
pub struct PAD5PULL_R(crate::FieldReader<bool, PAD5PULL_A>);
impl PAD5PULL_R {
    pub(crate) fn new(bits: bool) -> Self {
        PAD5PULL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAD5PULL_A {
        match self.bits {
            false => PAD5PULL_A::DIS,
            true => PAD5PULL_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        **self == PAD5PULL_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        **self == PAD5PULL_A::EN
    }
}
impl core::ops::Deref for PAD5PULL_R {
    type Target = crate::FieldReader<bool, PAD5PULL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PAD5PULL` writer - Pad 5 pullup enable"]
pub struct PAD5PULL_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD5PULL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD5PULL_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Pullup disabled value."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(PAD5PULL_A::DIS)
    }
    #[doc = "Pullup enabled value."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(PAD5PULL_A::EN)
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
#[doc = "Pad 4 function select\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PAD4FNCSEL_A {
    #[doc = "0: Configure as the UART0 CTS input signal value."]
    UA0CTS = 0,
    #[doc = "1: Configure as the IOSLAVE interrupt out signal value."]
    SLINT = 1,
    #[doc = "2: IOM/SPI nCE group 4 value."]
    NCE4 = 2,
    #[doc = "3: Configure as GPIO4 value."]
    GPIO4 = 3,
    #[doc = "5: Configure as the UART0 RX input value."]
    UART0RX = 5,
    #[doc = "6: CTIMER connection 17 value."]
    CT17 = 6,
    #[doc = "7: MSPI data connection 2 value."]
    MSPI2 = 7,
}
impl From<PAD4FNCSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: PAD4FNCSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `PAD4FNCSEL` reader - Pad 4 function select"]
pub struct PAD4FNCSEL_R(crate::FieldReader<u8, PAD4FNCSEL_A>);
impl PAD4FNCSEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        PAD4FNCSEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PAD4FNCSEL_A> {
        match self.bits {
            0 => Some(PAD4FNCSEL_A::UA0CTS),
            1 => Some(PAD4FNCSEL_A::SLINT),
            2 => Some(PAD4FNCSEL_A::NCE4),
            3 => Some(PAD4FNCSEL_A::GPIO4),
            5 => Some(PAD4FNCSEL_A::UART0RX),
            6 => Some(PAD4FNCSEL_A::CT17),
            7 => Some(PAD4FNCSEL_A::MSPI2),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `UA0CTS`"]
    #[inline(always)]
    pub fn is_ua0cts(&self) -> bool {
        **self == PAD4FNCSEL_A::UA0CTS
    }
    #[doc = "Checks if the value of the field is `SLINT`"]
    #[inline(always)]
    pub fn is_slint(&self) -> bool {
        **self == PAD4FNCSEL_A::SLINT
    }
    #[doc = "Checks if the value of the field is `NCE4`"]
    #[inline(always)]
    pub fn is_nce4(&self) -> bool {
        **self == PAD4FNCSEL_A::NCE4
    }
    #[doc = "Checks if the value of the field is `GPIO4`"]
    #[inline(always)]
    pub fn is_gpio4(&self) -> bool {
        **self == PAD4FNCSEL_A::GPIO4
    }
    #[doc = "Checks if the value of the field is `UART0RX`"]
    #[inline(always)]
    pub fn is_uart0rx(&self) -> bool {
        **self == PAD4FNCSEL_A::UART0RX
    }
    #[doc = "Checks if the value of the field is `CT17`"]
    #[inline(always)]
    pub fn is_ct17(&self) -> bool {
        **self == PAD4FNCSEL_A::CT17
    }
    #[doc = "Checks if the value of the field is `MSPI2`"]
    #[inline(always)]
    pub fn is_mspi2(&self) -> bool {
        **self == PAD4FNCSEL_A::MSPI2
    }
}
impl core::ops::Deref for PAD4FNCSEL_R {
    type Target = crate::FieldReader<u8, PAD4FNCSEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PAD4FNCSEL` writer - Pad 4 function select"]
pub struct PAD4FNCSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD4FNCSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD4FNCSEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Configure as the UART0 CTS input signal value."]
    #[inline(always)]
    pub fn ua0cts(self) -> &'a mut W {
        self.variant(PAD4FNCSEL_A::UA0CTS)
    }
    #[doc = "Configure as the IOSLAVE interrupt out signal value."]
    #[inline(always)]
    pub fn slint(self) -> &'a mut W {
        self.variant(PAD4FNCSEL_A::SLINT)
    }
    #[doc = "IOM/SPI nCE group 4 value."]
    #[inline(always)]
    pub fn nce4(self) -> &'a mut W {
        self.variant(PAD4FNCSEL_A::NCE4)
    }
    #[doc = "Configure as GPIO4 value."]
    #[inline(always)]
    pub fn gpio4(self) -> &'a mut W {
        self.variant(PAD4FNCSEL_A::GPIO4)
    }
    #[doc = "Configure as the UART0 RX input value."]
    #[inline(always)]
    pub fn uart0rx(self) -> &'a mut W {
        self.variant(PAD4FNCSEL_A::UART0RX)
    }
    #[doc = "CTIMER connection 17 value."]
    #[inline(always)]
    pub fn ct17(self) -> &'a mut W {
        self.variant(PAD4FNCSEL_A::CT17)
    }
    #[doc = "MSPI data connection 2 value."]
    #[inline(always)]
    pub fn mspi2(self) -> &'a mut W {
        self.variant(PAD4FNCSEL_A::MSPI2)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 3)) | ((value as u32 & 0x07) << 3);
        self.w
    }
}
#[doc = "Pad 4 drive strength\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD4STRNG_A {
    #[doc = "0: Low drive strength value."]
    LOW = 0,
    #[doc = "1: High drive strength value."]
    HIGH = 1,
}
impl From<PAD4STRNG_A> for bool {
    #[inline(always)]
    fn from(variant: PAD4STRNG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PAD4STRNG` reader - Pad 4 drive strength"]
pub struct PAD4STRNG_R(crate::FieldReader<bool, PAD4STRNG_A>);
impl PAD4STRNG_R {
    pub(crate) fn new(bits: bool) -> Self {
        PAD4STRNG_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAD4STRNG_A {
        match self.bits {
            false => PAD4STRNG_A::LOW,
            true => PAD4STRNG_A::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        **self == PAD4STRNG_A::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        **self == PAD4STRNG_A::HIGH
    }
}
impl core::ops::Deref for PAD4STRNG_R {
    type Target = crate::FieldReader<bool, PAD4STRNG_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PAD4STRNG` writer - Pad 4 drive strength"]
pub struct PAD4STRNG_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD4STRNG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD4STRNG_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Low drive strength value."]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(PAD4STRNG_A::LOW)
    }
    #[doc = "High drive strength value."]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(PAD4STRNG_A::HIGH)
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
#[doc = "Pad 4 input enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD4INPEN_A {
    #[doc = "0: Pad input disabled value."]
    DIS = 0,
    #[doc = "1: Pad input enabled value."]
    EN = 1,
}
impl From<PAD4INPEN_A> for bool {
    #[inline(always)]
    fn from(variant: PAD4INPEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PAD4INPEN` reader - Pad 4 input enable"]
pub struct PAD4INPEN_R(crate::FieldReader<bool, PAD4INPEN_A>);
impl PAD4INPEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        PAD4INPEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAD4INPEN_A {
        match self.bits {
            false => PAD4INPEN_A::DIS,
            true => PAD4INPEN_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        **self == PAD4INPEN_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        **self == PAD4INPEN_A::EN
    }
}
impl core::ops::Deref for PAD4INPEN_R {
    type Target = crate::FieldReader<bool, PAD4INPEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PAD4INPEN` writer - Pad 4 input enable"]
pub struct PAD4INPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD4INPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD4INPEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Pad input disabled value."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(PAD4INPEN_A::DIS)
    }
    #[doc = "Pad input enabled value."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(PAD4INPEN_A::EN)
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
#[doc = "Pad 4 pullup enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD4PULL_A {
    #[doc = "0: Pullup disabled value."]
    DIS = 0,
    #[doc = "1: Pullup enabled value."]
    EN = 1,
}
impl From<PAD4PULL_A> for bool {
    #[inline(always)]
    fn from(variant: PAD4PULL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PAD4PULL` reader - Pad 4 pullup enable"]
pub struct PAD4PULL_R(crate::FieldReader<bool, PAD4PULL_A>);
impl PAD4PULL_R {
    pub(crate) fn new(bits: bool) -> Self {
        PAD4PULL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAD4PULL_A {
        match self.bits {
            false => PAD4PULL_A::DIS,
            true => PAD4PULL_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        **self == PAD4PULL_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        **self == PAD4PULL_A::EN
    }
}
impl core::ops::Deref for PAD4PULL_R {
    type Target = crate::FieldReader<bool, PAD4PULL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PAD4PULL` writer - Pad 4 pullup enable"]
pub struct PAD4PULL_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD4PULL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD4PULL_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Pullup disabled value."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(PAD4PULL_A::DIS)
    }
    #[doc = "Pullup enabled value."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(PAD4PULL_A::EN)
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
    #[doc = "Bits 27:29 - Pad 7 function select"]
    #[inline(always)]
    pub fn pad7fncsel(&self) -> PAD7FNCSEL_R {
        PAD7FNCSEL_R::new(((self.bits >> 27) & 0x07) as u8)
    }
    #[doc = "Bit 26 - Pad 7 drive strength"]
    #[inline(always)]
    pub fn pad7strng(&self) -> PAD7STRNG_R {
        PAD7STRNG_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Pad 7 input enable"]
    #[inline(always)]
    pub fn pad7inpen(&self) -> PAD7INPEN_R {
        PAD7INPEN_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Pad 7 pullup enable"]
    #[inline(always)]
    pub fn pad7pull(&self) -> PAD7PULL_R {
        PAD7PULL_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bits 22:23 - Pad 6 pullup resistor selection."]
    #[inline(always)]
    pub fn pad6rsel(&self) -> PAD6RSEL_R {
        PAD6RSEL_R::new(((self.bits >> 22) & 0x03) as u8)
    }
    #[doc = "Bits 19:21 - Pad 6 function select"]
    #[inline(always)]
    pub fn pad6fncsel(&self) -> PAD6FNCSEL_R {
        PAD6FNCSEL_R::new(((self.bits >> 19) & 0x07) as u8)
    }
    #[doc = "Bit 18 - Pad 6 drive strength"]
    #[inline(always)]
    pub fn pad6strng(&self) -> PAD6STRNG_R {
        PAD6STRNG_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Pad 6 input enable"]
    #[inline(always)]
    pub fn pad6inpen(&self) -> PAD6INPEN_R {
        PAD6INPEN_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Pad 6 pullup enable"]
    #[inline(always)]
    pub fn pad6pull(&self) -> PAD6PULL_R {
        PAD6PULL_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bits 14:15 - Pad 5 pullup resistor selection."]
    #[inline(always)]
    pub fn pad5rsel(&self) -> PAD5RSEL_R {
        PAD5RSEL_R::new(((self.bits >> 14) & 0x03) as u8)
    }
    #[doc = "Bits 11:13 - Pad 5 function select"]
    #[inline(always)]
    pub fn pad5fncsel(&self) -> PAD5FNCSEL_R {
        PAD5FNCSEL_R::new(((self.bits >> 11) & 0x07) as u8)
    }
    #[doc = "Bit 10 - Pad 5 drive strength"]
    #[inline(always)]
    pub fn pad5strng(&self) -> PAD5STRNG_R {
        PAD5STRNG_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Pad 5 input enable"]
    #[inline(always)]
    pub fn pad5inpen(&self) -> PAD5INPEN_R {
        PAD5INPEN_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Pad 5 pullup enable"]
    #[inline(always)]
    pub fn pad5pull(&self) -> PAD5PULL_R {
        PAD5PULL_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bits 3:5 - Pad 4 function select"]
    #[inline(always)]
    pub fn pad4fncsel(&self) -> PAD4FNCSEL_R {
        PAD4FNCSEL_R::new(((self.bits >> 3) & 0x07) as u8)
    }
    #[doc = "Bit 2 - Pad 4 drive strength"]
    #[inline(always)]
    pub fn pad4strng(&self) -> PAD4STRNG_R {
        PAD4STRNG_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - Pad 4 input enable"]
    #[inline(always)]
    pub fn pad4inpen(&self) -> PAD4INPEN_R {
        PAD4INPEN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Pad 4 pullup enable"]
    #[inline(always)]
    pub fn pad4pull(&self) -> PAD4PULL_R {
        PAD4PULL_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 27:29 - Pad 7 function select"]
    #[inline(always)]
    pub fn pad7fncsel(&mut self) -> PAD7FNCSEL_W {
        PAD7FNCSEL_W { w: self }
    }
    #[doc = "Bit 26 - Pad 7 drive strength"]
    #[inline(always)]
    pub fn pad7strng(&mut self) -> PAD7STRNG_W {
        PAD7STRNG_W { w: self }
    }
    #[doc = "Bit 25 - Pad 7 input enable"]
    #[inline(always)]
    pub fn pad7inpen(&mut self) -> PAD7INPEN_W {
        PAD7INPEN_W { w: self }
    }
    #[doc = "Bit 24 - Pad 7 pullup enable"]
    #[inline(always)]
    pub fn pad7pull(&mut self) -> PAD7PULL_W {
        PAD7PULL_W { w: self }
    }
    #[doc = "Bits 22:23 - Pad 6 pullup resistor selection."]
    #[inline(always)]
    pub fn pad6rsel(&mut self) -> PAD6RSEL_W {
        PAD6RSEL_W { w: self }
    }
    #[doc = "Bits 19:21 - Pad 6 function select"]
    #[inline(always)]
    pub fn pad6fncsel(&mut self) -> PAD6FNCSEL_W {
        PAD6FNCSEL_W { w: self }
    }
    #[doc = "Bit 18 - Pad 6 drive strength"]
    #[inline(always)]
    pub fn pad6strng(&mut self) -> PAD6STRNG_W {
        PAD6STRNG_W { w: self }
    }
    #[doc = "Bit 17 - Pad 6 input enable"]
    #[inline(always)]
    pub fn pad6inpen(&mut self) -> PAD6INPEN_W {
        PAD6INPEN_W { w: self }
    }
    #[doc = "Bit 16 - Pad 6 pullup enable"]
    #[inline(always)]
    pub fn pad6pull(&mut self) -> PAD6PULL_W {
        PAD6PULL_W { w: self }
    }
    #[doc = "Bits 14:15 - Pad 5 pullup resistor selection."]
    #[inline(always)]
    pub fn pad5rsel(&mut self) -> PAD5RSEL_W {
        PAD5RSEL_W { w: self }
    }
    #[doc = "Bits 11:13 - Pad 5 function select"]
    #[inline(always)]
    pub fn pad5fncsel(&mut self) -> PAD5FNCSEL_W {
        PAD5FNCSEL_W { w: self }
    }
    #[doc = "Bit 10 - Pad 5 drive strength"]
    #[inline(always)]
    pub fn pad5strng(&mut self) -> PAD5STRNG_W {
        PAD5STRNG_W { w: self }
    }
    #[doc = "Bit 9 - Pad 5 input enable"]
    #[inline(always)]
    pub fn pad5inpen(&mut self) -> PAD5INPEN_W {
        PAD5INPEN_W { w: self }
    }
    #[doc = "Bit 8 - Pad 5 pullup enable"]
    #[inline(always)]
    pub fn pad5pull(&mut self) -> PAD5PULL_W {
        PAD5PULL_W { w: self }
    }
    #[doc = "Bits 3:5 - Pad 4 function select"]
    #[inline(always)]
    pub fn pad4fncsel(&mut self) -> PAD4FNCSEL_W {
        PAD4FNCSEL_W { w: self }
    }
    #[doc = "Bit 2 - Pad 4 drive strength"]
    #[inline(always)]
    pub fn pad4strng(&mut self) -> PAD4STRNG_W {
        PAD4STRNG_W { w: self }
    }
    #[doc = "Bit 1 - Pad 4 input enable"]
    #[inline(always)]
    pub fn pad4inpen(&mut self) -> PAD4INPEN_W {
        PAD4INPEN_W { w: self }
    }
    #[doc = "Bit 0 - Pad 4 pullup enable"]
    #[inline(always)]
    pub fn pad4pull(&mut self) -> PAD4PULL_W {
        PAD4PULL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Pad Configuration Register B (Pads 4-7)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [padregb](index.html) module"]
pub struct PADREGB_SPEC;
impl crate::RegisterSpec for PADREGB_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [padregb::R](R) reader structure"]
impl crate::Readable for PADREGB_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [padregb::W](W) writer structure"]
impl crate::Writable for PADREGB_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PADREGB to value 0x1818_1818"]
impl crate::Resettable for PADREGB_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x1818_1818
    }
}
