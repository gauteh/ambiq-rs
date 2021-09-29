#[doc = "Register `VCFG` reader"]
pub struct R(crate::R<VCFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<VCFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<VCFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<VCFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `VCFG` writer"]
pub struct W(crate::W<VCFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<VCFG_SPEC>;
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
impl From<crate::W<VCFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<VCFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Enable the IO clock.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IOCLKEN_A {
    #[doc = "0: Disable FIFO read. value."]
    DIS = 0,
    #[doc = "1: Enable FIFO read. value."]
    EN = 1,
}
impl From<IOCLKEN_A> for bool {
    #[inline(always)]
    fn from(variant: IOCLKEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IOCLKEN` reader - Enable the IO clock."]
pub struct IOCLKEN_R(crate::FieldReader<bool, IOCLKEN_A>);
impl IOCLKEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        IOCLKEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IOCLKEN_A {
        match self.bits {
            false => IOCLKEN_A::DIS,
            true => IOCLKEN_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        **self == IOCLKEN_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        **self == IOCLKEN_A::EN
    }
}
impl core::ops::Deref for IOCLKEN_R {
    type Target = crate::FieldReader<bool, IOCLKEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IOCLKEN` writer - Enable the IO clock."]
pub struct IOCLKEN_W<'a> {
    w: &'a mut W,
}
impl<'a> IOCLKEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IOCLKEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable FIFO read. value."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(IOCLKEN_A::DIS)
    }
    #[doc = "Enable FIFO read. value."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(IOCLKEN_A::EN)
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | ((value as u32 & 0x01) << 31);
        self.w
    }
}
#[doc = "Reset the IP core.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RSTB_A {
    #[doc = "0: Reset the core. value."]
    RESET = 0,
    #[doc = "1: Enable the core. value."]
    NORM = 1,
}
impl From<RSTB_A> for bool {
    #[inline(always)]
    fn from(variant: RSTB_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RSTB` reader - Reset the IP core."]
pub struct RSTB_R(crate::FieldReader<bool, RSTB_A>);
impl RSTB_R {
    pub(crate) fn new(bits: bool) -> Self {
        RSTB_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RSTB_A {
        match self.bits {
            false => RSTB_A::RESET,
            true => RSTB_A::NORM,
        }
    }
    #[doc = "Checks if the value of the field is `RESET`"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        **self == RSTB_A::RESET
    }
    #[doc = "Checks if the value of the field is `NORM`"]
    #[inline(always)]
    pub fn is_norm(&self) -> bool {
        **self == RSTB_A::NORM
    }
}
impl core::ops::Deref for RSTB_R {
    type Target = crate::FieldReader<bool, RSTB_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RSTB` writer - Reset the IP core."]
pub struct RSTB_W<'a> {
    w: &'a mut W,
}
impl<'a> RSTB_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RSTB_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Reset the core. value."]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(RSTB_A::RESET)
    }
    #[doc = "Enable the core. value."]
    #[inline(always)]
    pub fn norm(self) -> &'a mut W {
        self.variant(RSTB_A::NORM)
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
#[doc = "Select the PDM input clock.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PDMCLKSEL_A {
    #[doc = "0: Static value. value."]
    DISABLE = 0,
    #[doc = "1: PDM clock is 12 MHz. value."]
    _12MHZ = 1,
    #[doc = "2: PDM clock is 6 MHz. value."]
    _6MHZ = 2,
    #[doc = "3: PDM clock is 3 MHz. value."]
    _3MHZ = 3,
    #[doc = "4: PDM clock is 1.5 MHz. value."]
    _1_5MHZ = 4,
    #[doc = "5: PDM clock is 750 KHz. value."]
    _750KHZ = 5,
    #[doc = "6: PDM clock is 375 KHz. value."]
    _375KHZ = 6,
    #[doc = "7: PDM clock is 187.5 KHz. value."]
    _187KHZ = 7,
}
impl From<PDMCLKSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: PDMCLKSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `PDMCLKSEL` reader - Select the PDM input clock."]
pub struct PDMCLKSEL_R(crate::FieldReader<u8, PDMCLKSEL_A>);
impl PDMCLKSEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        PDMCLKSEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PDMCLKSEL_A {
        match self.bits {
            0 => PDMCLKSEL_A::DISABLE,
            1 => PDMCLKSEL_A::_12MHZ,
            2 => PDMCLKSEL_A::_6MHZ,
            3 => PDMCLKSEL_A::_3MHZ,
            4 => PDMCLKSEL_A::_1_5MHZ,
            5 => PDMCLKSEL_A::_750KHZ,
            6 => PDMCLKSEL_A::_375KHZ,
            7 => PDMCLKSEL_A::_187KHZ,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        **self == PDMCLKSEL_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `_12MHZ`"]
    #[inline(always)]
    pub fn is_12mhz(&self) -> bool {
        **self == PDMCLKSEL_A::_12MHZ
    }
    #[doc = "Checks if the value of the field is `_6MHZ`"]
    #[inline(always)]
    pub fn is_6mhz(&self) -> bool {
        **self == PDMCLKSEL_A::_6MHZ
    }
    #[doc = "Checks if the value of the field is `_3MHZ`"]
    #[inline(always)]
    pub fn is_3mhz(&self) -> bool {
        **self == PDMCLKSEL_A::_3MHZ
    }
    #[doc = "Checks if the value of the field is `_1_5MHZ`"]
    #[inline(always)]
    pub fn is_1_5mhz(&self) -> bool {
        **self == PDMCLKSEL_A::_1_5MHZ
    }
    #[doc = "Checks if the value of the field is `_750KHZ`"]
    #[inline(always)]
    pub fn is_750khz(&self) -> bool {
        **self == PDMCLKSEL_A::_750KHZ
    }
    #[doc = "Checks if the value of the field is `_375KHZ`"]
    #[inline(always)]
    pub fn is_375khz(&self) -> bool {
        **self == PDMCLKSEL_A::_375KHZ
    }
    #[doc = "Checks if the value of the field is `_187KHZ`"]
    #[inline(always)]
    pub fn is_187khz(&self) -> bool {
        **self == PDMCLKSEL_A::_187KHZ
    }
}
impl core::ops::Deref for PDMCLKSEL_R {
    type Target = crate::FieldReader<u8, PDMCLKSEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PDMCLKSEL` writer - Select the PDM input clock."]
pub struct PDMCLKSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> PDMCLKSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PDMCLKSEL_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Static value. value."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(PDMCLKSEL_A::DISABLE)
    }
    #[doc = "PDM clock is 12 MHz. value."]
    #[inline(always)]
    pub fn _12mhz(self) -> &'a mut W {
        self.variant(PDMCLKSEL_A::_12MHZ)
    }
    #[doc = "PDM clock is 6 MHz. value."]
    #[inline(always)]
    pub fn _6mhz(self) -> &'a mut W {
        self.variant(PDMCLKSEL_A::_6MHZ)
    }
    #[doc = "PDM clock is 3 MHz. value."]
    #[inline(always)]
    pub fn _3mhz(self) -> &'a mut W {
        self.variant(PDMCLKSEL_A::_3MHZ)
    }
    #[doc = "PDM clock is 1.5 MHz. value."]
    #[inline(always)]
    pub fn _1_5mhz(self) -> &'a mut W {
        self.variant(PDMCLKSEL_A::_1_5MHZ)
    }
    #[doc = "PDM clock is 750 KHz. value."]
    #[inline(always)]
    pub fn _750khz(self) -> &'a mut W {
        self.variant(PDMCLKSEL_A::_750KHZ)
    }
    #[doc = "PDM clock is 375 KHz. value."]
    #[inline(always)]
    pub fn _375khz(self) -> &'a mut W {
        self.variant(PDMCLKSEL_A::_375KHZ)
    }
    #[doc = "PDM clock is 187.5 KHz. value."]
    #[inline(always)]
    pub fn _187khz(self) -> &'a mut W {
        self.variant(PDMCLKSEL_A::_187KHZ)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 27)) | ((value as u32 & 0x07) << 27);
        self.w
    }
}
#[doc = "Enable the serial clock.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PDMCLKEN_A {
    #[doc = "0: Disable serial clock. value."]
    DIS = 0,
    #[doc = "1: Enable serial clock. value."]
    EN = 1,
}
impl From<PDMCLKEN_A> for bool {
    #[inline(always)]
    fn from(variant: PDMCLKEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PDMCLKEN` reader - Enable the serial clock."]
pub struct PDMCLKEN_R(crate::FieldReader<bool, PDMCLKEN_A>);
impl PDMCLKEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        PDMCLKEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PDMCLKEN_A {
        match self.bits {
            false => PDMCLKEN_A::DIS,
            true => PDMCLKEN_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        **self == PDMCLKEN_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        **self == PDMCLKEN_A::EN
    }
}
impl core::ops::Deref for PDMCLKEN_R {
    type Target = crate::FieldReader<bool, PDMCLKEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PDMCLKEN` writer - Enable the serial clock."]
pub struct PDMCLKEN_W<'a> {
    w: &'a mut W,
}
impl<'a> PDMCLKEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PDMCLKEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable serial clock. value."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(PDMCLKEN_A::DIS)
    }
    #[doc = "Enable serial clock. value."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(PDMCLKEN_A::EN)
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
#[doc = "I2S interface enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum I2SEN_A {
    #[doc = "0: Disable I2S interface. value."]
    DIS = 0,
    #[doc = "1: Enable I2S interface. value."]
    EN = 1,
}
impl From<I2SEN_A> for bool {
    #[inline(always)]
    fn from(variant: I2SEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `I2SEN` reader - I2S interface enable."]
pub struct I2SEN_R(crate::FieldReader<bool, I2SEN_A>);
impl I2SEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        I2SEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> I2SEN_A {
        match self.bits {
            false => I2SEN_A::DIS,
            true => I2SEN_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        **self == I2SEN_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        **self == I2SEN_A::EN
    }
}
impl core::ops::Deref for I2SEN_R {
    type Target = crate::FieldReader<bool, I2SEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `I2SEN` writer - I2S interface enable."]
pub struct I2SEN_W<'a> {
    w: &'a mut W,
}
impl<'a> I2SEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: I2SEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable I2S interface. value."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(I2SEN_A::DIS)
    }
    #[doc = "Enable I2S interface. value."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(I2SEN_A::EN)
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | ((value as u32 & 0x01) << 20);
        self.w
    }
}
#[doc = "I2S BCLK input inversion.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BCLKINV_A {
    #[doc = "0: BCLK inverted. value."]
    INV = 0,
    #[doc = "1: BCLK not inverted. value."]
    NORM = 1,
}
impl From<BCLKINV_A> for bool {
    #[inline(always)]
    fn from(variant: BCLKINV_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BCLKINV` reader - I2S BCLK input inversion."]
pub struct BCLKINV_R(crate::FieldReader<bool, BCLKINV_A>);
impl BCLKINV_R {
    pub(crate) fn new(bits: bool) -> Self {
        BCLKINV_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BCLKINV_A {
        match self.bits {
            false => BCLKINV_A::INV,
            true => BCLKINV_A::NORM,
        }
    }
    #[doc = "Checks if the value of the field is `INV`"]
    #[inline(always)]
    pub fn is_inv(&self) -> bool {
        **self == BCLKINV_A::INV
    }
    #[doc = "Checks if the value of the field is `NORM`"]
    #[inline(always)]
    pub fn is_norm(&self) -> bool {
        **self == BCLKINV_A::NORM
    }
}
impl core::ops::Deref for BCLKINV_R {
    type Target = crate::FieldReader<bool, BCLKINV_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BCLKINV` writer - I2S BCLK input inversion."]
pub struct BCLKINV_W<'a> {
    w: &'a mut W,
}
impl<'a> BCLKINV_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BCLKINV_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "BCLK inverted. value."]
    #[inline(always)]
    pub fn inv(self) -> &'a mut W {
        self.variant(BCLKINV_A::INV)
    }
    #[doc = "BCLK not inverted. value."]
    #[inline(always)]
    pub fn norm(self) -> &'a mut W {
        self.variant(BCLKINV_A::NORM)
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | ((value as u32 & 0x01) << 19);
        self.w
    }
}
#[doc = "PDM clock sampling delay.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMICKDEL_A {
    #[doc = "0: No delay. value."]
    _0CYC = 0,
    #[doc = "1: 1 cycle delay. value."]
    _1CYC = 1,
}
impl From<DMICKDEL_A> for bool {
    #[inline(always)]
    fn from(variant: DMICKDEL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMICKDEL` reader - PDM clock sampling delay."]
pub struct DMICKDEL_R(crate::FieldReader<bool, DMICKDEL_A>);
impl DMICKDEL_R {
    pub(crate) fn new(bits: bool) -> Self {
        DMICKDEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMICKDEL_A {
        match self.bits {
            false => DMICKDEL_A::_0CYC,
            true => DMICKDEL_A::_1CYC,
        }
    }
    #[doc = "Checks if the value of the field is `_0CYC`"]
    #[inline(always)]
    pub fn is_0cyc(&self) -> bool {
        **self == DMICKDEL_A::_0CYC
    }
    #[doc = "Checks if the value of the field is `_1CYC`"]
    #[inline(always)]
    pub fn is_1cyc(&self) -> bool {
        **self == DMICKDEL_A::_1CYC
    }
}
impl core::ops::Deref for DMICKDEL_R {
    type Target = crate::FieldReader<bool, DMICKDEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DMICKDEL` writer - PDM clock sampling delay."]
pub struct DMICKDEL_W<'a> {
    w: &'a mut W,
}
impl<'a> DMICKDEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DMICKDEL_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No delay. value."]
    #[inline(always)]
    pub fn _0cyc(self) -> &'a mut W {
        self.variant(DMICKDEL_A::_0CYC)
    }
    #[doc = "1 cycle delay. value."]
    #[inline(always)]
    pub fn _1cyc(self) -> &'a mut W {
        self.variant(DMICKDEL_A::_1CYC)
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
#[doc = "Select PDM input clock source.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SELAP_A {
    #[doc = "1: Clock source from I2S BCLK. value."]
    I2S = 1,
    #[doc = "0: Clock source from internal clock generator. value."]
    INTERNAL = 0,
}
impl From<SELAP_A> for bool {
    #[inline(always)]
    fn from(variant: SELAP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SELAP` reader - Select PDM input clock source."]
pub struct SELAP_R(crate::FieldReader<bool, SELAP_A>);
impl SELAP_R {
    pub(crate) fn new(bits: bool) -> Self {
        SELAP_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SELAP_A {
        match self.bits {
            true => SELAP_A::I2S,
            false => SELAP_A::INTERNAL,
        }
    }
    #[doc = "Checks if the value of the field is `I2S`"]
    #[inline(always)]
    pub fn is_i2s(&self) -> bool {
        **self == SELAP_A::I2S
    }
    #[doc = "Checks if the value of the field is `INTERNAL`"]
    #[inline(always)]
    pub fn is_internal(&self) -> bool {
        **self == SELAP_A::INTERNAL
    }
}
impl core::ops::Deref for SELAP_R {
    type Target = crate::FieldReader<bool, SELAP_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SELAP` writer - Select PDM input clock source."]
pub struct SELAP_W<'a> {
    w: &'a mut W,
}
impl<'a> SELAP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SELAP_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clock source from I2S BCLK. value."]
    #[inline(always)]
    pub fn i2s(self) -> &'a mut W {
        self.variant(SELAP_A::I2S)
    }
    #[doc = "Clock source from internal clock generator. value."]
    #[inline(always)]
    pub fn internal(self) -> &'a mut W {
        self.variant(SELAP_A::INTERNAL)
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
#[doc = "PCM data packing enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PCMPACK_A {
    #[doc = "0: Disable PCM packing. value."]
    DIS = 0,
    #[doc = "1: Enable PCM packing. value."]
    EN = 1,
}
impl From<PCMPACK_A> for bool {
    #[inline(always)]
    fn from(variant: PCMPACK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PCMPACK` reader - PCM data packing enable."]
pub struct PCMPACK_R(crate::FieldReader<bool, PCMPACK_A>);
impl PCMPACK_R {
    pub(crate) fn new(bits: bool) -> Self {
        PCMPACK_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PCMPACK_A {
        match self.bits {
            false => PCMPACK_A::DIS,
            true => PCMPACK_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        **self == PCMPACK_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        **self == PCMPACK_A::EN
    }
}
impl core::ops::Deref for PCMPACK_R {
    type Target = crate::FieldReader<bool, PCMPACK_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PCMPACK` writer - PCM data packing enable."]
pub struct PCMPACK_W<'a> {
    w: &'a mut W,
}
impl<'a> PCMPACK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PCMPACK_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable PCM packing. value."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(PCMPACK_A::DIS)
    }
    #[doc = "Enable PCM packing. value."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(PCMPACK_A::EN)
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
#[doc = "Set PCM channels.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CHSET_A {
    #[doc = "0: Channel disabled. value."]
    DIS = 0,
    #[doc = "1: Mono left channel. value."]
    LEFT = 1,
    #[doc = "2: Mono right channel. value."]
    RIGHT = 2,
    #[doc = "3: Stereo channels. value."]
    STEREO = 3,
}
impl From<CHSET_A> for u8 {
    #[inline(always)]
    fn from(variant: CHSET_A) -> Self {
        variant as _
    }
}
#[doc = "Field `CHSET` reader - Set PCM channels."]
pub struct CHSET_R(crate::FieldReader<u8, CHSET_A>);
impl CHSET_R {
    pub(crate) fn new(bits: u8) -> Self {
        CHSET_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CHSET_A {
        match self.bits {
            0 => CHSET_A::DIS,
            1 => CHSET_A::LEFT,
            2 => CHSET_A::RIGHT,
            3 => CHSET_A::STEREO,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        **self == CHSET_A::DIS
    }
    #[doc = "Checks if the value of the field is `LEFT`"]
    #[inline(always)]
    pub fn is_left(&self) -> bool {
        **self == CHSET_A::LEFT
    }
    #[doc = "Checks if the value of the field is `RIGHT`"]
    #[inline(always)]
    pub fn is_right(&self) -> bool {
        **self == CHSET_A::RIGHT
    }
    #[doc = "Checks if the value of the field is `STEREO`"]
    #[inline(always)]
    pub fn is_stereo(&self) -> bool {
        **self == CHSET_A::STEREO
    }
}
impl core::ops::Deref for CHSET_R {
    type Target = crate::FieldReader<u8, CHSET_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CHSET` writer - Set PCM channels."]
pub struct CHSET_W<'a> {
    w: &'a mut W,
}
impl<'a> CHSET_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CHSET_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Channel disabled. value."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(CHSET_A::DIS)
    }
    #[doc = "Mono left channel. value."]
    #[inline(always)]
    pub fn left(self) -> &'a mut W {
        self.variant(CHSET_A::LEFT)
    }
    #[doc = "Mono right channel. value."]
    #[inline(always)]
    pub fn right(self) -> &'a mut W {
        self.variant(CHSET_A::RIGHT)
    }
    #[doc = "Stereo channels. value."]
    #[inline(always)]
    pub fn stereo(self) -> &'a mut W {
        self.variant(CHSET_A::STEREO)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 3)) | ((value as u32 & 0x03) << 3);
        self.w
    }
}
impl R {
    #[doc = "Bit 31 - Enable the IO clock."]
    #[inline(always)]
    pub fn ioclken(&self) -> IOCLKEN_R {
        IOCLKEN_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bit 30 - Reset the IP core."]
    #[inline(always)]
    pub fn rstb(&self) -> RSTB_R {
        RSTB_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bits 27:29 - Select the PDM input clock."]
    #[inline(always)]
    pub fn pdmclksel(&self) -> PDMCLKSEL_R {
        PDMCLKSEL_R::new(((self.bits >> 27) & 0x07) as u8)
    }
    #[doc = "Bit 26 - Enable the serial clock."]
    #[inline(always)]
    pub fn pdmclken(&self) -> PDMCLKEN_R {
        PDMCLKEN_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 20 - I2S interface enable."]
    #[inline(always)]
    pub fn i2sen(&self) -> I2SEN_R {
        I2SEN_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 19 - I2S BCLK input inversion."]
    #[inline(always)]
    pub fn bclkinv(&self) -> BCLKINV_R {
        BCLKINV_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 17 - PDM clock sampling delay."]
    #[inline(always)]
    pub fn dmickdel(&self) -> DMICKDEL_R {
        DMICKDEL_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Select PDM input clock source."]
    #[inline(always)]
    pub fn selap(&self) -> SELAP_R {
        SELAP_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 8 - PCM data packing enable."]
    #[inline(always)]
    pub fn pcmpack(&self) -> PCMPACK_R {
        PCMPACK_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bits 3:4 - Set PCM channels."]
    #[inline(always)]
    pub fn chset(&self) -> CHSET_R {
        CHSET_R::new(((self.bits >> 3) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bit 31 - Enable the IO clock."]
    #[inline(always)]
    pub fn ioclken(&mut self) -> IOCLKEN_W {
        IOCLKEN_W { w: self }
    }
    #[doc = "Bit 30 - Reset the IP core."]
    #[inline(always)]
    pub fn rstb(&mut self) -> RSTB_W {
        RSTB_W { w: self }
    }
    #[doc = "Bits 27:29 - Select the PDM input clock."]
    #[inline(always)]
    pub fn pdmclksel(&mut self) -> PDMCLKSEL_W {
        PDMCLKSEL_W { w: self }
    }
    #[doc = "Bit 26 - Enable the serial clock."]
    #[inline(always)]
    pub fn pdmclken(&mut self) -> PDMCLKEN_W {
        PDMCLKEN_W { w: self }
    }
    #[doc = "Bit 20 - I2S interface enable."]
    #[inline(always)]
    pub fn i2sen(&mut self) -> I2SEN_W {
        I2SEN_W { w: self }
    }
    #[doc = "Bit 19 - I2S BCLK input inversion."]
    #[inline(always)]
    pub fn bclkinv(&mut self) -> BCLKINV_W {
        BCLKINV_W { w: self }
    }
    #[doc = "Bit 17 - PDM clock sampling delay."]
    #[inline(always)]
    pub fn dmickdel(&mut self) -> DMICKDEL_W {
        DMICKDEL_W { w: self }
    }
    #[doc = "Bit 16 - Select PDM input clock source."]
    #[inline(always)]
    pub fn selap(&mut self) -> SELAP_W {
        SELAP_W { w: self }
    }
    #[doc = "Bit 8 - PCM data packing enable."]
    #[inline(always)]
    pub fn pcmpack(&mut self) -> PCMPACK_W {
        PCMPACK_W { w: self }
    }
    #[doc = "Bits 3:4 - Set PCM channels."]
    #[inline(always)]
    pub fn chset(&mut self) -> CHSET_W {
        CHSET_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Voice Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [vcfg](index.html) module"]
pub struct VCFG_SPEC;
impl crate::RegisterSpec for VCFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [vcfg::R](R) reader structure"]
impl crate::Readable for VCFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [vcfg::W](W) writer structure"]
impl crate::Writable for VCFG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets VCFG to value 0x08"]
impl crate::Resettable for VCFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x08
    }
}
