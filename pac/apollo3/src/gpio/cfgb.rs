#[doc = "Register `CFGB` reader"]
pub struct R(crate::R<CFGB_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CFGB_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CFGB_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CFGB_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CFGB` writer"]
pub struct W(crate::W<CFGB_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CFGB_SPEC>;
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
impl From<crate::W<CFGB_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CFGB_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "GPIO15 interrupt direction.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO15INTD_A {
    #[doc = "0: FNCSEL = 0x1 - nCE polarity active low value."]
    NCELOW = 0,
    #[doc = "1: FNCSEL = 0x1 - nCE polarity active high value."]
    NCEHIGH = 1,
}
impl From<GPIO15INTD_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO15INTD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIO15INTD` reader - GPIO15 interrupt direction."]
pub struct GPIO15INTD_R(crate::FieldReader<bool, GPIO15INTD_A>);
impl GPIO15INTD_R {
    pub(crate) fn new(bits: bool) -> Self {
        GPIO15INTD_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO15INTD_A {
        match self.bits {
            false => GPIO15INTD_A::NCELOW,
            true => GPIO15INTD_A::NCEHIGH,
        }
    }
    #[doc = "Checks if the value of the field is `NCELOW`"]
    #[inline(always)]
    pub fn is_n_celow(&self) -> bool {
        **self == GPIO15INTD_A::NCELOW
    }
    #[doc = "Checks if the value of the field is `NCEHIGH`"]
    #[inline(always)]
    pub fn is_n_cehigh(&self) -> bool {
        **self == GPIO15INTD_A::NCEHIGH
    }
}
impl core::ops::Deref for GPIO15INTD_R {
    type Target = crate::FieldReader<bool, GPIO15INTD_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIO15INTD` writer - GPIO15 interrupt direction."]
pub struct GPIO15INTD_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO15INTD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIO15INTD_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "FNCSEL = 0x1 - nCE polarity active low value."]
    #[inline(always)]
    pub fn n_celow(self) -> &'a mut W {
        self.variant(GPIO15INTD_A::NCELOW)
    }
    #[doc = "FNCSEL = 0x1 - nCE polarity active high value."]
    #[inline(always)]
    pub fn n_cehigh(self) -> &'a mut W {
        self.variant(GPIO15INTD_A::NCEHIGH)
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
#[doc = "GPIO15 output configuration.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum GPIO15OUTCFG_A {
    #[doc = "0: FNCSEL = 0x3 - Output disabled value."]
    DIS = 0,
    #[doc = "1: FNCSEL = 0x3 - Output is push-pull value."]
    PUSHPULL = 1,
    #[doc = "2: FNCSEL = 0x3 - Output is open drain value."]
    OD = 2,
    #[doc = "3: FNCSEL = 0x3 - Output is tri-state value."]
    TS = 3,
}
impl From<GPIO15OUTCFG_A> for u8 {
    #[inline(always)]
    fn from(variant: GPIO15OUTCFG_A) -> Self {
        variant as _
    }
}
#[doc = "Field `GPIO15OUTCFG` reader - GPIO15 output configuration."]
pub struct GPIO15OUTCFG_R(crate::FieldReader<u8, GPIO15OUTCFG_A>);
impl GPIO15OUTCFG_R {
    pub(crate) fn new(bits: u8) -> Self {
        GPIO15OUTCFG_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO15OUTCFG_A {
        match self.bits {
            0 => GPIO15OUTCFG_A::DIS,
            1 => GPIO15OUTCFG_A::PUSHPULL,
            2 => GPIO15OUTCFG_A::OD,
            3 => GPIO15OUTCFG_A::TS,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        **self == GPIO15OUTCFG_A::DIS
    }
    #[doc = "Checks if the value of the field is `PUSHPULL`"]
    #[inline(always)]
    pub fn is_pushpull(&self) -> bool {
        **self == GPIO15OUTCFG_A::PUSHPULL
    }
    #[doc = "Checks if the value of the field is `OD`"]
    #[inline(always)]
    pub fn is_od(&self) -> bool {
        **self == GPIO15OUTCFG_A::OD
    }
    #[doc = "Checks if the value of the field is `TS`"]
    #[inline(always)]
    pub fn is_ts(&self) -> bool {
        **self == GPIO15OUTCFG_A::TS
    }
}
impl core::ops::Deref for GPIO15OUTCFG_R {
    type Target = crate::FieldReader<u8, GPIO15OUTCFG_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIO15OUTCFG` writer - GPIO15 output configuration."]
pub struct GPIO15OUTCFG_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO15OUTCFG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIO15OUTCFG_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "FNCSEL = 0x3 - Output disabled value."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(GPIO15OUTCFG_A::DIS)
    }
    #[doc = "FNCSEL = 0x3 - Output is push-pull value."]
    #[inline(always)]
    pub fn pushpull(self) -> &'a mut W {
        self.variant(GPIO15OUTCFG_A::PUSHPULL)
    }
    #[doc = "FNCSEL = 0x3 - Output is open drain value."]
    #[inline(always)]
    pub fn od(self) -> &'a mut W {
        self.variant(GPIO15OUTCFG_A::OD)
    }
    #[doc = "FNCSEL = 0x3 - Output is tri-state value."]
    #[inline(always)]
    pub fn ts(self) -> &'a mut W {
        self.variant(GPIO15OUTCFG_A::TS)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 29)) | ((value as u32 & 0x03) << 29);
        self.w
    }
}
#[doc = "GPIO15 input enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO15INCFG_A {
    #[doc = "0: Read the GPIO pin data value."]
    READ = 0,
    #[doc = "1: INTD = 0 - Readback will always be zero value."]
    RDZERO = 1,
}
impl From<GPIO15INCFG_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO15INCFG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIO15INCFG` reader - GPIO15 input enable."]
pub struct GPIO15INCFG_R(crate::FieldReader<bool, GPIO15INCFG_A>);
impl GPIO15INCFG_R {
    pub(crate) fn new(bits: bool) -> Self {
        GPIO15INCFG_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO15INCFG_A {
        match self.bits {
            false => GPIO15INCFG_A::READ,
            true => GPIO15INCFG_A::RDZERO,
        }
    }
    #[doc = "Checks if the value of the field is `READ`"]
    #[inline(always)]
    pub fn is_read(&self) -> bool {
        **self == GPIO15INCFG_A::READ
    }
    #[doc = "Checks if the value of the field is `RDZERO`"]
    #[inline(always)]
    pub fn is_rdzero(&self) -> bool {
        **self == GPIO15INCFG_A::RDZERO
    }
}
impl core::ops::Deref for GPIO15INCFG_R {
    type Target = crate::FieldReader<bool, GPIO15INCFG_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIO15INCFG` writer - GPIO15 input enable."]
pub struct GPIO15INCFG_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO15INCFG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIO15INCFG_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Read the GPIO pin data value."]
    #[inline(always)]
    pub fn read(self) -> &'a mut W {
        self.variant(GPIO15INCFG_A::READ)
    }
    #[doc = "INTD = 0 - Readback will always be zero value."]
    #[inline(always)]
    pub fn rdzero(self) -> &'a mut W {
        self.variant(GPIO15INCFG_A::RDZERO)
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
#[doc = "GPIO14 interrupt direction.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO14INTD_A {
    #[doc = "0: FNCSEL = 0x1 - nCE polarity active low value."]
    NCELOW = 0,
    #[doc = "1: FNCSEL = 0x1 - nCE polarity active high value."]
    NCEHIGH = 1,
}
impl From<GPIO14INTD_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO14INTD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIO14INTD` reader - GPIO14 interrupt direction."]
pub struct GPIO14INTD_R(crate::FieldReader<bool, GPIO14INTD_A>);
impl GPIO14INTD_R {
    pub(crate) fn new(bits: bool) -> Self {
        GPIO14INTD_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO14INTD_A {
        match self.bits {
            false => GPIO14INTD_A::NCELOW,
            true => GPIO14INTD_A::NCEHIGH,
        }
    }
    #[doc = "Checks if the value of the field is `NCELOW`"]
    #[inline(always)]
    pub fn is_n_celow(&self) -> bool {
        **self == GPIO14INTD_A::NCELOW
    }
    #[doc = "Checks if the value of the field is `NCEHIGH`"]
    #[inline(always)]
    pub fn is_n_cehigh(&self) -> bool {
        **self == GPIO14INTD_A::NCEHIGH
    }
}
impl core::ops::Deref for GPIO14INTD_R {
    type Target = crate::FieldReader<bool, GPIO14INTD_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIO14INTD` writer - GPIO14 interrupt direction."]
pub struct GPIO14INTD_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO14INTD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIO14INTD_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "FNCSEL = 0x1 - nCE polarity active low value."]
    #[inline(always)]
    pub fn n_celow(self) -> &'a mut W {
        self.variant(GPIO14INTD_A::NCELOW)
    }
    #[doc = "FNCSEL = 0x1 - nCE polarity active high value."]
    #[inline(always)]
    pub fn n_cehigh(self) -> &'a mut W {
        self.variant(GPIO14INTD_A::NCEHIGH)
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
#[doc = "GPIO14 output configuration.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum GPIO14OUTCFG_A {
    #[doc = "0: FNCSEL = 0x3 - Output disabled value."]
    DIS = 0,
    #[doc = "1: FNCSEL = 0x3 - Output is push-pull value."]
    PUSHPULL = 1,
    #[doc = "2: FNCSEL = 0x3 - Output is open drain value."]
    OD = 2,
    #[doc = "3: FNCSEL = 0x3 - Output is tri-state value."]
    TS = 3,
}
impl From<GPIO14OUTCFG_A> for u8 {
    #[inline(always)]
    fn from(variant: GPIO14OUTCFG_A) -> Self {
        variant as _
    }
}
#[doc = "Field `GPIO14OUTCFG` reader - GPIO14 output configuration."]
pub struct GPIO14OUTCFG_R(crate::FieldReader<u8, GPIO14OUTCFG_A>);
impl GPIO14OUTCFG_R {
    pub(crate) fn new(bits: u8) -> Self {
        GPIO14OUTCFG_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO14OUTCFG_A {
        match self.bits {
            0 => GPIO14OUTCFG_A::DIS,
            1 => GPIO14OUTCFG_A::PUSHPULL,
            2 => GPIO14OUTCFG_A::OD,
            3 => GPIO14OUTCFG_A::TS,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        **self == GPIO14OUTCFG_A::DIS
    }
    #[doc = "Checks if the value of the field is `PUSHPULL`"]
    #[inline(always)]
    pub fn is_pushpull(&self) -> bool {
        **self == GPIO14OUTCFG_A::PUSHPULL
    }
    #[doc = "Checks if the value of the field is `OD`"]
    #[inline(always)]
    pub fn is_od(&self) -> bool {
        **self == GPIO14OUTCFG_A::OD
    }
    #[doc = "Checks if the value of the field is `TS`"]
    #[inline(always)]
    pub fn is_ts(&self) -> bool {
        **self == GPIO14OUTCFG_A::TS
    }
}
impl core::ops::Deref for GPIO14OUTCFG_R {
    type Target = crate::FieldReader<u8, GPIO14OUTCFG_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIO14OUTCFG` writer - GPIO14 output configuration."]
pub struct GPIO14OUTCFG_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO14OUTCFG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIO14OUTCFG_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "FNCSEL = 0x3 - Output disabled value."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(GPIO14OUTCFG_A::DIS)
    }
    #[doc = "FNCSEL = 0x3 - Output is push-pull value."]
    #[inline(always)]
    pub fn pushpull(self) -> &'a mut W {
        self.variant(GPIO14OUTCFG_A::PUSHPULL)
    }
    #[doc = "FNCSEL = 0x3 - Output is open drain value."]
    #[inline(always)]
    pub fn od(self) -> &'a mut W {
        self.variant(GPIO14OUTCFG_A::OD)
    }
    #[doc = "FNCSEL = 0x3 - Output is tri-state value."]
    #[inline(always)]
    pub fn ts(self) -> &'a mut W {
        self.variant(GPIO14OUTCFG_A::TS)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 25)) | ((value as u32 & 0x03) << 25);
        self.w
    }
}
#[doc = "GPIO14 input enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO14INCFG_A {
    #[doc = "0: Read the GPIO pin data value."]
    READ = 0,
    #[doc = "1: INTD = 0 - Readback will always be zero value."]
    RDZERO = 1,
}
impl From<GPIO14INCFG_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO14INCFG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIO14INCFG` reader - GPIO14 input enable."]
pub struct GPIO14INCFG_R(crate::FieldReader<bool, GPIO14INCFG_A>);
impl GPIO14INCFG_R {
    pub(crate) fn new(bits: bool) -> Self {
        GPIO14INCFG_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO14INCFG_A {
        match self.bits {
            false => GPIO14INCFG_A::READ,
            true => GPIO14INCFG_A::RDZERO,
        }
    }
    #[doc = "Checks if the value of the field is `READ`"]
    #[inline(always)]
    pub fn is_read(&self) -> bool {
        **self == GPIO14INCFG_A::READ
    }
    #[doc = "Checks if the value of the field is `RDZERO`"]
    #[inline(always)]
    pub fn is_rdzero(&self) -> bool {
        **self == GPIO14INCFG_A::RDZERO
    }
}
impl core::ops::Deref for GPIO14INCFG_R {
    type Target = crate::FieldReader<bool, GPIO14INCFG_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIO14INCFG` writer - GPIO14 input enable."]
pub struct GPIO14INCFG_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO14INCFG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIO14INCFG_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Read the GPIO pin data value."]
    #[inline(always)]
    pub fn read(self) -> &'a mut W {
        self.variant(GPIO14INCFG_A::READ)
    }
    #[doc = "INTD = 0 - Readback will always be zero value."]
    #[inline(always)]
    pub fn rdzero(self) -> &'a mut W {
        self.variant(GPIO14INCFG_A::RDZERO)
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
#[doc = "GPIO13 interrupt direction.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO13INTD_A {
    #[doc = "0: FNCSEL = 0x1 - nCE polarity active low value."]
    NCELOW = 0,
    #[doc = "1: FNCSEL = 0x1 - nCE polarity active high value."]
    NCEHIGH = 1,
}
impl From<GPIO13INTD_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO13INTD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIO13INTD` reader - GPIO13 interrupt direction."]
pub struct GPIO13INTD_R(crate::FieldReader<bool, GPIO13INTD_A>);
impl GPIO13INTD_R {
    pub(crate) fn new(bits: bool) -> Self {
        GPIO13INTD_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO13INTD_A {
        match self.bits {
            false => GPIO13INTD_A::NCELOW,
            true => GPIO13INTD_A::NCEHIGH,
        }
    }
    #[doc = "Checks if the value of the field is `NCELOW`"]
    #[inline(always)]
    pub fn is_n_celow(&self) -> bool {
        **self == GPIO13INTD_A::NCELOW
    }
    #[doc = "Checks if the value of the field is `NCEHIGH`"]
    #[inline(always)]
    pub fn is_n_cehigh(&self) -> bool {
        **self == GPIO13INTD_A::NCEHIGH
    }
}
impl core::ops::Deref for GPIO13INTD_R {
    type Target = crate::FieldReader<bool, GPIO13INTD_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIO13INTD` writer - GPIO13 interrupt direction."]
pub struct GPIO13INTD_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO13INTD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIO13INTD_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "FNCSEL = 0x1 - nCE polarity active low value."]
    #[inline(always)]
    pub fn n_celow(self) -> &'a mut W {
        self.variant(GPIO13INTD_A::NCELOW)
    }
    #[doc = "FNCSEL = 0x1 - nCE polarity active high value."]
    #[inline(always)]
    pub fn n_cehigh(self) -> &'a mut W {
        self.variant(GPIO13INTD_A::NCEHIGH)
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
#[doc = "GPIO13 output configuration.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum GPIO13OUTCFG_A {
    #[doc = "0: FNCSEL = 0x3 - Output disabled value."]
    DIS = 0,
    #[doc = "1: FNCSEL = 0x3 - Output is push-pull value."]
    PUSHPULL = 1,
    #[doc = "2: FNCSEL = 0x3 - Output is open drain value."]
    OD = 2,
    #[doc = "3: FNCSEL = 0x3 - Output is tri-state value."]
    TS = 3,
}
impl From<GPIO13OUTCFG_A> for u8 {
    #[inline(always)]
    fn from(variant: GPIO13OUTCFG_A) -> Self {
        variant as _
    }
}
#[doc = "Field `GPIO13OUTCFG` reader - GPIO13 output configuration."]
pub struct GPIO13OUTCFG_R(crate::FieldReader<u8, GPIO13OUTCFG_A>);
impl GPIO13OUTCFG_R {
    pub(crate) fn new(bits: u8) -> Self {
        GPIO13OUTCFG_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO13OUTCFG_A {
        match self.bits {
            0 => GPIO13OUTCFG_A::DIS,
            1 => GPIO13OUTCFG_A::PUSHPULL,
            2 => GPIO13OUTCFG_A::OD,
            3 => GPIO13OUTCFG_A::TS,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        **self == GPIO13OUTCFG_A::DIS
    }
    #[doc = "Checks if the value of the field is `PUSHPULL`"]
    #[inline(always)]
    pub fn is_pushpull(&self) -> bool {
        **self == GPIO13OUTCFG_A::PUSHPULL
    }
    #[doc = "Checks if the value of the field is `OD`"]
    #[inline(always)]
    pub fn is_od(&self) -> bool {
        **self == GPIO13OUTCFG_A::OD
    }
    #[doc = "Checks if the value of the field is `TS`"]
    #[inline(always)]
    pub fn is_ts(&self) -> bool {
        **self == GPIO13OUTCFG_A::TS
    }
}
impl core::ops::Deref for GPIO13OUTCFG_R {
    type Target = crate::FieldReader<u8, GPIO13OUTCFG_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIO13OUTCFG` writer - GPIO13 output configuration."]
pub struct GPIO13OUTCFG_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO13OUTCFG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIO13OUTCFG_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "FNCSEL = 0x3 - Output disabled value."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(GPIO13OUTCFG_A::DIS)
    }
    #[doc = "FNCSEL = 0x3 - Output is push-pull value."]
    #[inline(always)]
    pub fn pushpull(self) -> &'a mut W {
        self.variant(GPIO13OUTCFG_A::PUSHPULL)
    }
    #[doc = "FNCSEL = 0x3 - Output is open drain value."]
    #[inline(always)]
    pub fn od(self) -> &'a mut W {
        self.variant(GPIO13OUTCFG_A::OD)
    }
    #[doc = "FNCSEL = 0x3 - Output is tri-state value."]
    #[inline(always)]
    pub fn ts(self) -> &'a mut W {
        self.variant(GPIO13OUTCFG_A::TS)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 21)) | ((value as u32 & 0x03) << 21);
        self.w
    }
}
#[doc = "GPIO13 input enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO13INCFG_A {
    #[doc = "0: Read the GPIO pin data value."]
    READ = 0,
    #[doc = "1: INTD = 0 - Readback will always be zero value."]
    RDZERO = 1,
}
impl From<GPIO13INCFG_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO13INCFG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIO13INCFG` reader - GPIO13 input enable."]
pub struct GPIO13INCFG_R(crate::FieldReader<bool, GPIO13INCFG_A>);
impl GPIO13INCFG_R {
    pub(crate) fn new(bits: bool) -> Self {
        GPIO13INCFG_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO13INCFG_A {
        match self.bits {
            false => GPIO13INCFG_A::READ,
            true => GPIO13INCFG_A::RDZERO,
        }
    }
    #[doc = "Checks if the value of the field is `READ`"]
    #[inline(always)]
    pub fn is_read(&self) -> bool {
        **self == GPIO13INCFG_A::READ
    }
    #[doc = "Checks if the value of the field is `RDZERO`"]
    #[inline(always)]
    pub fn is_rdzero(&self) -> bool {
        **self == GPIO13INCFG_A::RDZERO
    }
}
impl core::ops::Deref for GPIO13INCFG_R {
    type Target = crate::FieldReader<bool, GPIO13INCFG_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIO13INCFG` writer - GPIO13 input enable."]
pub struct GPIO13INCFG_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO13INCFG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIO13INCFG_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Read the GPIO pin data value."]
    #[inline(always)]
    pub fn read(self) -> &'a mut W {
        self.variant(GPIO13INCFG_A::READ)
    }
    #[doc = "INTD = 0 - Readback will always be zero value."]
    #[inline(always)]
    pub fn rdzero(self) -> &'a mut W {
        self.variant(GPIO13INCFG_A::RDZERO)
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
#[doc = "GPIO12 interrupt direction.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO12INTD_A {
    #[doc = "0: FNCSEL = 0x1 - nCE polarity active low value."]
    NCELOW = 0,
    #[doc = "1: FNCSEL = 0x1 - nCE polarity active high value."]
    NCEHIGH = 1,
}
impl From<GPIO12INTD_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO12INTD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIO12INTD` reader - GPIO12 interrupt direction."]
pub struct GPIO12INTD_R(crate::FieldReader<bool, GPIO12INTD_A>);
impl GPIO12INTD_R {
    pub(crate) fn new(bits: bool) -> Self {
        GPIO12INTD_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO12INTD_A {
        match self.bits {
            false => GPIO12INTD_A::NCELOW,
            true => GPIO12INTD_A::NCEHIGH,
        }
    }
    #[doc = "Checks if the value of the field is `NCELOW`"]
    #[inline(always)]
    pub fn is_n_celow(&self) -> bool {
        **self == GPIO12INTD_A::NCELOW
    }
    #[doc = "Checks if the value of the field is `NCEHIGH`"]
    #[inline(always)]
    pub fn is_n_cehigh(&self) -> bool {
        **self == GPIO12INTD_A::NCEHIGH
    }
}
impl core::ops::Deref for GPIO12INTD_R {
    type Target = crate::FieldReader<bool, GPIO12INTD_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIO12INTD` writer - GPIO12 interrupt direction."]
pub struct GPIO12INTD_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO12INTD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIO12INTD_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "FNCSEL = 0x1 - nCE polarity active low value."]
    #[inline(always)]
    pub fn n_celow(self) -> &'a mut W {
        self.variant(GPIO12INTD_A::NCELOW)
    }
    #[doc = "FNCSEL = 0x1 - nCE polarity active high value."]
    #[inline(always)]
    pub fn n_cehigh(self) -> &'a mut W {
        self.variant(GPIO12INTD_A::NCEHIGH)
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
#[doc = "GPIO12 output configuration.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum GPIO12OUTCFG_A {
    #[doc = "0: FNCSEL = 0x3 - Output disabled value."]
    DIS = 0,
    #[doc = "1: FNCSEL = 0x3 - Output is push-pull value."]
    PUSHPULL = 1,
    #[doc = "2: FNCSEL = 0x3 - Output is open drain value."]
    OD = 2,
    #[doc = "3: FNCSEL = 0x3 - Output is tri-state value."]
    TS = 3,
}
impl From<GPIO12OUTCFG_A> for u8 {
    #[inline(always)]
    fn from(variant: GPIO12OUTCFG_A) -> Self {
        variant as _
    }
}
#[doc = "Field `GPIO12OUTCFG` reader - GPIO12 output configuration."]
pub struct GPIO12OUTCFG_R(crate::FieldReader<u8, GPIO12OUTCFG_A>);
impl GPIO12OUTCFG_R {
    pub(crate) fn new(bits: u8) -> Self {
        GPIO12OUTCFG_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO12OUTCFG_A {
        match self.bits {
            0 => GPIO12OUTCFG_A::DIS,
            1 => GPIO12OUTCFG_A::PUSHPULL,
            2 => GPIO12OUTCFG_A::OD,
            3 => GPIO12OUTCFG_A::TS,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        **self == GPIO12OUTCFG_A::DIS
    }
    #[doc = "Checks if the value of the field is `PUSHPULL`"]
    #[inline(always)]
    pub fn is_pushpull(&self) -> bool {
        **self == GPIO12OUTCFG_A::PUSHPULL
    }
    #[doc = "Checks if the value of the field is `OD`"]
    #[inline(always)]
    pub fn is_od(&self) -> bool {
        **self == GPIO12OUTCFG_A::OD
    }
    #[doc = "Checks if the value of the field is `TS`"]
    #[inline(always)]
    pub fn is_ts(&self) -> bool {
        **self == GPIO12OUTCFG_A::TS
    }
}
impl core::ops::Deref for GPIO12OUTCFG_R {
    type Target = crate::FieldReader<u8, GPIO12OUTCFG_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIO12OUTCFG` writer - GPIO12 output configuration."]
pub struct GPIO12OUTCFG_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO12OUTCFG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIO12OUTCFG_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "FNCSEL = 0x3 - Output disabled value."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(GPIO12OUTCFG_A::DIS)
    }
    #[doc = "FNCSEL = 0x3 - Output is push-pull value."]
    #[inline(always)]
    pub fn pushpull(self) -> &'a mut W {
        self.variant(GPIO12OUTCFG_A::PUSHPULL)
    }
    #[doc = "FNCSEL = 0x3 - Output is open drain value."]
    #[inline(always)]
    pub fn od(self) -> &'a mut W {
        self.variant(GPIO12OUTCFG_A::OD)
    }
    #[doc = "FNCSEL = 0x3 - Output is tri-state value."]
    #[inline(always)]
    pub fn ts(self) -> &'a mut W {
        self.variant(GPIO12OUTCFG_A::TS)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 17)) | ((value as u32 & 0x03) << 17);
        self.w
    }
}
#[doc = "GPIO12 input enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO12INCFG_A {
    #[doc = "0: Read the GPIO pin data value."]
    READ = 0,
    #[doc = "1: INTD = 0 - Readback will always be zero value."]
    RDZERO = 1,
}
impl From<GPIO12INCFG_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO12INCFG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIO12INCFG` reader - GPIO12 input enable."]
pub struct GPIO12INCFG_R(crate::FieldReader<bool, GPIO12INCFG_A>);
impl GPIO12INCFG_R {
    pub(crate) fn new(bits: bool) -> Self {
        GPIO12INCFG_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO12INCFG_A {
        match self.bits {
            false => GPIO12INCFG_A::READ,
            true => GPIO12INCFG_A::RDZERO,
        }
    }
    #[doc = "Checks if the value of the field is `READ`"]
    #[inline(always)]
    pub fn is_read(&self) -> bool {
        **self == GPIO12INCFG_A::READ
    }
    #[doc = "Checks if the value of the field is `RDZERO`"]
    #[inline(always)]
    pub fn is_rdzero(&self) -> bool {
        **self == GPIO12INCFG_A::RDZERO
    }
}
impl core::ops::Deref for GPIO12INCFG_R {
    type Target = crate::FieldReader<bool, GPIO12INCFG_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIO12INCFG` writer - GPIO12 input enable."]
pub struct GPIO12INCFG_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO12INCFG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIO12INCFG_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Read the GPIO pin data value."]
    #[inline(always)]
    pub fn read(self) -> &'a mut W {
        self.variant(GPIO12INCFG_A::READ)
    }
    #[doc = "INTD = 0 - Readback will always be zero value."]
    #[inline(always)]
    pub fn rdzero(self) -> &'a mut W {
        self.variant(GPIO12INCFG_A::RDZERO)
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
#[doc = "GPIO11 interrupt direction.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO11INTD_A {
    #[doc = "0: FNCSEL = 0x1 - nCE polarity active low value."]
    NCELOW = 0,
    #[doc = "1: FNCSEL = 0x1 - nCE polarity active high value."]
    NCEHIGH = 1,
}
impl From<GPIO11INTD_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO11INTD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIO11INTD` reader - GPIO11 interrupt direction."]
pub struct GPIO11INTD_R(crate::FieldReader<bool, GPIO11INTD_A>);
impl GPIO11INTD_R {
    pub(crate) fn new(bits: bool) -> Self {
        GPIO11INTD_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO11INTD_A {
        match self.bits {
            false => GPIO11INTD_A::NCELOW,
            true => GPIO11INTD_A::NCEHIGH,
        }
    }
    #[doc = "Checks if the value of the field is `NCELOW`"]
    #[inline(always)]
    pub fn is_n_celow(&self) -> bool {
        **self == GPIO11INTD_A::NCELOW
    }
    #[doc = "Checks if the value of the field is `NCEHIGH`"]
    #[inline(always)]
    pub fn is_n_cehigh(&self) -> bool {
        **self == GPIO11INTD_A::NCEHIGH
    }
}
impl core::ops::Deref for GPIO11INTD_R {
    type Target = crate::FieldReader<bool, GPIO11INTD_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIO11INTD` writer - GPIO11 interrupt direction."]
pub struct GPIO11INTD_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO11INTD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIO11INTD_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "FNCSEL = 0x1 - nCE polarity active low value."]
    #[inline(always)]
    pub fn n_celow(self) -> &'a mut W {
        self.variant(GPIO11INTD_A::NCELOW)
    }
    #[doc = "FNCSEL = 0x1 - nCE polarity active high value."]
    #[inline(always)]
    pub fn n_cehigh(self) -> &'a mut W {
        self.variant(GPIO11INTD_A::NCEHIGH)
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
#[doc = "GPIO11 output configuration.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum GPIO11OUTCFG_A {
    #[doc = "0: FNCSEL = 0x3 - Output disabled value."]
    DIS = 0,
    #[doc = "1: FNCSEL = 0x3 - Output is push-pull value."]
    PUSHPULL = 1,
    #[doc = "2: FNCSEL = 0x3 - Output is open drain value."]
    OD = 2,
    #[doc = "3: FNCSEL = 0x3 - Output is tri-state value."]
    TS = 3,
}
impl From<GPIO11OUTCFG_A> for u8 {
    #[inline(always)]
    fn from(variant: GPIO11OUTCFG_A) -> Self {
        variant as _
    }
}
#[doc = "Field `GPIO11OUTCFG` reader - GPIO11 output configuration."]
pub struct GPIO11OUTCFG_R(crate::FieldReader<u8, GPIO11OUTCFG_A>);
impl GPIO11OUTCFG_R {
    pub(crate) fn new(bits: u8) -> Self {
        GPIO11OUTCFG_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO11OUTCFG_A {
        match self.bits {
            0 => GPIO11OUTCFG_A::DIS,
            1 => GPIO11OUTCFG_A::PUSHPULL,
            2 => GPIO11OUTCFG_A::OD,
            3 => GPIO11OUTCFG_A::TS,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        **self == GPIO11OUTCFG_A::DIS
    }
    #[doc = "Checks if the value of the field is `PUSHPULL`"]
    #[inline(always)]
    pub fn is_pushpull(&self) -> bool {
        **self == GPIO11OUTCFG_A::PUSHPULL
    }
    #[doc = "Checks if the value of the field is `OD`"]
    #[inline(always)]
    pub fn is_od(&self) -> bool {
        **self == GPIO11OUTCFG_A::OD
    }
    #[doc = "Checks if the value of the field is `TS`"]
    #[inline(always)]
    pub fn is_ts(&self) -> bool {
        **self == GPIO11OUTCFG_A::TS
    }
}
impl core::ops::Deref for GPIO11OUTCFG_R {
    type Target = crate::FieldReader<u8, GPIO11OUTCFG_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIO11OUTCFG` writer - GPIO11 output configuration."]
pub struct GPIO11OUTCFG_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO11OUTCFG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIO11OUTCFG_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "FNCSEL = 0x3 - Output disabled value."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(GPIO11OUTCFG_A::DIS)
    }
    #[doc = "FNCSEL = 0x3 - Output is push-pull value."]
    #[inline(always)]
    pub fn pushpull(self) -> &'a mut W {
        self.variant(GPIO11OUTCFG_A::PUSHPULL)
    }
    #[doc = "FNCSEL = 0x3 - Output is open drain value."]
    #[inline(always)]
    pub fn od(self) -> &'a mut W {
        self.variant(GPIO11OUTCFG_A::OD)
    }
    #[doc = "FNCSEL = 0x3 - Output is tri-state value."]
    #[inline(always)]
    pub fn ts(self) -> &'a mut W {
        self.variant(GPIO11OUTCFG_A::TS)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 13)) | ((value as u32 & 0x03) << 13);
        self.w
    }
}
#[doc = "GPIO11 input enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO11INCFG_A {
    #[doc = "0: Read the GPIO pin data value."]
    READ = 0,
    #[doc = "1: INTD = 0 - Readback will always be zero value."]
    RDZERO = 1,
}
impl From<GPIO11INCFG_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO11INCFG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIO11INCFG` reader - GPIO11 input enable."]
pub struct GPIO11INCFG_R(crate::FieldReader<bool, GPIO11INCFG_A>);
impl GPIO11INCFG_R {
    pub(crate) fn new(bits: bool) -> Self {
        GPIO11INCFG_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO11INCFG_A {
        match self.bits {
            false => GPIO11INCFG_A::READ,
            true => GPIO11INCFG_A::RDZERO,
        }
    }
    #[doc = "Checks if the value of the field is `READ`"]
    #[inline(always)]
    pub fn is_read(&self) -> bool {
        **self == GPIO11INCFG_A::READ
    }
    #[doc = "Checks if the value of the field is `RDZERO`"]
    #[inline(always)]
    pub fn is_rdzero(&self) -> bool {
        **self == GPIO11INCFG_A::RDZERO
    }
}
impl core::ops::Deref for GPIO11INCFG_R {
    type Target = crate::FieldReader<bool, GPIO11INCFG_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIO11INCFG` writer - GPIO11 input enable."]
pub struct GPIO11INCFG_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO11INCFG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIO11INCFG_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Read the GPIO pin data value."]
    #[inline(always)]
    pub fn read(self) -> &'a mut W {
        self.variant(GPIO11INCFG_A::READ)
    }
    #[doc = "INTD = 0 - Readback will always be zero value."]
    #[inline(always)]
    pub fn rdzero(self) -> &'a mut W {
        self.variant(GPIO11INCFG_A::RDZERO)
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
#[doc = "GPIO10 interrupt direction.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO10INTD_A {
    #[doc = "0: FNCSEL = 0x2 - nCE polarity active low value."]
    NCELOW = 0,
    #[doc = "1: FNCSEL = 0x2 - nCE polarity active high value."]
    NCEHIGH = 1,
}
impl From<GPIO10INTD_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO10INTD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIO10INTD` reader - GPIO10 interrupt direction."]
pub struct GPIO10INTD_R(crate::FieldReader<bool, GPIO10INTD_A>);
impl GPIO10INTD_R {
    pub(crate) fn new(bits: bool) -> Self {
        GPIO10INTD_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO10INTD_A {
        match self.bits {
            false => GPIO10INTD_A::NCELOW,
            true => GPIO10INTD_A::NCEHIGH,
        }
    }
    #[doc = "Checks if the value of the field is `NCELOW`"]
    #[inline(always)]
    pub fn is_n_celow(&self) -> bool {
        **self == GPIO10INTD_A::NCELOW
    }
    #[doc = "Checks if the value of the field is `NCEHIGH`"]
    #[inline(always)]
    pub fn is_n_cehigh(&self) -> bool {
        **self == GPIO10INTD_A::NCEHIGH
    }
}
impl core::ops::Deref for GPIO10INTD_R {
    type Target = crate::FieldReader<bool, GPIO10INTD_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIO10INTD` writer - GPIO10 interrupt direction."]
pub struct GPIO10INTD_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO10INTD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIO10INTD_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "FNCSEL = 0x2 - nCE polarity active low value."]
    #[inline(always)]
    pub fn n_celow(self) -> &'a mut W {
        self.variant(GPIO10INTD_A::NCELOW)
    }
    #[doc = "FNCSEL = 0x2 - nCE polarity active high value."]
    #[inline(always)]
    pub fn n_cehigh(self) -> &'a mut W {
        self.variant(GPIO10INTD_A::NCEHIGH)
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
#[doc = "GPIO10 output configuration.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum GPIO10OUTCFG_A {
    #[doc = "0: FNCSEL = 0x3 - Output disabled value."]
    DIS = 0,
    #[doc = "1: FNCSEL = 0x3 - Output is push-pull value."]
    PUSHPULL = 1,
    #[doc = "2: FNCSEL = 0x3 - Output is open drain value."]
    OD = 2,
    #[doc = "3: FNCSEL = 0x3 - Output is tri-state value."]
    TS = 3,
}
impl From<GPIO10OUTCFG_A> for u8 {
    #[inline(always)]
    fn from(variant: GPIO10OUTCFG_A) -> Self {
        variant as _
    }
}
#[doc = "Field `GPIO10OUTCFG` reader - GPIO10 output configuration."]
pub struct GPIO10OUTCFG_R(crate::FieldReader<u8, GPIO10OUTCFG_A>);
impl GPIO10OUTCFG_R {
    pub(crate) fn new(bits: u8) -> Self {
        GPIO10OUTCFG_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO10OUTCFG_A {
        match self.bits {
            0 => GPIO10OUTCFG_A::DIS,
            1 => GPIO10OUTCFG_A::PUSHPULL,
            2 => GPIO10OUTCFG_A::OD,
            3 => GPIO10OUTCFG_A::TS,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        **self == GPIO10OUTCFG_A::DIS
    }
    #[doc = "Checks if the value of the field is `PUSHPULL`"]
    #[inline(always)]
    pub fn is_pushpull(&self) -> bool {
        **self == GPIO10OUTCFG_A::PUSHPULL
    }
    #[doc = "Checks if the value of the field is `OD`"]
    #[inline(always)]
    pub fn is_od(&self) -> bool {
        **self == GPIO10OUTCFG_A::OD
    }
    #[doc = "Checks if the value of the field is `TS`"]
    #[inline(always)]
    pub fn is_ts(&self) -> bool {
        **self == GPIO10OUTCFG_A::TS
    }
}
impl core::ops::Deref for GPIO10OUTCFG_R {
    type Target = crate::FieldReader<u8, GPIO10OUTCFG_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIO10OUTCFG` writer - GPIO10 output configuration."]
pub struct GPIO10OUTCFG_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO10OUTCFG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIO10OUTCFG_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "FNCSEL = 0x3 - Output disabled value."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(GPIO10OUTCFG_A::DIS)
    }
    #[doc = "FNCSEL = 0x3 - Output is push-pull value."]
    #[inline(always)]
    pub fn pushpull(self) -> &'a mut W {
        self.variant(GPIO10OUTCFG_A::PUSHPULL)
    }
    #[doc = "FNCSEL = 0x3 - Output is open drain value."]
    #[inline(always)]
    pub fn od(self) -> &'a mut W {
        self.variant(GPIO10OUTCFG_A::OD)
    }
    #[doc = "FNCSEL = 0x3 - Output is tri-state value."]
    #[inline(always)]
    pub fn ts(self) -> &'a mut W {
        self.variant(GPIO10OUTCFG_A::TS)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 9)) | ((value as u32 & 0x03) << 9);
        self.w
    }
}
#[doc = "GPIO10 input enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO10INCFG_A {
    #[doc = "0: Read the GPIO pin data value."]
    READ = 0,
    #[doc = "1: INTD = 0 - Readback will always be zero value."]
    RDZERO = 1,
}
impl From<GPIO10INCFG_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO10INCFG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIO10INCFG` reader - GPIO10 input enable."]
pub struct GPIO10INCFG_R(crate::FieldReader<bool, GPIO10INCFG_A>);
impl GPIO10INCFG_R {
    pub(crate) fn new(bits: bool) -> Self {
        GPIO10INCFG_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO10INCFG_A {
        match self.bits {
            false => GPIO10INCFG_A::READ,
            true => GPIO10INCFG_A::RDZERO,
        }
    }
    #[doc = "Checks if the value of the field is `READ`"]
    #[inline(always)]
    pub fn is_read(&self) -> bool {
        **self == GPIO10INCFG_A::READ
    }
    #[doc = "Checks if the value of the field is `RDZERO`"]
    #[inline(always)]
    pub fn is_rdzero(&self) -> bool {
        **self == GPIO10INCFG_A::RDZERO
    }
}
impl core::ops::Deref for GPIO10INCFG_R {
    type Target = crate::FieldReader<bool, GPIO10INCFG_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIO10INCFG` writer - GPIO10 input enable."]
pub struct GPIO10INCFG_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO10INCFG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIO10INCFG_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Read the GPIO pin data value."]
    #[inline(always)]
    pub fn read(self) -> &'a mut W {
        self.variant(GPIO10INCFG_A::READ)
    }
    #[doc = "INTD = 0 - Readback will always be zero value."]
    #[inline(always)]
    pub fn rdzero(self) -> &'a mut W {
        self.variant(GPIO10INCFG_A::RDZERO)
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
#[doc = "GPIO9 interrupt direction.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO9INTD_A {
    #[doc = "0: FNCSEL = 0x2 - nCE polarity active low value."]
    NCELOW = 0,
    #[doc = "1: FNCSEL = 0x2 - nCE polarity active high value."]
    NCEHIGH = 1,
}
impl From<GPIO9INTD_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO9INTD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIO9INTD` reader - GPIO9 interrupt direction."]
pub struct GPIO9INTD_R(crate::FieldReader<bool, GPIO9INTD_A>);
impl GPIO9INTD_R {
    pub(crate) fn new(bits: bool) -> Self {
        GPIO9INTD_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO9INTD_A {
        match self.bits {
            false => GPIO9INTD_A::NCELOW,
            true => GPIO9INTD_A::NCEHIGH,
        }
    }
    #[doc = "Checks if the value of the field is `NCELOW`"]
    #[inline(always)]
    pub fn is_n_celow(&self) -> bool {
        **self == GPIO9INTD_A::NCELOW
    }
    #[doc = "Checks if the value of the field is `NCEHIGH`"]
    #[inline(always)]
    pub fn is_n_cehigh(&self) -> bool {
        **self == GPIO9INTD_A::NCEHIGH
    }
}
impl core::ops::Deref for GPIO9INTD_R {
    type Target = crate::FieldReader<bool, GPIO9INTD_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIO9INTD` writer - GPIO9 interrupt direction."]
pub struct GPIO9INTD_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO9INTD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIO9INTD_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "FNCSEL = 0x2 - nCE polarity active low value."]
    #[inline(always)]
    pub fn n_celow(self) -> &'a mut W {
        self.variant(GPIO9INTD_A::NCELOW)
    }
    #[doc = "FNCSEL = 0x2 - nCE polarity active high value."]
    #[inline(always)]
    pub fn n_cehigh(self) -> &'a mut W {
        self.variant(GPIO9INTD_A::NCEHIGH)
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
#[doc = "GPIO9 output configuration.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum GPIO9OUTCFG_A {
    #[doc = "0: FNCSEL = 0x3 - Output disabled value."]
    DIS = 0,
    #[doc = "1: FNCSEL = 0x3 - Output is push-pull value."]
    PUSHPULL = 1,
    #[doc = "2: FNCSEL = 0x3 - Output is open drain value."]
    OD = 2,
    #[doc = "3: FNCSEL = 0x3 - Output is tri-state value."]
    TS = 3,
}
impl From<GPIO9OUTCFG_A> for u8 {
    #[inline(always)]
    fn from(variant: GPIO9OUTCFG_A) -> Self {
        variant as _
    }
}
#[doc = "Field `GPIO9OUTCFG` reader - GPIO9 output configuration."]
pub struct GPIO9OUTCFG_R(crate::FieldReader<u8, GPIO9OUTCFG_A>);
impl GPIO9OUTCFG_R {
    pub(crate) fn new(bits: u8) -> Self {
        GPIO9OUTCFG_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO9OUTCFG_A {
        match self.bits {
            0 => GPIO9OUTCFG_A::DIS,
            1 => GPIO9OUTCFG_A::PUSHPULL,
            2 => GPIO9OUTCFG_A::OD,
            3 => GPIO9OUTCFG_A::TS,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        **self == GPIO9OUTCFG_A::DIS
    }
    #[doc = "Checks if the value of the field is `PUSHPULL`"]
    #[inline(always)]
    pub fn is_pushpull(&self) -> bool {
        **self == GPIO9OUTCFG_A::PUSHPULL
    }
    #[doc = "Checks if the value of the field is `OD`"]
    #[inline(always)]
    pub fn is_od(&self) -> bool {
        **self == GPIO9OUTCFG_A::OD
    }
    #[doc = "Checks if the value of the field is `TS`"]
    #[inline(always)]
    pub fn is_ts(&self) -> bool {
        **self == GPIO9OUTCFG_A::TS
    }
}
impl core::ops::Deref for GPIO9OUTCFG_R {
    type Target = crate::FieldReader<u8, GPIO9OUTCFG_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIO9OUTCFG` writer - GPIO9 output configuration."]
pub struct GPIO9OUTCFG_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO9OUTCFG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIO9OUTCFG_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "FNCSEL = 0x3 - Output disabled value."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(GPIO9OUTCFG_A::DIS)
    }
    #[doc = "FNCSEL = 0x3 - Output is push-pull value."]
    #[inline(always)]
    pub fn pushpull(self) -> &'a mut W {
        self.variant(GPIO9OUTCFG_A::PUSHPULL)
    }
    #[doc = "FNCSEL = 0x3 - Output is open drain value."]
    #[inline(always)]
    pub fn od(self) -> &'a mut W {
        self.variant(GPIO9OUTCFG_A::OD)
    }
    #[doc = "FNCSEL = 0x3 - Output is tri-state value."]
    #[inline(always)]
    pub fn ts(self) -> &'a mut W {
        self.variant(GPIO9OUTCFG_A::TS)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 5)) | ((value as u32 & 0x03) << 5);
        self.w
    }
}
#[doc = "GPIO9 input enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO9INCFG_A {
    #[doc = "0: Read the GPIO pin data value."]
    READ = 0,
    #[doc = "1: INTD = 0 - Readback will always be zero value."]
    RDZERO = 1,
}
impl From<GPIO9INCFG_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO9INCFG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIO9INCFG` reader - GPIO9 input enable."]
pub struct GPIO9INCFG_R(crate::FieldReader<bool, GPIO9INCFG_A>);
impl GPIO9INCFG_R {
    pub(crate) fn new(bits: bool) -> Self {
        GPIO9INCFG_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO9INCFG_A {
        match self.bits {
            false => GPIO9INCFG_A::READ,
            true => GPIO9INCFG_A::RDZERO,
        }
    }
    #[doc = "Checks if the value of the field is `READ`"]
    #[inline(always)]
    pub fn is_read(&self) -> bool {
        **self == GPIO9INCFG_A::READ
    }
    #[doc = "Checks if the value of the field is `RDZERO`"]
    #[inline(always)]
    pub fn is_rdzero(&self) -> bool {
        **self == GPIO9INCFG_A::RDZERO
    }
}
impl core::ops::Deref for GPIO9INCFG_R {
    type Target = crate::FieldReader<bool, GPIO9INCFG_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIO9INCFG` writer - GPIO9 input enable."]
pub struct GPIO9INCFG_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO9INCFG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIO9INCFG_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Read the GPIO pin data value."]
    #[inline(always)]
    pub fn read(self) -> &'a mut W {
        self.variant(GPIO9INCFG_A::READ)
    }
    #[doc = "INTD = 0 - Readback will always be zero value."]
    #[inline(always)]
    pub fn rdzero(self) -> &'a mut W {
        self.variant(GPIO9INCFG_A::RDZERO)
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
#[doc = "GPIO8 interrupt direction.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO8INTD_A {
    #[doc = "0: FNCSEL = 0x2 - nCE polarity active low value."]
    NCELOW = 0,
    #[doc = "1: FNCSEL = 0x2 - nCE polarity active high value."]
    NCEHIGH = 1,
}
impl From<GPIO8INTD_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO8INTD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIO8INTD` reader - GPIO8 interrupt direction."]
pub struct GPIO8INTD_R(crate::FieldReader<bool, GPIO8INTD_A>);
impl GPIO8INTD_R {
    pub(crate) fn new(bits: bool) -> Self {
        GPIO8INTD_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO8INTD_A {
        match self.bits {
            false => GPIO8INTD_A::NCELOW,
            true => GPIO8INTD_A::NCEHIGH,
        }
    }
    #[doc = "Checks if the value of the field is `NCELOW`"]
    #[inline(always)]
    pub fn is_n_celow(&self) -> bool {
        **self == GPIO8INTD_A::NCELOW
    }
    #[doc = "Checks if the value of the field is `NCEHIGH`"]
    #[inline(always)]
    pub fn is_n_cehigh(&self) -> bool {
        **self == GPIO8INTD_A::NCEHIGH
    }
}
impl core::ops::Deref for GPIO8INTD_R {
    type Target = crate::FieldReader<bool, GPIO8INTD_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIO8INTD` writer - GPIO8 interrupt direction."]
pub struct GPIO8INTD_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO8INTD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIO8INTD_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "FNCSEL = 0x2 - nCE polarity active low value."]
    #[inline(always)]
    pub fn n_celow(self) -> &'a mut W {
        self.variant(GPIO8INTD_A::NCELOW)
    }
    #[doc = "FNCSEL = 0x2 - nCE polarity active high value."]
    #[inline(always)]
    pub fn n_cehigh(self) -> &'a mut W {
        self.variant(GPIO8INTD_A::NCEHIGH)
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
#[doc = "GPIO8 output configuration.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum GPIO8OUTCFG_A {
    #[doc = "0: FNCSEL = 0x3 - Output disabled value."]
    DIS = 0,
    #[doc = "1: FNCSEL = 0x3 - Output is push-pull value."]
    PUSHPULL = 1,
    #[doc = "2: FNCSEL = 0x3 - Output is open drain value."]
    OD = 2,
    #[doc = "3: FNCSEL = 0x3 - Output is tri-state value."]
    TS = 3,
}
impl From<GPIO8OUTCFG_A> for u8 {
    #[inline(always)]
    fn from(variant: GPIO8OUTCFG_A) -> Self {
        variant as _
    }
}
#[doc = "Field `GPIO8OUTCFG` reader - GPIO8 output configuration."]
pub struct GPIO8OUTCFG_R(crate::FieldReader<u8, GPIO8OUTCFG_A>);
impl GPIO8OUTCFG_R {
    pub(crate) fn new(bits: u8) -> Self {
        GPIO8OUTCFG_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO8OUTCFG_A {
        match self.bits {
            0 => GPIO8OUTCFG_A::DIS,
            1 => GPIO8OUTCFG_A::PUSHPULL,
            2 => GPIO8OUTCFG_A::OD,
            3 => GPIO8OUTCFG_A::TS,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        **self == GPIO8OUTCFG_A::DIS
    }
    #[doc = "Checks if the value of the field is `PUSHPULL`"]
    #[inline(always)]
    pub fn is_pushpull(&self) -> bool {
        **self == GPIO8OUTCFG_A::PUSHPULL
    }
    #[doc = "Checks if the value of the field is `OD`"]
    #[inline(always)]
    pub fn is_od(&self) -> bool {
        **self == GPIO8OUTCFG_A::OD
    }
    #[doc = "Checks if the value of the field is `TS`"]
    #[inline(always)]
    pub fn is_ts(&self) -> bool {
        **self == GPIO8OUTCFG_A::TS
    }
}
impl core::ops::Deref for GPIO8OUTCFG_R {
    type Target = crate::FieldReader<u8, GPIO8OUTCFG_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIO8OUTCFG` writer - GPIO8 output configuration."]
pub struct GPIO8OUTCFG_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO8OUTCFG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIO8OUTCFG_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "FNCSEL = 0x3 - Output disabled value."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(GPIO8OUTCFG_A::DIS)
    }
    #[doc = "FNCSEL = 0x3 - Output is push-pull value."]
    #[inline(always)]
    pub fn pushpull(self) -> &'a mut W {
        self.variant(GPIO8OUTCFG_A::PUSHPULL)
    }
    #[doc = "FNCSEL = 0x3 - Output is open drain value."]
    #[inline(always)]
    pub fn od(self) -> &'a mut W {
        self.variant(GPIO8OUTCFG_A::OD)
    }
    #[doc = "FNCSEL = 0x3 - Output is tri-state value."]
    #[inline(always)]
    pub fn ts(self) -> &'a mut W {
        self.variant(GPIO8OUTCFG_A::TS)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 1)) | ((value as u32 & 0x03) << 1);
        self.w
    }
}
#[doc = "GPIO8 input enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO8INCFG_A {
    #[doc = "0: Read the GPIO pin data value."]
    READ = 0,
    #[doc = "1: INTD = 0 - Readback will always be zero value."]
    RDZERO = 1,
}
impl From<GPIO8INCFG_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO8INCFG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIO8INCFG` reader - GPIO8 input enable."]
pub struct GPIO8INCFG_R(crate::FieldReader<bool, GPIO8INCFG_A>);
impl GPIO8INCFG_R {
    pub(crate) fn new(bits: bool) -> Self {
        GPIO8INCFG_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO8INCFG_A {
        match self.bits {
            false => GPIO8INCFG_A::READ,
            true => GPIO8INCFG_A::RDZERO,
        }
    }
    #[doc = "Checks if the value of the field is `READ`"]
    #[inline(always)]
    pub fn is_read(&self) -> bool {
        **self == GPIO8INCFG_A::READ
    }
    #[doc = "Checks if the value of the field is `RDZERO`"]
    #[inline(always)]
    pub fn is_rdzero(&self) -> bool {
        **self == GPIO8INCFG_A::RDZERO
    }
}
impl core::ops::Deref for GPIO8INCFG_R {
    type Target = crate::FieldReader<bool, GPIO8INCFG_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIO8INCFG` writer - GPIO8 input enable."]
pub struct GPIO8INCFG_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO8INCFG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIO8INCFG_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Read the GPIO pin data value."]
    #[inline(always)]
    pub fn read(self) -> &'a mut W {
        self.variant(GPIO8INCFG_A::READ)
    }
    #[doc = "INTD = 0 - Readback will always be zero value."]
    #[inline(always)]
    pub fn rdzero(self) -> &'a mut W {
        self.variant(GPIO8INCFG_A::RDZERO)
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
    #[doc = "Bit 31 - GPIO15 interrupt direction."]
    #[inline(always)]
    pub fn gpio15intd(&self) -> GPIO15INTD_R {
        GPIO15INTD_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bits 29:30 - GPIO15 output configuration."]
    #[inline(always)]
    pub fn gpio15outcfg(&self) -> GPIO15OUTCFG_R {
        GPIO15OUTCFG_R::new(((self.bits >> 29) & 0x03) as u8)
    }
    #[doc = "Bit 28 - GPIO15 input enable."]
    #[inline(always)]
    pub fn gpio15incfg(&self) -> GPIO15INCFG_R {
        GPIO15INCFG_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 27 - GPIO14 interrupt direction."]
    #[inline(always)]
    pub fn gpio14intd(&self) -> GPIO14INTD_R {
        GPIO14INTD_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bits 25:26 - GPIO14 output configuration."]
    #[inline(always)]
    pub fn gpio14outcfg(&self) -> GPIO14OUTCFG_R {
        GPIO14OUTCFG_R::new(((self.bits >> 25) & 0x03) as u8)
    }
    #[doc = "Bit 24 - GPIO14 input enable."]
    #[inline(always)]
    pub fn gpio14incfg(&self) -> GPIO14INCFG_R {
        GPIO14INCFG_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 23 - GPIO13 interrupt direction."]
    #[inline(always)]
    pub fn gpio13intd(&self) -> GPIO13INTD_R {
        GPIO13INTD_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bits 21:22 - GPIO13 output configuration."]
    #[inline(always)]
    pub fn gpio13outcfg(&self) -> GPIO13OUTCFG_R {
        GPIO13OUTCFG_R::new(((self.bits >> 21) & 0x03) as u8)
    }
    #[doc = "Bit 20 - GPIO13 input enable."]
    #[inline(always)]
    pub fn gpio13incfg(&self) -> GPIO13INCFG_R {
        GPIO13INCFG_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 19 - GPIO12 interrupt direction."]
    #[inline(always)]
    pub fn gpio12intd(&self) -> GPIO12INTD_R {
        GPIO12INTD_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bits 17:18 - GPIO12 output configuration."]
    #[inline(always)]
    pub fn gpio12outcfg(&self) -> GPIO12OUTCFG_R {
        GPIO12OUTCFG_R::new(((self.bits >> 17) & 0x03) as u8)
    }
    #[doc = "Bit 16 - GPIO12 input enable."]
    #[inline(always)]
    pub fn gpio12incfg(&self) -> GPIO12INCFG_R {
        GPIO12INCFG_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 15 - GPIO11 interrupt direction."]
    #[inline(always)]
    pub fn gpio11intd(&self) -> GPIO11INTD_R {
        GPIO11INTD_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bits 13:14 - GPIO11 output configuration."]
    #[inline(always)]
    pub fn gpio11outcfg(&self) -> GPIO11OUTCFG_R {
        GPIO11OUTCFG_R::new(((self.bits >> 13) & 0x03) as u8)
    }
    #[doc = "Bit 12 - GPIO11 input enable."]
    #[inline(always)]
    pub fn gpio11incfg(&self) -> GPIO11INCFG_R {
        GPIO11INCFG_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11 - GPIO10 interrupt direction."]
    #[inline(always)]
    pub fn gpio10intd(&self) -> GPIO10INTD_R {
        GPIO10INTD_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bits 9:10 - GPIO10 output configuration."]
    #[inline(always)]
    pub fn gpio10outcfg(&self) -> GPIO10OUTCFG_R {
        GPIO10OUTCFG_R::new(((self.bits >> 9) & 0x03) as u8)
    }
    #[doc = "Bit 8 - GPIO10 input enable."]
    #[inline(always)]
    pub fn gpio10incfg(&self) -> GPIO10INCFG_R {
        GPIO10INCFG_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7 - GPIO9 interrupt direction."]
    #[inline(always)]
    pub fn gpio9intd(&self) -> GPIO9INTD_R {
        GPIO9INTD_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 5:6 - GPIO9 output configuration."]
    #[inline(always)]
    pub fn gpio9outcfg(&self) -> GPIO9OUTCFG_R {
        GPIO9OUTCFG_R::new(((self.bits >> 5) & 0x03) as u8)
    }
    #[doc = "Bit 4 - GPIO9 input enable."]
    #[inline(always)]
    pub fn gpio9incfg(&self) -> GPIO9INCFG_R {
        GPIO9INCFG_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - GPIO8 interrupt direction."]
    #[inline(always)]
    pub fn gpio8intd(&self) -> GPIO8INTD_R {
        GPIO8INTD_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bits 1:2 - GPIO8 output configuration."]
    #[inline(always)]
    pub fn gpio8outcfg(&self) -> GPIO8OUTCFG_R {
        GPIO8OUTCFG_R::new(((self.bits >> 1) & 0x03) as u8)
    }
    #[doc = "Bit 0 - GPIO8 input enable."]
    #[inline(always)]
    pub fn gpio8incfg(&self) -> GPIO8INCFG_R {
        GPIO8INCFG_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 31 - GPIO15 interrupt direction."]
    #[inline(always)]
    pub fn gpio15intd(&mut self) -> GPIO15INTD_W {
        GPIO15INTD_W { w: self }
    }
    #[doc = "Bits 29:30 - GPIO15 output configuration."]
    #[inline(always)]
    pub fn gpio15outcfg(&mut self) -> GPIO15OUTCFG_W {
        GPIO15OUTCFG_W { w: self }
    }
    #[doc = "Bit 28 - GPIO15 input enable."]
    #[inline(always)]
    pub fn gpio15incfg(&mut self) -> GPIO15INCFG_W {
        GPIO15INCFG_W { w: self }
    }
    #[doc = "Bit 27 - GPIO14 interrupt direction."]
    #[inline(always)]
    pub fn gpio14intd(&mut self) -> GPIO14INTD_W {
        GPIO14INTD_W { w: self }
    }
    #[doc = "Bits 25:26 - GPIO14 output configuration."]
    #[inline(always)]
    pub fn gpio14outcfg(&mut self) -> GPIO14OUTCFG_W {
        GPIO14OUTCFG_W { w: self }
    }
    #[doc = "Bit 24 - GPIO14 input enable."]
    #[inline(always)]
    pub fn gpio14incfg(&mut self) -> GPIO14INCFG_W {
        GPIO14INCFG_W { w: self }
    }
    #[doc = "Bit 23 - GPIO13 interrupt direction."]
    #[inline(always)]
    pub fn gpio13intd(&mut self) -> GPIO13INTD_W {
        GPIO13INTD_W { w: self }
    }
    #[doc = "Bits 21:22 - GPIO13 output configuration."]
    #[inline(always)]
    pub fn gpio13outcfg(&mut self) -> GPIO13OUTCFG_W {
        GPIO13OUTCFG_W { w: self }
    }
    #[doc = "Bit 20 - GPIO13 input enable."]
    #[inline(always)]
    pub fn gpio13incfg(&mut self) -> GPIO13INCFG_W {
        GPIO13INCFG_W { w: self }
    }
    #[doc = "Bit 19 - GPIO12 interrupt direction."]
    #[inline(always)]
    pub fn gpio12intd(&mut self) -> GPIO12INTD_W {
        GPIO12INTD_W { w: self }
    }
    #[doc = "Bits 17:18 - GPIO12 output configuration."]
    #[inline(always)]
    pub fn gpio12outcfg(&mut self) -> GPIO12OUTCFG_W {
        GPIO12OUTCFG_W { w: self }
    }
    #[doc = "Bit 16 - GPIO12 input enable."]
    #[inline(always)]
    pub fn gpio12incfg(&mut self) -> GPIO12INCFG_W {
        GPIO12INCFG_W { w: self }
    }
    #[doc = "Bit 15 - GPIO11 interrupt direction."]
    #[inline(always)]
    pub fn gpio11intd(&mut self) -> GPIO11INTD_W {
        GPIO11INTD_W { w: self }
    }
    #[doc = "Bits 13:14 - GPIO11 output configuration."]
    #[inline(always)]
    pub fn gpio11outcfg(&mut self) -> GPIO11OUTCFG_W {
        GPIO11OUTCFG_W { w: self }
    }
    #[doc = "Bit 12 - GPIO11 input enable."]
    #[inline(always)]
    pub fn gpio11incfg(&mut self) -> GPIO11INCFG_W {
        GPIO11INCFG_W { w: self }
    }
    #[doc = "Bit 11 - GPIO10 interrupt direction."]
    #[inline(always)]
    pub fn gpio10intd(&mut self) -> GPIO10INTD_W {
        GPIO10INTD_W { w: self }
    }
    #[doc = "Bits 9:10 - GPIO10 output configuration."]
    #[inline(always)]
    pub fn gpio10outcfg(&mut self) -> GPIO10OUTCFG_W {
        GPIO10OUTCFG_W { w: self }
    }
    #[doc = "Bit 8 - GPIO10 input enable."]
    #[inline(always)]
    pub fn gpio10incfg(&mut self) -> GPIO10INCFG_W {
        GPIO10INCFG_W { w: self }
    }
    #[doc = "Bit 7 - GPIO9 interrupt direction."]
    #[inline(always)]
    pub fn gpio9intd(&mut self) -> GPIO9INTD_W {
        GPIO9INTD_W { w: self }
    }
    #[doc = "Bits 5:6 - GPIO9 output configuration."]
    #[inline(always)]
    pub fn gpio9outcfg(&mut self) -> GPIO9OUTCFG_W {
        GPIO9OUTCFG_W { w: self }
    }
    #[doc = "Bit 4 - GPIO9 input enable."]
    #[inline(always)]
    pub fn gpio9incfg(&mut self) -> GPIO9INCFG_W {
        GPIO9INCFG_W { w: self }
    }
    #[doc = "Bit 3 - GPIO8 interrupt direction."]
    #[inline(always)]
    pub fn gpio8intd(&mut self) -> GPIO8INTD_W {
        GPIO8INTD_W { w: self }
    }
    #[doc = "Bits 1:2 - GPIO8 output configuration."]
    #[inline(always)]
    pub fn gpio8outcfg(&mut self) -> GPIO8OUTCFG_W {
        GPIO8OUTCFG_W { w: self }
    }
    #[doc = "Bit 0 - GPIO8 input enable."]
    #[inline(always)]
    pub fn gpio8incfg(&mut self) -> GPIO8INCFG_W {
        GPIO8INCFG_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPIO Configuration Register B (Pads 8-15)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cfgb](index.html) module"]
pub struct CFGB_SPEC;
impl crate::RegisterSpec for CFGB_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cfgb::R](R) reader structure"]
impl crate::Readable for CFGB_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cfgb::W](W) writer structure"]
impl crate::Writable for CFGB_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CFGB to value 0"]
impl crate::Resettable for CFGB_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
