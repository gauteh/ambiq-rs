#[doc = "Register `CFGA` reader"]
pub struct R(crate::R<CFGA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CFGA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CFGA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CFGA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CFGA` writer"]
pub struct W(crate::W<CFGA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CFGA_SPEC>;
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
impl From<crate::W<CFGA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CFGA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "GPIO7 interrupt direction, nCE polarity.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO7INTD_A {
    #[doc = "0: FNCSEL = 0x0 - nCE polarity active low value."]
    NCELOW = 0,
    #[doc = "1: FNCSEL = 0x0 - nCE polarity active high value."]
    NCEHIGH = 1,
}
impl From<GPIO7INTD_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO7INTD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIO7INTD` reader - GPIO7 interrupt direction, nCE polarity."]
pub struct GPIO7INTD_R(crate::FieldReader<bool, GPIO7INTD_A>);
impl GPIO7INTD_R {
    pub(crate) fn new(bits: bool) -> Self {
        GPIO7INTD_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO7INTD_A {
        match self.bits {
            false => GPIO7INTD_A::NCELOW,
            true => GPIO7INTD_A::NCEHIGH,
        }
    }
    #[doc = "Checks if the value of the field is `NCELOW`"]
    #[inline(always)]
    pub fn is_n_celow(&self) -> bool {
        **self == GPIO7INTD_A::NCELOW
    }
    #[doc = "Checks if the value of the field is `NCEHIGH`"]
    #[inline(always)]
    pub fn is_n_cehigh(&self) -> bool {
        **self == GPIO7INTD_A::NCEHIGH
    }
}
impl core::ops::Deref for GPIO7INTD_R {
    type Target = crate::FieldReader<bool, GPIO7INTD_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIO7INTD` writer - GPIO7 interrupt direction, nCE polarity."]
pub struct GPIO7INTD_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO7INTD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIO7INTD_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "FNCSEL = 0x0 - nCE polarity active low value."]
    #[inline(always)]
    pub fn n_celow(self) -> &'a mut W {
        self.variant(GPIO7INTD_A::NCELOW)
    }
    #[doc = "FNCSEL = 0x0 - nCE polarity active high value."]
    #[inline(always)]
    pub fn n_cehigh(self) -> &'a mut W {
        self.variant(GPIO7INTD_A::NCEHIGH)
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
#[doc = "GPIO7 output configuration.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum GPIO7OUTCFG_A {
    #[doc = "0: FNCSEL = 0x3 - Output disabled value."]
    DIS = 0,
    #[doc = "1: FNCSEL = 0x3 - Output is push-pull value."]
    PUSHPULL = 1,
    #[doc = "2: FNCSEL = 0x3 - Output is open drain value."]
    OD = 2,
    #[doc = "3: FNCSEL = 0x3 - Output is tri-state value."]
    TS = 3,
}
impl From<GPIO7OUTCFG_A> for u8 {
    #[inline(always)]
    fn from(variant: GPIO7OUTCFG_A) -> Self {
        variant as _
    }
}
#[doc = "Field `GPIO7OUTCFG` reader - GPIO7 output configuration."]
pub struct GPIO7OUTCFG_R(crate::FieldReader<u8, GPIO7OUTCFG_A>);
impl GPIO7OUTCFG_R {
    pub(crate) fn new(bits: u8) -> Self {
        GPIO7OUTCFG_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO7OUTCFG_A {
        match self.bits {
            0 => GPIO7OUTCFG_A::DIS,
            1 => GPIO7OUTCFG_A::PUSHPULL,
            2 => GPIO7OUTCFG_A::OD,
            3 => GPIO7OUTCFG_A::TS,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        **self == GPIO7OUTCFG_A::DIS
    }
    #[doc = "Checks if the value of the field is `PUSHPULL`"]
    #[inline(always)]
    pub fn is_pushpull(&self) -> bool {
        **self == GPIO7OUTCFG_A::PUSHPULL
    }
    #[doc = "Checks if the value of the field is `OD`"]
    #[inline(always)]
    pub fn is_od(&self) -> bool {
        **self == GPIO7OUTCFG_A::OD
    }
    #[doc = "Checks if the value of the field is `TS`"]
    #[inline(always)]
    pub fn is_ts(&self) -> bool {
        **self == GPIO7OUTCFG_A::TS
    }
}
impl core::ops::Deref for GPIO7OUTCFG_R {
    type Target = crate::FieldReader<u8, GPIO7OUTCFG_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIO7OUTCFG` writer - GPIO7 output configuration."]
pub struct GPIO7OUTCFG_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO7OUTCFG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIO7OUTCFG_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "FNCSEL = 0x3 - Output disabled value."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(GPIO7OUTCFG_A::DIS)
    }
    #[doc = "FNCSEL = 0x3 - Output is push-pull value."]
    #[inline(always)]
    pub fn pushpull(self) -> &'a mut W {
        self.variant(GPIO7OUTCFG_A::PUSHPULL)
    }
    #[doc = "FNCSEL = 0x3 - Output is open drain value."]
    #[inline(always)]
    pub fn od(self) -> &'a mut W {
        self.variant(GPIO7OUTCFG_A::OD)
    }
    #[doc = "FNCSEL = 0x3 - Output is tri-state value."]
    #[inline(always)]
    pub fn ts(self) -> &'a mut W {
        self.variant(GPIO7OUTCFG_A::TS)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 29)) | ((value as u32 & 0x03) << 29);
        self.w
    }
}
#[doc = "GPIO7 input enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO7INCFG_A {
    #[doc = "0: Read the GPIO pin data value."]
    READ = 0,
    #[doc = "1: INTD = 0 - Readback will always be zero value."]
    RDZERO = 1,
}
impl From<GPIO7INCFG_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO7INCFG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIO7INCFG` reader - GPIO7 input enable."]
pub struct GPIO7INCFG_R(crate::FieldReader<bool, GPIO7INCFG_A>);
impl GPIO7INCFG_R {
    pub(crate) fn new(bits: bool) -> Self {
        GPIO7INCFG_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO7INCFG_A {
        match self.bits {
            false => GPIO7INCFG_A::READ,
            true => GPIO7INCFG_A::RDZERO,
        }
    }
    #[doc = "Checks if the value of the field is `READ`"]
    #[inline(always)]
    pub fn is_read(&self) -> bool {
        **self == GPIO7INCFG_A::READ
    }
    #[doc = "Checks if the value of the field is `RDZERO`"]
    #[inline(always)]
    pub fn is_rdzero(&self) -> bool {
        **self == GPIO7INCFG_A::RDZERO
    }
}
impl core::ops::Deref for GPIO7INCFG_R {
    type Target = crate::FieldReader<bool, GPIO7INCFG_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIO7INCFG` writer - GPIO7 input enable."]
pub struct GPIO7INCFG_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO7INCFG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIO7INCFG_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Read the GPIO pin data value."]
    #[inline(always)]
    pub fn read(self) -> &'a mut W {
        self.variant(GPIO7INCFG_A::READ)
    }
    #[doc = "INTD = 0 - Readback will always be zero value."]
    #[inline(always)]
    pub fn rdzero(self) -> &'a mut W {
        self.variant(GPIO7INCFG_A::RDZERO)
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
        self.w.bits = (self.w.bits & !(0x01 << 28)) | ((value as u32 & 0x01) << 28);
        self.w
    }
}
#[doc = "GPIO6 interrupt direction.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO6INTD_A {
    #[doc = "0: INCFG = 1 - No interrupt on GPIO transition value."]
    INTDIS = 0,
    #[doc = "1: INCFG = 1 - Interrupt on either low to high or high to low GPIO transition value."]
    INTBOTH = 1,
}
impl From<GPIO6INTD_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO6INTD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIO6INTD` reader - GPIO6 interrupt direction."]
pub struct GPIO6INTD_R(crate::FieldReader<bool, GPIO6INTD_A>);
impl GPIO6INTD_R {
    pub(crate) fn new(bits: bool) -> Self {
        GPIO6INTD_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO6INTD_A {
        match self.bits {
            false => GPIO6INTD_A::INTDIS,
            true => GPIO6INTD_A::INTBOTH,
        }
    }
    #[doc = "Checks if the value of the field is `INTDIS`"]
    #[inline(always)]
    pub fn is_intdis(&self) -> bool {
        **self == GPIO6INTD_A::INTDIS
    }
    #[doc = "Checks if the value of the field is `INTBOTH`"]
    #[inline(always)]
    pub fn is_intboth(&self) -> bool {
        **self == GPIO6INTD_A::INTBOTH
    }
}
impl core::ops::Deref for GPIO6INTD_R {
    type Target = crate::FieldReader<bool, GPIO6INTD_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIO6INTD` writer - GPIO6 interrupt direction."]
pub struct GPIO6INTD_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO6INTD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIO6INTD_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "INCFG = 1 - No interrupt on GPIO transition value."]
    #[inline(always)]
    pub fn intdis(self) -> &'a mut W {
        self.variant(GPIO6INTD_A::INTDIS)
    }
    #[doc = "INCFG = 1 - Interrupt on either low to high or high to low GPIO transition value."]
    #[inline(always)]
    pub fn intboth(self) -> &'a mut W {
        self.variant(GPIO6INTD_A::INTBOTH)
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
        self.w.bits = (self.w.bits & !(0x01 << 27)) | ((value as u32 & 0x01) << 27);
        self.w
    }
}
#[doc = "GPIO6 output configuration.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum GPIO6OUTCFG_A {
    #[doc = "0: FNCSEL = 0x3 - Output disabled value."]
    DIS = 0,
    #[doc = "1: FNCSEL = 0x3 - Output is push-pull value."]
    PUSHPULL = 1,
    #[doc = "2: FNCSEL = 0x3 - Output is open drain value."]
    OD = 2,
    #[doc = "3: FNCSEL = 0x3 - Output is tri-state value."]
    TS = 3,
}
impl From<GPIO6OUTCFG_A> for u8 {
    #[inline(always)]
    fn from(variant: GPIO6OUTCFG_A) -> Self {
        variant as _
    }
}
#[doc = "Field `GPIO6OUTCFG` reader - GPIO6 output configuration."]
pub struct GPIO6OUTCFG_R(crate::FieldReader<u8, GPIO6OUTCFG_A>);
impl GPIO6OUTCFG_R {
    pub(crate) fn new(bits: u8) -> Self {
        GPIO6OUTCFG_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO6OUTCFG_A {
        match self.bits {
            0 => GPIO6OUTCFG_A::DIS,
            1 => GPIO6OUTCFG_A::PUSHPULL,
            2 => GPIO6OUTCFG_A::OD,
            3 => GPIO6OUTCFG_A::TS,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        **self == GPIO6OUTCFG_A::DIS
    }
    #[doc = "Checks if the value of the field is `PUSHPULL`"]
    #[inline(always)]
    pub fn is_pushpull(&self) -> bool {
        **self == GPIO6OUTCFG_A::PUSHPULL
    }
    #[doc = "Checks if the value of the field is `OD`"]
    #[inline(always)]
    pub fn is_od(&self) -> bool {
        **self == GPIO6OUTCFG_A::OD
    }
    #[doc = "Checks if the value of the field is `TS`"]
    #[inline(always)]
    pub fn is_ts(&self) -> bool {
        **self == GPIO6OUTCFG_A::TS
    }
}
impl core::ops::Deref for GPIO6OUTCFG_R {
    type Target = crate::FieldReader<u8, GPIO6OUTCFG_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIO6OUTCFG` writer - GPIO6 output configuration."]
pub struct GPIO6OUTCFG_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO6OUTCFG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIO6OUTCFG_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "FNCSEL = 0x3 - Output disabled value."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(GPIO6OUTCFG_A::DIS)
    }
    #[doc = "FNCSEL = 0x3 - Output is push-pull value."]
    #[inline(always)]
    pub fn pushpull(self) -> &'a mut W {
        self.variant(GPIO6OUTCFG_A::PUSHPULL)
    }
    #[doc = "FNCSEL = 0x3 - Output is open drain value."]
    #[inline(always)]
    pub fn od(self) -> &'a mut W {
        self.variant(GPIO6OUTCFG_A::OD)
    }
    #[doc = "FNCSEL = 0x3 - Output is tri-state value."]
    #[inline(always)]
    pub fn ts(self) -> &'a mut W {
        self.variant(GPIO6OUTCFG_A::TS)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 25)) | ((value as u32 & 0x03) << 25);
        self.w
    }
}
#[doc = "GPIO6 input enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO6INCFG_A {
    #[doc = "0: Read the GPIO pin data value."]
    READ = 0,
    #[doc = "1: INTD = 0 - Readback will always be zero value."]
    RDZERO = 1,
}
impl From<GPIO6INCFG_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO6INCFG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIO6INCFG` reader - GPIO6 input enable."]
pub struct GPIO6INCFG_R(crate::FieldReader<bool, GPIO6INCFG_A>);
impl GPIO6INCFG_R {
    pub(crate) fn new(bits: bool) -> Self {
        GPIO6INCFG_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO6INCFG_A {
        match self.bits {
            false => GPIO6INCFG_A::READ,
            true => GPIO6INCFG_A::RDZERO,
        }
    }
    #[doc = "Checks if the value of the field is `READ`"]
    #[inline(always)]
    pub fn is_read(&self) -> bool {
        **self == GPIO6INCFG_A::READ
    }
    #[doc = "Checks if the value of the field is `RDZERO`"]
    #[inline(always)]
    pub fn is_rdzero(&self) -> bool {
        **self == GPIO6INCFG_A::RDZERO
    }
}
impl core::ops::Deref for GPIO6INCFG_R {
    type Target = crate::FieldReader<bool, GPIO6INCFG_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIO6INCFG` writer - GPIO6 input enable."]
pub struct GPIO6INCFG_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO6INCFG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIO6INCFG_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Read the GPIO pin data value."]
    #[inline(always)]
    pub fn read(self) -> &'a mut W {
        self.variant(GPIO6INCFG_A::READ)
    }
    #[doc = "INTD = 0 - Readback will always be zero value."]
    #[inline(always)]
    pub fn rdzero(self) -> &'a mut W {
        self.variant(GPIO6INCFG_A::RDZERO)
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
#[doc = "GPIO5 interrupt direction.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO5INTD_A {
    #[doc = "0: INCFG = 1 - No interrupt on GPIO transition value."]
    INTDIS = 0,
    #[doc = "1: INCFG = 1 - Interrupt on either low to high or high to low GPIO transition value."]
    INTBOTH = 1,
}
impl From<GPIO5INTD_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO5INTD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIO5INTD` reader - GPIO5 interrupt direction."]
pub struct GPIO5INTD_R(crate::FieldReader<bool, GPIO5INTD_A>);
impl GPIO5INTD_R {
    pub(crate) fn new(bits: bool) -> Self {
        GPIO5INTD_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO5INTD_A {
        match self.bits {
            false => GPIO5INTD_A::INTDIS,
            true => GPIO5INTD_A::INTBOTH,
        }
    }
    #[doc = "Checks if the value of the field is `INTDIS`"]
    #[inline(always)]
    pub fn is_intdis(&self) -> bool {
        **self == GPIO5INTD_A::INTDIS
    }
    #[doc = "Checks if the value of the field is `INTBOTH`"]
    #[inline(always)]
    pub fn is_intboth(&self) -> bool {
        **self == GPIO5INTD_A::INTBOTH
    }
}
impl core::ops::Deref for GPIO5INTD_R {
    type Target = crate::FieldReader<bool, GPIO5INTD_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIO5INTD` writer - GPIO5 interrupt direction."]
pub struct GPIO5INTD_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO5INTD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIO5INTD_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "INCFG = 1 - No interrupt on GPIO transition value."]
    #[inline(always)]
    pub fn intdis(self) -> &'a mut W {
        self.variant(GPIO5INTD_A::INTDIS)
    }
    #[doc = "INCFG = 1 - Interrupt on either low to high or high to low GPIO transition value."]
    #[inline(always)]
    pub fn intboth(self) -> &'a mut W {
        self.variant(GPIO5INTD_A::INTBOTH)
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | ((value as u32 & 0x01) << 23);
        self.w
    }
}
#[doc = "GPIO5 output configuration.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum GPIO5OUTCFG_A {
    #[doc = "0: FNCSEL = 0x3 - Output disabled value."]
    DIS = 0,
    #[doc = "1: FNCSEL = 0x3 - Output is push-pull value."]
    PUSHPULL = 1,
    #[doc = "2: FNCSEL = 0x3 - Output is open drain value."]
    OD = 2,
    #[doc = "3: FNCSEL = 0x3 - Output is tri-state value."]
    TS = 3,
}
impl From<GPIO5OUTCFG_A> for u8 {
    #[inline(always)]
    fn from(variant: GPIO5OUTCFG_A) -> Self {
        variant as _
    }
}
#[doc = "Field `GPIO5OUTCFG` reader - GPIO5 output configuration."]
pub struct GPIO5OUTCFG_R(crate::FieldReader<u8, GPIO5OUTCFG_A>);
impl GPIO5OUTCFG_R {
    pub(crate) fn new(bits: u8) -> Self {
        GPIO5OUTCFG_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO5OUTCFG_A {
        match self.bits {
            0 => GPIO5OUTCFG_A::DIS,
            1 => GPIO5OUTCFG_A::PUSHPULL,
            2 => GPIO5OUTCFG_A::OD,
            3 => GPIO5OUTCFG_A::TS,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        **self == GPIO5OUTCFG_A::DIS
    }
    #[doc = "Checks if the value of the field is `PUSHPULL`"]
    #[inline(always)]
    pub fn is_pushpull(&self) -> bool {
        **self == GPIO5OUTCFG_A::PUSHPULL
    }
    #[doc = "Checks if the value of the field is `OD`"]
    #[inline(always)]
    pub fn is_od(&self) -> bool {
        **self == GPIO5OUTCFG_A::OD
    }
    #[doc = "Checks if the value of the field is `TS`"]
    #[inline(always)]
    pub fn is_ts(&self) -> bool {
        **self == GPIO5OUTCFG_A::TS
    }
}
impl core::ops::Deref for GPIO5OUTCFG_R {
    type Target = crate::FieldReader<u8, GPIO5OUTCFG_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIO5OUTCFG` writer - GPIO5 output configuration."]
pub struct GPIO5OUTCFG_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO5OUTCFG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIO5OUTCFG_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "FNCSEL = 0x3 - Output disabled value."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(GPIO5OUTCFG_A::DIS)
    }
    #[doc = "FNCSEL = 0x3 - Output is push-pull value."]
    #[inline(always)]
    pub fn pushpull(self) -> &'a mut W {
        self.variant(GPIO5OUTCFG_A::PUSHPULL)
    }
    #[doc = "FNCSEL = 0x3 - Output is open drain value."]
    #[inline(always)]
    pub fn od(self) -> &'a mut W {
        self.variant(GPIO5OUTCFG_A::OD)
    }
    #[doc = "FNCSEL = 0x3 - Output is tri-state value."]
    #[inline(always)]
    pub fn ts(self) -> &'a mut W {
        self.variant(GPIO5OUTCFG_A::TS)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 21)) | ((value as u32 & 0x03) << 21);
        self.w
    }
}
#[doc = "GPIO5 input enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO5INCFG_A {
    #[doc = "0: Read the GPIO pin data value."]
    READ = 0,
    #[doc = "1: INTD = 0 - Readback will always be zero value."]
    RDZERO = 1,
}
impl From<GPIO5INCFG_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO5INCFG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIO5INCFG` reader - GPIO5 input enable."]
pub struct GPIO5INCFG_R(crate::FieldReader<bool, GPIO5INCFG_A>);
impl GPIO5INCFG_R {
    pub(crate) fn new(bits: bool) -> Self {
        GPIO5INCFG_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO5INCFG_A {
        match self.bits {
            false => GPIO5INCFG_A::READ,
            true => GPIO5INCFG_A::RDZERO,
        }
    }
    #[doc = "Checks if the value of the field is `READ`"]
    #[inline(always)]
    pub fn is_read(&self) -> bool {
        **self == GPIO5INCFG_A::READ
    }
    #[doc = "Checks if the value of the field is `RDZERO`"]
    #[inline(always)]
    pub fn is_rdzero(&self) -> bool {
        **self == GPIO5INCFG_A::RDZERO
    }
}
impl core::ops::Deref for GPIO5INCFG_R {
    type Target = crate::FieldReader<bool, GPIO5INCFG_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIO5INCFG` writer - GPIO5 input enable."]
pub struct GPIO5INCFG_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO5INCFG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIO5INCFG_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Read the GPIO pin data value."]
    #[inline(always)]
    pub fn read(self) -> &'a mut W {
        self.variant(GPIO5INCFG_A::READ)
    }
    #[doc = "INTD = 0 - Readback will always be zero value."]
    #[inline(always)]
    pub fn rdzero(self) -> &'a mut W {
        self.variant(GPIO5INCFG_A::RDZERO)
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
#[doc = "GPIO4 interrupt direction.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO4INTD_A {
    #[doc = "0: FNCSEL = 0x2 - nCE polarity active low value."]
    NCELOW = 0,
    #[doc = "1: FNCSEL = 0x2 - nCE polarity active high value."]
    NCEHIGH = 1,
}
impl From<GPIO4INTD_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO4INTD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIO4INTD` reader - GPIO4 interrupt direction."]
pub struct GPIO4INTD_R(crate::FieldReader<bool, GPIO4INTD_A>);
impl GPIO4INTD_R {
    pub(crate) fn new(bits: bool) -> Self {
        GPIO4INTD_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO4INTD_A {
        match self.bits {
            false => GPIO4INTD_A::NCELOW,
            true => GPIO4INTD_A::NCEHIGH,
        }
    }
    #[doc = "Checks if the value of the field is `NCELOW`"]
    #[inline(always)]
    pub fn is_n_celow(&self) -> bool {
        **self == GPIO4INTD_A::NCELOW
    }
    #[doc = "Checks if the value of the field is `NCEHIGH`"]
    #[inline(always)]
    pub fn is_n_cehigh(&self) -> bool {
        **self == GPIO4INTD_A::NCEHIGH
    }
}
impl core::ops::Deref for GPIO4INTD_R {
    type Target = crate::FieldReader<bool, GPIO4INTD_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIO4INTD` writer - GPIO4 interrupt direction."]
pub struct GPIO4INTD_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO4INTD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIO4INTD_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "FNCSEL = 0x2 - nCE polarity active low value."]
    #[inline(always)]
    pub fn n_celow(self) -> &'a mut W {
        self.variant(GPIO4INTD_A::NCELOW)
    }
    #[doc = "FNCSEL = 0x2 - nCE polarity active high value."]
    #[inline(always)]
    pub fn n_cehigh(self) -> &'a mut W {
        self.variant(GPIO4INTD_A::NCEHIGH)
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
#[doc = "GPIO4 output configuration.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum GPIO4OUTCFG_A {
    #[doc = "0: FNCSEL = 0x3 - Output disabled value."]
    DIS = 0,
    #[doc = "1: FNCSEL = 0x3 - Output is push-pull value."]
    PUSHPULL = 1,
    #[doc = "2: FNCSEL = 0x3 - Output is open drain value."]
    OD = 2,
    #[doc = "3: FNCSEL = 0x3 - Output is tri-state value."]
    TS = 3,
}
impl From<GPIO4OUTCFG_A> for u8 {
    #[inline(always)]
    fn from(variant: GPIO4OUTCFG_A) -> Self {
        variant as _
    }
}
#[doc = "Field `GPIO4OUTCFG` reader - GPIO4 output configuration."]
pub struct GPIO4OUTCFG_R(crate::FieldReader<u8, GPIO4OUTCFG_A>);
impl GPIO4OUTCFG_R {
    pub(crate) fn new(bits: u8) -> Self {
        GPIO4OUTCFG_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO4OUTCFG_A {
        match self.bits {
            0 => GPIO4OUTCFG_A::DIS,
            1 => GPIO4OUTCFG_A::PUSHPULL,
            2 => GPIO4OUTCFG_A::OD,
            3 => GPIO4OUTCFG_A::TS,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        **self == GPIO4OUTCFG_A::DIS
    }
    #[doc = "Checks if the value of the field is `PUSHPULL`"]
    #[inline(always)]
    pub fn is_pushpull(&self) -> bool {
        **self == GPIO4OUTCFG_A::PUSHPULL
    }
    #[doc = "Checks if the value of the field is `OD`"]
    #[inline(always)]
    pub fn is_od(&self) -> bool {
        **self == GPIO4OUTCFG_A::OD
    }
    #[doc = "Checks if the value of the field is `TS`"]
    #[inline(always)]
    pub fn is_ts(&self) -> bool {
        **self == GPIO4OUTCFG_A::TS
    }
}
impl core::ops::Deref for GPIO4OUTCFG_R {
    type Target = crate::FieldReader<u8, GPIO4OUTCFG_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIO4OUTCFG` writer - GPIO4 output configuration."]
pub struct GPIO4OUTCFG_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO4OUTCFG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIO4OUTCFG_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "FNCSEL = 0x3 - Output disabled value."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(GPIO4OUTCFG_A::DIS)
    }
    #[doc = "FNCSEL = 0x3 - Output is push-pull value."]
    #[inline(always)]
    pub fn pushpull(self) -> &'a mut W {
        self.variant(GPIO4OUTCFG_A::PUSHPULL)
    }
    #[doc = "FNCSEL = 0x3 - Output is open drain value."]
    #[inline(always)]
    pub fn od(self) -> &'a mut W {
        self.variant(GPIO4OUTCFG_A::OD)
    }
    #[doc = "FNCSEL = 0x3 - Output is tri-state value."]
    #[inline(always)]
    pub fn ts(self) -> &'a mut W {
        self.variant(GPIO4OUTCFG_A::TS)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 17)) | ((value as u32 & 0x03) << 17);
        self.w
    }
}
#[doc = "GPIO4 input enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO4INCFG_A {
    #[doc = "0: Read the GPIO pin data value."]
    READ = 0,
    #[doc = "1: INTD = 0 - Readback will always be zero value."]
    RDZERO = 1,
}
impl From<GPIO4INCFG_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO4INCFG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIO4INCFG` reader - GPIO4 input enable."]
pub struct GPIO4INCFG_R(crate::FieldReader<bool, GPIO4INCFG_A>);
impl GPIO4INCFG_R {
    pub(crate) fn new(bits: bool) -> Self {
        GPIO4INCFG_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO4INCFG_A {
        match self.bits {
            false => GPIO4INCFG_A::READ,
            true => GPIO4INCFG_A::RDZERO,
        }
    }
    #[doc = "Checks if the value of the field is `READ`"]
    #[inline(always)]
    pub fn is_read(&self) -> bool {
        **self == GPIO4INCFG_A::READ
    }
    #[doc = "Checks if the value of the field is `RDZERO`"]
    #[inline(always)]
    pub fn is_rdzero(&self) -> bool {
        **self == GPIO4INCFG_A::RDZERO
    }
}
impl core::ops::Deref for GPIO4INCFG_R {
    type Target = crate::FieldReader<bool, GPIO4INCFG_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIO4INCFG` writer - GPIO4 input enable."]
pub struct GPIO4INCFG_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO4INCFG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIO4INCFG_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Read the GPIO pin data value."]
    #[inline(always)]
    pub fn read(self) -> &'a mut W {
        self.variant(GPIO4INCFG_A::READ)
    }
    #[doc = "INTD = 0 - Readback will always be zero value."]
    #[inline(always)]
    pub fn rdzero(self) -> &'a mut W {
        self.variant(GPIO4INCFG_A::RDZERO)
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
#[doc = "GPIO3 interrupt direction.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO3INTD_A {
    #[doc = "0: FNCSEL = 0x2 - nCE polarity active low value."]
    NCELOW = 0,
    #[doc = "1: FNCSEL = 0x2 - nCE polarity active high value."]
    NCEHIGH = 1,
}
impl From<GPIO3INTD_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO3INTD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIO3INTD` reader - GPIO3 interrupt direction."]
pub struct GPIO3INTD_R(crate::FieldReader<bool, GPIO3INTD_A>);
impl GPIO3INTD_R {
    pub(crate) fn new(bits: bool) -> Self {
        GPIO3INTD_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO3INTD_A {
        match self.bits {
            false => GPIO3INTD_A::NCELOW,
            true => GPIO3INTD_A::NCEHIGH,
        }
    }
    #[doc = "Checks if the value of the field is `NCELOW`"]
    #[inline(always)]
    pub fn is_n_celow(&self) -> bool {
        **self == GPIO3INTD_A::NCELOW
    }
    #[doc = "Checks if the value of the field is `NCEHIGH`"]
    #[inline(always)]
    pub fn is_n_cehigh(&self) -> bool {
        **self == GPIO3INTD_A::NCEHIGH
    }
}
impl core::ops::Deref for GPIO3INTD_R {
    type Target = crate::FieldReader<bool, GPIO3INTD_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIO3INTD` writer - GPIO3 interrupt direction."]
pub struct GPIO3INTD_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO3INTD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIO3INTD_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "FNCSEL = 0x2 - nCE polarity active low value."]
    #[inline(always)]
    pub fn n_celow(self) -> &'a mut W {
        self.variant(GPIO3INTD_A::NCELOW)
    }
    #[doc = "FNCSEL = 0x2 - nCE polarity active high value."]
    #[inline(always)]
    pub fn n_cehigh(self) -> &'a mut W {
        self.variant(GPIO3INTD_A::NCEHIGH)
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | ((value as u32 & 0x01) << 15);
        self.w
    }
}
#[doc = "GPIO3 output configuration.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum GPIO3OUTCFG_A {
    #[doc = "0: FNCSEL = 0x3 - Output disabled value."]
    DIS = 0,
    #[doc = "1: FNCSEL = 0x3 - Output is push-pull value."]
    PUSHPULL = 1,
    #[doc = "2: FNCSEL = 0x3 - Output is open drain value."]
    OD = 2,
    #[doc = "3: FNCSEL = 0x3 - Output is tri-state value."]
    TS = 3,
}
impl From<GPIO3OUTCFG_A> for u8 {
    #[inline(always)]
    fn from(variant: GPIO3OUTCFG_A) -> Self {
        variant as _
    }
}
#[doc = "Field `GPIO3OUTCFG` reader - GPIO3 output configuration."]
pub struct GPIO3OUTCFG_R(crate::FieldReader<u8, GPIO3OUTCFG_A>);
impl GPIO3OUTCFG_R {
    pub(crate) fn new(bits: u8) -> Self {
        GPIO3OUTCFG_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO3OUTCFG_A {
        match self.bits {
            0 => GPIO3OUTCFG_A::DIS,
            1 => GPIO3OUTCFG_A::PUSHPULL,
            2 => GPIO3OUTCFG_A::OD,
            3 => GPIO3OUTCFG_A::TS,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        **self == GPIO3OUTCFG_A::DIS
    }
    #[doc = "Checks if the value of the field is `PUSHPULL`"]
    #[inline(always)]
    pub fn is_pushpull(&self) -> bool {
        **self == GPIO3OUTCFG_A::PUSHPULL
    }
    #[doc = "Checks if the value of the field is `OD`"]
    #[inline(always)]
    pub fn is_od(&self) -> bool {
        **self == GPIO3OUTCFG_A::OD
    }
    #[doc = "Checks if the value of the field is `TS`"]
    #[inline(always)]
    pub fn is_ts(&self) -> bool {
        **self == GPIO3OUTCFG_A::TS
    }
}
impl core::ops::Deref for GPIO3OUTCFG_R {
    type Target = crate::FieldReader<u8, GPIO3OUTCFG_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIO3OUTCFG` writer - GPIO3 output configuration."]
pub struct GPIO3OUTCFG_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO3OUTCFG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIO3OUTCFG_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "FNCSEL = 0x3 - Output disabled value."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(GPIO3OUTCFG_A::DIS)
    }
    #[doc = "FNCSEL = 0x3 - Output is push-pull value."]
    #[inline(always)]
    pub fn pushpull(self) -> &'a mut W {
        self.variant(GPIO3OUTCFG_A::PUSHPULL)
    }
    #[doc = "FNCSEL = 0x3 - Output is open drain value."]
    #[inline(always)]
    pub fn od(self) -> &'a mut W {
        self.variant(GPIO3OUTCFG_A::OD)
    }
    #[doc = "FNCSEL = 0x3 - Output is tri-state value."]
    #[inline(always)]
    pub fn ts(self) -> &'a mut W {
        self.variant(GPIO3OUTCFG_A::TS)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 13)) | ((value as u32 & 0x03) << 13);
        self.w
    }
}
#[doc = "GPIO3 input enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO3INCFG_A {
    #[doc = "0: Read the GPIO pin data value."]
    READ = 0,
    #[doc = "1: INTD = 0 - Readback will always be zero value."]
    RDZERO = 1,
}
impl From<GPIO3INCFG_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO3INCFG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIO3INCFG` reader - GPIO3 input enable."]
pub struct GPIO3INCFG_R(crate::FieldReader<bool, GPIO3INCFG_A>);
impl GPIO3INCFG_R {
    pub(crate) fn new(bits: bool) -> Self {
        GPIO3INCFG_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO3INCFG_A {
        match self.bits {
            false => GPIO3INCFG_A::READ,
            true => GPIO3INCFG_A::RDZERO,
        }
    }
    #[doc = "Checks if the value of the field is `READ`"]
    #[inline(always)]
    pub fn is_read(&self) -> bool {
        **self == GPIO3INCFG_A::READ
    }
    #[doc = "Checks if the value of the field is `RDZERO`"]
    #[inline(always)]
    pub fn is_rdzero(&self) -> bool {
        **self == GPIO3INCFG_A::RDZERO
    }
}
impl core::ops::Deref for GPIO3INCFG_R {
    type Target = crate::FieldReader<bool, GPIO3INCFG_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIO3INCFG` writer - GPIO3 input enable."]
pub struct GPIO3INCFG_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO3INCFG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIO3INCFG_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Read the GPIO pin data value."]
    #[inline(always)]
    pub fn read(self) -> &'a mut W {
        self.variant(GPIO3INCFG_A::READ)
    }
    #[doc = "INTD = 0 - Readback will always be zero value."]
    #[inline(always)]
    pub fn rdzero(self) -> &'a mut W {
        self.variant(GPIO3INCFG_A::RDZERO)
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | ((value as u32 & 0x01) << 12);
        self.w
    }
}
#[doc = "GPIO2 interrupt direction.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO2INTD_A {
    #[doc = "0: FNCSEL = 0x7 - nCE polarity active low value."]
    NCELOW = 0,
    #[doc = "1: FNCSEL = 0x7 - nCE polarity active high value."]
    NCEHIGH = 1,
}
impl From<GPIO2INTD_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO2INTD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIO2INTD` reader - GPIO2 interrupt direction."]
pub struct GPIO2INTD_R(crate::FieldReader<bool, GPIO2INTD_A>);
impl GPIO2INTD_R {
    pub(crate) fn new(bits: bool) -> Self {
        GPIO2INTD_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO2INTD_A {
        match self.bits {
            false => GPIO2INTD_A::NCELOW,
            true => GPIO2INTD_A::NCEHIGH,
        }
    }
    #[doc = "Checks if the value of the field is `NCELOW`"]
    #[inline(always)]
    pub fn is_n_celow(&self) -> bool {
        **self == GPIO2INTD_A::NCELOW
    }
    #[doc = "Checks if the value of the field is `NCEHIGH`"]
    #[inline(always)]
    pub fn is_n_cehigh(&self) -> bool {
        **self == GPIO2INTD_A::NCEHIGH
    }
}
impl core::ops::Deref for GPIO2INTD_R {
    type Target = crate::FieldReader<bool, GPIO2INTD_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIO2INTD` writer - GPIO2 interrupt direction."]
pub struct GPIO2INTD_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO2INTD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIO2INTD_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "FNCSEL = 0x7 - nCE polarity active low value."]
    #[inline(always)]
    pub fn n_celow(self) -> &'a mut W {
        self.variant(GPIO2INTD_A::NCELOW)
    }
    #[doc = "FNCSEL = 0x7 - nCE polarity active high value."]
    #[inline(always)]
    pub fn n_cehigh(self) -> &'a mut W {
        self.variant(GPIO2INTD_A::NCEHIGH)
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | ((value as u32 & 0x01) << 11);
        self.w
    }
}
#[doc = "GPIO2 output configuration.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum GPIO2OUTCFG_A {
    #[doc = "0: FNCSEL = 0x3 - Output disabled value."]
    DIS = 0,
    #[doc = "1: FNCSEL = 0x3 - Output is push-pull value."]
    PUSHPULL = 1,
    #[doc = "2: FNCSEL = 0x3 - Output is open drain value."]
    OD = 2,
    #[doc = "3: FNCSEL = 0x3 - Output is tri-state value."]
    TS = 3,
}
impl From<GPIO2OUTCFG_A> for u8 {
    #[inline(always)]
    fn from(variant: GPIO2OUTCFG_A) -> Self {
        variant as _
    }
}
#[doc = "Field `GPIO2OUTCFG` reader - GPIO2 output configuration."]
pub struct GPIO2OUTCFG_R(crate::FieldReader<u8, GPIO2OUTCFG_A>);
impl GPIO2OUTCFG_R {
    pub(crate) fn new(bits: u8) -> Self {
        GPIO2OUTCFG_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO2OUTCFG_A {
        match self.bits {
            0 => GPIO2OUTCFG_A::DIS,
            1 => GPIO2OUTCFG_A::PUSHPULL,
            2 => GPIO2OUTCFG_A::OD,
            3 => GPIO2OUTCFG_A::TS,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        **self == GPIO2OUTCFG_A::DIS
    }
    #[doc = "Checks if the value of the field is `PUSHPULL`"]
    #[inline(always)]
    pub fn is_pushpull(&self) -> bool {
        **self == GPIO2OUTCFG_A::PUSHPULL
    }
    #[doc = "Checks if the value of the field is `OD`"]
    #[inline(always)]
    pub fn is_od(&self) -> bool {
        **self == GPIO2OUTCFG_A::OD
    }
    #[doc = "Checks if the value of the field is `TS`"]
    #[inline(always)]
    pub fn is_ts(&self) -> bool {
        **self == GPIO2OUTCFG_A::TS
    }
}
impl core::ops::Deref for GPIO2OUTCFG_R {
    type Target = crate::FieldReader<u8, GPIO2OUTCFG_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIO2OUTCFG` writer - GPIO2 output configuration."]
pub struct GPIO2OUTCFG_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO2OUTCFG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIO2OUTCFG_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "FNCSEL = 0x3 - Output disabled value."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(GPIO2OUTCFG_A::DIS)
    }
    #[doc = "FNCSEL = 0x3 - Output is push-pull value."]
    #[inline(always)]
    pub fn pushpull(self) -> &'a mut W {
        self.variant(GPIO2OUTCFG_A::PUSHPULL)
    }
    #[doc = "FNCSEL = 0x3 - Output is open drain value."]
    #[inline(always)]
    pub fn od(self) -> &'a mut W {
        self.variant(GPIO2OUTCFG_A::OD)
    }
    #[doc = "FNCSEL = 0x3 - Output is tri-state value."]
    #[inline(always)]
    pub fn ts(self) -> &'a mut W {
        self.variant(GPIO2OUTCFG_A::TS)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 9)) | ((value as u32 & 0x03) << 9);
        self.w
    }
}
#[doc = "GPIO2 input enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO2INCFG_A {
    #[doc = "0: Read the GPIO pin data value."]
    READ = 0,
    #[doc = "1: INTD = 0 - Readback will always be zero value."]
    RDZERO = 1,
}
impl From<GPIO2INCFG_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO2INCFG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIO2INCFG` reader - GPIO2 input enable."]
pub struct GPIO2INCFG_R(crate::FieldReader<bool, GPIO2INCFG_A>);
impl GPIO2INCFG_R {
    pub(crate) fn new(bits: bool) -> Self {
        GPIO2INCFG_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO2INCFG_A {
        match self.bits {
            false => GPIO2INCFG_A::READ,
            true => GPIO2INCFG_A::RDZERO,
        }
    }
    #[doc = "Checks if the value of the field is `READ`"]
    #[inline(always)]
    pub fn is_read(&self) -> bool {
        **self == GPIO2INCFG_A::READ
    }
    #[doc = "Checks if the value of the field is `RDZERO`"]
    #[inline(always)]
    pub fn is_rdzero(&self) -> bool {
        **self == GPIO2INCFG_A::RDZERO
    }
}
impl core::ops::Deref for GPIO2INCFG_R {
    type Target = crate::FieldReader<bool, GPIO2INCFG_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIO2INCFG` writer - GPIO2 input enable."]
pub struct GPIO2INCFG_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO2INCFG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIO2INCFG_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Read the GPIO pin data value."]
    #[inline(always)]
    pub fn read(self) -> &'a mut W {
        self.variant(GPIO2INCFG_A::READ)
    }
    #[doc = "INTD = 0 - Readback will always be zero value."]
    #[inline(always)]
    pub fn rdzero(self) -> &'a mut W {
        self.variant(GPIO2INCFG_A::RDZERO)
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
#[doc = "GPIO1 interrupt direction.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO1INTD_A {
    #[doc = "0: FNCSEL = 0x7 - nCE polarity active low value."]
    NCELOW = 0,
    #[doc = "1: FNCSEL = 0x7 - nCE polarity active high value."]
    NCEHIGH = 1,
}
impl From<GPIO1INTD_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO1INTD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIO1INTD` reader - GPIO1 interrupt direction."]
pub struct GPIO1INTD_R(crate::FieldReader<bool, GPIO1INTD_A>);
impl GPIO1INTD_R {
    pub(crate) fn new(bits: bool) -> Self {
        GPIO1INTD_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO1INTD_A {
        match self.bits {
            false => GPIO1INTD_A::NCELOW,
            true => GPIO1INTD_A::NCEHIGH,
        }
    }
    #[doc = "Checks if the value of the field is `NCELOW`"]
    #[inline(always)]
    pub fn is_n_celow(&self) -> bool {
        **self == GPIO1INTD_A::NCELOW
    }
    #[doc = "Checks if the value of the field is `NCEHIGH`"]
    #[inline(always)]
    pub fn is_n_cehigh(&self) -> bool {
        **self == GPIO1INTD_A::NCEHIGH
    }
}
impl core::ops::Deref for GPIO1INTD_R {
    type Target = crate::FieldReader<bool, GPIO1INTD_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIO1INTD` writer - GPIO1 interrupt direction."]
pub struct GPIO1INTD_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO1INTD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIO1INTD_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "FNCSEL = 0x7 - nCE polarity active low value."]
    #[inline(always)]
    pub fn n_celow(self) -> &'a mut W {
        self.variant(GPIO1INTD_A::NCELOW)
    }
    #[doc = "FNCSEL = 0x7 - nCE polarity active high value."]
    #[inline(always)]
    pub fn n_cehigh(self) -> &'a mut W {
        self.variant(GPIO1INTD_A::NCEHIGH)
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | ((value as u32 & 0x01) << 7);
        self.w
    }
}
#[doc = "GPIO1 output configuration.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum GPIO1OUTCFG_A {
    #[doc = "0: FNCSEL = 0x3 - Output disabled value."]
    DIS = 0,
    #[doc = "1: FNCSEL = 0x3 - Output is push-pull value."]
    PUSHPULL = 1,
    #[doc = "2: FNCSEL = 0x3 - Output is open drain value."]
    OD = 2,
    #[doc = "3: FNCSEL = 0x3 - Output is tri-state value."]
    TS = 3,
}
impl From<GPIO1OUTCFG_A> for u8 {
    #[inline(always)]
    fn from(variant: GPIO1OUTCFG_A) -> Self {
        variant as _
    }
}
#[doc = "Field `GPIO1OUTCFG` reader - GPIO1 output configuration."]
pub struct GPIO1OUTCFG_R(crate::FieldReader<u8, GPIO1OUTCFG_A>);
impl GPIO1OUTCFG_R {
    pub(crate) fn new(bits: u8) -> Self {
        GPIO1OUTCFG_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO1OUTCFG_A {
        match self.bits {
            0 => GPIO1OUTCFG_A::DIS,
            1 => GPIO1OUTCFG_A::PUSHPULL,
            2 => GPIO1OUTCFG_A::OD,
            3 => GPIO1OUTCFG_A::TS,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        **self == GPIO1OUTCFG_A::DIS
    }
    #[doc = "Checks if the value of the field is `PUSHPULL`"]
    #[inline(always)]
    pub fn is_pushpull(&self) -> bool {
        **self == GPIO1OUTCFG_A::PUSHPULL
    }
    #[doc = "Checks if the value of the field is `OD`"]
    #[inline(always)]
    pub fn is_od(&self) -> bool {
        **self == GPIO1OUTCFG_A::OD
    }
    #[doc = "Checks if the value of the field is `TS`"]
    #[inline(always)]
    pub fn is_ts(&self) -> bool {
        **self == GPIO1OUTCFG_A::TS
    }
}
impl core::ops::Deref for GPIO1OUTCFG_R {
    type Target = crate::FieldReader<u8, GPIO1OUTCFG_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIO1OUTCFG` writer - GPIO1 output configuration."]
pub struct GPIO1OUTCFG_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO1OUTCFG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIO1OUTCFG_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "FNCSEL = 0x3 - Output disabled value."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(GPIO1OUTCFG_A::DIS)
    }
    #[doc = "FNCSEL = 0x3 - Output is push-pull value."]
    #[inline(always)]
    pub fn pushpull(self) -> &'a mut W {
        self.variant(GPIO1OUTCFG_A::PUSHPULL)
    }
    #[doc = "FNCSEL = 0x3 - Output is open drain value."]
    #[inline(always)]
    pub fn od(self) -> &'a mut W {
        self.variant(GPIO1OUTCFG_A::OD)
    }
    #[doc = "FNCSEL = 0x3 - Output is tri-state value."]
    #[inline(always)]
    pub fn ts(self) -> &'a mut W {
        self.variant(GPIO1OUTCFG_A::TS)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 5)) | ((value as u32 & 0x03) << 5);
        self.w
    }
}
#[doc = "GPIO1 input enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO1INCFG_A {
    #[doc = "0: Read the GPIO pin data value."]
    READ = 0,
    #[doc = "1: INTD = 0 - Readback will always be zero value."]
    RDZERO = 1,
}
impl From<GPIO1INCFG_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO1INCFG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIO1INCFG` reader - GPIO1 input enable."]
pub struct GPIO1INCFG_R(crate::FieldReader<bool, GPIO1INCFG_A>);
impl GPIO1INCFG_R {
    pub(crate) fn new(bits: bool) -> Self {
        GPIO1INCFG_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO1INCFG_A {
        match self.bits {
            false => GPIO1INCFG_A::READ,
            true => GPIO1INCFG_A::RDZERO,
        }
    }
    #[doc = "Checks if the value of the field is `READ`"]
    #[inline(always)]
    pub fn is_read(&self) -> bool {
        **self == GPIO1INCFG_A::READ
    }
    #[doc = "Checks if the value of the field is `RDZERO`"]
    #[inline(always)]
    pub fn is_rdzero(&self) -> bool {
        **self == GPIO1INCFG_A::RDZERO
    }
}
impl core::ops::Deref for GPIO1INCFG_R {
    type Target = crate::FieldReader<bool, GPIO1INCFG_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIO1INCFG` writer - GPIO1 input enable."]
pub struct GPIO1INCFG_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO1INCFG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIO1INCFG_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Read the GPIO pin data value."]
    #[inline(always)]
    pub fn read(self) -> &'a mut W {
        self.variant(GPIO1INCFG_A::READ)
    }
    #[doc = "INTD = 0 - Readback will always be zero value."]
    #[inline(always)]
    pub fn rdzero(self) -> &'a mut W {
        self.variant(GPIO1INCFG_A::RDZERO)
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u32 & 0x01) << 4);
        self.w
    }
}
#[doc = "GPIO0 interrupt direction.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO0INTD_A {
    #[doc = "0: FNCSEL = 0x7 - nCE polarity active low value."]
    NCELOW = 0,
    #[doc = "1: FNCSEL = 0x7 - nCE polarity active high value."]
    NCEHIGH = 1,
}
impl From<GPIO0INTD_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO0INTD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIO0INTD` reader - GPIO0 interrupt direction."]
pub struct GPIO0INTD_R(crate::FieldReader<bool, GPIO0INTD_A>);
impl GPIO0INTD_R {
    pub(crate) fn new(bits: bool) -> Self {
        GPIO0INTD_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO0INTD_A {
        match self.bits {
            false => GPIO0INTD_A::NCELOW,
            true => GPIO0INTD_A::NCEHIGH,
        }
    }
    #[doc = "Checks if the value of the field is `NCELOW`"]
    #[inline(always)]
    pub fn is_n_celow(&self) -> bool {
        **self == GPIO0INTD_A::NCELOW
    }
    #[doc = "Checks if the value of the field is `NCEHIGH`"]
    #[inline(always)]
    pub fn is_n_cehigh(&self) -> bool {
        **self == GPIO0INTD_A::NCEHIGH
    }
}
impl core::ops::Deref for GPIO0INTD_R {
    type Target = crate::FieldReader<bool, GPIO0INTD_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIO0INTD` writer - GPIO0 interrupt direction."]
pub struct GPIO0INTD_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO0INTD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIO0INTD_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "FNCSEL = 0x7 - nCE polarity active low value."]
    #[inline(always)]
    pub fn n_celow(self) -> &'a mut W {
        self.variant(GPIO0INTD_A::NCELOW)
    }
    #[doc = "FNCSEL = 0x7 - nCE polarity active high value."]
    #[inline(always)]
    pub fn n_cehigh(self) -> &'a mut W {
        self.variant(GPIO0INTD_A::NCEHIGH)
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u32 & 0x01) << 3);
        self.w
    }
}
#[doc = "GPIO0 output configuration.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum GPIO0OUTCFG_A {
    #[doc = "0: FNCSEL = 0x3 - Output disabled value."]
    DIS = 0,
    #[doc = "1: FNCSEL = 0x3 - Output is push-pull value."]
    PUSHPULL = 1,
    #[doc = "2: FNCSEL = 0x3 - Output is open drain value."]
    OD = 2,
    #[doc = "3: FNCSEL = 0x3 - Output is tri-state value."]
    TS = 3,
}
impl From<GPIO0OUTCFG_A> for u8 {
    #[inline(always)]
    fn from(variant: GPIO0OUTCFG_A) -> Self {
        variant as _
    }
}
#[doc = "Field `GPIO0OUTCFG` reader - GPIO0 output configuration."]
pub struct GPIO0OUTCFG_R(crate::FieldReader<u8, GPIO0OUTCFG_A>);
impl GPIO0OUTCFG_R {
    pub(crate) fn new(bits: u8) -> Self {
        GPIO0OUTCFG_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO0OUTCFG_A {
        match self.bits {
            0 => GPIO0OUTCFG_A::DIS,
            1 => GPIO0OUTCFG_A::PUSHPULL,
            2 => GPIO0OUTCFG_A::OD,
            3 => GPIO0OUTCFG_A::TS,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        **self == GPIO0OUTCFG_A::DIS
    }
    #[doc = "Checks if the value of the field is `PUSHPULL`"]
    #[inline(always)]
    pub fn is_pushpull(&self) -> bool {
        **self == GPIO0OUTCFG_A::PUSHPULL
    }
    #[doc = "Checks if the value of the field is `OD`"]
    #[inline(always)]
    pub fn is_od(&self) -> bool {
        **self == GPIO0OUTCFG_A::OD
    }
    #[doc = "Checks if the value of the field is `TS`"]
    #[inline(always)]
    pub fn is_ts(&self) -> bool {
        **self == GPIO0OUTCFG_A::TS
    }
}
impl core::ops::Deref for GPIO0OUTCFG_R {
    type Target = crate::FieldReader<u8, GPIO0OUTCFG_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIO0OUTCFG` writer - GPIO0 output configuration."]
pub struct GPIO0OUTCFG_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO0OUTCFG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIO0OUTCFG_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "FNCSEL = 0x3 - Output disabled value."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(GPIO0OUTCFG_A::DIS)
    }
    #[doc = "FNCSEL = 0x3 - Output is push-pull value."]
    #[inline(always)]
    pub fn pushpull(self) -> &'a mut W {
        self.variant(GPIO0OUTCFG_A::PUSHPULL)
    }
    #[doc = "FNCSEL = 0x3 - Output is open drain value."]
    #[inline(always)]
    pub fn od(self) -> &'a mut W {
        self.variant(GPIO0OUTCFG_A::OD)
    }
    #[doc = "FNCSEL = 0x3 - Output is tri-state value."]
    #[inline(always)]
    pub fn ts(self) -> &'a mut W {
        self.variant(GPIO0OUTCFG_A::TS)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 1)) | ((value as u32 & 0x03) << 1);
        self.w
    }
}
#[doc = "GPIO0 input enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO0INCFG_A {
    #[doc = "0: Read the GPIO pin data value."]
    READ = 0,
    #[doc = "1: INTD = 0 - Readback will always be zero value."]
    RDZERO = 1,
}
impl From<GPIO0INCFG_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO0INCFG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIO0INCFG` reader - GPIO0 input enable."]
pub struct GPIO0INCFG_R(crate::FieldReader<bool, GPIO0INCFG_A>);
impl GPIO0INCFG_R {
    pub(crate) fn new(bits: bool) -> Self {
        GPIO0INCFG_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO0INCFG_A {
        match self.bits {
            false => GPIO0INCFG_A::READ,
            true => GPIO0INCFG_A::RDZERO,
        }
    }
    #[doc = "Checks if the value of the field is `READ`"]
    #[inline(always)]
    pub fn is_read(&self) -> bool {
        **self == GPIO0INCFG_A::READ
    }
    #[doc = "Checks if the value of the field is `RDZERO`"]
    #[inline(always)]
    pub fn is_rdzero(&self) -> bool {
        **self == GPIO0INCFG_A::RDZERO
    }
}
impl core::ops::Deref for GPIO0INCFG_R {
    type Target = crate::FieldReader<bool, GPIO0INCFG_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIO0INCFG` writer - GPIO0 input enable."]
pub struct GPIO0INCFG_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO0INCFG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIO0INCFG_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Read the GPIO pin data value."]
    #[inline(always)]
    pub fn read(self) -> &'a mut W {
        self.variant(GPIO0INCFG_A::READ)
    }
    #[doc = "INTD = 0 - Readback will always be zero value."]
    #[inline(always)]
    pub fn rdzero(self) -> &'a mut W {
        self.variant(GPIO0INCFG_A::RDZERO)
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
    #[doc = "Bit 31 - GPIO7 interrupt direction, nCE polarity."]
    #[inline(always)]
    pub fn gpio7intd(&self) -> GPIO7INTD_R {
        GPIO7INTD_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bits 29:30 - GPIO7 output configuration."]
    #[inline(always)]
    pub fn gpio7outcfg(&self) -> GPIO7OUTCFG_R {
        GPIO7OUTCFG_R::new(((self.bits >> 29) & 0x03) as u8)
    }
    #[doc = "Bit 28 - GPIO7 input enable."]
    #[inline(always)]
    pub fn gpio7incfg(&self) -> GPIO7INCFG_R {
        GPIO7INCFG_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 27 - GPIO6 interrupt direction."]
    #[inline(always)]
    pub fn gpio6intd(&self) -> GPIO6INTD_R {
        GPIO6INTD_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bits 25:26 - GPIO6 output configuration."]
    #[inline(always)]
    pub fn gpio6outcfg(&self) -> GPIO6OUTCFG_R {
        GPIO6OUTCFG_R::new(((self.bits >> 25) & 0x03) as u8)
    }
    #[doc = "Bit 24 - GPIO6 input enable."]
    #[inline(always)]
    pub fn gpio6incfg(&self) -> GPIO6INCFG_R {
        GPIO6INCFG_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 23 - GPIO5 interrupt direction."]
    #[inline(always)]
    pub fn gpio5intd(&self) -> GPIO5INTD_R {
        GPIO5INTD_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bits 21:22 - GPIO5 output configuration."]
    #[inline(always)]
    pub fn gpio5outcfg(&self) -> GPIO5OUTCFG_R {
        GPIO5OUTCFG_R::new(((self.bits >> 21) & 0x03) as u8)
    }
    #[doc = "Bit 20 - GPIO5 input enable."]
    #[inline(always)]
    pub fn gpio5incfg(&self) -> GPIO5INCFG_R {
        GPIO5INCFG_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 19 - GPIO4 interrupt direction."]
    #[inline(always)]
    pub fn gpio4intd(&self) -> GPIO4INTD_R {
        GPIO4INTD_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bits 17:18 - GPIO4 output configuration."]
    #[inline(always)]
    pub fn gpio4outcfg(&self) -> GPIO4OUTCFG_R {
        GPIO4OUTCFG_R::new(((self.bits >> 17) & 0x03) as u8)
    }
    #[doc = "Bit 16 - GPIO4 input enable."]
    #[inline(always)]
    pub fn gpio4incfg(&self) -> GPIO4INCFG_R {
        GPIO4INCFG_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 15 - GPIO3 interrupt direction."]
    #[inline(always)]
    pub fn gpio3intd(&self) -> GPIO3INTD_R {
        GPIO3INTD_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bits 13:14 - GPIO3 output configuration."]
    #[inline(always)]
    pub fn gpio3outcfg(&self) -> GPIO3OUTCFG_R {
        GPIO3OUTCFG_R::new(((self.bits >> 13) & 0x03) as u8)
    }
    #[doc = "Bit 12 - GPIO3 input enable."]
    #[inline(always)]
    pub fn gpio3incfg(&self) -> GPIO3INCFG_R {
        GPIO3INCFG_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11 - GPIO2 interrupt direction."]
    #[inline(always)]
    pub fn gpio2intd(&self) -> GPIO2INTD_R {
        GPIO2INTD_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bits 9:10 - GPIO2 output configuration."]
    #[inline(always)]
    pub fn gpio2outcfg(&self) -> GPIO2OUTCFG_R {
        GPIO2OUTCFG_R::new(((self.bits >> 9) & 0x03) as u8)
    }
    #[doc = "Bit 8 - GPIO2 input enable."]
    #[inline(always)]
    pub fn gpio2incfg(&self) -> GPIO2INCFG_R {
        GPIO2INCFG_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7 - GPIO1 interrupt direction."]
    #[inline(always)]
    pub fn gpio1intd(&self) -> GPIO1INTD_R {
        GPIO1INTD_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 5:6 - GPIO1 output configuration."]
    #[inline(always)]
    pub fn gpio1outcfg(&self) -> GPIO1OUTCFG_R {
        GPIO1OUTCFG_R::new(((self.bits >> 5) & 0x03) as u8)
    }
    #[doc = "Bit 4 - GPIO1 input enable."]
    #[inline(always)]
    pub fn gpio1incfg(&self) -> GPIO1INCFG_R {
        GPIO1INCFG_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - GPIO0 interrupt direction."]
    #[inline(always)]
    pub fn gpio0intd(&self) -> GPIO0INTD_R {
        GPIO0INTD_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bits 1:2 - GPIO0 output configuration."]
    #[inline(always)]
    pub fn gpio0outcfg(&self) -> GPIO0OUTCFG_R {
        GPIO0OUTCFG_R::new(((self.bits >> 1) & 0x03) as u8)
    }
    #[doc = "Bit 0 - GPIO0 input enable."]
    #[inline(always)]
    pub fn gpio0incfg(&self) -> GPIO0INCFG_R {
        GPIO0INCFG_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 31 - GPIO7 interrupt direction, nCE polarity."]
    #[inline(always)]
    pub fn gpio7intd(&mut self) -> GPIO7INTD_W {
        GPIO7INTD_W { w: self }
    }
    #[doc = "Bits 29:30 - GPIO7 output configuration."]
    #[inline(always)]
    pub fn gpio7outcfg(&mut self) -> GPIO7OUTCFG_W {
        GPIO7OUTCFG_W { w: self }
    }
    #[doc = "Bit 28 - GPIO7 input enable."]
    #[inline(always)]
    pub fn gpio7incfg(&mut self) -> GPIO7INCFG_W {
        GPIO7INCFG_W { w: self }
    }
    #[doc = "Bit 27 - GPIO6 interrupt direction."]
    #[inline(always)]
    pub fn gpio6intd(&mut self) -> GPIO6INTD_W {
        GPIO6INTD_W { w: self }
    }
    #[doc = "Bits 25:26 - GPIO6 output configuration."]
    #[inline(always)]
    pub fn gpio6outcfg(&mut self) -> GPIO6OUTCFG_W {
        GPIO6OUTCFG_W { w: self }
    }
    #[doc = "Bit 24 - GPIO6 input enable."]
    #[inline(always)]
    pub fn gpio6incfg(&mut self) -> GPIO6INCFG_W {
        GPIO6INCFG_W { w: self }
    }
    #[doc = "Bit 23 - GPIO5 interrupt direction."]
    #[inline(always)]
    pub fn gpio5intd(&mut self) -> GPIO5INTD_W {
        GPIO5INTD_W { w: self }
    }
    #[doc = "Bits 21:22 - GPIO5 output configuration."]
    #[inline(always)]
    pub fn gpio5outcfg(&mut self) -> GPIO5OUTCFG_W {
        GPIO5OUTCFG_W { w: self }
    }
    #[doc = "Bit 20 - GPIO5 input enable."]
    #[inline(always)]
    pub fn gpio5incfg(&mut self) -> GPIO5INCFG_W {
        GPIO5INCFG_W { w: self }
    }
    #[doc = "Bit 19 - GPIO4 interrupt direction."]
    #[inline(always)]
    pub fn gpio4intd(&mut self) -> GPIO4INTD_W {
        GPIO4INTD_W { w: self }
    }
    #[doc = "Bits 17:18 - GPIO4 output configuration."]
    #[inline(always)]
    pub fn gpio4outcfg(&mut self) -> GPIO4OUTCFG_W {
        GPIO4OUTCFG_W { w: self }
    }
    #[doc = "Bit 16 - GPIO4 input enable."]
    #[inline(always)]
    pub fn gpio4incfg(&mut self) -> GPIO4INCFG_W {
        GPIO4INCFG_W { w: self }
    }
    #[doc = "Bit 15 - GPIO3 interrupt direction."]
    #[inline(always)]
    pub fn gpio3intd(&mut self) -> GPIO3INTD_W {
        GPIO3INTD_W { w: self }
    }
    #[doc = "Bits 13:14 - GPIO3 output configuration."]
    #[inline(always)]
    pub fn gpio3outcfg(&mut self) -> GPIO3OUTCFG_W {
        GPIO3OUTCFG_W { w: self }
    }
    #[doc = "Bit 12 - GPIO3 input enable."]
    #[inline(always)]
    pub fn gpio3incfg(&mut self) -> GPIO3INCFG_W {
        GPIO3INCFG_W { w: self }
    }
    #[doc = "Bit 11 - GPIO2 interrupt direction."]
    #[inline(always)]
    pub fn gpio2intd(&mut self) -> GPIO2INTD_W {
        GPIO2INTD_W { w: self }
    }
    #[doc = "Bits 9:10 - GPIO2 output configuration."]
    #[inline(always)]
    pub fn gpio2outcfg(&mut self) -> GPIO2OUTCFG_W {
        GPIO2OUTCFG_W { w: self }
    }
    #[doc = "Bit 8 - GPIO2 input enable."]
    #[inline(always)]
    pub fn gpio2incfg(&mut self) -> GPIO2INCFG_W {
        GPIO2INCFG_W { w: self }
    }
    #[doc = "Bit 7 - GPIO1 interrupt direction."]
    #[inline(always)]
    pub fn gpio1intd(&mut self) -> GPIO1INTD_W {
        GPIO1INTD_W { w: self }
    }
    #[doc = "Bits 5:6 - GPIO1 output configuration."]
    #[inline(always)]
    pub fn gpio1outcfg(&mut self) -> GPIO1OUTCFG_W {
        GPIO1OUTCFG_W { w: self }
    }
    #[doc = "Bit 4 - GPIO1 input enable."]
    #[inline(always)]
    pub fn gpio1incfg(&mut self) -> GPIO1INCFG_W {
        GPIO1INCFG_W { w: self }
    }
    #[doc = "Bit 3 - GPIO0 interrupt direction."]
    #[inline(always)]
    pub fn gpio0intd(&mut self) -> GPIO0INTD_W {
        GPIO0INTD_W { w: self }
    }
    #[doc = "Bits 1:2 - GPIO0 output configuration."]
    #[inline(always)]
    pub fn gpio0outcfg(&mut self) -> GPIO0OUTCFG_W {
        GPIO0OUTCFG_W { w: self }
    }
    #[doc = "Bit 0 - GPIO0 input enable."]
    #[inline(always)]
    pub fn gpio0incfg(&mut self) -> GPIO0INCFG_W {
        GPIO0INCFG_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPIO Configuration Register A (Pads 0-7)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cfga](index.html) module"]
pub struct CFGA_SPEC;
impl crate::RegisterSpec for CFGA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cfga::R](R) reader structure"]
impl crate::Readable for CFGA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cfga::W](W) writer structure"]
impl crate::Writable for CFGA_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CFGA to value 0"]
impl crate::Resettable for CFGA_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
