#[doc = "Register `PADREGH` reader"]
pub struct R(crate::R<PADREGH_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PADREGH_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PADREGH_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PADREGH_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PADREGH` writer"]
pub struct W(crate::W<PADREGH_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PADREGH_SPEC>;
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
impl From<crate::W<PADREGH_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PADREGH_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Pad 31 function select\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PAD31FNCSEL_A {
    #[doc = "0: Configure as the analog input for ADC single ended input 3 value."]
    ADCSE3 = 0,
    #[doc = "1: IOM/MSPI nCE group 31 value."]
    NCE31 = 1,
    #[doc = "2: CTIMER connection 13 value."]
    CT13 = 2,
    #[doc = "3: Configure as GPIO31 value."]
    GPIO31 = 3,
    #[doc = "4: Configure as the UART0 RX input signal value."]
    UART0RX = 4,
    #[doc = "5: SCARD serial clock output value."]
    SCCCLK = 5,
    #[doc = "7: Configure as UART1 RTS output signal value."]
    UA1RTS = 7,
}
impl From<PAD31FNCSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: PAD31FNCSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `PAD31FNCSEL` reader - Pad 31 function select"]
pub struct PAD31FNCSEL_R(crate::FieldReader<u8, PAD31FNCSEL_A>);
impl PAD31FNCSEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        PAD31FNCSEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PAD31FNCSEL_A> {
        match self.bits {
            0 => Some(PAD31FNCSEL_A::ADCSE3),
            1 => Some(PAD31FNCSEL_A::NCE31),
            2 => Some(PAD31FNCSEL_A::CT13),
            3 => Some(PAD31FNCSEL_A::GPIO31),
            4 => Some(PAD31FNCSEL_A::UART0RX),
            5 => Some(PAD31FNCSEL_A::SCCCLK),
            7 => Some(PAD31FNCSEL_A::UA1RTS),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `ADCSE3`"]
    #[inline(always)]
    pub fn is_adcse3(&self) -> bool {
        **self == PAD31FNCSEL_A::ADCSE3
    }
    #[doc = "Checks if the value of the field is `NCE31`"]
    #[inline(always)]
    pub fn is_nce31(&self) -> bool {
        **self == PAD31FNCSEL_A::NCE31
    }
    #[doc = "Checks if the value of the field is `CT13`"]
    #[inline(always)]
    pub fn is_ct13(&self) -> bool {
        **self == PAD31FNCSEL_A::CT13
    }
    #[doc = "Checks if the value of the field is `GPIO31`"]
    #[inline(always)]
    pub fn is_gpio31(&self) -> bool {
        **self == PAD31FNCSEL_A::GPIO31
    }
    #[doc = "Checks if the value of the field is `UART0RX`"]
    #[inline(always)]
    pub fn is_uart0rx(&self) -> bool {
        **self == PAD31FNCSEL_A::UART0RX
    }
    #[doc = "Checks if the value of the field is `SCCCLK`"]
    #[inline(always)]
    pub fn is_sccclk(&self) -> bool {
        **self == PAD31FNCSEL_A::SCCCLK
    }
    #[doc = "Checks if the value of the field is `UA1RTS`"]
    #[inline(always)]
    pub fn is_ua1rts(&self) -> bool {
        **self == PAD31FNCSEL_A::UA1RTS
    }
}
impl core::ops::Deref for PAD31FNCSEL_R {
    type Target = crate::FieldReader<u8, PAD31FNCSEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PAD31FNCSEL` writer - Pad 31 function select"]
pub struct PAD31FNCSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD31FNCSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD31FNCSEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Configure as the analog input for ADC single ended input 3 value."]
    #[inline(always)]
    pub fn adcse3(self) -> &'a mut W {
        self.variant(PAD31FNCSEL_A::ADCSE3)
    }
    #[doc = "IOM/MSPI nCE group 31 value."]
    #[inline(always)]
    pub fn nce31(self) -> &'a mut W {
        self.variant(PAD31FNCSEL_A::NCE31)
    }
    #[doc = "CTIMER connection 13 value."]
    #[inline(always)]
    pub fn ct13(self) -> &'a mut W {
        self.variant(PAD31FNCSEL_A::CT13)
    }
    #[doc = "Configure as GPIO31 value."]
    #[inline(always)]
    pub fn gpio31(self) -> &'a mut W {
        self.variant(PAD31FNCSEL_A::GPIO31)
    }
    #[doc = "Configure as the UART0 RX input signal value."]
    #[inline(always)]
    pub fn uart0rx(self) -> &'a mut W {
        self.variant(PAD31FNCSEL_A::UART0RX)
    }
    #[doc = "SCARD serial clock output value."]
    #[inline(always)]
    pub fn sccclk(self) -> &'a mut W {
        self.variant(PAD31FNCSEL_A::SCCCLK)
    }
    #[doc = "Configure as UART1 RTS output signal value."]
    #[inline(always)]
    pub fn ua1rts(self) -> &'a mut W {
        self.variant(PAD31FNCSEL_A::UA1RTS)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 27)) | ((value as u32 & 0x07) << 27);
        self.w
    }
}
#[doc = "Pad 31 drive strength\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD31STRNG_A {
    #[doc = "0: Low drive strength value."]
    LOW = 0,
    #[doc = "1: High drive strength value."]
    HIGH = 1,
}
impl From<PAD31STRNG_A> for bool {
    #[inline(always)]
    fn from(variant: PAD31STRNG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PAD31STRNG` reader - Pad 31 drive strength"]
pub struct PAD31STRNG_R(crate::FieldReader<bool, PAD31STRNG_A>);
impl PAD31STRNG_R {
    pub(crate) fn new(bits: bool) -> Self {
        PAD31STRNG_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAD31STRNG_A {
        match self.bits {
            false => PAD31STRNG_A::LOW,
            true => PAD31STRNG_A::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        **self == PAD31STRNG_A::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        **self == PAD31STRNG_A::HIGH
    }
}
impl core::ops::Deref for PAD31STRNG_R {
    type Target = crate::FieldReader<bool, PAD31STRNG_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PAD31STRNG` writer - Pad 31 drive strength"]
pub struct PAD31STRNG_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD31STRNG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD31STRNG_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Low drive strength value."]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(PAD31STRNG_A::LOW)
    }
    #[doc = "High drive strength value."]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(PAD31STRNG_A::HIGH)
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
#[doc = "Pad 31 input enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD31INPEN_A {
    #[doc = "0: Pad input disabled value."]
    DIS = 0,
    #[doc = "1: Pad input enabled value."]
    EN = 1,
}
impl From<PAD31INPEN_A> for bool {
    #[inline(always)]
    fn from(variant: PAD31INPEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PAD31INPEN` reader - Pad 31 input enable"]
pub struct PAD31INPEN_R(crate::FieldReader<bool, PAD31INPEN_A>);
impl PAD31INPEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        PAD31INPEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAD31INPEN_A {
        match self.bits {
            false => PAD31INPEN_A::DIS,
            true => PAD31INPEN_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        **self == PAD31INPEN_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        **self == PAD31INPEN_A::EN
    }
}
impl core::ops::Deref for PAD31INPEN_R {
    type Target = crate::FieldReader<bool, PAD31INPEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PAD31INPEN` writer - Pad 31 input enable"]
pub struct PAD31INPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD31INPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD31INPEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Pad input disabled value."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(PAD31INPEN_A::DIS)
    }
    #[doc = "Pad input enabled value."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(PAD31INPEN_A::EN)
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
#[doc = "Pad 31 pullup enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD31PULL_A {
    #[doc = "0: Pullup disabled value."]
    DIS = 0,
    #[doc = "1: Pullup enabled value."]
    EN = 1,
}
impl From<PAD31PULL_A> for bool {
    #[inline(always)]
    fn from(variant: PAD31PULL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PAD31PULL` reader - Pad 31 pullup enable"]
pub struct PAD31PULL_R(crate::FieldReader<bool, PAD31PULL_A>);
impl PAD31PULL_R {
    pub(crate) fn new(bits: bool) -> Self {
        PAD31PULL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAD31PULL_A {
        match self.bits {
            false => PAD31PULL_A::DIS,
            true => PAD31PULL_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        **self == PAD31PULL_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        **self == PAD31PULL_A::EN
    }
}
impl core::ops::Deref for PAD31PULL_R {
    type Target = crate::FieldReader<bool, PAD31PULL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PAD31PULL` writer - Pad 31 pullup enable"]
pub struct PAD31PULL_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD31PULL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD31PULL_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Pullup disabled value."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(PAD31PULL_A::DIS)
    }
    #[doc = "Pullup enabled value."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(PAD31PULL_A::EN)
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
#[doc = "Pad 30 function select\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PAD30FNCSEL_A {
    #[doc = "0: Configure as the ANATEST1 I/O signal value."]
    ANATEST1 = 0,
    #[doc = "1: IOM/MSPI nCE group 30 value."]
    NCE30 = 1,
    #[doc = "2: CTIMER connection 11 value."]
    CT11 = 2,
    #[doc = "3: Configure as GPIO30 value."]
    GPIO30 = 3,
    #[doc = "4: Configure as UART0 TX output signal value."]
    UART0TX = 4,
    #[doc = "5: Configure as UART1 RTS output signal value."]
    UA1RTS = 5,
    #[doc = "7: Configure as the PDM I2S Data output signal value."]
    I2S_DAT = 7,
}
impl From<PAD30FNCSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: PAD30FNCSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `PAD30FNCSEL` reader - Pad 30 function select"]
pub struct PAD30FNCSEL_R(crate::FieldReader<u8, PAD30FNCSEL_A>);
impl PAD30FNCSEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        PAD30FNCSEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PAD30FNCSEL_A> {
        match self.bits {
            0 => Some(PAD30FNCSEL_A::ANATEST1),
            1 => Some(PAD30FNCSEL_A::NCE30),
            2 => Some(PAD30FNCSEL_A::CT11),
            3 => Some(PAD30FNCSEL_A::GPIO30),
            4 => Some(PAD30FNCSEL_A::UART0TX),
            5 => Some(PAD30FNCSEL_A::UA1RTS),
            7 => Some(PAD30FNCSEL_A::I2S_DAT),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `ANATEST1`"]
    #[inline(always)]
    pub fn is_anatest1(&self) -> bool {
        **self == PAD30FNCSEL_A::ANATEST1
    }
    #[doc = "Checks if the value of the field is `NCE30`"]
    #[inline(always)]
    pub fn is_nce30(&self) -> bool {
        **self == PAD30FNCSEL_A::NCE30
    }
    #[doc = "Checks if the value of the field is `CT11`"]
    #[inline(always)]
    pub fn is_ct11(&self) -> bool {
        **self == PAD30FNCSEL_A::CT11
    }
    #[doc = "Checks if the value of the field is `GPIO30`"]
    #[inline(always)]
    pub fn is_gpio30(&self) -> bool {
        **self == PAD30FNCSEL_A::GPIO30
    }
    #[doc = "Checks if the value of the field is `UART0TX`"]
    #[inline(always)]
    pub fn is_uart0tx(&self) -> bool {
        **self == PAD30FNCSEL_A::UART0TX
    }
    #[doc = "Checks if the value of the field is `UA1RTS`"]
    #[inline(always)]
    pub fn is_ua1rts(&self) -> bool {
        **self == PAD30FNCSEL_A::UA1RTS
    }
    #[doc = "Checks if the value of the field is `I2S_DAT`"]
    #[inline(always)]
    pub fn is_i2s_dat(&self) -> bool {
        **self == PAD30FNCSEL_A::I2S_DAT
    }
}
impl core::ops::Deref for PAD30FNCSEL_R {
    type Target = crate::FieldReader<u8, PAD30FNCSEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PAD30FNCSEL` writer - Pad 30 function select"]
pub struct PAD30FNCSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD30FNCSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD30FNCSEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Configure as the ANATEST1 I/O signal value."]
    #[inline(always)]
    pub fn anatest1(self) -> &'a mut W {
        self.variant(PAD30FNCSEL_A::ANATEST1)
    }
    #[doc = "IOM/MSPI nCE group 30 value."]
    #[inline(always)]
    pub fn nce30(self) -> &'a mut W {
        self.variant(PAD30FNCSEL_A::NCE30)
    }
    #[doc = "CTIMER connection 11 value."]
    #[inline(always)]
    pub fn ct11(self) -> &'a mut W {
        self.variant(PAD30FNCSEL_A::CT11)
    }
    #[doc = "Configure as GPIO30 value."]
    #[inline(always)]
    pub fn gpio30(self) -> &'a mut W {
        self.variant(PAD30FNCSEL_A::GPIO30)
    }
    #[doc = "Configure as UART0 TX output signal value."]
    #[inline(always)]
    pub fn uart0tx(self) -> &'a mut W {
        self.variant(PAD30FNCSEL_A::UART0TX)
    }
    #[doc = "Configure as UART1 RTS output signal value."]
    #[inline(always)]
    pub fn ua1rts(self) -> &'a mut W {
        self.variant(PAD30FNCSEL_A::UA1RTS)
    }
    #[doc = "Configure as the PDM I2S Data output signal value."]
    #[inline(always)]
    pub fn i2s_dat(self) -> &'a mut W {
        self.variant(PAD30FNCSEL_A::I2S_DAT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 19)) | ((value as u32 & 0x07) << 19);
        self.w
    }
}
#[doc = "Pad 30 drive strength\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD30STRNG_A {
    #[doc = "0: Low drive strength value."]
    LOW = 0,
    #[doc = "1: High drive strength value."]
    HIGH = 1,
}
impl From<PAD30STRNG_A> for bool {
    #[inline(always)]
    fn from(variant: PAD30STRNG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PAD30STRNG` reader - Pad 30 drive strength"]
pub struct PAD30STRNG_R(crate::FieldReader<bool, PAD30STRNG_A>);
impl PAD30STRNG_R {
    pub(crate) fn new(bits: bool) -> Self {
        PAD30STRNG_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAD30STRNG_A {
        match self.bits {
            false => PAD30STRNG_A::LOW,
            true => PAD30STRNG_A::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        **self == PAD30STRNG_A::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        **self == PAD30STRNG_A::HIGH
    }
}
impl core::ops::Deref for PAD30STRNG_R {
    type Target = crate::FieldReader<bool, PAD30STRNG_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PAD30STRNG` writer - Pad 30 drive strength"]
pub struct PAD30STRNG_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD30STRNG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD30STRNG_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Low drive strength value."]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(PAD30STRNG_A::LOW)
    }
    #[doc = "High drive strength value."]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(PAD30STRNG_A::HIGH)
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
#[doc = "Pad 30 input enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD30INPEN_A {
    #[doc = "0: Pad input disabled value."]
    DIS = 0,
    #[doc = "1: Pad input enabled value."]
    EN = 1,
}
impl From<PAD30INPEN_A> for bool {
    #[inline(always)]
    fn from(variant: PAD30INPEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PAD30INPEN` reader - Pad 30 input enable"]
pub struct PAD30INPEN_R(crate::FieldReader<bool, PAD30INPEN_A>);
impl PAD30INPEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        PAD30INPEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAD30INPEN_A {
        match self.bits {
            false => PAD30INPEN_A::DIS,
            true => PAD30INPEN_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        **self == PAD30INPEN_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        **self == PAD30INPEN_A::EN
    }
}
impl core::ops::Deref for PAD30INPEN_R {
    type Target = crate::FieldReader<bool, PAD30INPEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PAD30INPEN` writer - Pad 30 input enable"]
pub struct PAD30INPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD30INPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD30INPEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Pad input disabled value."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(PAD30INPEN_A::DIS)
    }
    #[doc = "Pad input enabled value."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(PAD30INPEN_A::EN)
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
#[doc = "Pad 30 pullup enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD30PULL_A {
    #[doc = "0: Pullup disabled value."]
    DIS = 0,
    #[doc = "1: Pullup enabled value."]
    EN = 1,
}
impl From<PAD30PULL_A> for bool {
    #[inline(always)]
    fn from(variant: PAD30PULL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PAD30PULL` reader - Pad 30 pullup enable"]
pub struct PAD30PULL_R(crate::FieldReader<bool, PAD30PULL_A>);
impl PAD30PULL_R {
    pub(crate) fn new(bits: bool) -> Self {
        PAD30PULL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAD30PULL_A {
        match self.bits {
            false => PAD30PULL_A::DIS,
            true => PAD30PULL_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        **self == PAD30PULL_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        **self == PAD30PULL_A::EN
    }
}
impl core::ops::Deref for PAD30PULL_R {
    type Target = crate::FieldReader<bool, PAD30PULL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PAD30PULL` writer - Pad 30 pullup enable"]
pub struct PAD30PULL_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD30PULL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD30PULL_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Pullup disabled value."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(PAD30PULL_A::DIS)
    }
    #[doc = "Pullup enabled value."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(PAD30PULL_A::EN)
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
#[doc = "Pad 29 function select\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PAD29FNCSEL_A {
    #[doc = "0: Configure as the analog input for ADC single ended input 1 value."]
    ADCSE1 = 0,
    #[doc = "1: IOM/MSPI nCE group 29 value."]
    NCE29 = 1,
    #[doc = "2: CTIMER connection 9 value."]
    CT9 = 2,
    #[doc = "3: Configure as GPIO29 value."]
    GPIO29 = 3,
    #[doc = "4: Configure as the UART0 CTS input signal value."]
    UA0CTS = 4,
    #[doc = "5: Configure as the UART1 CTS input signal value."]
    UA1CTS = 5,
    #[doc = "6: Configure as the UART0 RX input signal value."]
    UART0RX = 6,
    #[doc = "7: Configure as PDM DATA input value."]
    PDM_DATA = 7,
}
impl From<PAD29FNCSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: PAD29FNCSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `PAD29FNCSEL` reader - Pad 29 function select"]
pub struct PAD29FNCSEL_R(crate::FieldReader<u8, PAD29FNCSEL_A>);
impl PAD29FNCSEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        PAD29FNCSEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAD29FNCSEL_A {
        match self.bits {
            0 => PAD29FNCSEL_A::ADCSE1,
            1 => PAD29FNCSEL_A::NCE29,
            2 => PAD29FNCSEL_A::CT9,
            3 => PAD29FNCSEL_A::GPIO29,
            4 => PAD29FNCSEL_A::UA0CTS,
            5 => PAD29FNCSEL_A::UA1CTS,
            6 => PAD29FNCSEL_A::UART0RX,
            7 => PAD29FNCSEL_A::PDM_DATA,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ADCSE1`"]
    #[inline(always)]
    pub fn is_adcse1(&self) -> bool {
        **self == PAD29FNCSEL_A::ADCSE1
    }
    #[doc = "Checks if the value of the field is `NCE29`"]
    #[inline(always)]
    pub fn is_nce29(&self) -> bool {
        **self == PAD29FNCSEL_A::NCE29
    }
    #[doc = "Checks if the value of the field is `CT9`"]
    #[inline(always)]
    pub fn is_ct9(&self) -> bool {
        **self == PAD29FNCSEL_A::CT9
    }
    #[doc = "Checks if the value of the field is `GPIO29`"]
    #[inline(always)]
    pub fn is_gpio29(&self) -> bool {
        **self == PAD29FNCSEL_A::GPIO29
    }
    #[doc = "Checks if the value of the field is `UA0CTS`"]
    #[inline(always)]
    pub fn is_ua0cts(&self) -> bool {
        **self == PAD29FNCSEL_A::UA0CTS
    }
    #[doc = "Checks if the value of the field is `UA1CTS`"]
    #[inline(always)]
    pub fn is_ua1cts(&self) -> bool {
        **self == PAD29FNCSEL_A::UA1CTS
    }
    #[doc = "Checks if the value of the field is `UART0RX`"]
    #[inline(always)]
    pub fn is_uart0rx(&self) -> bool {
        **self == PAD29FNCSEL_A::UART0RX
    }
    #[doc = "Checks if the value of the field is `PDM_DATA`"]
    #[inline(always)]
    pub fn is_pdm_data(&self) -> bool {
        **self == PAD29FNCSEL_A::PDM_DATA
    }
}
impl core::ops::Deref for PAD29FNCSEL_R {
    type Target = crate::FieldReader<u8, PAD29FNCSEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PAD29FNCSEL` writer - Pad 29 function select"]
pub struct PAD29FNCSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD29FNCSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD29FNCSEL_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Configure as the analog input for ADC single ended input 1 value."]
    #[inline(always)]
    pub fn adcse1(self) -> &'a mut W {
        self.variant(PAD29FNCSEL_A::ADCSE1)
    }
    #[doc = "IOM/MSPI nCE group 29 value."]
    #[inline(always)]
    pub fn nce29(self) -> &'a mut W {
        self.variant(PAD29FNCSEL_A::NCE29)
    }
    #[doc = "CTIMER connection 9 value."]
    #[inline(always)]
    pub fn ct9(self) -> &'a mut W {
        self.variant(PAD29FNCSEL_A::CT9)
    }
    #[doc = "Configure as GPIO29 value."]
    #[inline(always)]
    pub fn gpio29(self) -> &'a mut W {
        self.variant(PAD29FNCSEL_A::GPIO29)
    }
    #[doc = "Configure as the UART0 CTS input signal value."]
    #[inline(always)]
    pub fn ua0cts(self) -> &'a mut W {
        self.variant(PAD29FNCSEL_A::UA0CTS)
    }
    #[doc = "Configure as the UART1 CTS input signal value."]
    #[inline(always)]
    pub fn ua1cts(self) -> &'a mut W {
        self.variant(PAD29FNCSEL_A::UA1CTS)
    }
    #[doc = "Configure as the UART0 RX input signal value."]
    #[inline(always)]
    pub fn uart0rx(self) -> &'a mut W {
        self.variant(PAD29FNCSEL_A::UART0RX)
    }
    #[doc = "Configure as PDM DATA input value."]
    #[inline(always)]
    pub fn pdm_data(self) -> &'a mut W {
        self.variant(PAD29FNCSEL_A::PDM_DATA)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 11)) | ((value as u32 & 0x07) << 11);
        self.w
    }
}
#[doc = "Pad 29 drive strength\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD29STRNG_A {
    #[doc = "0: Low drive strength value."]
    LOW = 0,
    #[doc = "1: High drive strength value."]
    HIGH = 1,
}
impl From<PAD29STRNG_A> for bool {
    #[inline(always)]
    fn from(variant: PAD29STRNG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PAD29STRNG` reader - Pad 29 drive strength"]
pub struct PAD29STRNG_R(crate::FieldReader<bool, PAD29STRNG_A>);
impl PAD29STRNG_R {
    pub(crate) fn new(bits: bool) -> Self {
        PAD29STRNG_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAD29STRNG_A {
        match self.bits {
            false => PAD29STRNG_A::LOW,
            true => PAD29STRNG_A::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        **self == PAD29STRNG_A::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        **self == PAD29STRNG_A::HIGH
    }
}
impl core::ops::Deref for PAD29STRNG_R {
    type Target = crate::FieldReader<bool, PAD29STRNG_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PAD29STRNG` writer - Pad 29 drive strength"]
pub struct PAD29STRNG_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD29STRNG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD29STRNG_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Low drive strength value."]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(PAD29STRNG_A::LOW)
    }
    #[doc = "High drive strength value."]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(PAD29STRNG_A::HIGH)
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
#[doc = "Pad 29 input enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD29INPEN_A {
    #[doc = "0: Pad input disabled value."]
    DIS = 0,
    #[doc = "1: Pad input enabled value."]
    EN = 1,
}
impl From<PAD29INPEN_A> for bool {
    #[inline(always)]
    fn from(variant: PAD29INPEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PAD29INPEN` reader - Pad 29 input enable"]
pub struct PAD29INPEN_R(crate::FieldReader<bool, PAD29INPEN_A>);
impl PAD29INPEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        PAD29INPEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAD29INPEN_A {
        match self.bits {
            false => PAD29INPEN_A::DIS,
            true => PAD29INPEN_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        **self == PAD29INPEN_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        **self == PAD29INPEN_A::EN
    }
}
impl core::ops::Deref for PAD29INPEN_R {
    type Target = crate::FieldReader<bool, PAD29INPEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PAD29INPEN` writer - Pad 29 input enable"]
pub struct PAD29INPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD29INPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD29INPEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Pad input disabled value."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(PAD29INPEN_A::DIS)
    }
    #[doc = "Pad input enabled value."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(PAD29INPEN_A::EN)
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
#[doc = "Pad 29 pullup enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD29PULL_A {
    #[doc = "0: Pullup disabled value."]
    DIS = 0,
    #[doc = "1: Pullup enabled value."]
    EN = 1,
}
impl From<PAD29PULL_A> for bool {
    #[inline(always)]
    fn from(variant: PAD29PULL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PAD29PULL` reader - Pad 29 pullup enable"]
pub struct PAD29PULL_R(crate::FieldReader<bool, PAD29PULL_A>);
impl PAD29PULL_R {
    pub(crate) fn new(bits: bool) -> Self {
        PAD29PULL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAD29PULL_A {
        match self.bits {
            false => PAD29PULL_A::DIS,
            true => PAD29PULL_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        **self == PAD29PULL_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        **self == PAD29PULL_A::EN
    }
}
impl core::ops::Deref for PAD29PULL_R {
    type Target = crate::FieldReader<bool, PAD29PULL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PAD29PULL` writer - Pad 29 pullup enable"]
pub struct PAD29PULL_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD29PULL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD29PULL_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Pullup disabled value."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(PAD29PULL_A::DIS)
    }
    #[doc = "Pullup enabled value."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(PAD29PULL_A::EN)
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
#[doc = "Pad 28 function select\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PAD28FNCSEL_A {
    #[doc = "0: Configure as the PDM I2S Word Clock input value."]
    I2S_WCLK = 0,
    #[doc = "1: IOM/MSPI nCE group 28 value."]
    NCE28 = 1,
    #[doc = "2: CTIMER connection 7 value."]
    CT7 = 2,
    #[doc = "3: Configure as GPIO28 value."]
    GPIO28 = 3,
    #[doc = "5: Configure as the IOMSTR2 SPI MOSI output signal value."]
    M2MOSI = 5,
    #[doc = "6: Configure as the UART0 TX output signal value."]
    UART0TX = 6,
}
impl From<PAD28FNCSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: PAD28FNCSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `PAD28FNCSEL` reader - Pad 28 function select"]
pub struct PAD28FNCSEL_R(crate::FieldReader<u8, PAD28FNCSEL_A>);
impl PAD28FNCSEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        PAD28FNCSEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PAD28FNCSEL_A> {
        match self.bits {
            0 => Some(PAD28FNCSEL_A::I2S_WCLK),
            1 => Some(PAD28FNCSEL_A::NCE28),
            2 => Some(PAD28FNCSEL_A::CT7),
            3 => Some(PAD28FNCSEL_A::GPIO28),
            5 => Some(PAD28FNCSEL_A::M2MOSI),
            6 => Some(PAD28FNCSEL_A::UART0TX),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `I2S_WCLK`"]
    #[inline(always)]
    pub fn is_i2s_wclk(&self) -> bool {
        **self == PAD28FNCSEL_A::I2S_WCLK
    }
    #[doc = "Checks if the value of the field is `NCE28`"]
    #[inline(always)]
    pub fn is_nce28(&self) -> bool {
        **self == PAD28FNCSEL_A::NCE28
    }
    #[doc = "Checks if the value of the field is `CT7`"]
    #[inline(always)]
    pub fn is_ct7(&self) -> bool {
        **self == PAD28FNCSEL_A::CT7
    }
    #[doc = "Checks if the value of the field is `GPIO28`"]
    #[inline(always)]
    pub fn is_gpio28(&self) -> bool {
        **self == PAD28FNCSEL_A::GPIO28
    }
    #[doc = "Checks if the value of the field is `M2MOSI`"]
    #[inline(always)]
    pub fn is_m2mosi(&self) -> bool {
        **self == PAD28FNCSEL_A::M2MOSI
    }
    #[doc = "Checks if the value of the field is `UART0TX`"]
    #[inline(always)]
    pub fn is_uart0tx(&self) -> bool {
        **self == PAD28FNCSEL_A::UART0TX
    }
}
impl core::ops::Deref for PAD28FNCSEL_R {
    type Target = crate::FieldReader<u8, PAD28FNCSEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PAD28FNCSEL` writer - Pad 28 function select"]
pub struct PAD28FNCSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD28FNCSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD28FNCSEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Configure as the PDM I2S Word Clock input value."]
    #[inline(always)]
    pub fn i2s_wclk(self) -> &'a mut W {
        self.variant(PAD28FNCSEL_A::I2S_WCLK)
    }
    #[doc = "IOM/MSPI nCE group 28 value."]
    #[inline(always)]
    pub fn nce28(self) -> &'a mut W {
        self.variant(PAD28FNCSEL_A::NCE28)
    }
    #[doc = "CTIMER connection 7 value."]
    #[inline(always)]
    pub fn ct7(self) -> &'a mut W {
        self.variant(PAD28FNCSEL_A::CT7)
    }
    #[doc = "Configure as GPIO28 value."]
    #[inline(always)]
    pub fn gpio28(self) -> &'a mut W {
        self.variant(PAD28FNCSEL_A::GPIO28)
    }
    #[doc = "Configure as the IOMSTR2 SPI MOSI output signal value."]
    #[inline(always)]
    pub fn m2mosi(self) -> &'a mut W {
        self.variant(PAD28FNCSEL_A::M2MOSI)
    }
    #[doc = "Configure as the UART0 TX output signal value."]
    #[inline(always)]
    pub fn uart0tx(self) -> &'a mut W {
        self.variant(PAD28FNCSEL_A::UART0TX)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 3)) | ((value as u32 & 0x07) << 3);
        self.w
    }
}
#[doc = "Pad 28 drive strength\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD28STRNG_A {
    #[doc = "0: Low drive strength value."]
    LOW = 0,
    #[doc = "1: High drive strength value."]
    HIGH = 1,
}
impl From<PAD28STRNG_A> for bool {
    #[inline(always)]
    fn from(variant: PAD28STRNG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PAD28STRNG` reader - Pad 28 drive strength"]
pub struct PAD28STRNG_R(crate::FieldReader<bool, PAD28STRNG_A>);
impl PAD28STRNG_R {
    pub(crate) fn new(bits: bool) -> Self {
        PAD28STRNG_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAD28STRNG_A {
        match self.bits {
            false => PAD28STRNG_A::LOW,
            true => PAD28STRNG_A::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        **self == PAD28STRNG_A::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        **self == PAD28STRNG_A::HIGH
    }
}
impl core::ops::Deref for PAD28STRNG_R {
    type Target = crate::FieldReader<bool, PAD28STRNG_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PAD28STRNG` writer - Pad 28 drive strength"]
pub struct PAD28STRNG_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD28STRNG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD28STRNG_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Low drive strength value."]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(PAD28STRNG_A::LOW)
    }
    #[doc = "High drive strength value."]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(PAD28STRNG_A::HIGH)
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
#[doc = "Pad 28 input enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD28INPEN_A {
    #[doc = "0: Pad input disabled value."]
    DIS = 0,
    #[doc = "1: Pad input enabled value."]
    EN = 1,
}
impl From<PAD28INPEN_A> for bool {
    #[inline(always)]
    fn from(variant: PAD28INPEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PAD28INPEN` reader - Pad 28 input enable"]
pub struct PAD28INPEN_R(crate::FieldReader<bool, PAD28INPEN_A>);
impl PAD28INPEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        PAD28INPEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAD28INPEN_A {
        match self.bits {
            false => PAD28INPEN_A::DIS,
            true => PAD28INPEN_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        **self == PAD28INPEN_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        **self == PAD28INPEN_A::EN
    }
}
impl core::ops::Deref for PAD28INPEN_R {
    type Target = crate::FieldReader<bool, PAD28INPEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PAD28INPEN` writer - Pad 28 input enable"]
pub struct PAD28INPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD28INPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD28INPEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Pad input disabled value."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(PAD28INPEN_A::DIS)
    }
    #[doc = "Pad input enabled value."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(PAD28INPEN_A::EN)
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
#[doc = "Pad 28 pullup enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD28PULL_A {
    #[doc = "0: Pullup disabled value."]
    DIS = 0,
    #[doc = "1: Pullup enabled value."]
    EN = 1,
}
impl From<PAD28PULL_A> for bool {
    #[inline(always)]
    fn from(variant: PAD28PULL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PAD28PULL` reader - Pad 28 pullup enable"]
pub struct PAD28PULL_R(crate::FieldReader<bool, PAD28PULL_A>);
impl PAD28PULL_R {
    pub(crate) fn new(bits: bool) -> Self {
        PAD28PULL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAD28PULL_A {
        match self.bits {
            false => PAD28PULL_A::DIS,
            true => PAD28PULL_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        **self == PAD28PULL_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        **self == PAD28PULL_A::EN
    }
}
impl core::ops::Deref for PAD28PULL_R {
    type Target = crate::FieldReader<bool, PAD28PULL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PAD28PULL` writer - Pad 28 pullup enable"]
pub struct PAD28PULL_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD28PULL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD28PULL_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Pullup disabled value."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(PAD28PULL_A::DIS)
    }
    #[doc = "Pullup enabled value."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(PAD28PULL_A::EN)
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
    #[doc = "Bits 27:29 - Pad 31 function select"]
    #[inline(always)]
    pub fn pad31fncsel(&self) -> PAD31FNCSEL_R {
        PAD31FNCSEL_R::new(((self.bits >> 27) & 0x07) as u8)
    }
    #[doc = "Bit 26 - Pad 31 drive strength"]
    #[inline(always)]
    pub fn pad31strng(&self) -> PAD31STRNG_R {
        PAD31STRNG_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Pad 31 input enable"]
    #[inline(always)]
    pub fn pad31inpen(&self) -> PAD31INPEN_R {
        PAD31INPEN_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Pad 31 pullup enable"]
    #[inline(always)]
    pub fn pad31pull(&self) -> PAD31PULL_R {
        PAD31PULL_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bits 19:21 - Pad 30 function select"]
    #[inline(always)]
    pub fn pad30fncsel(&self) -> PAD30FNCSEL_R {
        PAD30FNCSEL_R::new(((self.bits >> 19) & 0x07) as u8)
    }
    #[doc = "Bit 18 - Pad 30 drive strength"]
    #[inline(always)]
    pub fn pad30strng(&self) -> PAD30STRNG_R {
        PAD30STRNG_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Pad 30 input enable"]
    #[inline(always)]
    pub fn pad30inpen(&self) -> PAD30INPEN_R {
        PAD30INPEN_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Pad 30 pullup enable"]
    #[inline(always)]
    pub fn pad30pull(&self) -> PAD30PULL_R {
        PAD30PULL_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bits 11:13 - Pad 29 function select"]
    #[inline(always)]
    pub fn pad29fncsel(&self) -> PAD29FNCSEL_R {
        PAD29FNCSEL_R::new(((self.bits >> 11) & 0x07) as u8)
    }
    #[doc = "Bit 10 - Pad 29 drive strength"]
    #[inline(always)]
    pub fn pad29strng(&self) -> PAD29STRNG_R {
        PAD29STRNG_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Pad 29 input enable"]
    #[inline(always)]
    pub fn pad29inpen(&self) -> PAD29INPEN_R {
        PAD29INPEN_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Pad 29 pullup enable"]
    #[inline(always)]
    pub fn pad29pull(&self) -> PAD29PULL_R {
        PAD29PULL_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bits 3:5 - Pad 28 function select"]
    #[inline(always)]
    pub fn pad28fncsel(&self) -> PAD28FNCSEL_R {
        PAD28FNCSEL_R::new(((self.bits >> 3) & 0x07) as u8)
    }
    #[doc = "Bit 2 - Pad 28 drive strength"]
    #[inline(always)]
    pub fn pad28strng(&self) -> PAD28STRNG_R {
        PAD28STRNG_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - Pad 28 input enable"]
    #[inline(always)]
    pub fn pad28inpen(&self) -> PAD28INPEN_R {
        PAD28INPEN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Pad 28 pullup enable"]
    #[inline(always)]
    pub fn pad28pull(&self) -> PAD28PULL_R {
        PAD28PULL_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 27:29 - Pad 31 function select"]
    #[inline(always)]
    pub fn pad31fncsel(&mut self) -> PAD31FNCSEL_W {
        PAD31FNCSEL_W { w: self }
    }
    #[doc = "Bit 26 - Pad 31 drive strength"]
    #[inline(always)]
    pub fn pad31strng(&mut self) -> PAD31STRNG_W {
        PAD31STRNG_W { w: self }
    }
    #[doc = "Bit 25 - Pad 31 input enable"]
    #[inline(always)]
    pub fn pad31inpen(&mut self) -> PAD31INPEN_W {
        PAD31INPEN_W { w: self }
    }
    #[doc = "Bit 24 - Pad 31 pullup enable"]
    #[inline(always)]
    pub fn pad31pull(&mut self) -> PAD31PULL_W {
        PAD31PULL_W { w: self }
    }
    #[doc = "Bits 19:21 - Pad 30 function select"]
    #[inline(always)]
    pub fn pad30fncsel(&mut self) -> PAD30FNCSEL_W {
        PAD30FNCSEL_W { w: self }
    }
    #[doc = "Bit 18 - Pad 30 drive strength"]
    #[inline(always)]
    pub fn pad30strng(&mut self) -> PAD30STRNG_W {
        PAD30STRNG_W { w: self }
    }
    #[doc = "Bit 17 - Pad 30 input enable"]
    #[inline(always)]
    pub fn pad30inpen(&mut self) -> PAD30INPEN_W {
        PAD30INPEN_W { w: self }
    }
    #[doc = "Bit 16 - Pad 30 pullup enable"]
    #[inline(always)]
    pub fn pad30pull(&mut self) -> PAD30PULL_W {
        PAD30PULL_W { w: self }
    }
    #[doc = "Bits 11:13 - Pad 29 function select"]
    #[inline(always)]
    pub fn pad29fncsel(&mut self) -> PAD29FNCSEL_W {
        PAD29FNCSEL_W { w: self }
    }
    #[doc = "Bit 10 - Pad 29 drive strength"]
    #[inline(always)]
    pub fn pad29strng(&mut self) -> PAD29STRNG_W {
        PAD29STRNG_W { w: self }
    }
    #[doc = "Bit 9 - Pad 29 input enable"]
    #[inline(always)]
    pub fn pad29inpen(&mut self) -> PAD29INPEN_W {
        PAD29INPEN_W { w: self }
    }
    #[doc = "Bit 8 - Pad 29 pullup enable"]
    #[inline(always)]
    pub fn pad29pull(&mut self) -> PAD29PULL_W {
        PAD29PULL_W { w: self }
    }
    #[doc = "Bits 3:5 - Pad 28 function select"]
    #[inline(always)]
    pub fn pad28fncsel(&mut self) -> PAD28FNCSEL_W {
        PAD28FNCSEL_W { w: self }
    }
    #[doc = "Bit 2 - Pad 28 drive strength"]
    #[inline(always)]
    pub fn pad28strng(&mut self) -> PAD28STRNG_W {
        PAD28STRNG_W { w: self }
    }
    #[doc = "Bit 1 - Pad 28 input enable"]
    #[inline(always)]
    pub fn pad28inpen(&mut self) -> PAD28INPEN_W {
        PAD28INPEN_W { w: self }
    }
    #[doc = "Bit 0 - Pad 28 pullup enable"]
    #[inline(always)]
    pub fn pad28pull(&mut self) -> PAD28PULL_W {
        PAD28PULL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Pad Configuration Register H (Pads 28-31)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [padregh](index.html) module"]
pub struct PADREGH_SPEC;
impl crate::RegisterSpec for PADREGH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [padregh::R](R) reader structure"]
impl crate::Readable for PADREGH_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [padregh::W](W) writer structure"]
impl crate::Writable for PADREGH_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PADREGH to value 0x1818_1818"]
impl crate::Resettable for PADREGH_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x1818_1818
    }
}
