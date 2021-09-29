#[doc = "Register `PADREGE` reader"]
pub struct R(crate::R<PADREGE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PADREGE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PADREGE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PADREGE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PADREGE` writer"]
pub struct W(crate::W<PADREGE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PADREGE_SPEC>;
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
impl From<crate::W<PADREGE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PADREGE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Pad 19 function select\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PAD19FNCSEL_A {
    #[doc = "0: Configure as the analog comparator reference 0 signal value."]
    CMPRF0 = 0,
    #[doc = "1: IOM/MSPI nCE group 19 value."]
    NCE19 = 1,
    #[doc = "2: CTIMER conenction 6 value."]
    CT6 = 2,
    #[doc = "3: Configure as GPIO19 value."]
    GPIO19 = 3,
    #[doc = "4: SCARD serial clock value."]
    SCCLK = 4,
    #[doc = "5: Configure as the ANATEST1 I/O signal value."]
    ANATEST1 = 5,
    #[doc = "6: Configure as the UART1 RX input signal value."]
    UART1RX = 6,
    #[doc = "7: Configure as the PDM I2S bit clock input signal value."]
    I2SBCLK = 7,
}
impl From<PAD19FNCSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: PAD19FNCSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `PAD19FNCSEL` reader - Pad 19 function select"]
pub struct PAD19FNCSEL_R(crate::FieldReader<u8, PAD19FNCSEL_A>);
impl PAD19FNCSEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        PAD19FNCSEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAD19FNCSEL_A {
        match self.bits {
            0 => PAD19FNCSEL_A::CMPRF0,
            1 => PAD19FNCSEL_A::NCE19,
            2 => PAD19FNCSEL_A::CT6,
            3 => PAD19FNCSEL_A::GPIO19,
            4 => PAD19FNCSEL_A::SCCLK,
            5 => PAD19FNCSEL_A::ANATEST1,
            6 => PAD19FNCSEL_A::UART1RX,
            7 => PAD19FNCSEL_A::I2SBCLK,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `CMPRF0`"]
    #[inline(always)]
    pub fn is_cmprf0(&self) -> bool {
        **self == PAD19FNCSEL_A::CMPRF0
    }
    #[doc = "Checks if the value of the field is `NCE19`"]
    #[inline(always)]
    pub fn is_nce19(&self) -> bool {
        **self == PAD19FNCSEL_A::NCE19
    }
    #[doc = "Checks if the value of the field is `CT6`"]
    #[inline(always)]
    pub fn is_ct6(&self) -> bool {
        **self == PAD19FNCSEL_A::CT6
    }
    #[doc = "Checks if the value of the field is `GPIO19`"]
    #[inline(always)]
    pub fn is_gpio19(&self) -> bool {
        **self == PAD19FNCSEL_A::GPIO19
    }
    #[doc = "Checks if the value of the field is `SCCLK`"]
    #[inline(always)]
    pub fn is_scclk(&self) -> bool {
        **self == PAD19FNCSEL_A::SCCLK
    }
    #[doc = "Checks if the value of the field is `ANATEST1`"]
    #[inline(always)]
    pub fn is_anatest1(&self) -> bool {
        **self == PAD19FNCSEL_A::ANATEST1
    }
    #[doc = "Checks if the value of the field is `UART1RX`"]
    #[inline(always)]
    pub fn is_uart1rx(&self) -> bool {
        **self == PAD19FNCSEL_A::UART1RX
    }
    #[doc = "Checks if the value of the field is `I2SBCLK`"]
    #[inline(always)]
    pub fn is_i2sbclk(&self) -> bool {
        **self == PAD19FNCSEL_A::I2SBCLK
    }
}
impl core::ops::Deref for PAD19FNCSEL_R {
    type Target = crate::FieldReader<u8, PAD19FNCSEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PAD19FNCSEL` writer - Pad 19 function select"]
pub struct PAD19FNCSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD19FNCSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD19FNCSEL_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Configure as the analog comparator reference 0 signal value."]
    #[inline(always)]
    pub fn cmprf0(self) -> &'a mut W {
        self.variant(PAD19FNCSEL_A::CMPRF0)
    }
    #[doc = "IOM/MSPI nCE group 19 value."]
    #[inline(always)]
    pub fn nce19(self) -> &'a mut W {
        self.variant(PAD19FNCSEL_A::NCE19)
    }
    #[doc = "CTIMER conenction 6 value."]
    #[inline(always)]
    pub fn ct6(self) -> &'a mut W {
        self.variant(PAD19FNCSEL_A::CT6)
    }
    #[doc = "Configure as GPIO19 value."]
    #[inline(always)]
    pub fn gpio19(self) -> &'a mut W {
        self.variant(PAD19FNCSEL_A::GPIO19)
    }
    #[doc = "SCARD serial clock value."]
    #[inline(always)]
    pub fn scclk(self) -> &'a mut W {
        self.variant(PAD19FNCSEL_A::SCCLK)
    }
    #[doc = "Configure as the ANATEST1 I/O signal value."]
    #[inline(always)]
    pub fn anatest1(self) -> &'a mut W {
        self.variant(PAD19FNCSEL_A::ANATEST1)
    }
    #[doc = "Configure as the UART1 RX input signal value."]
    #[inline(always)]
    pub fn uart1rx(self) -> &'a mut W {
        self.variant(PAD19FNCSEL_A::UART1RX)
    }
    #[doc = "Configure as the PDM I2S bit clock input signal value."]
    #[inline(always)]
    pub fn i2sbclk(self) -> &'a mut W {
        self.variant(PAD19FNCSEL_A::I2SBCLK)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 27)) | ((value as u32 & 0x07) << 27);
        self.w
    }
}
#[doc = "Pad 19 drive strength\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD19STRNG_A {
    #[doc = "0: Low drive strength value."]
    LOW = 0,
    #[doc = "1: High drive strength value."]
    HIGH = 1,
}
impl From<PAD19STRNG_A> for bool {
    #[inline(always)]
    fn from(variant: PAD19STRNG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PAD19STRNG` reader - Pad 19 drive strength"]
pub struct PAD19STRNG_R(crate::FieldReader<bool, PAD19STRNG_A>);
impl PAD19STRNG_R {
    pub(crate) fn new(bits: bool) -> Self {
        PAD19STRNG_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAD19STRNG_A {
        match self.bits {
            false => PAD19STRNG_A::LOW,
            true => PAD19STRNG_A::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        **self == PAD19STRNG_A::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        **self == PAD19STRNG_A::HIGH
    }
}
impl core::ops::Deref for PAD19STRNG_R {
    type Target = crate::FieldReader<bool, PAD19STRNG_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PAD19STRNG` writer - Pad 19 drive strength"]
pub struct PAD19STRNG_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD19STRNG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD19STRNG_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Low drive strength value."]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(PAD19STRNG_A::LOW)
    }
    #[doc = "High drive strength value."]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(PAD19STRNG_A::HIGH)
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
#[doc = "Pad 19 input enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD19INPEN_A {
    #[doc = "0: Pad input disabled value."]
    DIS = 0,
    #[doc = "1: Pad input enabled value."]
    EN = 1,
}
impl From<PAD19INPEN_A> for bool {
    #[inline(always)]
    fn from(variant: PAD19INPEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PAD19INPEN` reader - Pad 19 input enable"]
pub struct PAD19INPEN_R(crate::FieldReader<bool, PAD19INPEN_A>);
impl PAD19INPEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        PAD19INPEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAD19INPEN_A {
        match self.bits {
            false => PAD19INPEN_A::DIS,
            true => PAD19INPEN_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        **self == PAD19INPEN_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        **self == PAD19INPEN_A::EN
    }
}
impl core::ops::Deref for PAD19INPEN_R {
    type Target = crate::FieldReader<bool, PAD19INPEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PAD19INPEN` writer - Pad 19 input enable"]
pub struct PAD19INPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD19INPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD19INPEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Pad input disabled value."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(PAD19INPEN_A::DIS)
    }
    #[doc = "Pad input enabled value."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(PAD19INPEN_A::EN)
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
#[doc = "Pad 19 pullup enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD19PULL_A {
    #[doc = "0: Pullup disabled value."]
    DIS = 0,
    #[doc = "1: Pullup enabled value."]
    EN = 1,
}
impl From<PAD19PULL_A> for bool {
    #[inline(always)]
    fn from(variant: PAD19PULL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PAD19PULL` reader - Pad 19 pullup enable"]
pub struct PAD19PULL_R(crate::FieldReader<bool, PAD19PULL_A>);
impl PAD19PULL_R {
    pub(crate) fn new(bits: bool) -> Self {
        PAD19PULL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAD19PULL_A {
        match self.bits {
            false => PAD19PULL_A::DIS,
            true => PAD19PULL_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        **self == PAD19PULL_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        **self == PAD19PULL_A::EN
    }
}
impl core::ops::Deref for PAD19PULL_R {
    type Target = crate::FieldReader<bool, PAD19PULL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PAD19PULL` writer - Pad 19 pullup enable"]
pub struct PAD19PULL_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD19PULL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD19PULL_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Pullup disabled value."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(PAD19PULL_A::DIS)
    }
    #[doc = "Pullup enabled value."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(PAD19PULL_A::EN)
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
#[doc = "Pad 18 function select\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PAD18FNCSEL_A {
    #[doc = "0: Configure as the analog comparator input 1 signal value."]
    CMPIN1 = 0,
    #[doc = "1: IOM/MSPI nCE group 18 value."]
    NCE18 = 1,
    #[doc = "2: CTIMER connection 4 value."]
    CT4 = 2,
    #[doc = "3: Configure as GPIO18 value."]
    GPIO18 = 3,
    #[doc = "4: Configure as UART0 RTS output signal value."]
    UA0RTS = 4,
    #[doc = "5: Configure as ANATEST2 I/O signal value."]
    ANATEST2 = 5,
    #[doc = "6: Configure as UART1 TX output signal value."]
    UART1TX = 6,
    #[doc = "7: SCARD data input/output connectin value."]
    SCCIO = 7,
}
impl From<PAD18FNCSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: PAD18FNCSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `PAD18FNCSEL` reader - Pad 18 function select"]
pub struct PAD18FNCSEL_R(crate::FieldReader<u8, PAD18FNCSEL_A>);
impl PAD18FNCSEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        PAD18FNCSEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAD18FNCSEL_A {
        match self.bits {
            0 => PAD18FNCSEL_A::CMPIN1,
            1 => PAD18FNCSEL_A::NCE18,
            2 => PAD18FNCSEL_A::CT4,
            3 => PAD18FNCSEL_A::GPIO18,
            4 => PAD18FNCSEL_A::UA0RTS,
            5 => PAD18FNCSEL_A::ANATEST2,
            6 => PAD18FNCSEL_A::UART1TX,
            7 => PAD18FNCSEL_A::SCCIO,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `CMPIN1`"]
    #[inline(always)]
    pub fn is_cmpin1(&self) -> bool {
        **self == PAD18FNCSEL_A::CMPIN1
    }
    #[doc = "Checks if the value of the field is `NCE18`"]
    #[inline(always)]
    pub fn is_nce18(&self) -> bool {
        **self == PAD18FNCSEL_A::NCE18
    }
    #[doc = "Checks if the value of the field is `CT4`"]
    #[inline(always)]
    pub fn is_ct4(&self) -> bool {
        **self == PAD18FNCSEL_A::CT4
    }
    #[doc = "Checks if the value of the field is `GPIO18`"]
    #[inline(always)]
    pub fn is_gpio18(&self) -> bool {
        **self == PAD18FNCSEL_A::GPIO18
    }
    #[doc = "Checks if the value of the field is `UA0RTS`"]
    #[inline(always)]
    pub fn is_ua0rts(&self) -> bool {
        **self == PAD18FNCSEL_A::UA0RTS
    }
    #[doc = "Checks if the value of the field is `ANATEST2`"]
    #[inline(always)]
    pub fn is_anatest2(&self) -> bool {
        **self == PAD18FNCSEL_A::ANATEST2
    }
    #[doc = "Checks if the value of the field is `UART1TX`"]
    #[inline(always)]
    pub fn is_uart1tx(&self) -> bool {
        **self == PAD18FNCSEL_A::UART1TX
    }
    #[doc = "Checks if the value of the field is `SCCIO`"]
    #[inline(always)]
    pub fn is_sccio(&self) -> bool {
        **self == PAD18FNCSEL_A::SCCIO
    }
}
impl core::ops::Deref for PAD18FNCSEL_R {
    type Target = crate::FieldReader<u8, PAD18FNCSEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PAD18FNCSEL` writer - Pad 18 function select"]
pub struct PAD18FNCSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD18FNCSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD18FNCSEL_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Configure as the analog comparator input 1 signal value."]
    #[inline(always)]
    pub fn cmpin1(self) -> &'a mut W {
        self.variant(PAD18FNCSEL_A::CMPIN1)
    }
    #[doc = "IOM/MSPI nCE group 18 value."]
    #[inline(always)]
    pub fn nce18(self) -> &'a mut W {
        self.variant(PAD18FNCSEL_A::NCE18)
    }
    #[doc = "CTIMER connection 4 value."]
    #[inline(always)]
    pub fn ct4(self) -> &'a mut W {
        self.variant(PAD18FNCSEL_A::CT4)
    }
    #[doc = "Configure as GPIO18 value."]
    #[inline(always)]
    pub fn gpio18(self) -> &'a mut W {
        self.variant(PAD18FNCSEL_A::GPIO18)
    }
    #[doc = "Configure as UART0 RTS output signal value."]
    #[inline(always)]
    pub fn ua0rts(self) -> &'a mut W {
        self.variant(PAD18FNCSEL_A::UA0RTS)
    }
    #[doc = "Configure as ANATEST2 I/O signal value."]
    #[inline(always)]
    pub fn anatest2(self) -> &'a mut W {
        self.variant(PAD18FNCSEL_A::ANATEST2)
    }
    #[doc = "Configure as UART1 TX output signal value."]
    #[inline(always)]
    pub fn uart1tx(self) -> &'a mut W {
        self.variant(PAD18FNCSEL_A::UART1TX)
    }
    #[doc = "SCARD data input/output connectin value."]
    #[inline(always)]
    pub fn sccio(self) -> &'a mut W {
        self.variant(PAD18FNCSEL_A::SCCIO)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 19)) | ((value as u32 & 0x07) << 19);
        self.w
    }
}
#[doc = "Pad 18 drive strength\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD18STRNG_A {
    #[doc = "0: Low drive strength value."]
    LOW = 0,
    #[doc = "1: High drive strength value."]
    HIGH = 1,
}
impl From<PAD18STRNG_A> for bool {
    #[inline(always)]
    fn from(variant: PAD18STRNG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PAD18STRNG` reader - Pad 18 drive strength"]
pub struct PAD18STRNG_R(crate::FieldReader<bool, PAD18STRNG_A>);
impl PAD18STRNG_R {
    pub(crate) fn new(bits: bool) -> Self {
        PAD18STRNG_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAD18STRNG_A {
        match self.bits {
            false => PAD18STRNG_A::LOW,
            true => PAD18STRNG_A::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        **self == PAD18STRNG_A::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        **self == PAD18STRNG_A::HIGH
    }
}
impl core::ops::Deref for PAD18STRNG_R {
    type Target = crate::FieldReader<bool, PAD18STRNG_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PAD18STRNG` writer - Pad 18 drive strength"]
pub struct PAD18STRNG_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD18STRNG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD18STRNG_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Low drive strength value."]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(PAD18STRNG_A::LOW)
    }
    #[doc = "High drive strength value."]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(PAD18STRNG_A::HIGH)
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
#[doc = "Pad 18 input enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD18INPEN_A {
    #[doc = "0: Pad input disabled value."]
    DIS = 0,
    #[doc = "1: Pad input enabled value."]
    EN = 1,
}
impl From<PAD18INPEN_A> for bool {
    #[inline(always)]
    fn from(variant: PAD18INPEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PAD18INPEN` reader - Pad 18 input enable"]
pub struct PAD18INPEN_R(crate::FieldReader<bool, PAD18INPEN_A>);
impl PAD18INPEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        PAD18INPEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAD18INPEN_A {
        match self.bits {
            false => PAD18INPEN_A::DIS,
            true => PAD18INPEN_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        **self == PAD18INPEN_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        **self == PAD18INPEN_A::EN
    }
}
impl core::ops::Deref for PAD18INPEN_R {
    type Target = crate::FieldReader<bool, PAD18INPEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PAD18INPEN` writer - Pad 18 input enable"]
pub struct PAD18INPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD18INPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD18INPEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Pad input disabled value."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(PAD18INPEN_A::DIS)
    }
    #[doc = "Pad input enabled value."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(PAD18INPEN_A::EN)
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
#[doc = "Pad 18 pullup enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD18PULL_A {
    #[doc = "0: Pullup disabled value."]
    DIS = 0,
    #[doc = "1: Pullup enabled value."]
    EN = 1,
}
impl From<PAD18PULL_A> for bool {
    #[inline(always)]
    fn from(variant: PAD18PULL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PAD18PULL` reader - Pad 18 pullup enable"]
pub struct PAD18PULL_R(crate::FieldReader<bool, PAD18PULL_A>);
impl PAD18PULL_R {
    pub(crate) fn new(bits: bool) -> Self {
        PAD18PULL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAD18PULL_A {
        match self.bits {
            false => PAD18PULL_A::DIS,
            true => PAD18PULL_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        **self == PAD18PULL_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        **self == PAD18PULL_A::EN
    }
}
impl core::ops::Deref for PAD18PULL_R {
    type Target = crate::FieldReader<bool, PAD18PULL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PAD18PULL` writer - Pad 18 pullup enable"]
pub struct PAD18PULL_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD18PULL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD18PULL_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Pullup disabled value."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(PAD18PULL_A::DIS)
    }
    #[doc = "Pullup enabled value."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(PAD18PULL_A::EN)
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
#[doc = "Pad 17 function select\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PAD17FNCSEL_A {
    #[doc = "0: Configure as the analog comparator reference signal 1 input signal value."]
    CMPRF1 = 0,
    #[doc = "1: IOM/MSPI nCE group 17 value."]
    NCE17 = 1,
    #[doc = "2: Configure as the ADC Trigger 1 signal value."]
    TRIG1 = 2,
    #[doc = "3: Configure as GPIO17 value."]
    GPIO17 = 3,
    #[doc = "4: SCARD serial clock output value."]
    SCCCLK = 4,
    #[doc = "6: Configure as UART0 RX input signal value."]
    UART0RX = 6,
    #[doc = "7: Configure as UART1 CTS input signal value."]
    UA1CTS = 7,
}
impl From<PAD17FNCSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: PAD17FNCSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `PAD17FNCSEL` reader - Pad 17 function select"]
pub struct PAD17FNCSEL_R(crate::FieldReader<u8, PAD17FNCSEL_A>);
impl PAD17FNCSEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        PAD17FNCSEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PAD17FNCSEL_A> {
        match self.bits {
            0 => Some(PAD17FNCSEL_A::CMPRF1),
            1 => Some(PAD17FNCSEL_A::NCE17),
            2 => Some(PAD17FNCSEL_A::TRIG1),
            3 => Some(PAD17FNCSEL_A::GPIO17),
            4 => Some(PAD17FNCSEL_A::SCCCLK),
            6 => Some(PAD17FNCSEL_A::UART0RX),
            7 => Some(PAD17FNCSEL_A::UA1CTS),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `CMPRF1`"]
    #[inline(always)]
    pub fn is_cmprf1(&self) -> bool {
        **self == PAD17FNCSEL_A::CMPRF1
    }
    #[doc = "Checks if the value of the field is `NCE17`"]
    #[inline(always)]
    pub fn is_nce17(&self) -> bool {
        **self == PAD17FNCSEL_A::NCE17
    }
    #[doc = "Checks if the value of the field is `TRIG1`"]
    #[inline(always)]
    pub fn is_trig1(&self) -> bool {
        **self == PAD17FNCSEL_A::TRIG1
    }
    #[doc = "Checks if the value of the field is `GPIO17`"]
    #[inline(always)]
    pub fn is_gpio17(&self) -> bool {
        **self == PAD17FNCSEL_A::GPIO17
    }
    #[doc = "Checks if the value of the field is `SCCCLK`"]
    #[inline(always)]
    pub fn is_sccclk(&self) -> bool {
        **self == PAD17FNCSEL_A::SCCCLK
    }
    #[doc = "Checks if the value of the field is `UART0RX`"]
    #[inline(always)]
    pub fn is_uart0rx(&self) -> bool {
        **self == PAD17FNCSEL_A::UART0RX
    }
    #[doc = "Checks if the value of the field is `UA1CTS`"]
    #[inline(always)]
    pub fn is_ua1cts(&self) -> bool {
        **self == PAD17FNCSEL_A::UA1CTS
    }
}
impl core::ops::Deref for PAD17FNCSEL_R {
    type Target = crate::FieldReader<u8, PAD17FNCSEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PAD17FNCSEL` writer - Pad 17 function select"]
pub struct PAD17FNCSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD17FNCSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD17FNCSEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Configure as the analog comparator reference signal 1 input signal value."]
    #[inline(always)]
    pub fn cmprf1(self) -> &'a mut W {
        self.variant(PAD17FNCSEL_A::CMPRF1)
    }
    #[doc = "IOM/MSPI nCE group 17 value."]
    #[inline(always)]
    pub fn nce17(self) -> &'a mut W {
        self.variant(PAD17FNCSEL_A::NCE17)
    }
    #[doc = "Configure as the ADC Trigger 1 signal value."]
    #[inline(always)]
    pub fn trig1(self) -> &'a mut W {
        self.variant(PAD17FNCSEL_A::TRIG1)
    }
    #[doc = "Configure as GPIO17 value."]
    #[inline(always)]
    pub fn gpio17(self) -> &'a mut W {
        self.variant(PAD17FNCSEL_A::GPIO17)
    }
    #[doc = "SCARD serial clock output value."]
    #[inline(always)]
    pub fn sccclk(self) -> &'a mut W {
        self.variant(PAD17FNCSEL_A::SCCCLK)
    }
    #[doc = "Configure as UART0 RX input signal value."]
    #[inline(always)]
    pub fn uart0rx(self) -> &'a mut W {
        self.variant(PAD17FNCSEL_A::UART0RX)
    }
    #[doc = "Configure as UART1 CTS input signal value."]
    #[inline(always)]
    pub fn ua1cts(self) -> &'a mut W {
        self.variant(PAD17FNCSEL_A::UA1CTS)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 11)) | ((value as u32 & 0x07) << 11);
        self.w
    }
}
#[doc = "Pad 17 drive strength\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD17STRNG_A {
    #[doc = "0: Low drive strength value."]
    LOW = 0,
    #[doc = "1: High drive strength value."]
    HIGH = 1,
}
impl From<PAD17STRNG_A> for bool {
    #[inline(always)]
    fn from(variant: PAD17STRNG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PAD17STRNG` reader - Pad 17 drive strength"]
pub struct PAD17STRNG_R(crate::FieldReader<bool, PAD17STRNG_A>);
impl PAD17STRNG_R {
    pub(crate) fn new(bits: bool) -> Self {
        PAD17STRNG_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAD17STRNG_A {
        match self.bits {
            false => PAD17STRNG_A::LOW,
            true => PAD17STRNG_A::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        **self == PAD17STRNG_A::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        **self == PAD17STRNG_A::HIGH
    }
}
impl core::ops::Deref for PAD17STRNG_R {
    type Target = crate::FieldReader<bool, PAD17STRNG_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PAD17STRNG` writer - Pad 17 drive strength"]
pub struct PAD17STRNG_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD17STRNG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD17STRNG_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Low drive strength value."]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(PAD17STRNG_A::LOW)
    }
    #[doc = "High drive strength value."]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(PAD17STRNG_A::HIGH)
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
#[doc = "Pad 17 input enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD17INPEN_A {
    #[doc = "0: Pad input disabled value."]
    DIS = 0,
    #[doc = "1: Pad input enabled value."]
    EN = 1,
}
impl From<PAD17INPEN_A> for bool {
    #[inline(always)]
    fn from(variant: PAD17INPEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PAD17INPEN` reader - Pad 17 input enable"]
pub struct PAD17INPEN_R(crate::FieldReader<bool, PAD17INPEN_A>);
impl PAD17INPEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        PAD17INPEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAD17INPEN_A {
        match self.bits {
            false => PAD17INPEN_A::DIS,
            true => PAD17INPEN_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        **self == PAD17INPEN_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        **self == PAD17INPEN_A::EN
    }
}
impl core::ops::Deref for PAD17INPEN_R {
    type Target = crate::FieldReader<bool, PAD17INPEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PAD17INPEN` writer - Pad 17 input enable"]
pub struct PAD17INPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD17INPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD17INPEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Pad input disabled value."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(PAD17INPEN_A::DIS)
    }
    #[doc = "Pad input enabled value."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(PAD17INPEN_A::EN)
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
#[doc = "Pad 17 pullup enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD17PULL_A {
    #[doc = "0: Pullup disabled value."]
    DIS = 0,
    #[doc = "1: Pullup enabled value."]
    EN = 1,
}
impl From<PAD17PULL_A> for bool {
    #[inline(always)]
    fn from(variant: PAD17PULL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PAD17PULL` reader - Pad 17 pullup enable"]
pub struct PAD17PULL_R(crate::FieldReader<bool, PAD17PULL_A>);
impl PAD17PULL_R {
    pub(crate) fn new(bits: bool) -> Self {
        PAD17PULL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAD17PULL_A {
        match self.bits {
            false => PAD17PULL_A::DIS,
            true => PAD17PULL_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        **self == PAD17PULL_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        **self == PAD17PULL_A::EN
    }
}
impl core::ops::Deref for PAD17PULL_R {
    type Target = crate::FieldReader<bool, PAD17PULL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PAD17PULL` writer - Pad 17 pullup enable"]
pub struct PAD17PULL_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD17PULL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD17PULL_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Pullup disabled value."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(PAD17PULL_A::DIS)
    }
    #[doc = "Pullup enabled value."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(PAD17PULL_A::EN)
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
#[doc = "Pad 16 function select\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PAD16FNCSEL_A {
    #[doc = "0: Configure as the analog ADC single ended port 0 input signal value."]
    ADCSE0 = 0,
    #[doc = "1: IOM/MSPI nCE group 16 value."]
    NCE16 = 1,
    #[doc = "2: Configure as the ADC Trigger 0 signal value."]
    TRIG0 = 2,
    #[doc = "3: Configure as GPIO16 value."]
    GPIO16 = 3,
    #[doc = "4: SCARD reset output value."]
    SCCRST = 4,
    #[doc = "5: Configure as comparator input 0 signal value."]
    CMPIN0 = 5,
    #[doc = "6: Configure as UART0 TX output signal value."]
    UART0TX = 6,
    #[doc = "7: Configure as UART1 RTS output signal value."]
    UA1RTS = 7,
}
impl From<PAD16FNCSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: PAD16FNCSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `PAD16FNCSEL` reader - Pad 16 function select"]
pub struct PAD16FNCSEL_R(crate::FieldReader<u8, PAD16FNCSEL_A>);
impl PAD16FNCSEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        PAD16FNCSEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAD16FNCSEL_A {
        match self.bits {
            0 => PAD16FNCSEL_A::ADCSE0,
            1 => PAD16FNCSEL_A::NCE16,
            2 => PAD16FNCSEL_A::TRIG0,
            3 => PAD16FNCSEL_A::GPIO16,
            4 => PAD16FNCSEL_A::SCCRST,
            5 => PAD16FNCSEL_A::CMPIN0,
            6 => PAD16FNCSEL_A::UART0TX,
            7 => PAD16FNCSEL_A::UA1RTS,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ADCSE0`"]
    #[inline(always)]
    pub fn is_adcse0(&self) -> bool {
        **self == PAD16FNCSEL_A::ADCSE0
    }
    #[doc = "Checks if the value of the field is `NCE16`"]
    #[inline(always)]
    pub fn is_nce16(&self) -> bool {
        **self == PAD16FNCSEL_A::NCE16
    }
    #[doc = "Checks if the value of the field is `TRIG0`"]
    #[inline(always)]
    pub fn is_trig0(&self) -> bool {
        **self == PAD16FNCSEL_A::TRIG0
    }
    #[doc = "Checks if the value of the field is `GPIO16`"]
    #[inline(always)]
    pub fn is_gpio16(&self) -> bool {
        **self == PAD16FNCSEL_A::GPIO16
    }
    #[doc = "Checks if the value of the field is `SCCRST`"]
    #[inline(always)]
    pub fn is_sccrst(&self) -> bool {
        **self == PAD16FNCSEL_A::SCCRST
    }
    #[doc = "Checks if the value of the field is `CMPIN0`"]
    #[inline(always)]
    pub fn is_cmpin0(&self) -> bool {
        **self == PAD16FNCSEL_A::CMPIN0
    }
    #[doc = "Checks if the value of the field is `UART0TX`"]
    #[inline(always)]
    pub fn is_uart0tx(&self) -> bool {
        **self == PAD16FNCSEL_A::UART0TX
    }
    #[doc = "Checks if the value of the field is `UA1RTS`"]
    #[inline(always)]
    pub fn is_ua1rts(&self) -> bool {
        **self == PAD16FNCSEL_A::UA1RTS
    }
}
impl core::ops::Deref for PAD16FNCSEL_R {
    type Target = crate::FieldReader<u8, PAD16FNCSEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PAD16FNCSEL` writer - Pad 16 function select"]
pub struct PAD16FNCSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD16FNCSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD16FNCSEL_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Configure as the analog ADC single ended port 0 input signal value."]
    #[inline(always)]
    pub fn adcse0(self) -> &'a mut W {
        self.variant(PAD16FNCSEL_A::ADCSE0)
    }
    #[doc = "IOM/MSPI nCE group 16 value."]
    #[inline(always)]
    pub fn nce16(self) -> &'a mut W {
        self.variant(PAD16FNCSEL_A::NCE16)
    }
    #[doc = "Configure as the ADC Trigger 0 signal value."]
    #[inline(always)]
    pub fn trig0(self) -> &'a mut W {
        self.variant(PAD16FNCSEL_A::TRIG0)
    }
    #[doc = "Configure as GPIO16 value."]
    #[inline(always)]
    pub fn gpio16(self) -> &'a mut W {
        self.variant(PAD16FNCSEL_A::GPIO16)
    }
    #[doc = "SCARD reset output value."]
    #[inline(always)]
    pub fn sccrst(self) -> &'a mut W {
        self.variant(PAD16FNCSEL_A::SCCRST)
    }
    #[doc = "Configure as comparator input 0 signal value."]
    #[inline(always)]
    pub fn cmpin0(self) -> &'a mut W {
        self.variant(PAD16FNCSEL_A::CMPIN0)
    }
    #[doc = "Configure as UART0 TX output signal value."]
    #[inline(always)]
    pub fn uart0tx(self) -> &'a mut W {
        self.variant(PAD16FNCSEL_A::UART0TX)
    }
    #[doc = "Configure as UART1 RTS output signal value."]
    #[inline(always)]
    pub fn ua1rts(self) -> &'a mut W {
        self.variant(PAD16FNCSEL_A::UA1RTS)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 3)) | ((value as u32 & 0x07) << 3);
        self.w
    }
}
#[doc = "Pad 16 drive strength\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD16STRNG_A {
    #[doc = "0: Low drive strength value."]
    LOW = 0,
    #[doc = "1: High drive strength value."]
    HIGH = 1,
}
impl From<PAD16STRNG_A> for bool {
    #[inline(always)]
    fn from(variant: PAD16STRNG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PAD16STRNG` reader - Pad 16 drive strength"]
pub struct PAD16STRNG_R(crate::FieldReader<bool, PAD16STRNG_A>);
impl PAD16STRNG_R {
    pub(crate) fn new(bits: bool) -> Self {
        PAD16STRNG_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAD16STRNG_A {
        match self.bits {
            false => PAD16STRNG_A::LOW,
            true => PAD16STRNG_A::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        **self == PAD16STRNG_A::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        **self == PAD16STRNG_A::HIGH
    }
}
impl core::ops::Deref for PAD16STRNG_R {
    type Target = crate::FieldReader<bool, PAD16STRNG_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PAD16STRNG` writer - Pad 16 drive strength"]
pub struct PAD16STRNG_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD16STRNG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD16STRNG_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Low drive strength value."]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(PAD16STRNG_A::LOW)
    }
    #[doc = "High drive strength value."]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(PAD16STRNG_A::HIGH)
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
#[doc = "Pad 16 input enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD16INPEN_A {
    #[doc = "0: Pad input disabled value."]
    DIS = 0,
    #[doc = "1: Pad input enabled value."]
    EN = 1,
}
impl From<PAD16INPEN_A> for bool {
    #[inline(always)]
    fn from(variant: PAD16INPEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PAD16INPEN` reader - Pad 16 input enable"]
pub struct PAD16INPEN_R(crate::FieldReader<bool, PAD16INPEN_A>);
impl PAD16INPEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        PAD16INPEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAD16INPEN_A {
        match self.bits {
            false => PAD16INPEN_A::DIS,
            true => PAD16INPEN_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        **self == PAD16INPEN_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        **self == PAD16INPEN_A::EN
    }
}
impl core::ops::Deref for PAD16INPEN_R {
    type Target = crate::FieldReader<bool, PAD16INPEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PAD16INPEN` writer - Pad 16 input enable"]
pub struct PAD16INPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD16INPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD16INPEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Pad input disabled value."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(PAD16INPEN_A::DIS)
    }
    #[doc = "Pad input enabled value."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(PAD16INPEN_A::EN)
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
#[doc = "Pad 16 pullup enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD16PULL_A {
    #[doc = "0: Pullup disabled value."]
    DIS = 0,
    #[doc = "1: Pullup enabled value."]
    EN = 1,
}
impl From<PAD16PULL_A> for bool {
    #[inline(always)]
    fn from(variant: PAD16PULL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PAD16PULL` reader - Pad 16 pullup enable"]
pub struct PAD16PULL_R(crate::FieldReader<bool, PAD16PULL_A>);
impl PAD16PULL_R {
    pub(crate) fn new(bits: bool) -> Self {
        PAD16PULL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAD16PULL_A {
        match self.bits {
            false => PAD16PULL_A::DIS,
            true => PAD16PULL_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        **self == PAD16PULL_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        **self == PAD16PULL_A::EN
    }
}
impl core::ops::Deref for PAD16PULL_R {
    type Target = crate::FieldReader<bool, PAD16PULL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PAD16PULL` writer - Pad 16 pullup enable"]
pub struct PAD16PULL_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD16PULL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD16PULL_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Pullup disabled value."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(PAD16PULL_A::DIS)
    }
    #[doc = "Pullup enabled value."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(PAD16PULL_A::EN)
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
    #[doc = "Bits 27:29 - Pad 19 function select"]
    #[inline(always)]
    pub fn pad19fncsel(&self) -> PAD19FNCSEL_R {
        PAD19FNCSEL_R::new(((self.bits >> 27) & 0x07) as u8)
    }
    #[doc = "Bit 26 - Pad 19 drive strength"]
    #[inline(always)]
    pub fn pad19strng(&self) -> PAD19STRNG_R {
        PAD19STRNG_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Pad 19 input enable"]
    #[inline(always)]
    pub fn pad19inpen(&self) -> PAD19INPEN_R {
        PAD19INPEN_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Pad 19 pullup enable"]
    #[inline(always)]
    pub fn pad19pull(&self) -> PAD19PULL_R {
        PAD19PULL_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bits 19:21 - Pad 18 function select"]
    #[inline(always)]
    pub fn pad18fncsel(&self) -> PAD18FNCSEL_R {
        PAD18FNCSEL_R::new(((self.bits >> 19) & 0x07) as u8)
    }
    #[doc = "Bit 18 - Pad 18 drive strength"]
    #[inline(always)]
    pub fn pad18strng(&self) -> PAD18STRNG_R {
        PAD18STRNG_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Pad 18 input enable"]
    #[inline(always)]
    pub fn pad18inpen(&self) -> PAD18INPEN_R {
        PAD18INPEN_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Pad 18 pullup enable"]
    #[inline(always)]
    pub fn pad18pull(&self) -> PAD18PULL_R {
        PAD18PULL_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bits 11:13 - Pad 17 function select"]
    #[inline(always)]
    pub fn pad17fncsel(&self) -> PAD17FNCSEL_R {
        PAD17FNCSEL_R::new(((self.bits >> 11) & 0x07) as u8)
    }
    #[doc = "Bit 10 - Pad 17 drive strength"]
    #[inline(always)]
    pub fn pad17strng(&self) -> PAD17STRNG_R {
        PAD17STRNG_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Pad 17 input enable"]
    #[inline(always)]
    pub fn pad17inpen(&self) -> PAD17INPEN_R {
        PAD17INPEN_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Pad 17 pullup enable"]
    #[inline(always)]
    pub fn pad17pull(&self) -> PAD17PULL_R {
        PAD17PULL_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bits 3:5 - Pad 16 function select"]
    #[inline(always)]
    pub fn pad16fncsel(&self) -> PAD16FNCSEL_R {
        PAD16FNCSEL_R::new(((self.bits >> 3) & 0x07) as u8)
    }
    #[doc = "Bit 2 - Pad 16 drive strength"]
    #[inline(always)]
    pub fn pad16strng(&self) -> PAD16STRNG_R {
        PAD16STRNG_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - Pad 16 input enable"]
    #[inline(always)]
    pub fn pad16inpen(&self) -> PAD16INPEN_R {
        PAD16INPEN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Pad 16 pullup enable"]
    #[inline(always)]
    pub fn pad16pull(&self) -> PAD16PULL_R {
        PAD16PULL_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 27:29 - Pad 19 function select"]
    #[inline(always)]
    pub fn pad19fncsel(&mut self) -> PAD19FNCSEL_W {
        PAD19FNCSEL_W { w: self }
    }
    #[doc = "Bit 26 - Pad 19 drive strength"]
    #[inline(always)]
    pub fn pad19strng(&mut self) -> PAD19STRNG_W {
        PAD19STRNG_W { w: self }
    }
    #[doc = "Bit 25 - Pad 19 input enable"]
    #[inline(always)]
    pub fn pad19inpen(&mut self) -> PAD19INPEN_W {
        PAD19INPEN_W { w: self }
    }
    #[doc = "Bit 24 - Pad 19 pullup enable"]
    #[inline(always)]
    pub fn pad19pull(&mut self) -> PAD19PULL_W {
        PAD19PULL_W { w: self }
    }
    #[doc = "Bits 19:21 - Pad 18 function select"]
    #[inline(always)]
    pub fn pad18fncsel(&mut self) -> PAD18FNCSEL_W {
        PAD18FNCSEL_W { w: self }
    }
    #[doc = "Bit 18 - Pad 18 drive strength"]
    #[inline(always)]
    pub fn pad18strng(&mut self) -> PAD18STRNG_W {
        PAD18STRNG_W { w: self }
    }
    #[doc = "Bit 17 - Pad 18 input enable"]
    #[inline(always)]
    pub fn pad18inpen(&mut self) -> PAD18INPEN_W {
        PAD18INPEN_W { w: self }
    }
    #[doc = "Bit 16 - Pad 18 pullup enable"]
    #[inline(always)]
    pub fn pad18pull(&mut self) -> PAD18PULL_W {
        PAD18PULL_W { w: self }
    }
    #[doc = "Bits 11:13 - Pad 17 function select"]
    #[inline(always)]
    pub fn pad17fncsel(&mut self) -> PAD17FNCSEL_W {
        PAD17FNCSEL_W { w: self }
    }
    #[doc = "Bit 10 - Pad 17 drive strength"]
    #[inline(always)]
    pub fn pad17strng(&mut self) -> PAD17STRNG_W {
        PAD17STRNG_W { w: self }
    }
    #[doc = "Bit 9 - Pad 17 input enable"]
    #[inline(always)]
    pub fn pad17inpen(&mut self) -> PAD17INPEN_W {
        PAD17INPEN_W { w: self }
    }
    #[doc = "Bit 8 - Pad 17 pullup enable"]
    #[inline(always)]
    pub fn pad17pull(&mut self) -> PAD17PULL_W {
        PAD17PULL_W { w: self }
    }
    #[doc = "Bits 3:5 - Pad 16 function select"]
    #[inline(always)]
    pub fn pad16fncsel(&mut self) -> PAD16FNCSEL_W {
        PAD16FNCSEL_W { w: self }
    }
    #[doc = "Bit 2 - Pad 16 drive strength"]
    #[inline(always)]
    pub fn pad16strng(&mut self) -> PAD16STRNG_W {
        PAD16STRNG_W { w: self }
    }
    #[doc = "Bit 1 - Pad 16 input enable"]
    #[inline(always)]
    pub fn pad16inpen(&mut self) -> PAD16INPEN_W {
        PAD16INPEN_W { w: self }
    }
    #[doc = "Bit 0 - Pad 16 pullup enable"]
    #[inline(always)]
    pub fn pad16pull(&mut self) -> PAD16PULL_W {
        PAD16PULL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Pad Configuration Register E (Pads 16-19)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [padrege](index.html) module"]
pub struct PADREGE_SPEC;
impl crate::RegisterSpec for PADREGE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [padrege::R](R) reader structure"]
impl crate::Readable for PADREGE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [padrege::W](W) writer structure"]
impl crate::Writable for PADREGE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PADREGE to value 0x1818_1818"]
impl crate::Resettable for PADREGE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x1818_1818
    }
}
