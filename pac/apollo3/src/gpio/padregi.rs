#[doc = "Register `PADREGI` reader"]
pub struct R(crate::R<PADREGI_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PADREGI_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PADREGI_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PADREGI_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PADREGI` writer"]
pub struct W(crate::W<PADREGI_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PADREGI_SPEC>;
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
impl From<crate::W<PADREGI_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PADREGI_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Pad 35 function select\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PAD35FNCSEL_A {
    #[doc = "0: Configure as the analog input for ADC single ended input 7 value."]
    ADCSE7 = 0,
    #[doc = "1: IOM/MSPI nCE group 35 value."]
    NCE35 = 1,
    #[doc = "2: Configure as the UART1 TX signal value."]
    UART1TX = 2,
    #[doc = "3: Configure as GPIO35 value."]
    GPIO35 = 3,
    #[doc = "4: I2S serial data output value."]
    I2SDAT = 4,
    #[doc = "5: CTIMER connection 27 value."]
    CT27 = 5,
    #[doc = "6: Configure as the UART0 RTS output value."]
    UA0RTS = 6,
}
impl From<PAD35FNCSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: PAD35FNCSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `PAD35FNCSEL` reader - Pad 35 function select"]
pub struct PAD35FNCSEL_R(crate::FieldReader<u8, PAD35FNCSEL_A>);
impl PAD35FNCSEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        PAD35FNCSEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PAD35FNCSEL_A> {
        match self.bits {
            0 => Some(PAD35FNCSEL_A::ADCSE7),
            1 => Some(PAD35FNCSEL_A::NCE35),
            2 => Some(PAD35FNCSEL_A::UART1TX),
            3 => Some(PAD35FNCSEL_A::GPIO35),
            4 => Some(PAD35FNCSEL_A::I2SDAT),
            5 => Some(PAD35FNCSEL_A::CT27),
            6 => Some(PAD35FNCSEL_A::UA0RTS),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `ADCSE7`"]
    #[inline(always)]
    pub fn is_adcse7(&self) -> bool {
        **self == PAD35FNCSEL_A::ADCSE7
    }
    #[doc = "Checks if the value of the field is `NCE35`"]
    #[inline(always)]
    pub fn is_nce35(&self) -> bool {
        **self == PAD35FNCSEL_A::NCE35
    }
    #[doc = "Checks if the value of the field is `UART1TX`"]
    #[inline(always)]
    pub fn is_uart1tx(&self) -> bool {
        **self == PAD35FNCSEL_A::UART1TX
    }
    #[doc = "Checks if the value of the field is `GPIO35`"]
    #[inline(always)]
    pub fn is_gpio35(&self) -> bool {
        **self == PAD35FNCSEL_A::GPIO35
    }
    #[doc = "Checks if the value of the field is `I2SDAT`"]
    #[inline(always)]
    pub fn is_i2sdat(&self) -> bool {
        **self == PAD35FNCSEL_A::I2SDAT
    }
    #[doc = "Checks if the value of the field is `CT27`"]
    #[inline(always)]
    pub fn is_ct27(&self) -> bool {
        **self == PAD35FNCSEL_A::CT27
    }
    #[doc = "Checks if the value of the field is `UA0RTS`"]
    #[inline(always)]
    pub fn is_ua0rts(&self) -> bool {
        **self == PAD35FNCSEL_A::UA0RTS
    }
}
impl core::ops::Deref for PAD35FNCSEL_R {
    type Target = crate::FieldReader<u8, PAD35FNCSEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PAD35FNCSEL` writer - Pad 35 function select"]
pub struct PAD35FNCSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD35FNCSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD35FNCSEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Configure as the analog input for ADC single ended input 7 value."]
    #[inline(always)]
    pub fn adcse7(self) -> &'a mut W {
        self.variant(PAD35FNCSEL_A::ADCSE7)
    }
    #[doc = "IOM/MSPI nCE group 35 value."]
    #[inline(always)]
    pub fn nce35(self) -> &'a mut W {
        self.variant(PAD35FNCSEL_A::NCE35)
    }
    #[doc = "Configure as the UART1 TX signal value."]
    #[inline(always)]
    pub fn uart1tx(self) -> &'a mut W {
        self.variant(PAD35FNCSEL_A::UART1TX)
    }
    #[doc = "Configure as GPIO35 value."]
    #[inline(always)]
    pub fn gpio35(self) -> &'a mut W {
        self.variant(PAD35FNCSEL_A::GPIO35)
    }
    #[doc = "I2S serial data output value."]
    #[inline(always)]
    pub fn i2sdat(self) -> &'a mut W {
        self.variant(PAD35FNCSEL_A::I2SDAT)
    }
    #[doc = "CTIMER connection 27 value."]
    #[inline(always)]
    pub fn ct27(self) -> &'a mut W {
        self.variant(PAD35FNCSEL_A::CT27)
    }
    #[doc = "Configure as the UART0 RTS output value."]
    #[inline(always)]
    pub fn ua0rts(self) -> &'a mut W {
        self.variant(PAD35FNCSEL_A::UA0RTS)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 27)) | ((value as u32 & 0x07) << 27);
        self.w
    }
}
#[doc = "Pad 35 drive strength\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD35STRNG_A {
    #[doc = "0: Low drive strength value."]
    LOW = 0,
    #[doc = "1: High drive strength value."]
    HIGH = 1,
}
impl From<PAD35STRNG_A> for bool {
    #[inline(always)]
    fn from(variant: PAD35STRNG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PAD35STRNG` reader - Pad 35 drive strength"]
pub struct PAD35STRNG_R(crate::FieldReader<bool, PAD35STRNG_A>);
impl PAD35STRNG_R {
    pub(crate) fn new(bits: bool) -> Self {
        PAD35STRNG_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAD35STRNG_A {
        match self.bits {
            false => PAD35STRNG_A::LOW,
            true => PAD35STRNG_A::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        **self == PAD35STRNG_A::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        **self == PAD35STRNG_A::HIGH
    }
}
impl core::ops::Deref for PAD35STRNG_R {
    type Target = crate::FieldReader<bool, PAD35STRNG_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PAD35STRNG` writer - Pad 35 drive strength"]
pub struct PAD35STRNG_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD35STRNG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD35STRNG_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Low drive strength value."]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(PAD35STRNG_A::LOW)
    }
    #[doc = "High drive strength value."]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(PAD35STRNG_A::HIGH)
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
#[doc = "Pad 35 input enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD35INPEN_A {
    #[doc = "0: Pad input disabled value."]
    DIS = 0,
    #[doc = "1: Pad input enabled value."]
    EN = 1,
}
impl From<PAD35INPEN_A> for bool {
    #[inline(always)]
    fn from(variant: PAD35INPEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PAD35INPEN` reader - Pad 35 input enable"]
pub struct PAD35INPEN_R(crate::FieldReader<bool, PAD35INPEN_A>);
impl PAD35INPEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        PAD35INPEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAD35INPEN_A {
        match self.bits {
            false => PAD35INPEN_A::DIS,
            true => PAD35INPEN_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        **self == PAD35INPEN_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        **self == PAD35INPEN_A::EN
    }
}
impl core::ops::Deref for PAD35INPEN_R {
    type Target = crate::FieldReader<bool, PAD35INPEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PAD35INPEN` writer - Pad 35 input enable"]
pub struct PAD35INPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD35INPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD35INPEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Pad input disabled value."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(PAD35INPEN_A::DIS)
    }
    #[doc = "Pad input enabled value."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(PAD35INPEN_A::EN)
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
#[doc = "Pad 35 pullup enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD35PULL_A {
    #[doc = "0: Pullup disabled value."]
    DIS = 0,
    #[doc = "1: Pullup enabled value."]
    EN = 1,
}
impl From<PAD35PULL_A> for bool {
    #[inline(always)]
    fn from(variant: PAD35PULL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PAD35PULL` reader - Pad 35 pullup enable"]
pub struct PAD35PULL_R(crate::FieldReader<bool, PAD35PULL_A>);
impl PAD35PULL_R {
    pub(crate) fn new(bits: bool) -> Self {
        PAD35PULL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAD35PULL_A {
        match self.bits {
            false => PAD35PULL_A::DIS,
            true => PAD35PULL_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        **self == PAD35PULL_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        **self == PAD35PULL_A::EN
    }
}
impl core::ops::Deref for PAD35PULL_R {
    type Target = crate::FieldReader<bool, PAD35PULL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PAD35PULL` writer - Pad 35 pullup enable"]
pub struct PAD35PULL_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD35PULL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD35PULL_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Pullup disabled value."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(PAD35PULL_A::DIS)
    }
    #[doc = "Pullup enabled value."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(PAD35PULL_A::EN)
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
#[doc = "Pad 34 function select\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PAD34FNCSEL_A {
    #[doc = "0: Configure as the analog input for ADC single ended input 6 value."]
    ADCSE6 = 0,
    #[doc = "1: IOM/MSPI nCE group 34 value."]
    NCE34 = 1,
    #[doc = "2: Configure as the UART1 RTS output value."]
    UA1RTS = 2,
    #[doc = "3: Configure as GPIO34 value."]
    GPIO34 = 3,
    #[doc = "4: Configure as the analog comparator reference 2 signal value."]
    CMPRF2 = 4,
    #[doc = "5: Configure as the UART0 RTS output value."]
    UA0RTS = 5,
    #[doc = "6: Configure as the UART0 RX input value."]
    UART0RX = 6,
    #[doc = "7: PDM serial data input value."]
    PDMDATA = 7,
}
impl From<PAD34FNCSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: PAD34FNCSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `PAD34FNCSEL` reader - Pad 34 function select"]
pub struct PAD34FNCSEL_R(crate::FieldReader<u8, PAD34FNCSEL_A>);
impl PAD34FNCSEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        PAD34FNCSEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAD34FNCSEL_A {
        match self.bits {
            0 => PAD34FNCSEL_A::ADCSE6,
            1 => PAD34FNCSEL_A::NCE34,
            2 => PAD34FNCSEL_A::UA1RTS,
            3 => PAD34FNCSEL_A::GPIO34,
            4 => PAD34FNCSEL_A::CMPRF2,
            5 => PAD34FNCSEL_A::UA0RTS,
            6 => PAD34FNCSEL_A::UART0RX,
            7 => PAD34FNCSEL_A::PDMDATA,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ADCSE6`"]
    #[inline(always)]
    pub fn is_adcse6(&self) -> bool {
        **self == PAD34FNCSEL_A::ADCSE6
    }
    #[doc = "Checks if the value of the field is `NCE34`"]
    #[inline(always)]
    pub fn is_nce34(&self) -> bool {
        **self == PAD34FNCSEL_A::NCE34
    }
    #[doc = "Checks if the value of the field is `UA1RTS`"]
    #[inline(always)]
    pub fn is_ua1rts(&self) -> bool {
        **self == PAD34FNCSEL_A::UA1RTS
    }
    #[doc = "Checks if the value of the field is `GPIO34`"]
    #[inline(always)]
    pub fn is_gpio34(&self) -> bool {
        **self == PAD34FNCSEL_A::GPIO34
    }
    #[doc = "Checks if the value of the field is `CMPRF2`"]
    #[inline(always)]
    pub fn is_cmprf2(&self) -> bool {
        **self == PAD34FNCSEL_A::CMPRF2
    }
    #[doc = "Checks if the value of the field is `UA0RTS`"]
    #[inline(always)]
    pub fn is_ua0rts(&self) -> bool {
        **self == PAD34FNCSEL_A::UA0RTS
    }
    #[doc = "Checks if the value of the field is `UART0RX`"]
    #[inline(always)]
    pub fn is_uart0rx(&self) -> bool {
        **self == PAD34FNCSEL_A::UART0RX
    }
    #[doc = "Checks if the value of the field is `PDMDATA`"]
    #[inline(always)]
    pub fn is_pdmdata(&self) -> bool {
        **self == PAD34FNCSEL_A::PDMDATA
    }
}
impl core::ops::Deref for PAD34FNCSEL_R {
    type Target = crate::FieldReader<u8, PAD34FNCSEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PAD34FNCSEL` writer - Pad 34 function select"]
pub struct PAD34FNCSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD34FNCSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD34FNCSEL_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Configure as the analog input for ADC single ended input 6 value."]
    #[inline(always)]
    pub fn adcse6(self) -> &'a mut W {
        self.variant(PAD34FNCSEL_A::ADCSE6)
    }
    #[doc = "IOM/MSPI nCE group 34 value."]
    #[inline(always)]
    pub fn nce34(self) -> &'a mut W {
        self.variant(PAD34FNCSEL_A::NCE34)
    }
    #[doc = "Configure as the UART1 RTS output value."]
    #[inline(always)]
    pub fn ua1rts(self) -> &'a mut W {
        self.variant(PAD34FNCSEL_A::UA1RTS)
    }
    #[doc = "Configure as GPIO34 value."]
    #[inline(always)]
    pub fn gpio34(self) -> &'a mut W {
        self.variant(PAD34FNCSEL_A::GPIO34)
    }
    #[doc = "Configure as the analog comparator reference 2 signal value."]
    #[inline(always)]
    pub fn cmprf2(self) -> &'a mut W {
        self.variant(PAD34FNCSEL_A::CMPRF2)
    }
    #[doc = "Configure as the UART0 RTS output value."]
    #[inline(always)]
    pub fn ua0rts(self) -> &'a mut W {
        self.variant(PAD34FNCSEL_A::UA0RTS)
    }
    #[doc = "Configure as the UART0 RX input value."]
    #[inline(always)]
    pub fn uart0rx(self) -> &'a mut W {
        self.variant(PAD34FNCSEL_A::UART0RX)
    }
    #[doc = "PDM serial data input value."]
    #[inline(always)]
    pub fn pdmdata(self) -> &'a mut W {
        self.variant(PAD34FNCSEL_A::PDMDATA)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 19)) | ((value as u32 & 0x07) << 19);
        self.w
    }
}
#[doc = "Pad 34 drive strength\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD34STRNG_A {
    #[doc = "0: Low drive strength value."]
    LOW = 0,
    #[doc = "1: High drive strength value."]
    HIGH = 1,
}
impl From<PAD34STRNG_A> for bool {
    #[inline(always)]
    fn from(variant: PAD34STRNG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PAD34STRNG` reader - Pad 34 drive strength"]
pub struct PAD34STRNG_R(crate::FieldReader<bool, PAD34STRNG_A>);
impl PAD34STRNG_R {
    pub(crate) fn new(bits: bool) -> Self {
        PAD34STRNG_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAD34STRNG_A {
        match self.bits {
            false => PAD34STRNG_A::LOW,
            true => PAD34STRNG_A::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        **self == PAD34STRNG_A::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        **self == PAD34STRNG_A::HIGH
    }
}
impl core::ops::Deref for PAD34STRNG_R {
    type Target = crate::FieldReader<bool, PAD34STRNG_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PAD34STRNG` writer - Pad 34 drive strength"]
pub struct PAD34STRNG_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD34STRNG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD34STRNG_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Low drive strength value."]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(PAD34STRNG_A::LOW)
    }
    #[doc = "High drive strength value."]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(PAD34STRNG_A::HIGH)
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
#[doc = "Pad 34 input enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD34INPEN_A {
    #[doc = "0: Pad input disabled value."]
    DIS = 0,
    #[doc = "1: Pad input enabled value."]
    EN = 1,
}
impl From<PAD34INPEN_A> for bool {
    #[inline(always)]
    fn from(variant: PAD34INPEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PAD34INPEN` reader - Pad 34 input enable"]
pub struct PAD34INPEN_R(crate::FieldReader<bool, PAD34INPEN_A>);
impl PAD34INPEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        PAD34INPEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAD34INPEN_A {
        match self.bits {
            false => PAD34INPEN_A::DIS,
            true => PAD34INPEN_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        **self == PAD34INPEN_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        **self == PAD34INPEN_A::EN
    }
}
impl core::ops::Deref for PAD34INPEN_R {
    type Target = crate::FieldReader<bool, PAD34INPEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PAD34INPEN` writer - Pad 34 input enable"]
pub struct PAD34INPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD34INPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD34INPEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Pad input disabled value."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(PAD34INPEN_A::DIS)
    }
    #[doc = "Pad input enabled value."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(PAD34INPEN_A::EN)
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
#[doc = "Pad 34 pullup enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD34PULL_A {
    #[doc = "0: Pullup disabled value."]
    DIS = 0,
    #[doc = "1: Pullup enabled value."]
    EN = 1,
}
impl From<PAD34PULL_A> for bool {
    #[inline(always)]
    fn from(variant: PAD34PULL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PAD34PULL` reader - Pad 34 pullup enable"]
pub struct PAD34PULL_R(crate::FieldReader<bool, PAD34PULL_A>);
impl PAD34PULL_R {
    pub(crate) fn new(bits: bool) -> Self {
        PAD34PULL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAD34PULL_A {
        match self.bits {
            false => PAD34PULL_A::DIS,
            true => PAD34PULL_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        **self == PAD34PULL_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        **self == PAD34PULL_A::EN
    }
}
impl core::ops::Deref for PAD34PULL_R {
    type Target = crate::FieldReader<bool, PAD34PULL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PAD34PULL` writer - Pad 34 pullup enable"]
pub struct PAD34PULL_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD34PULL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD34PULL_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Pullup disabled value."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(PAD34PULL_A::DIS)
    }
    #[doc = "Pullup enabled value."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(PAD34PULL_A::EN)
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
#[doc = "Pad 33 function select\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PAD33FNCSEL_A {
    #[doc = "0: Configure as the analog ADC single ended port 5 input signal value."]
    ADCSE5 = 0,
    #[doc = "1: IOM/MSPI nCE group 33 value."]
    NCE33 = 1,
    #[doc = "2: Configure as the 32kHz crystal output signal value."]
    _32KHZXT = 2,
    #[doc = "3: Configure as GPIO33 value."]
    GPIO33 = 3,
    #[doc = "5: Configure as the UART0 CTS input value."]
    UA0CTS = 5,
    #[doc = "6: CTIMER connection 23 value."]
    CT23 = 6,
    #[doc = "7: Configure as the serial trace data output signal value."]
    SWO = 7,
}
impl From<PAD33FNCSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: PAD33FNCSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `PAD33FNCSEL` reader - Pad 33 function select"]
pub struct PAD33FNCSEL_R(crate::FieldReader<u8, PAD33FNCSEL_A>);
impl PAD33FNCSEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        PAD33FNCSEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PAD33FNCSEL_A> {
        match self.bits {
            0 => Some(PAD33FNCSEL_A::ADCSE5),
            1 => Some(PAD33FNCSEL_A::NCE33),
            2 => Some(PAD33FNCSEL_A::_32KHZXT),
            3 => Some(PAD33FNCSEL_A::GPIO33),
            5 => Some(PAD33FNCSEL_A::UA0CTS),
            6 => Some(PAD33FNCSEL_A::CT23),
            7 => Some(PAD33FNCSEL_A::SWO),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `ADCSE5`"]
    #[inline(always)]
    pub fn is_adcse5(&self) -> bool {
        **self == PAD33FNCSEL_A::ADCSE5
    }
    #[doc = "Checks if the value of the field is `NCE33`"]
    #[inline(always)]
    pub fn is_nce33(&self) -> bool {
        **self == PAD33FNCSEL_A::NCE33
    }
    #[doc = "Checks if the value of the field is `_32KHZXT`"]
    #[inline(always)]
    pub fn is_32k_hz_xt(&self) -> bool {
        **self == PAD33FNCSEL_A::_32KHZXT
    }
    #[doc = "Checks if the value of the field is `GPIO33`"]
    #[inline(always)]
    pub fn is_gpio33(&self) -> bool {
        **self == PAD33FNCSEL_A::GPIO33
    }
    #[doc = "Checks if the value of the field is `UA0CTS`"]
    #[inline(always)]
    pub fn is_ua0cts(&self) -> bool {
        **self == PAD33FNCSEL_A::UA0CTS
    }
    #[doc = "Checks if the value of the field is `CT23`"]
    #[inline(always)]
    pub fn is_ct23(&self) -> bool {
        **self == PAD33FNCSEL_A::CT23
    }
    #[doc = "Checks if the value of the field is `SWO`"]
    #[inline(always)]
    pub fn is_swo(&self) -> bool {
        **self == PAD33FNCSEL_A::SWO
    }
}
impl core::ops::Deref for PAD33FNCSEL_R {
    type Target = crate::FieldReader<u8, PAD33FNCSEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PAD33FNCSEL` writer - Pad 33 function select"]
pub struct PAD33FNCSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD33FNCSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD33FNCSEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Configure as the analog ADC single ended port 5 input signal value."]
    #[inline(always)]
    pub fn adcse5(self) -> &'a mut W {
        self.variant(PAD33FNCSEL_A::ADCSE5)
    }
    #[doc = "IOM/MSPI nCE group 33 value."]
    #[inline(always)]
    pub fn nce33(self) -> &'a mut W {
        self.variant(PAD33FNCSEL_A::NCE33)
    }
    #[doc = "Configure as the 32kHz crystal output signal value."]
    #[inline(always)]
    pub fn _32k_hz_xt(self) -> &'a mut W {
        self.variant(PAD33FNCSEL_A::_32KHZXT)
    }
    #[doc = "Configure as GPIO33 value."]
    #[inline(always)]
    pub fn gpio33(self) -> &'a mut W {
        self.variant(PAD33FNCSEL_A::GPIO33)
    }
    #[doc = "Configure as the UART0 CTS input value."]
    #[inline(always)]
    pub fn ua0cts(self) -> &'a mut W {
        self.variant(PAD33FNCSEL_A::UA0CTS)
    }
    #[doc = "CTIMER connection 23 value."]
    #[inline(always)]
    pub fn ct23(self) -> &'a mut W {
        self.variant(PAD33FNCSEL_A::CT23)
    }
    #[doc = "Configure as the serial trace data output signal value."]
    #[inline(always)]
    pub fn swo(self) -> &'a mut W {
        self.variant(PAD33FNCSEL_A::SWO)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 11)) | ((value as u32 & 0x07) << 11);
        self.w
    }
}
#[doc = "Pad 33 drive strength\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD33STRNG_A {
    #[doc = "0: Low drive strength value."]
    LOW = 0,
    #[doc = "1: High drive strength value."]
    HIGH = 1,
}
impl From<PAD33STRNG_A> for bool {
    #[inline(always)]
    fn from(variant: PAD33STRNG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PAD33STRNG` reader - Pad 33 drive strength"]
pub struct PAD33STRNG_R(crate::FieldReader<bool, PAD33STRNG_A>);
impl PAD33STRNG_R {
    pub(crate) fn new(bits: bool) -> Self {
        PAD33STRNG_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAD33STRNG_A {
        match self.bits {
            false => PAD33STRNG_A::LOW,
            true => PAD33STRNG_A::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        **self == PAD33STRNG_A::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        **self == PAD33STRNG_A::HIGH
    }
}
impl core::ops::Deref for PAD33STRNG_R {
    type Target = crate::FieldReader<bool, PAD33STRNG_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PAD33STRNG` writer - Pad 33 drive strength"]
pub struct PAD33STRNG_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD33STRNG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD33STRNG_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Low drive strength value."]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(PAD33STRNG_A::LOW)
    }
    #[doc = "High drive strength value."]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(PAD33STRNG_A::HIGH)
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
#[doc = "Pad 33 input enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD33INPEN_A {
    #[doc = "0: Pad input disabled value."]
    DIS = 0,
    #[doc = "1: Pad input enabled value."]
    EN = 1,
}
impl From<PAD33INPEN_A> for bool {
    #[inline(always)]
    fn from(variant: PAD33INPEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PAD33INPEN` reader - Pad 33 input enable"]
pub struct PAD33INPEN_R(crate::FieldReader<bool, PAD33INPEN_A>);
impl PAD33INPEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        PAD33INPEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAD33INPEN_A {
        match self.bits {
            false => PAD33INPEN_A::DIS,
            true => PAD33INPEN_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        **self == PAD33INPEN_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        **self == PAD33INPEN_A::EN
    }
}
impl core::ops::Deref for PAD33INPEN_R {
    type Target = crate::FieldReader<bool, PAD33INPEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PAD33INPEN` writer - Pad 33 input enable"]
pub struct PAD33INPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD33INPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD33INPEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Pad input disabled value."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(PAD33INPEN_A::DIS)
    }
    #[doc = "Pad input enabled value."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(PAD33INPEN_A::EN)
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
#[doc = "Pad 33 pullup enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD33PULL_A {
    #[doc = "0: Pullup disabled value."]
    DIS = 0,
    #[doc = "1: Pullup enabled value."]
    EN = 1,
}
impl From<PAD33PULL_A> for bool {
    #[inline(always)]
    fn from(variant: PAD33PULL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PAD33PULL` reader - Pad 33 pullup enable"]
pub struct PAD33PULL_R(crate::FieldReader<bool, PAD33PULL_A>);
impl PAD33PULL_R {
    pub(crate) fn new(bits: bool) -> Self {
        PAD33PULL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAD33PULL_A {
        match self.bits {
            false => PAD33PULL_A::DIS,
            true => PAD33PULL_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        **self == PAD33PULL_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        **self == PAD33PULL_A::EN
    }
}
impl core::ops::Deref for PAD33PULL_R {
    type Target = crate::FieldReader<bool, PAD33PULL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PAD33PULL` writer - Pad 33 pullup enable"]
pub struct PAD33PULL_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD33PULL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD33PULL_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Pullup disabled value."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(PAD33PULL_A::DIS)
    }
    #[doc = "Pullup enabled value."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(PAD33PULL_A::EN)
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
#[doc = "Pad 32 function select\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PAD32FNCSEL_A {
    #[doc = "0: Configure as the analog input for ADC single ended input 4 value."]
    ADCSE4 = 0,
    #[doc = "1: IOM/MSPI nCE group 32 value."]
    NCE32 = 1,
    #[doc = "2: CTIMER connection 15 value."]
    CT15 = 2,
    #[doc = "3: Configure as GPIO32 value."]
    GPIO32 = 3,
    #[doc = "4: SCARD serial data input/output value."]
    SCCIO = 4,
    #[doc = "5: External input to the LFRC oscillator value."]
    EXTLF = 5,
    #[doc = "7: Configure as the UART1 CTS input value."]
    UA1CTS = 7,
}
impl From<PAD32FNCSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: PAD32FNCSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `PAD32FNCSEL` reader - Pad 32 function select"]
pub struct PAD32FNCSEL_R(crate::FieldReader<u8, PAD32FNCSEL_A>);
impl PAD32FNCSEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        PAD32FNCSEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PAD32FNCSEL_A> {
        match self.bits {
            0 => Some(PAD32FNCSEL_A::ADCSE4),
            1 => Some(PAD32FNCSEL_A::NCE32),
            2 => Some(PAD32FNCSEL_A::CT15),
            3 => Some(PAD32FNCSEL_A::GPIO32),
            4 => Some(PAD32FNCSEL_A::SCCIO),
            5 => Some(PAD32FNCSEL_A::EXTLF),
            7 => Some(PAD32FNCSEL_A::UA1CTS),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `ADCSE4`"]
    #[inline(always)]
    pub fn is_adcse4(&self) -> bool {
        **self == PAD32FNCSEL_A::ADCSE4
    }
    #[doc = "Checks if the value of the field is `NCE32`"]
    #[inline(always)]
    pub fn is_nce32(&self) -> bool {
        **self == PAD32FNCSEL_A::NCE32
    }
    #[doc = "Checks if the value of the field is `CT15`"]
    #[inline(always)]
    pub fn is_ct15(&self) -> bool {
        **self == PAD32FNCSEL_A::CT15
    }
    #[doc = "Checks if the value of the field is `GPIO32`"]
    #[inline(always)]
    pub fn is_gpio32(&self) -> bool {
        **self == PAD32FNCSEL_A::GPIO32
    }
    #[doc = "Checks if the value of the field is `SCCIO`"]
    #[inline(always)]
    pub fn is_sccio(&self) -> bool {
        **self == PAD32FNCSEL_A::SCCIO
    }
    #[doc = "Checks if the value of the field is `EXTLF`"]
    #[inline(always)]
    pub fn is_extlf(&self) -> bool {
        **self == PAD32FNCSEL_A::EXTLF
    }
    #[doc = "Checks if the value of the field is `UA1CTS`"]
    #[inline(always)]
    pub fn is_ua1cts(&self) -> bool {
        **self == PAD32FNCSEL_A::UA1CTS
    }
}
impl core::ops::Deref for PAD32FNCSEL_R {
    type Target = crate::FieldReader<u8, PAD32FNCSEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PAD32FNCSEL` writer - Pad 32 function select"]
pub struct PAD32FNCSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD32FNCSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD32FNCSEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Configure as the analog input for ADC single ended input 4 value."]
    #[inline(always)]
    pub fn adcse4(self) -> &'a mut W {
        self.variant(PAD32FNCSEL_A::ADCSE4)
    }
    #[doc = "IOM/MSPI nCE group 32 value."]
    #[inline(always)]
    pub fn nce32(self) -> &'a mut W {
        self.variant(PAD32FNCSEL_A::NCE32)
    }
    #[doc = "CTIMER connection 15 value."]
    #[inline(always)]
    pub fn ct15(self) -> &'a mut W {
        self.variant(PAD32FNCSEL_A::CT15)
    }
    #[doc = "Configure as GPIO32 value."]
    #[inline(always)]
    pub fn gpio32(self) -> &'a mut W {
        self.variant(PAD32FNCSEL_A::GPIO32)
    }
    #[doc = "SCARD serial data input/output value."]
    #[inline(always)]
    pub fn sccio(self) -> &'a mut W {
        self.variant(PAD32FNCSEL_A::SCCIO)
    }
    #[doc = "External input to the LFRC oscillator value."]
    #[inline(always)]
    pub fn extlf(self) -> &'a mut W {
        self.variant(PAD32FNCSEL_A::EXTLF)
    }
    #[doc = "Configure as the UART1 CTS input value."]
    #[inline(always)]
    pub fn ua1cts(self) -> &'a mut W {
        self.variant(PAD32FNCSEL_A::UA1CTS)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 3)) | ((value as u32 & 0x07) << 3);
        self.w
    }
}
#[doc = "Pad 32 drive strength\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD32STRNG_A {
    #[doc = "0: Low drive strength value."]
    LOW = 0,
    #[doc = "1: High drive strength value."]
    HIGH = 1,
}
impl From<PAD32STRNG_A> for bool {
    #[inline(always)]
    fn from(variant: PAD32STRNG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PAD32STRNG` reader - Pad 32 drive strength"]
pub struct PAD32STRNG_R(crate::FieldReader<bool, PAD32STRNG_A>);
impl PAD32STRNG_R {
    pub(crate) fn new(bits: bool) -> Self {
        PAD32STRNG_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAD32STRNG_A {
        match self.bits {
            false => PAD32STRNG_A::LOW,
            true => PAD32STRNG_A::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        **self == PAD32STRNG_A::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        **self == PAD32STRNG_A::HIGH
    }
}
impl core::ops::Deref for PAD32STRNG_R {
    type Target = crate::FieldReader<bool, PAD32STRNG_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PAD32STRNG` writer - Pad 32 drive strength"]
pub struct PAD32STRNG_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD32STRNG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD32STRNG_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Low drive strength value."]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(PAD32STRNG_A::LOW)
    }
    #[doc = "High drive strength value."]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(PAD32STRNG_A::HIGH)
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
#[doc = "Pad 32 input enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD32INPEN_A {
    #[doc = "0: Pad input disabled value."]
    DIS = 0,
    #[doc = "1: Pad input enabled value."]
    EN = 1,
}
impl From<PAD32INPEN_A> for bool {
    #[inline(always)]
    fn from(variant: PAD32INPEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PAD32INPEN` reader - Pad 32 input enable"]
pub struct PAD32INPEN_R(crate::FieldReader<bool, PAD32INPEN_A>);
impl PAD32INPEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        PAD32INPEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAD32INPEN_A {
        match self.bits {
            false => PAD32INPEN_A::DIS,
            true => PAD32INPEN_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        **self == PAD32INPEN_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        **self == PAD32INPEN_A::EN
    }
}
impl core::ops::Deref for PAD32INPEN_R {
    type Target = crate::FieldReader<bool, PAD32INPEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PAD32INPEN` writer - Pad 32 input enable"]
pub struct PAD32INPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD32INPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD32INPEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Pad input disabled value."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(PAD32INPEN_A::DIS)
    }
    #[doc = "Pad input enabled value."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(PAD32INPEN_A::EN)
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
#[doc = "Pad 32 pullup enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD32PULL_A {
    #[doc = "0: Pullup disabled value."]
    DIS = 0,
    #[doc = "1: Pullup enabled value."]
    EN = 1,
}
impl From<PAD32PULL_A> for bool {
    #[inline(always)]
    fn from(variant: PAD32PULL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PAD32PULL` reader - Pad 32 pullup enable"]
pub struct PAD32PULL_R(crate::FieldReader<bool, PAD32PULL_A>);
impl PAD32PULL_R {
    pub(crate) fn new(bits: bool) -> Self {
        PAD32PULL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAD32PULL_A {
        match self.bits {
            false => PAD32PULL_A::DIS,
            true => PAD32PULL_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        **self == PAD32PULL_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        **self == PAD32PULL_A::EN
    }
}
impl core::ops::Deref for PAD32PULL_R {
    type Target = crate::FieldReader<bool, PAD32PULL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PAD32PULL` writer - Pad 32 pullup enable"]
pub struct PAD32PULL_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD32PULL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD32PULL_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Pullup disabled value."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(PAD32PULL_A::DIS)
    }
    #[doc = "Pullup enabled value."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(PAD32PULL_A::EN)
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
    #[doc = "Bits 27:29 - Pad 35 function select"]
    #[inline(always)]
    pub fn pad35fncsel(&self) -> PAD35FNCSEL_R {
        PAD35FNCSEL_R::new(((self.bits >> 27) & 0x07) as u8)
    }
    #[doc = "Bit 26 - Pad 35 drive strength"]
    #[inline(always)]
    pub fn pad35strng(&self) -> PAD35STRNG_R {
        PAD35STRNG_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Pad 35 input enable"]
    #[inline(always)]
    pub fn pad35inpen(&self) -> PAD35INPEN_R {
        PAD35INPEN_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Pad 35 pullup enable"]
    #[inline(always)]
    pub fn pad35pull(&self) -> PAD35PULL_R {
        PAD35PULL_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bits 19:21 - Pad 34 function select"]
    #[inline(always)]
    pub fn pad34fncsel(&self) -> PAD34FNCSEL_R {
        PAD34FNCSEL_R::new(((self.bits >> 19) & 0x07) as u8)
    }
    #[doc = "Bit 18 - Pad 34 drive strength"]
    #[inline(always)]
    pub fn pad34strng(&self) -> PAD34STRNG_R {
        PAD34STRNG_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Pad 34 input enable"]
    #[inline(always)]
    pub fn pad34inpen(&self) -> PAD34INPEN_R {
        PAD34INPEN_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Pad 34 pullup enable"]
    #[inline(always)]
    pub fn pad34pull(&self) -> PAD34PULL_R {
        PAD34PULL_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bits 11:13 - Pad 33 function select"]
    #[inline(always)]
    pub fn pad33fncsel(&self) -> PAD33FNCSEL_R {
        PAD33FNCSEL_R::new(((self.bits >> 11) & 0x07) as u8)
    }
    #[doc = "Bit 10 - Pad 33 drive strength"]
    #[inline(always)]
    pub fn pad33strng(&self) -> PAD33STRNG_R {
        PAD33STRNG_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Pad 33 input enable"]
    #[inline(always)]
    pub fn pad33inpen(&self) -> PAD33INPEN_R {
        PAD33INPEN_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Pad 33 pullup enable"]
    #[inline(always)]
    pub fn pad33pull(&self) -> PAD33PULL_R {
        PAD33PULL_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bits 3:5 - Pad 32 function select"]
    #[inline(always)]
    pub fn pad32fncsel(&self) -> PAD32FNCSEL_R {
        PAD32FNCSEL_R::new(((self.bits >> 3) & 0x07) as u8)
    }
    #[doc = "Bit 2 - Pad 32 drive strength"]
    #[inline(always)]
    pub fn pad32strng(&self) -> PAD32STRNG_R {
        PAD32STRNG_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - Pad 32 input enable"]
    #[inline(always)]
    pub fn pad32inpen(&self) -> PAD32INPEN_R {
        PAD32INPEN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Pad 32 pullup enable"]
    #[inline(always)]
    pub fn pad32pull(&self) -> PAD32PULL_R {
        PAD32PULL_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 27:29 - Pad 35 function select"]
    #[inline(always)]
    pub fn pad35fncsel(&mut self) -> PAD35FNCSEL_W {
        PAD35FNCSEL_W { w: self }
    }
    #[doc = "Bit 26 - Pad 35 drive strength"]
    #[inline(always)]
    pub fn pad35strng(&mut self) -> PAD35STRNG_W {
        PAD35STRNG_W { w: self }
    }
    #[doc = "Bit 25 - Pad 35 input enable"]
    #[inline(always)]
    pub fn pad35inpen(&mut self) -> PAD35INPEN_W {
        PAD35INPEN_W { w: self }
    }
    #[doc = "Bit 24 - Pad 35 pullup enable"]
    #[inline(always)]
    pub fn pad35pull(&mut self) -> PAD35PULL_W {
        PAD35PULL_W { w: self }
    }
    #[doc = "Bits 19:21 - Pad 34 function select"]
    #[inline(always)]
    pub fn pad34fncsel(&mut self) -> PAD34FNCSEL_W {
        PAD34FNCSEL_W { w: self }
    }
    #[doc = "Bit 18 - Pad 34 drive strength"]
    #[inline(always)]
    pub fn pad34strng(&mut self) -> PAD34STRNG_W {
        PAD34STRNG_W { w: self }
    }
    #[doc = "Bit 17 - Pad 34 input enable"]
    #[inline(always)]
    pub fn pad34inpen(&mut self) -> PAD34INPEN_W {
        PAD34INPEN_W { w: self }
    }
    #[doc = "Bit 16 - Pad 34 pullup enable"]
    #[inline(always)]
    pub fn pad34pull(&mut self) -> PAD34PULL_W {
        PAD34PULL_W { w: self }
    }
    #[doc = "Bits 11:13 - Pad 33 function select"]
    #[inline(always)]
    pub fn pad33fncsel(&mut self) -> PAD33FNCSEL_W {
        PAD33FNCSEL_W { w: self }
    }
    #[doc = "Bit 10 - Pad 33 drive strength"]
    #[inline(always)]
    pub fn pad33strng(&mut self) -> PAD33STRNG_W {
        PAD33STRNG_W { w: self }
    }
    #[doc = "Bit 9 - Pad 33 input enable"]
    #[inline(always)]
    pub fn pad33inpen(&mut self) -> PAD33INPEN_W {
        PAD33INPEN_W { w: self }
    }
    #[doc = "Bit 8 - Pad 33 pullup enable"]
    #[inline(always)]
    pub fn pad33pull(&mut self) -> PAD33PULL_W {
        PAD33PULL_W { w: self }
    }
    #[doc = "Bits 3:5 - Pad 32 function select"]
    #[inline(always)]
    pub fn pad32fncsel(&mut self) -> PAD32FNCSEL_W {
        PAD32FNCSEL_W { w: self }
    }
    #[doc = "Bit 2 - Pad 32 drive strength"]
    #[inline(always)]
    pub fn pad32strng(&mut self) -> PAD32STRNG_W {
        PAD32STRNG_W { w: self }
    }
    #[doc = "Bit 1 - Pad 32 input enable"]
    #[inline(always)]
    pub fn pad32inpen(&mut self) -> PAD32INPEN_W {
        PAD32INPEN_W { w: self }
    }
    #[doc = "Bit 0 - Pad 32 pullup enable"]
    #[inline(always)]
    pub fn pad32pull(&mut self) -> PAD32PULL_W {
        PAD32PULL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Pad Configuration Register I (Pads 32-25)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [padregi](index.html) module"]
pub struct PADREGI_SPEC;
impl crate::RegisterSpec for PADREGI_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [padregi::R](R) reader structure"]
impl crate::Readable for PADREGI_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [padregi::W](W) writer structure"]
impl crate::Writable for PADREGI_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PADREGI to value 0x1818_1818"]
impl crate::Resettable for PADREGI_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x1818_1818
    }
}
