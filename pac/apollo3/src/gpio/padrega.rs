#[doc = "Register `PADREGA` reader"]
pub struct R(crate::R<PADREGA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PADREGA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PADREGA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PADREGA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PADREGA` writer"]
pub struct W(crate::W<PADREGA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PADREGA_SPEC>;
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
impl From<crate::W<PADREGA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PADREGA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Pad 3 VDD power switch enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD3PWRUP_A {
    #[doc = "0: Power switch disabled value."]
    DIS = 0,
    #[doc = "1: Power switch enabled (switched to VDD) value."]
    EN = 1,
}
impl From<PAD3PWRUP_A> for bool {
    #[inline(always)]
    fn from(variant: PAD3PWRUP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PAD3PWRUP` reader - Pad 3 VDD power switch enable"]
pub struct PAD3PWRUP_R(crate::FieldReader<bool, PAD3PWRUP_A>);
impl PAD3PWRUP_R {
    pub(crate) fn new(bits: bool) -> Self {
        PAD3PWRUP_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAD3PWRUP_A {
        match self.bits {
            false => PAD3PWRUP_A::DIS,
            true => PAD3PWRUP_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        **self == PAD3PWRUP_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        **self == PAD3PWRUP_A::EN
    }
}
impl core::ops::Deref for PAD3PWRUP_R {
    type Target = crate::FieldReader<bool, PAD3PWRUP_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PAD3PWRUP` writer - Pad 3 VDD power switch enable"]
pub struct PAD3PWRUP_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD3PWRUP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD3PWRUP_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Power switch disabled value."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(PAD3PWRUP_A::DIS)
    }
    #[doc = "Power switch enabled (switched to VDD) value."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(PAD3PWRUP_A::EN)
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | ((value as u32 & 0x01) << 30);
        self.w
    }
}
#[doc = "Pad 3 function select\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PAD3FNCSEL_A {
    #[doc = "0: Configure as the UART0 RTS output value."]
    UA0RTS = 0,
    #[doc = "1: Configure as the IOSLAVE SPI nCE signal value."]
    SLNCE = 1,
    #[doc = "2: IOM/MSPI nCE group 3 value."]
    NCE3 = 2,
    #[doc = "3: Configure as GPIO3 value."]
    GPIO3 = 3,
    #[doc = "5: MSPI data connection 7 value."]
    MSPI7 = 5,
    #[doc = "6: Configure as the ADC Trigger 1 signal value."]
    TRIG1 = 6,
    #[doc = "7: Configure as the PDM I2S Word Clock input value."]
    I2S_WCLK = 7,
}
impl From<PAD3FNCSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: PAD3FNCSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `PAD3FNCSEL` reader - Pad 3 function select"]
pub struct PAD3FNCSEL_R(crate::FieldReader<u8, PAD3FNCSEL_A>);
impl PAD3FNCSEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        PAD3FNCSEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PAD3FNCSEL_A> {
        match self.bits {
            0 => Some(PAD3FNCSEL_A::UA0RTS),
            1 => Some(PAD3FNCSEL_A::SLNCE),
            2 => Some(PAD3FNCSEL_A::NCE3),
            3 => Some(PAD3FNCSEL_A::GPIO3),
            5 => Some(PAD3FNCSEL_A::MSPI7),
            6 => Some(PAD3FNCSEL_A::TRIG1),
            7 => Some(PAD3FNCSEL_A::I2S_WCLK),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `UA0RTS`"]
    #[inline(always)]
    pub fn is_ua0rts(&self) -> bool {
        **self == PAD3FNCSEL_A::UA0RTS
    }
    #[doc = "Checks if the value of the field is `SLNCE`"]
    #[inline(always)]
    pub fn is_sln_ce(&self) -> bool {
        **self == PAD3FNCSEL_A::SLNCE
    }
    #[doc = "Checks if the value of the field is `NCE3`"]
    #[inline(always)]
    pub fn is_nce3(&self) -> bool {
        **self == PAD3FNCSEL_A::NCE3
    }
    #[doc = "Checks if the value of the field is `GPIO3`"]
    #[inline(always)]
    pub fn is_gpio3(&self) -> bool {
        **self == PAD3FNCSEL_A::GPIO3
    }
    #[doc = "Checks if the value of the field is `MSPI7`"]
    #[inline(always)]
    pub fn is_mspi7(&self) -> bool {
        **self == PAD3FNCSEL_A::MSPI7
    }
    #[doc = "Checks if the value of the field is `TRIG1`"]
    #[inline(always)]
    pub fn is_trig1(&self) -> bool {
        **self == PAD3FNCSEL_A::TRIG1
    }
    #[doc = "Checks if the value of the field is `I2S_WCLK`"]
    #[inline(always)]
    pub fn is_i2s_wclk(&self) -> bool {
        **self == PAD3FNCSEL_A::I2S_WCLK
    }
}
impl core::ops::Deref for PAD3FNCSEL_R {
    type Target = crate::FieldReader<u8, PAD3FNCSEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PAD3FNCSEL` writer - Pad 3 function select"]
pub struct PAD3FNCSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD3FNCSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD3FNCSEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Configure as the UART0 RTS output value."]
    #[inline(always)]
    pub fn ua0rts(self) -> &'a mut W {
        self.variant(PAD3FNCSEL_A::UA0RTS)
    }
    #[doc = "Configure as the IOSLAVE SPI nCE signal value."]
    #[inline(always)]
    pub fn sln_ce(self) -> &'a mut W {
        self.variant(PAD3FNCSEL_A::SLNCE)
    }
    #[doc = "IOM/MSPI nCE group 3 value."]
    #[inline(always)]
    pub fn nce3(self) -> &'a mut W {
        self.variant(PAD3FNCSEL_A::NCE3)
    }
    #[doc = "Configure as GPIO3 value."]
    #[inline(always)]
    pub fn gpio3(self) -> &'a mut W {
        self.variant(PAD3FNCSEL_A::GPIO3)
    }
    #[doc = "MSPI data connection 7 value."]
    #[inline(always)]
    pub fn mspi7(self) -> &'a mut W {
        self.variant(PAD3FNCSEL_A::MSPI7)
    }
    #[doc = "Configure as the ADC Trigger 1 signal value."]
    #[inline(always)]
    pub fn trig1(self) -> &'a mut W {
        self.variant(PAD3FNCSEL_A::TRIG1)
    }
    #[doc = "Configure as the PDM I2S Word Clock input value."]
    #[inline(always)]
    pub fn i2s_wclk(self) -> &'a mut W {
        self.variant(PAD3FNCSEL_A::I2S_WCLK)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 27)) | ((value as u32 & 0x07) << 27);
        self.w
    }
}
#[doc = "Pad 3 drive strength.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD3STRNG_A {
    #[doc = "0: Low drive strength value."]
    LOW = 0,
    #[doc = "1: High drive strength value."]
    HIGH = 1,
}
impl From<PAD3STRNG_A> for bool {
    #[inline(always)]
    fn from(variant: PAD3STRNG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PAD3STRNG` reader - Pad 3 drive strength."]
pub struct PAD3STRNG_R(crate::FieldReader<bool, PAD3STRNG_A>);
impl PAD3STRNG_R {
    pub(crate) fn new(bits: bool) -> Self {
        PAD3STRNG_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAD3STRNG_A {
        match self.bits {
            false => PAD3STRNG_A::LOW,
            true => PAD3STRNG_A::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        **self == PAD3STRNG_A::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        **self == PAD3STRNG_A::HIGH
    }
}
impl core::ops::Deref for PAD3STRNG_R {
    type Target = crate::FieldReader<bool, PAD3STRNG_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PAD3STRNG` writer - Pad 3 drive strength."]
pub struct PAD3STRNG_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD3STRNG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD3STRNG_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Low drive strength value."]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(PAD3STRNG_A::LOW)
    }
    #[doc = "High drive strength value."]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(PAD3STRNG_A::HIGH)
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
#[doc = "Pad 3 input enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD3INPEN_A {
    #[doc = "0: Pad input disabled value."]
    DIS = 0,
    #[doc = "1: Pad input enabled value."]
    EN = 1,
}
impl From<PAD3INPEN_A> for bool {
    #[inline(always)]
    fn from(variant: PAD3INPEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PAD3INPEN` reader - Pad 3 input enable."]
pub struct PAD3INPEN_R(crate::FieldReader<bool, PAD3INPEN_A>);
impl PAD3INPEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        PAD3INPEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAD3INPEN_A {
        match self.bits {
            false => PAD3INPEN_A::DIS,
            true => PAD3INPEN_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        **self == PAD3INPEN_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        **self == PAD3INPEN_A::EN
    }
}
impl core::ops::Deref for PAD3INPEN_R {
    type Target = crate::FieldReader<bool, PAD3INPEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PAD3INPEN` writer - Pad 3 input enable."]
pub struct PAD3INPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD3INPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD3INPEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Pad input disabled value."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(PAD3INPEN_A::DIS)
    }
    #[doc = "Pad input enabled value."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(PAD3INPEN_A::EN)
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
#[doc = "Pad 3 pullup enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD3PULL_A {
    #[doc = "0: Pullup disabled value."]
    DIS = 0,
    #[doc = "1: Pullup enabled value."]
    EN = 1,
}
impl From<PAD3PULL_A> for bool {
    #[inline(always)]
    fn from(variant: PAD3PULL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PAD3PULL` reader - Pad 3 pullup enable"]
pub struct PAD3PULL_R(crate::FieldReader<bool, PAD3PULL_A>);
impl PAD3PULL_R {
    pub(crate) fn new(bits: bool) -> Self {
        PAD3PULL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAD3PULL_A {
        match self.bits {
            false => PAD3PULL_A::DIS,
            true => PAD3PULL_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        **self == PAD3PULL_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        **self == PAD3PULL_A::EN
    }
}
impl core::ops::Deref for PAD3PULL_R {
    type Target = crate::FieldReader<bool, PAD3PULL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PAD3PULL` writer - Pad 3 pullup enable"]
pub struct PAD3PULL_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD3PULL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD3PULL_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Pullup disabled value."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(PAD3PULL_A::DIS)
    }
    #[doc = "Pullup enabled value."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(PAD3PULL_A::EN)
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
#[doc = "Pad 2 function select\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PAD2FNCSEL_A {
    #[doc = "1: Configure as the IOSLAVE SPI MISO signal value."]
    SLMISO = 1,
    #[doc = "2: Configure as the UART0 RX input value."]
    UART0RX = 2,
    #[doc = "3: Configure as GPIO2 value."]
    GPIO2 = 3,
    #[doc = "5: CMSPI data connection 6 value."]
    MSPI6 = 5,
    #[doc = "7: IOM/MSPI nCE group 2 value."]
    NCE2 = 7,
}
impl From<PAD2FNCSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: PAD2FNCSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `PAD2FNCSEL` reader - Pad 2 function select"]
pub struct PAD2FNCSEL_R(crate::FieldReader<u8, PAD2FNCSEL_A>);
impl PAD2FNCSEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        PAD2FNCSEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PAD2FNCSEL_A> {
        match self.bits {
            1 => Some(PAD2FNCSEL_A::SLMISO),
            2 => Some(PAD2FNCSEL_A::UART0RX),
            3 => Some(PAD2FNCSEL_A::GPIO2),
            5 => Some(PAD2FNCSEL_A::MSPI6),
            7 => Some(PAD2FNCSEL_A::NCE2),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `SLMISO`"]
    #[inline(always)]
    pub fn is_slmiso(&self) -> bool {
        **self == PAD2FNCSEL_A::SLMISO
    }
    #[doc = "Checks if the value of the field is `UART0RX`"]
    #[inline(always)]
    pub fn is_uart0rx(&self) -> bool {
        **self == PAD2FNCSEL_A::UART0RX
    }
    #[doc = "Checks if the value of the field is `GPIO2`"]
    #[inline(always)]
    pub fn is_gpio2(&self) -> bool {
        **self == PAD2FNCSEL_A::GPIO2
    }
    #[doc = "Checks if the value of the field is `MSPI6`"]
    #[inline(always)]
    pub fn is_mspi6(&self) -> bool {
        **self == PAD2FNCSEL_A::MSPI6
    }
    #[doc = "Checks if the value of the field is `NCE2`"]
    #[inline(always)]
    pub fn is_nce2(&self) -> bool {
        **self == PAD2FNCSEL_A::NCE2
    }
}
impl core::ops::Deref for PAD2FNCSEL_R {
    type Target = crate::FieldReader<u8, PAD2FNCSEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PAD2FNCSEL` writer - Pad 2 function select"]
pub struct PAD2FNCSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD2FNCSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD2FNCSEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Configure as the IOSLAVE SPI MISO signal value."]
    #[inline(always)]
    pub fn slmiso(self) -> &'a mut W {
        self.variant(PAD2FNCSEL_A::SLMISO)
    }
    #[doc = "Configure as the UART0 RX input value."]
    #[inline(always)]
    pub fn uart0rx(self) -> &'a mut W {
        self.variant(PAD2FNCSEL_A::UART0RX)
    }
    #[doc = "Configure as GPIO2 value."]
    #[inline(always)]
    pub fn gpio2(self) -> &'a mut W {
        self.variant(PAD2FNCSEL_A::GPIO2)
    }
    #[doc = "CMSPI data connection 6 value."]
    #[inline(always)]
    pub fn mspi6(self) -> &'a mut W {
        self.variant(PAD2FNCSEL_A::MSPI6)
    }
    #[doc = "IOM/MSPI nCE group 2 value."]
    #[inline(always)]
    pub fn nce2(self) -> &'a mut W {
        self.variant(PAD2FNCSEL_A::NCE2)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 19)) | ((value as u32 & 0x07) << 19);
        self.w
    }
}
#[doc = "Pad 2 drive strength\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD2STRNG_A {
    #[doc = "0: Low drive strength value."]
    LOW = 0,
    #[doc = "1: High drive strength value."]
    HIGH = 1,
}
impl From<PAD2STRNG_A> for bool {
    #[inline(always)]
    fn from(variant: PAD2STRNG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PAD2STRNG` reader - Pad 2 drive strength"]
pub struct PAD2STRNG_R(crate::FieldReader<bool, PAD2STRNG_A>);
impl PAD2STRNG_R {
    pub(crate) fn new(bits: bool) -> Self {
        PAD2STRNG_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAD2STRNG_A {
        match self.bits {
            false => PAD2STRNG_A::LOW,
            true => PAD2STRNG_A::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        **self == PAD2STRNG_A::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        **self == PAD2STRNG_A::HIGH
    }
}
impl core::ops::Deref for PAD2STRNG_R {
    type Target = crate::FieldReader<bool, PAD2STRNG_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PAD2STRNG` writer - Pad 2 drive strength"]
pub struct PAD2STRNG_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD2STRNG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD2STRNG_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Low drive strength value."]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(PAD2STRNG_A::LOW)
    }
    #[doc = "High drive strength value."]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(PAD2STRNG_A::HIGH)
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
#[doc = "Pad 2 input enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD2INPEN_A {
    #[doc = "0: Pad input disabled value."]
    DIS = 0,
    #[doc = "1: Pad input enabled value."]
    EN = 1,
}
impl From<PAD2INPEN_A> for bool {
    #[inline(always)]
    fn from(variant: PAD2INPEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PAD2INPEN` reader - Pad 2 input enable"]
pub struct PAD2INPEN_R(crate::FieldReader<bool, PAD2INPEN_A>);
impl PAD2INPEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        PAD2INPEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAD2INPEN_A {
        match self.bits {
            false => PAD2INPEN_A::DIS,
            true => PAD2INPEN_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        **self == PAD2INPEN_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        **self == PAD2INPEN_A::EN
    }
}
impl core::ops::Deref for PAD2INPEN_R {
    type Target = crate::FieldReader<bool, PAD2INPEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PAD2INPEN` writer - Pad 2 input enable"]
pub struct PAD2INPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD2INPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD2INPEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Pad input disabled value."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(PAD2INPEN_A::DIS)
    }
    #[doc = "Pad input enabled value."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(PAD2INPEN_A::EN)
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
#[doc = "Pad 2 pullup enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD2PULL_A {
    #[doc = "0: Pullup disabled value."]
    DIS = 0,
    #[doc = "1: Pullup enabled value."]
    EN = 1,
}
impl From<PAD2PULL_A> for bool {
    #[inline(always)]
    fn from(variant: PAD2PULL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PAD2PULL` reader - Pad 2 pullup enable"]
pub struct PAD2PULL_R(crate::FieldReader<bool, PAD2PULL_A>);
impl PAD2PULL_R {
    pub(crate) fn new(bits: bool) -> Self {
        PAD2PULL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAD2PULL_A {
        match self.bits {
            false => PAD2PULL_A::DIS,
            true => PAD2PULL_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        **self == PAD2PULL_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        **self == PAD2PULL_A::EN
    }
}
impl core::ops::Deref for PAD2PULL_R {
    type Target = crate::FieldReader<bool, PAD2PULL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PAD2PULL` writer - Pad 2 pullup enable"]
pub struct PAD2PULL_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD2PULL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD2PULL_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Pullup disabled value."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(PAD2PULL_A::DIS)
    }
    #[doc = "Pullup enabled value."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(PAD2PULL_A::EN)
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
#[doc = "Pad 1 pullup resistor selection.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PAD1RSEL_A {
    #[doc = "0: Pullup is ~1.5 KOhms value."]
    PULL1_5K = 0,
    #[doc = "1: Pullup is ~6 KOhms value."]
    PULL6K = 1,
    #[doc = "2: Pullup is ~12 KOhms value."]
    PULL12K = 2,
    #[doc = "3: Pullup is ~24 KOhms value."]
    PULL24K = 3,
}
impl From<PAD1RSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: PAD1RSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `PAD1RSEL` reader - Pad 1 pullup resistor selection."]
pub struct PAD1RSEL_R(crate::FieldReader<u8, PAD1RSEL_A>);
impl PAD1RSEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        PAD1RSEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAD1RSEL_A {
        match self.bits {
            0 => PAD1RSEL_A::PULL1_5K,
            1 => PAD1RSEL_A::PULL6K,
            2 => PAD1RSEL_A::PULL12K,
            3 => PAD1RSEL_A::PULL24K,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PULL1_5K`"]
    #[inline(always)]
    pub fn is_pull1_5k(&self) -> bool {
        **self == PAD1RSEL_A::PULL1_5K
    }
    #[doc = "Checks if the value of the field is `PULL6K`"]
    #[inline(always)]
    pub fn is_pull6k(&self) -> bool {
        **self == PAD1RSEL_A::PULL6K
    }
    #[doc = "Checks if the value of the field is `PULL12K`"]
    #[inline(always)]
    pub fn is_pull12k(&self) -> bool {
        **self == PAD1RSEL_A::PULL12K
    }
    #[doc = "Checks if the value of the field is `PULL24K`"]
    #[inline(always)]
    pub fn is_pull24k(&self) -> bool {
        **self == PAD1RSEL_A::PULL24K
    }
}
impl core::ops::Deref for PAD1RSEL_R {
    type Target = crate::FieldReader<u8, PAD1RSEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PAD1RSEL` writer - Pad 1 pullup resistor selection."]
pub struct PAD1RSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD1RSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD1RSEL_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Pullup is ~1.5 KOhms value."]
    #[inline(always)]
    pub fn pull1_5k(self) -> &'a mut W {
        self.variant(PAD1RSEL_A::PULL1_5K)
    }
    #[doc = "Pullup is ~6 KOhms value."]
    #[inline(always)]
    pub fn pull6k(self) -> &'a mut W {
        self.variant(PAD1RSEL_A::PULL6K)
    }
    #[doc = "Pullup is ~12 KOhms value."]
    #[inline(always)]
    pub fn pull12k(self) -> &'a mut W {
        self.variant(PAD1RSEL_A::PULL12K)
    }
    #[doc = "Pullup is ~24 KOhms value."]
    #[inline(always)]
    pub fn pull24k(self) -> &'a mut W {
        self.variant(PAD1RSEL_A::PULL24K)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 14)) | ((value as u32 & 0x03) << 14);
        self.w
    }
}
#[doc = "Pad 1 function select\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PAD1FNCSEL_A {
    #[doc = "0: Configure as the IOSLAVE I2C SDA or SPI WIR3 signal value."]
    SLSDAWIR3 = 0,
    #[doc = "1: Configure as the IOSLAVE SPI MOSI signal value."]
    SLMOSI = 1,
    #[doc = "2: Configure as the UART0 TX output signal value."]
    UART0TX = 2,
    #[doc = "3: Configure as GPIO1 value."]
    GPIO1 = 3,
    #[doc = "5: MSPI data connection 5 value."]
    MSPI5 = 5,
    #[doc = "7: IOM/MSPI nCE group 1 value."]
    NCE1 = 7,
}
impl From<PAD1FNCSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: PAD1FNCSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `PAD1FNCSEL` reader - Pad 1 function select"]
pub struct PAD1FNCSEL_R(crate::FieldReader<u8, PAD1FNCSEL_A>);
impl PAD1FNCSEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        PAD1FNCSEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PAD1FNCSEL_A> {
        match self.bits {
            0 => Some(PAD1FNCSEL_A::SLSDAWIR3),
            1 => Some(PAD1FNCSEL_A::SLMOSI),
            2 => Some(PAD1FNCSEL_A::UART0TX),
            3 => Some(PAD1FNCSEL_A::GPIO1),
            5 => Some(PAD1FNCSEL_A::MSPI5),
            7 => Some(PAD1FNCSEL_A::NCE1),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `SLSDAWIR3`"]
    #[inline(always)]
    pub fn is_slsdawir3(&self) -> bool {
        **self == PAD1FNCSEL_A::SLSDAWIR3
    }
    #[doc = "Checks if the value of the field is `SLMOSI`"]
    #[inline(always)]
    pub fn is_slmosi(&self) -> bool {
        **self == PAD1FNCSEL_A::SLMOSI
    }
    #[doc = "Checks if the value of the field is `UART0TX`"]
    #[inline(always)]
    pub fn is_uart0tx(&self) -> bool {
        **self == PAD1FNCSEL_A::UART0TX
    }
    #[doc = "Checks if the value of the field is `GPIO1`"]
    #[inline(always)]
    pub fn is_gpio1(&self) -> bool {
        **self == PAD1FNCSEL_A::GPIO1
    }
    #[doc = "Checks if the value of the field is `MSPI5`"]
    #[inline(always)]
    pub fn is_mspi5(&self) -> bool {
        **self == PAD1FNCSEL_A::MSPI5
    }
    #[doc = "Checks if the value of the field is `NCE1`"]
    #[inline(always)]
    pub fn is_nce1(&self) -> bool {
        **self == PAD1FNCSEL_A::NCE1
    }
}
impl core::ops::Deref for PAD1FNCSEL_R {
    type Target = crate::FieldReader<u8, PAD1FNCSEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PAD1FNCSEL` writer - Pad 1 function select"]
pub struct PAD1FNCSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD1FNCSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD1FNCSEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Configure as the IOSLAVE I2C SDA or SPI WIR3 signal value."]
    #[inline(always)]
    pub fn slsdawir3(self) -> &'a mut W {
        self.variant(PAD1FNCSEL_A::SLSDAWIR3)
    }
    #[doc = "Configure as the IOSLAVE SPI MOSI signal value."]
    #[inline(always)]
    pub fn slmosi(self) -> &'a mut W {
        self.variant(PAD1FNCSEL_A::SLMOSI)
    }
    #[doc = "Configure as the UART0 TX output signal value."]
    #[inline(always)]
    pub fn uart0tx(self) -> &'a mut W {
        self.variant(PAD1FNCSEL_A::UART0TX)
    }
    #[doc = "Configure as GPIO1 value."]
    #[inline(always)]
    pub fn gpio1(self) -> &'a mut W {
        self.variant(PAD1FNCSEL_A::GPIO1)
    }
    #[doc = "MSPI data connection 5 value."]
    #[inline(always)]
    pub fn mspi5(self) -> &'a mut W {
        self.variant(PAD1FNCSEL_A::MSPI5)
    }
    #[doc = "IOM/MSPI nCE group 1 value."]
    #[inline(always)]
    pub fn nce1(self) -> &'a mut W {
        self.variant(PAD1FNCSEL_A::NCE1)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 11)) | ((value as u32 & 0x07) << 11);
        self.w
    }
}
#[doc = "Pad 1 drive strength\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD1STRNG_A {
    #[doc = "0: Low drive strength value."]
    LOW = 0,
    #[doc = "1: High drive strength value."]
    HIGH = 1,
}
impl From<PAD1STRNG_A> for bool {
    #[inline(always)]
    fn from(variant: PAD1STRNG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PAD1STRNG` reader - Pad 1 drive strength"]
pub struct PAD1STRNG_R(crate::FieldReader<bool, PAD1STRNG_A>);
impl PAD1STRNG_R {
    pub(crate) fn new(bits: bool) -> Self {
        PAD1STRNG_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAD1STRNG_A {
        match self.bits {
            false => PAD1STRNG_A::LOW,
            true => PAD1STRNG_A::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        **self == PAD1STRNG_A::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        **self == PAD1STRNG_A::HIGH
    }
}
impl core::ops::Deref for PAD1STRNG_R {
    type Target = crate::FieldReader<bool, PAD1STRNG_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PAD1STRNG` writer - Pad 1 drive strength"]
pub struct PAD1STRNG_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD1STRNG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD1STRNG_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Low drive strength value."]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(PAD1STRNG_A::LOW)
    }
    #[doc = "High drive strength value."]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(PAD1STRNG_A::HIGH)
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
#[doc = "Pad 1 input enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD1INPEN_A {
    #[doc = "0: Pad input disabled value."]
    DIS = 0,
    #[doc = "1: Pad input enabled value."]
    EN = 1,
}
impl From<PAD1INPEN_A> for bool {
    #[inline(always)]
    fn from(variant: PAD1INPEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PAD1INPEN` reader - Pad 1 input enable"]
pub struct PAD1INPEN_R(crate::FieldReader<bool, PAD1INPEN_A>);
impl PAD1INPEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        PAD1INPEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAD1INPEN_A {
        match self.bits {
            false => PAD1INPEN_A::DIS,
            true => PAD1INPEN_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        **self == PAD1INPEN_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        **self == PAD1INPEN_A::EN
    }
}
impl core::ops::Deref for PAD1INPEN_R {
    type Target = crate::FieldReader<bool, PAD1INPEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PAD1INPEN` writer - Pad 1 input enable"]
pub struct PAD1INPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD1INPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD1INPEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Pad input disabled value."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(PAD1INPEN_A::DIS)
    }
    #[doc = "Pad input enabled value."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(PAD1INPEN_A::EN)
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
#[doc = "Pad 1 pullup enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD1PULL_A {
    #[doc = "0: Pullup disabled value."]
    DIS = 0,
    #[doc = "1: Pullup enabled value."]
    EN = 1,
}
impl From<PAD1PULL_A> for bool {
    #[inline(always)]
    fn from(variant: PAD1PULL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PAD1PULL` reader - Pad 1 pullup enable"]
pub struct PAD1PULL_R(crate::FieldReader<bool, PAD1PULL_A>);
impl PAD1PULL_R {
    pub(crate) fn new(bits: bool) -> Self {
        PAD1PULL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAD1PULL_A {
        match self.bits {
            false => PAD1PULL_A::DIS,
            true => PAD1PULL_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        **self == PAD1PULL_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        **self == PAD1PULL_A::EN
    }
}
impl core::ops::Deref for PAD1PULL_R {
    type Target = crate::FieldReader<bool, PAD1PULL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PAD1PULL` writer - Pad 1 pullup enable"]
pub struct PAD1PULL_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD1PULL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD1PULL_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Pullup disabled value."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(PAD1PULL_A::DIS)
    }
    #[doc = "Pullup enabled value."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(PAD1PULL_A::EN)
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
#[doc = "Pad 0 pullup resistor selection.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PAD0RSEL_A {
    #[doc = "0: Pullup is ~1.5 KOhms value."]
    PULL1_5K = 0,
    #[doc = "1: Pullup is ~6 KOhms value."]
    PULL6K = 1,
    #[doc = "2: Pullup is ~12 KOhms value."]
    PULL12K = 2,
    #[doc = "3: Pullup is ~24 KOhms value."]
    PULL24K = 3,
}
impl From<PAD0RSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: PAD0RSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `PAD0RSEL` reader - Pad 0 pullup resistor selection."]
pub struct PAD0RSEL_R(crate::FieldReader<u8, PAD0RSEL_A>);
impl PAD0RSEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        PAD0RSEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAD0RSEL_A {
        match self.bits {
            0 => PAD0RSEL_A::PULL1_5K,
            1 => PAD0RSEL_A::PULL6K,
            2 => PAD0RSEL_A::PULL12K,
            3 => PAD0RSEL_A::PULL24K,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PULL1_5K`"]
    #[inline(always)]
    pub fn is_pull1_5k(&self) -> bool {
        **self == PAD0RSEL_A::PULL1_5K
    }
    #[doc = "Checks if the value of the field is `PULL6K`"]
    #[inline(always)]
    pub fn is_pull6k(&self) -> bool {
        **self == PAD0RSEL_A::PULL6K
    }
    #[doc = "Checks if the value of the field is `PULL12K`"]
    #[inline(always)]
    pub fn is_pull12k(&self) -> bool {
        **self == PAD0RSEL_A::PULL12K
    }
    #[doc = "Checks if the value of the field is `PULL24K`"]
    #[inline(always)]
    pub fn is_pull24k(&self) -> bool {
        **self == PAD0RSEL_A::PULL24K
    }
}
impl core::ops::Deref for PAD0RSEL_R {
    type Target = crate::FieldReader<u8, PAD0RSEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PAD0RSEL` writer - Pad 0 pullup resistor selection."]
pub struct PAD0RSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD0RSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD0RSEL_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Pullup is ~1.5 KOhms value."]
    #[inline(always)]
    pub fn pull1_5k(self) -> &'a mut W {
        self.variant(PAD0RSEL_A::PULL1_5K)
    }
    #[doc = "Pullup is ~6 KOhms value."]
    #[inline(always)]
    pub fn pull6k(self) -> &'a mut W {
        self.variant(PAD0RSEL_A::PULL6K)
    }
    #[doc = "Pullup is ~12 KOhms value."]
    #[inline(always)]
    pub fn pull12k(self) -> &'a mut W {
        self.variant(PAD0RSEL_A::PULL12K)
    }
    #[doc = "Pullup is ~24 KOhms value."]
    #[inline(always)]
    pub fn pull24k(self) -> &'a mut W {
        self.variant(PAD0RSEL_A::PULL24K)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | ((value as u32 & 0x03) << 6);
        self.w
    }
}
#[doc = "Pad 0 function select\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PAD0FNCSEL_A {
    #[doc = "0: Configure as the IOSLAVE I2C SCL signal value."]
    SLSCL = 0,
    #[doc = "1: Configure as the IOSLAVE SPI SCK signal value."]
    SLSCK = 1,
    #[doc = "2: Configure as the CLKOUT signal value."]
    CLKOUT = 2,
    #[doc = "3: Configure as GPIO0 value."]
    GPIO0 = 3,
    #[doc = "5: MSPI data connection 4 value."]
    MSPI4 = 5,
    #[doc = "7: IOM/MSPI nCE group 0 value."]
    NCE0 = 7,
}
impl From<PAD0FNCSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: PAD0FNCSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `PAD0FNCSEL` reader - Pad 0 function select"]
pub struct PAD0FNCSEL_R(crate::FieldReader<u8, PAD0FNCSEL_A>);
impl PAD0FNCSEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        PAD0FNCSEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PAD0FNCSEL_A> {
        match self.bits {
            0 => Some(PAD0FNCSEL_A::SLSCL),
            1 => Some(PAD0FNCSEL_A::SLSCK),
            2 => Some(PAD0FNCSEL_A::CLKOUT),
            3 => Some(PAD0FNCSEL_A::GPIO0),
            5 => Some(PAD0FNCSEL_A::MSPI4),
            7 => Some(PAD0FNCSEL_A::NCE0),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `SLSCL`"]
    #[inline(always)]
    pub fn is_slscl(&self) -> bool {
        **self == PAD0FNCSEL_A::SLSCL
    }
    #[doc = "Checks if the value of the field is `SLSCK`"]
    #[inline(always)]
    pub fn is_slsck(&self) -> bool {
        **self == PAD0FNCSEL_A::SLSCK
    }
    #[doc = "Checks if the value of the field is `CLKOUT`"]
    #[inline(always)]
    pub fn is_clkout(&self) -> bool {
        **self == PAD0FNCSEL_A::CLKOUT
    }
    #[doc = "Checks if the value of the field is `GPIO0`"]
    #[inline(always)]
    pub fn is_gpio0(&self) -> bool {
        **self == PAD0FNCSEL_A::GPIO0
    }
    #[doc = "Checks if the value of the field is `MSPI4`"]
    #[inline(always)]
    pub fn is_mspi4(&self) -> bool {
        **self == PAD0FNCSEL_A::MSPI4
    }
    #[doc = "Checks if the value of the field is `NCE0`"]
    #[inline(always)]
    pub fn is_nce0(&self) -> bool {
        **self == PAD0FNCSEL_A::NCE0
    }
}
impl core::ops::Deref for PAD0FNCSEL_R {
    type Target = crate::FieldReader<u8, PAD0FNCSEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PAD0FNCSEL` writer - Pad 0 function select"]
pub struct PAD0FNCSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD0FNCSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD0FNCSEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Configure as the IOSLAVE I2C SCL signal value."]
    #[inline(always)]
    pub fn slscl(self) -> &'a mut W {
        self.variant(PAD0FNCSEL_A::SLSCL)
    }
    #[doc = "Configure as the IOSLAVE SPI SCK signal value."]
    #[inline(always)]
    pub fn slsck(self) -> &'a mut W {
        self.variant(PAD0FNCSEL_A::SLSCK)
    }
    #[doc = "Configure as the CLKOUT signal value."]
    #[inline(always)]
    pub fn clkout(self) -> &'a mut W {
        self.variant(PAD0FNCSEL_A::CLKOUT)
    }
    #[doc = "Configure as GPIO0 value."]
    #[inline(always)]
    pub fn gpio0(self) -> &'a mut W {
        self.variant(PAD0FNCSEL_A::GPIO0)
    }
    #[doc = "MSPI data connection 4 value."]
    #[inline(always)]
    pub fn mspi4(self) -> &'a mut W {
        self.variant(PAD0FNCSEL_A::MSPI4)
    }
    #[doc = "IOM/MSPI nCE group 0 value."]
    #[inline(always)]
    pub fn nce0(self) -> &'a mut W {
        self.variant(PAD0FNCSEL_A::NCE0)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 3)) | ((value as u32 & 0x07) << 3);
        self.w
    }
}
#[doc = "Pad 0 drive strength\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD0STRNG_A {
    #[doc = "0: Low drive strength value."]
    LOW = 0,
    #[doc = "1: High drive strength value."]
    HIGH = 1,
}
impl From<PAD0STRNG_A> for bool {
    #[inline(always)]
    fn from(variant: PAD0STRNG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PAD0STRNG` reader - Pad 0 drive strength"]
pub struct PAD0STRNG_R(crate::FieldReader<bool, PAD0STRNG_A>);
impl PAD0STRNG_R {
    pub(crate) fn new(bits: bool) -> Self {
        PAD0STRNG_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAD0STRNG_A {
        match self.bits {
            false => PAD0STRNG_A::LOW,
            true => PAD0STRNG_A::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        **self == PAD0STRNG_A::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        **self == PAD0STRNG_A::HIGH
    }
}
impl core::ops::Deref for PAD0STRNG_R {
    type Target = crate::FieldReader<bool, PAD0STRNG_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PAD0STRNG` writer - Pad 0 drive strength"]
pub struct PAD0STRNG_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD0STRNG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD0STRNG_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Low drive strength value."]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(PAD0STRNG_A::LOW)
    }
    #[doc = "High drive strength value."]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(PAD0STRNG_A::HIGH)
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
#[doc = "Pad 0 input enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD0INPEN_A {
    #[doc = "0: Pad input disabled value."]
    DIS = 0,
    #[doc = "1: Pad input enabled value."]
    EN = 1,
}
impl From<PAD0INPEN_A> for bool {
    #[inline(always)]
    fn from(variant: PAD0INPEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PAD0INPEN` reader - Pad 0 input enable"]
pub struct PAD0INPEN_R(crate::FieldReader<bool, PAD0INPEN_A>);
impl PAD0INPEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        PAD0INPEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAD0INPEN_A {
        match self.bits {
            false => PAD0INPEN_A::DIS,
            true => PAD0INPEN_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        **self == PAD0INPEN_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        **self == PAD0INPEN_A::EN
    }
}
impl core::ops::Deref for PAD0INPEN_R {
    type Target = crate::FieldReader<bool, PAD0INPEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PAD0INPEN` writer - Pad 0 input enable"]
pub struct PAD0INPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD0INPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD0INPEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Pad input disabled value."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(PAD0INPEN_A::DIS)
    }
    #[doc = "Pad input enabled value."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(PAD0INPEN_A::EN)
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
#[doc = "Pad 0 pullup enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD0PULL_A {
    #[doc = "0: Pullup disabled value."]
    DIS = 0,
    #[doc = "1: Pullup enabled value."]
    EN = 1,
}
impl From<PAD0PULL_A> for bool {
    #[inline(always)]
    fn from(variant: PAD0PULL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PAD0PULL` reader - Pad 0 pullup enable"]
pub struct PAD0PULL_R(crate::FieldReader<bool, PAD0PULL_A>);
impl PAD0PULL_R {
    pub(crate) fn new(bits: bool) -> Self {
        PAD0PULL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAD0PULL_A {
        match self.bits {
            false => PAD0PULL_A::DIS,
            true => PAD0PULL_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        **self == PAD0PULL_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        **self == PAD0PULL_A::EN
    }
}
impl core::ops::Deref for PAD0PULL_R {
    type Target = crate::FieldReader<bool, PAD0PULL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PAD0PULL` writer - Pad 0 pullup enable"]
pub struct PAD0PULL_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD0PULL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD0PULL_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Pullup disabled value."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(PAD0PULL_A::DIS)
    }
    #[doc = "Pullup enabled value."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(PAD0PULL_A::EN)
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
    #[doc = "Bit 30 - Pad 3 VDD power switch enable"]
    #[inline(always)]
    pub fn pad3pwrup(&self) -> PAD3PWRUP_R {
        PAD3PWRUP_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bits 27:29 - Pad 3 function select"]
    #[inline(always)]
    pub fn pad3fncsel(&self) -> PAD3FNCSEL_R {
        PAD3FNCSEL_R::new(((self.bits >> 27) & 0x07) as u8)
    }
    #[doc = "Bit 26 - Pad 3 drive strength."]
    #[inline(always)]
    pub fn pad3strng(&self) -> PAD3STRNG_R {
        PAD3STRNG_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Pad 3 input enable."]
    #[inline(always)]
    pub fn pad3inpen(&self) -> PAD3INPEN_R {
        PAD3INPEN_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Pad 3 pullup enable"]
    #[inline(always)]
    pub fn pad3pull(&self) -> PAD3PULL_R {
        PAD3PULL_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bits 19:21 - Pad 2 function select"]
    #[inline(always)]
    pub fn pad2fncsel(&self) -> PAD2FNCSEL_R {
        PAD2FNCSEL_R::new(((self.bits >> 19) & 0x07) as u8)
    }
    #[doc = "Bit 18 - Pad 2 drive strength"]
    #[inline(always)]
    pub fn pad2strng(&self) -> PAD2STRNG_R {
        PAD2STRNG_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Pad 2 input enable"]
    #[inline(always)]
    pub fn pad2inpen(&self) -> PAD2INPEN_R {
        PAD2INPEN_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Pad 2 pullup enable"]
    #[inline(always)]
    pub fn pad2pull(&self) -> PAD2PULL_R {
        PAD2PULL_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bits 14:15 - Pad 1 pullup resistor selection."]
    #[inline(always)]
    pub fn pad1rsel(&self) -> PAD1RSEL_R {
        PAD1RSEL_R::new(((self.bits >> 14) & 0x03) as u8)
    }
    #[doc = "Bits 11:13 - Pad 1 function select"]
    #[inline(always)]
    pub fn pad1fncsel(&self) -> PAD1FNCSEL_R {
        PAD1FNCSEL_R::new(((self.bits >> 11) & 0x07) as u8)
    }
    #[doc = "Bit 10 - Pad 1 drive strength"]
    #[inline(always)]
    pub fn pad1strng(&self) -> PAD1STRNG_R {
        PAD1STRNG_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Pad 1 input enable"]
    #[inline(always)]
    pub fn pad1inpen(&self) -> PAD1INPEN_R {
        PAD1INPEN_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Pad 1 pullup enable"]
    #[inline(always)]
    pub fn pad1pull(&self) -> PAD1PULL_R {
        PAD1PULL_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bits 6:7 - Pad 0 pullup resistor selection."]
    #[inline(always)]
    pub fn pad0rsel(&self) -> PAD0RSEL_R {
        PAD0RSEL_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bits 3:5 - Pad 0 function select"]
    #[inline(always)]
    pub fn pad0fncsel(&self) -> PAD0FNCSEL_R {
        PAD0FNCSEL_R::new(((self.bits >> 3) & 0x07) as u8)
    }
    #[doc = "Bit 2 - Pad 0 drive strength"]
    #[inline(always)]
    pub fn pad0strng(&self) -> PAD0STRNG_R {
        PAD0STRNG_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - Pad 0 input enable"]
    #[inline(always)]
    pub fn pad0inpen(&self) -> PAD0INPEN_R {
        PAD0INPEN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Pad 0 pullup enable"]
    #[inline(always)]
    pub fn pad0pull(&self) -> PAD0PULL_R {
        PAD0PULL_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 30 - Pad 3 VDD power switch enable"]
    #[inline(always)]
    pub fn pad3pwrup(&mut self) -> PAD3PWRUP_W {
        PAD3PWRUP_W { w: self }
    }
    #[doc = "Bits 27:29 - Pad 3 function select"]
    #[inline(always)]
    pub fn pad3fncsel(&mut self) -> PAD3FNCSEL_W {
        PAD3FNCSEL_W { w: self }
    }
    #[doc = "Bit 26 - Pad 3 drive strength."]
    #[inline(always)]
    pub fn pad3strng(&mut self) -> PAD3STRNG_W {
        PAD3STRNG_W { w: self }
    }
    #[doc = "Bit 25 - Pad 3 input enable."]
    #[inline(always)]
    pub fn pad3inpen(&mut self) -> PAD3INPEN_W {
        PAD3INPEN_W { w: self }
    }
    #[doc = "Bit 24 - Pad 3 pullup enable"]
    #[inline(always)]
    pub fn pad3pull(&mut self) -> PAD3PULL_W {
        PAD3PULL_W { w: self }
    }
    #[doc = "Bits 19:21 - Pad 2 function select"]
    #[inline(always)]
    pub fn pad2fncsel(&mut self) -> PAD2FNCSEL_W {
        PAD2FNCSEL_W { w: self }
    }
    #[doc = "Bit 18 - Pad 2 drive strength"]
    #[inline(always)]
    pub fn pad2strng(&mut self) -> PAD2STRNG_W {
        PAD2STRNG_W { w: self }
    }
    #[doc = "Bit 17 - Pad 2 input enable"]
    #[inline(always)]
    pub fn pad2inpen(&mut self) -> PAD2INPEN_W {
        PAD2INPEN_W { w: self }
    }
    #[doc = "Bit 16 - Pad 2 pullup enable"]
    #[inline(always)]
    pub fn pad2pull(&mut self) -> PAD2PULL_W {
        PAD2PULL_W { w: self }
    }
    #[doc = "Bits 14:15 - Pad 1 pullup resistor selection."]
    #[inline(always)]
    pub fn pad1rsel(&mut self) -> PAD1RSEL_W {
        PAD1RSEL_W { w: self }
    }
    #[doc = "Bits 11:13 - Pad 1 function select"]
    #[inline(always)]
    pub fn pad1fncsel(&mut self) -> PAD1FNCSEL_W {
        PAD1FNCSEL_W { w: self }
    }
    #[doc = "Bit 10 - Pad 1 drive strength"]
    #[inline(always)]
    pub fn pad1strng(&mut self) -> PAD1STRNG_W {
        PAD1STRNG_W { w: self }
    }
    #[doc = "Bit 9 - Pad 1 input enable"]
    #[inline(always)]
    pub fn pad1inpen(&mut self) -> PAD1INPEN_W {
        PAD1INPEN_W { w: self }
    }
    #[doc = "Bit 8 - Pad 1 pullup enable"]
    #[inline(always)]
    pub fn pad1pull(&mut self) -> PAD1PULL_W {
        PAD1PULL_W { w: self }
    }
    #[doc = "Bits 6:7 - Pad 0 pullup resistor selection."]
    #[inline(always)]
    pub fn pad0rsel(&mut self) -> PAD0RSEL_W {
        PAD0RSEL_W { w: self }
    }
    #[doc = "Bits 3:5 - Pad 0 function select"]
    #[inline(always)]
    pub fn pad0fncsel(&mut self) -> PAD0FNCSEL_W {
        PAD0FNCSEL_W { w: self }
    }
    #[doc = "Bit 2 - Pad 0 drive strength"]
    #[inline(always)]
    pub fn pad0strng(&mut self) -> PAD0STRNG_W {
        PAD0STRNG_W { w: self }
    }
    #[doc = "Bit 1 - Pad 0 input enable"]
    #[inline(always)]
    pub fn pad0inpen(&mut self) -> PAD0INPEN_W {
        PAD0INPEN_W { w: self }
    }
    #[doc = "Bit 0 - Pad 0 pullup enable"]
    #[inline(always)]
    pub fn pad0pull(&mut self) -> PAD0PULL_W {
        PAD0PULL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Pad Configuration Register A (Pads 0-3)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [padrega](index.html) module"]
pub struct PADREGA_SPEC;
impl crate::RegisterSpec for PADREGA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [padrega::R](R) reader structure"]
impl crate::Readable for PADREGA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [padrega::W](W) writer structure"]
impl crate::Writable for PADREGA_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PADREGA to value 0x1818_1818"]
impl crate::Resettable for PADREGA_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x1818_1818
    }
}
