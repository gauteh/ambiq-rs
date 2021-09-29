#[doc = "Register `CFGF` reader"]
pub struct R(crate::R<CFGF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CFGF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CFGF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CFGF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CFGF` writer"]
pub struct W(crate::W<CFGF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CFGF_SPEC>;
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
impl From<crate::W<CFGF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CFGF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "GPIO47 interrupt direction.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO47INTD_A {
    #[doc = "0: FNCSEL = 0x1 - nCE polarity active low value."]
    NCELOW = 0,
    #[doc = "1: FNCSEL = 0x1 - nCE polarity active high value."]
    NCEHIGH = 1,
}
impl From<GPIO47INTD_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO47INTD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIO47INTD` reader - GPIO47 interrupt direction."]
pub struct GPIO47INTD_R(crate::FieldReader<bool, GPIO47INTD_A>);
impl GPIO47INTD_R {
    pub(crate) fn new(bits: bool) -> Self {
        GPIO47INTD_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO47INTD_A {
        match self.bits {
            false => GPIO47INTD_A::NCELOW,
            true => GPIO47INTD_A::NCEHIGH,
        }
    }
    #[doc = "Checks if the value of the field is `NCELOW`"]
    #[inline(always)]
    pub fn is_n_celow(&self) -> bool {
        **self == GPIO47INTD_A::NCELOW
    }
    #[doc = "Checks if the value of the field is `NCEHIGH`"]
    #[inline(always)]
    pub fn is_n_cehigh(&self) -> bool {
        **self == GPIO47INTD_A::NCEHIGH
    }
}
impl core::ops::Deref for GPIO47INTD_R {
    type Target = crate::FieldReader<bool, GPIO47INTD_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIO47INTD` writer - GPIO47 interrupt direction."]
pub struct GPIO47INTD_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO47INTD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIO47INTD_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "FNCSEL = 0x1 - nCE polarity active low value."]
    #[inline(always)]
    pub fn n_celow(self) -> &'a mut W {
        self.variant(GPIO47INTD_A::NCELOW)
    }
    #[doc = "FNCSEL = 0x1 - nCE polarity active high value."]
    #[inline(always)]
    pub fn n_cehigh(self) -> &'a mut W {
        self.variant(GPIO47INTD_A::NCEHIGH)
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
#[doc = "GPIO47 output configuration.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum GPIO47OUTCFG_A {
    #[doc = "0: FNCSEL = 0x3 - Output disabled value."]
    DIS = 0,
    #[doc = "1: FNCSEL = 0x3 - Output is push-pull value."]
    PUSHPULL = 1,
    #[doc = "2: FNCSEL = 0x3 - Output is open drain value."]
    OD = 2,
    #[doc = "3: FNCSEL = 0x3 - Output is tri-state value."]
    TS = 3,
}
impl From<GPIO47OUTCFG_A> for u8 {
    #[inline(always)]
    fn from(variant: GPIO47OUTCFG_A) -> Self {
        variant as _
    }
}
#[doc = "Field `GPIO47OUTCFG` reader - GPIO47 output configuration."]
pub struct GPIO47OUTCFG_R(crate::FieldReader<u8, GPIO47OUTCFG_A>);
impl GPIO47OUTCFG_R {
    pub(crate) fn new(bits: u8) -> Self {
        GPIO47OUTCFG_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO47OUTCFG_A {
        match self.bits {
            0 => GPIO47OUTCFG_A::DIS,
            1 => GPIO47OUTCFG_A::PUSHPULL,
            2 => GPIO47OUTCFG_A::OD,
            3 => GPIO47OUTCFG_A::TS,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        **self == GPIO47OUTCFG_A::DIS
    }
    #[doc = "Checks if the value of the field is `PUSHPULL`"]
    #[inline(always)]
    pub fn is_pushpull(&self) -> bool {
        **self == GPIO47OUTCFG_A::PUSHPULL
    }
    #[doc = "Checks if the value of the field is `OD`"]
    #[inline(always)]
    pub fn is_od(&self) -> bool {
        **self == GPIO47OUTCFG_A::OD
    }
    #[doc = "Checks if the value of the field is `TS`"]
    #[inline(always)]
    pub fn is_ts(&self) -> bool {
        **self == GPIO47OUTCFG_A::TS
    }
}
impl core::ops::Deref for GPIO47OUTCFG_R {
    type Target = crate::FieldReader<u8, GPIO47OUTCFG_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIO47OUTCFG` writer - GPIO47 output configuration."]
pub struct GPIO47OUTCFG_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO47OUTCFG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIO47OUTCFG_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "FNCSEL = 0x3 - Output disabled value."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(GPIO47OUTCFG_A::DIS)
    }
    #[doc = "FNCSEL = 0x3 - Output is push-pull value."]
    #[inline(always)]
    pub fn pushpull(self) -> &'a mut W {
        self.variant(GPIO47OUTCFG_A::PUSHPULL)
    }
    #[doc = "FNCSEL = 0x3 - Output is open drain value."]
    #[inline(always)]
    pub fn od(self) -> &'a mut W {
        self.variant(GPIO47OUTCFG_A::OD)
    }
    #[doc = "FNCSEL = 0x3 - Output is tri-state value."]
    #[inline(always)]
    pub fn ts(self) -> &'a mut W {
        self.variant(GPIO47OUTCFG_A::TS)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 29)) | ((value as u32 & 0x03) << 29);
        self.w
    }
}
#[doc = "GPIO47 input enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO47INCFG_A {
    #[doc = "0: Read the GPIO pin data value."]
    READ = 0,
    #[doc = "1: INTD = 0 - Readback will always be zero value."]
    RDZERO = 1,
}
impl From<GPIO47INCFG_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO47INCFG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIO47INCFG` reader - GPIO47 input enable."]
pub struct GPIO47INCFG_R(crate::FieldReader<bool, GPIO47INCFG_A>);
impl GPIO47INCFG_R {
    pub(crate) fn new(bits: bool) -> Self {
        GPIO47INCFG_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO47INCFG_A {
        match self.bits {
            false => GPIO47INCFG_A::READ,
            true => GPIO47INCFG_A::RDZERO,
        }
    }
    #[doc = "Checks if the value of the field is `READ`"]
    #[inline(always)]
    pub fn is_read(&self) -> bool {
        **self == GPIO47INCFG_A::READ
    }
    #[doc = "Checks if the value of the field is `RDZERO`"]
    #[inline(always)]
    pub fn is_rdzero(&self) -> bool {
        **self == GPIO47INCFG_A::RDZERO
    }
}
impl core::ops::Deref for GPIO47INCFG_R {
    type Target = crate::FieldReader<bool, GPIO47INCFG_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIO47INCFG` writer - GPIO47 input enable."]
pub struct GPIO47INCFG_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO47INCFG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIO47INCFG_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Read the GPIO pin data value."]
    #[inline(always)]
    pub fn read(self) -> &'a mut W {
        self.variant(GPIO47INCFG_A::READ)
    }
    #[doc = "INTD = 0 - Readback will always be zero value."]
    #[inline(always)]
    pub fn rdzero(self) -> &'a mut W {
        self.variant(GPIO47INCFG_A::RDZERO)
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
#[doc = "GPIO46 interrupt direction.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO46INTD_A {
    #[doc = "0: FNCSEL = 0x1 - nCE polarity active low value."]
    NCELOW = 0,
    #[doc = "1: FNCSEL = 0x1 - nCE polarity active high value."]
    NCEHIGH = 1,
}
impl From<GPIO46INTD_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO46INTD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIO46INTD` reader - GPIO46 interrupt direction."]
pub struct GPIO46INTD_R(crate::FieldReader<bool, GPIO46INTD_A>);
impl GPIO46INTD_R {
    pub(crate) fn new(bits: bool) -> Self {
        GPIO46INTD_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO46INTD_A {
        match self.bits {
            false => GPIO46INTD_A::NCELOW,
            true => GPIO46INTD_A::NCEHIGH,
        }
    }
    #[doc = "Checks if the value of the field is `NCELOW`"]
    #[inline(always)]
    pub fn is_n_celow(&self) -> bool {
        **self == GPIO46INTD_A::NCELOW
    }
    #[doc = "Checks if the value of the field is `NCEHIGH`"]
    #[inline(always)]
    pub fn is_n_cehigh(&self) -> bool {
        **self == GPIO46INTD_A::NCEHIGH
    }
}
impl core::ops::Deref for GPIO46INTD_R {
    type Target = crate::FieldReader<bool, GPIO46INTD_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIO46INTD` writer - GPIO46 interrupt direction."]
pub struct GPIO46INTD_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO46INTD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIO46INTD_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "FNCSEL = 0x1 - nCE polarity active low value."]
    #[inline(always)]
    pub fn n_celow(self) -> &'a mut W {
        self.variant(GPIO46INTD_A::NCELOW)
    }
    #[doc = "FNCSEL = 0x1 - nCE polarity active high value."]
    #[inline(always)]
    pub fn n_cehigh(self) -> &'a mut W {
        self.variant(GPIO46INTD_A::NCEHIGH)
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
#[doc = "GPIO46 output configuration.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum GPIO46OUTCFG_A {
    #[doc = "0: FNCSEL = 0x3 - Output disabled value."]
    DIS = 0,
    #[doc = "1: FNCSEL = 0x3 - Output is push-pull value."]
    PUSHPULL = 1,
    #[doc = "2: FNCSEL = 0x3 - Output is open drain value."]
    OD = 2,
    #[doc = "3: FNCSEL = 0x3 - Output is tri-state value."]
    TS = 3,
}
impl From<GPIO46OUTCFG_A> for u8 {
    #[inline(always)]
    fn from(variant: GPIO46OUTCFG_A) -> Self {
        variant as _
    }
}
#[doc = "Field `GPIO46OUTCFG` reader - GPIO46 output configuration."]
pub struct GPIO46OUTCFG_R(crate::FieldReader<u8, GPIO46OUTCFG_A>);
impl GPIO46OUTCFG_R {
    pub(crate) fn new(bits: u8) -> Self {
        GPIO46OUTCFG_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO46OUTCFG_A {
        match self.bits {
            0 => GPIO46OUTCFG_A::DIS,
            1 => GPIO46OUTCFG_A::PUSHPULL,
            2 => GPIO46OUTCFG_A::OD,
            3 => GPIO46OUTCFG_A::TS,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        **self == GPIO46OUTCFG_A::DIS
    }
    #[doc = "Checks if the value of the field is `PUSHPULL`"]
    #[inline(always)]
    pub fn is_pushpull(&self) -> bool {
        **self == GPIO46OUTCFG_A::PUSHPULL
    }
    #[doc = "Checks if the value of the field is `OD`"]
    #[inline(always)]
    pub fn is_od(&self) -> bool {
        **self == GPIO46OUTCFG_A::OD
    }
    #[doc = "Checks if the value of the field is `TS`"]
    #[inline(always)]
    pub fn is_ts(&self) -> bool {
        **self == GPIO46OUTCFG_A::TS
    }
}
impl core::ops::Deref for GPIO46OUTCFG_R {
    type Target = crate::FieldReader<u8, GPIO46OUTCFG_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIO46OUTCFG` writer - GPIO46 output configuration."]
pub struct GPIO46OUTCFG_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO46OUTCFG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIO46OUTCFG_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "FNCSEL = 0x3 - Output disabled value."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(GPIO46OUTCFG_A::DIS)
    }
    #[doc = "FNCSEL = 0x3 - Output is push-pull value."]
    #[inline(always)]
    pub fn pushpull(self) -> &'a mut W {
        self.variant(GPIO46OUTCFG_A::PUSHPULL)
    }
    #[doc = "FNCSEL = 0x3 - Output is open drain value."]
    #[inline(always)]
    pub fn od(self) -> &'a mut W {
        self.variant(GPIO46OUTCFG_A::OD)
    }
    #[doc = "FNCSEL = 0x3 - Output is tri-state value."]
    #[inline(always)]
    pub fn ts(self) -> &'a mut W {
        self.variant(GPIO46OUTCFG_A::TS)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 25)) | ((value as u32 & 0x03) << 25);
        self.w
    }
}
#[doc = "GPIO46 input enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO46INCFG_A {
    #[doc = "0: Read the GPIO pin data value."]
    READ = 0,
    #[doc = "1: INTD = 0 - Readback will always be zero value."]
    RDZERO = 1,
}
impl From<GPIO46INCFG_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO46INCFG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIO46INCFG` reader - GPIO46 input enable."]
pub struct GPIO46INCFG_R(crate::FieldReader<bool, GPIO46INCFG_A>);
impl GPIO46INCFG_R {
    pub(crate) fn new(bits: bool) -> Self {
        GPIO46INCFG_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO46INCFG_A {
        match self.bits {
            false => GPIO46INCFG_A::READ,
            true => GPIO46INCFG_A::RDZERO,
        }
    }
    #[doc = "Checks if the value of the field is `READ`"]
    #[inline(always)]
    pub fn is_read(&self) -> bool {
        **self == GPIO46INCFG_A::READ
    }
    #[doc = "Checks if the value of the field is `RDZERO`"]
    #[inline(always)]
    pub fn is_rdzero(&self) -> bool {
        **self == GPIO46INCFG_A::RDZERO
    }
}
impl core::ops::Deref for GPIO46INCFG_R {
    type Target = crate::FieldReader<bool, GPIO46INCFG_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIO46INCFG` writer - GPIO46 input enable."]
pub struct GPIO46INCFG_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO46INCFG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIO46INCFG_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Read the GPIO pin data value."]
    #[inline(always)]
    pub fn read(self) -> &'a mut W {
        self.variant(GPIO46INCFG_A::READ)
    }
    #[doc = "INTD = 0 - Readback will always be zero value."]
    #[inline(always)]
    pub fn rdzero(self) -> &'a mut W {
        self.variant(GPIO46INCFG_A::RDZERO)
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
#[doc = "GPIO45 interrupt direction.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO45INTD_A {
    #[doc = "0: FNCSEL = 0x1 - nCE polarity active low value."]
    NCELOW = 0,
    #[doc = "1: FNCSEL = 0x1 - nCE polarity active high value."]
    NCEHIGH = 1,
}
impl From<GPIO45INTD_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO45INTD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIO45INTD` reader - GPIO45 interrupt direction."]
pub struct GPIO45INTD_R(crate::FieldReader<bool, GPIO45INTD_A>);
impl GPIO45INTD_R {
    pub(crate) fn new(bits: bool) -> Self {
        GPIO45INTD_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO45INTD_A {
        match self.bits {
            false => GPIO45INTD_A::NCELOW,
            true => GPIO45INTD_A::NCEHIGH,
        }
    }
    #[doc = "Checks if the value of the field is `NCELOW`"]
    #[inline(always)]
    pub fn is_n_celow(&self) -> bool {
        **self == GPIO45INTD_A::NCELOW
    }
    #[doc = "Checks if the value of the field is `NCEHIGH`"]
    #[inline(always)]
    pub fn is_n_cehigh(&self) -> bool {
        **self == GPIO45INTD_A::NCEHIGH
    }
}
impl core::ops::Deref for GPIO45INTD_R {
    type Target = crate::FieldReader<bool, GPIO45INTD_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIO45INTD` writer - GPIO45 interrupt direction."]
pub struct GPIO45INTD_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO45INTD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIO45INTD_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "FNCSEL = 0x1 - nCE polarity active low value."]
    #[inline(always)]
    pub fn n_celow(self) -> &'a mut W {
        self.variant(GPIO45INTD_A::NCELOW)
    }
    #[doc = "FNCSEL = 0x1 - nCE polarity active high value."]
    #[inline(always)]
    pub fn n_cehigh(self) -> &'a mut W {
        self.variant(GPIO45INTD_A::NCEHIGH)
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
#[doc = "GPIO45 output configuration.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum GPIO45OUTCFG_A {
    #[doc = "0: FNCSEL = 0x3 - Output disabled value."]
    DIS = 0,
    #[doc = "1: FNCSEL = 0x3 - Output is push-pull value."]
    PUSHPULL = 1,
    #[doc = "2: FNCSEL = 0x3 - Output is open drain value."]
    OD = 2,
    #[doc = "3: FNCSEL = 0x3 - Output is tri-state value."]
    TS = 3,
}
impl From<GPIO45OUTCFG_A> for u8 {
    #[inline(always)]
    fn from(variant: GPIO45OUTCFG_A) -> Self {
        variant as _
    }
}
#[doc = "Field `GPIO45OUTCFG` reader - GPIO45 output configuration."]
pub struct GPIO45OUTCFG_R(crate::FieldReader<u8, GPIO45OUTCFG_A>);
impl GPIO45OUTCFG_R {
    pub(crate) fn new(bits: u8) -> Self {
        GPIO45OUTCFG_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO45OUTCFG_A {
        match self.bits {
            0 => GPIO45OUTCFG_A::DIS,
            1 => GPIO45OUTCFG_A::PUSHPULL,
            2 => GPIO45OUTCFG_A::OD,
            3 => GPIO45OUTCFG_A::TS,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        **self == GPIO45OUTCFG_A::DIS
    }
    #[doc = "Checks if the value of the field is `PUSHPULL`"]
    #[inline(always)]
    pub fn is_pushpull(&self) -> bool {
        **self == GPIO45OUTCFG_A::PUSHPULL
    }
    #[doc = "Checks if the value of the field is `OD`"]
    #[inline(always)]
    pub fn is_od(&self) -> bool {
        **self == GPIO45OUTCFG_A::OD
    }
    #[doc = "Checks if the value of the field is `TS`"]
    #[inline(always)]
    pub fn is_ts(&self) -> bool {
        **self == GPIO45OUTCFG_A::TS
    }
}
impl core::ops::Deref for GPIO45OUTCFG_R {
    type Target = crate::FieldReader<u8, GPIO45OUTCFG_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIO45OUTCFG` writer - GPIO45 output configuration."]
pub struct GPIO45OUTCFG_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO45OUTCFG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIO45OUTCFG_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "FNCSEL = 0x3 - Output disabled value."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(GPIO45OUTCFG_A::DIS)
    }
    #[doc = "FNCSEL = 0x3 - Output is push-pull value."]
    #[inline(always)]
    pub fn pushpull(self) -> &'a mut W {
        self.variant(GPIO45OUTCFG_A::PUSHPULL)
    }
    #[doc = "FNCSEL = 0x3 - Output is open drain value."]
    #[inline(always)]
    pub fn od(self) -> &'a mut W {
        self.variant(GPIO45OUTCFG_A::OD)
    }
    #[doc = "FNCSEL = 0x3 - Output is tri-state value."]
    #[inline(always)]
    pub fn ts(self) -> &'a mut W {
        self.variant(GPIO45OUTCFG_A::TS)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 21)) | ((value as u32 & 0x03) << 21);
        self.w
    }
}
#[doc = "GPIO45 input enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO45INCFG_A {
    #[doc = "0: Read the GPIO pin data value."]
    READ = 0,
    #[doc = "1: INTD = 0 - Readback will always be zero value."]
    RDZERO = 1,
}
impl From<GPIO45INCFG_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO45INCFG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIO45INCFG` reader - GPIO45 input enable."]
pub struct GPIO45INCFG_R(crate::FieldReader<bool, GPIO45INCFG_A>);
impl GPIO45INCFG_R {
    pub(crate) fn new(bits: bool) -> Self {
        GPIO45INCFG_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO45INCFG_A {
        match self.bits {
            false => GPIO45INCFG_A::READ,
            true => GPIO45INCFG_A::RDZERO,
        }
    }
    #[doc = "Checks if the value of the field is `READ`"]
    #[inline(always)]
    pub fn is_read(&self) -> bool {
        **self == GPIO45INCFG_A::READ
    }
    #[doc = "Checks if the value of the field is `RDZERO`"]
    #[inline(always)]
    pub fn is_rdzero(&self) -> bool {
        **self == GPIO45INCFG_A::RDZERO
    }
}
impl core::ops::Deref for GPIO45INCFG_R {
    type Target = crate::FieldReader<bool, GPIO45INCFG_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIO45INCFG` writer - GPIO45 input enable."]
pub struct GPIO45INCFG_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO45INCFG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIO45INCFG_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Read the GPIO pin data value."]
    #[inline(always)]
    pub fn read(self) -> &'a mut W {
        self.variant(GPIO45INCFG_A::READ)
    }
    #[doc = "INTD = 0 - Readback will always be zero value."]
    #[inline(always)]
    pub fn rdzero(self) -> &'a mut W {
        self.variant(GPIO45INCFG_A::RDZERO)
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
#[doc = "GPIO44 interrupt direction.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO44INTD_A {
    #[doc = "0: FNCSEL = 0x1 - nCE polarity active low value."]
    NCELOW = 0,
    #[doc = "1: FNCSEL = 0x1 - nCE polarity active high value."]
    NCEHIGH = 1,
}
impl From<GPIO44INTD_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO44INTD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIO44INTD` reader - GPIO44 interrupt direction."]
pub struct GPIO44INTD_R(crate::FieldReader<bool, GPIO44INTD_A>);
impl GPIO44INTD_R {
    pub(crate) fn new(bits: bool) -> Self {
        GPIO44INTD_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO44INTD_A {
        match self.bits {
            false => GPIO44INTD_A::NCELOW,
            true => GPIO44INTD_A::NCEHIGH,
        }
    }
    #[doc = "Checks if the value of the field is `NCELOW`"]
    #[inline(always)]
    pub fn is_n_celow(&self) -> bool {
        **self == GPIO44INTD_A::NCELOW
    }
    #[doc = "Checks if the value of the field is `NCEHIGH`"]
    #[inline(always)]
    pub fn is_n_cehigh(&self) -> bool {
        **self == GPIO44INTD_A::NCEHIGH
    }
}
impl core::ops::Deref for GPIO44INTD_R {
    type Target = crate::FieldReader<bool, GPIO44INTD_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIO44INTD` writer - GPIO44 interrupt direction."]
pub struct GPIO44INTD_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO44INTD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIO44INTD_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "FNCSEL = 0x1 - nCE polarity active low value."]
    #[inline(always)]
    pub fn n_celow(self) -> &'a mut W {
        self.variant(GPIO44INTD_A::NCELOW)
    }
    #[doc = "FNCSEL = 0x1 - nCE polarity active high value."]
    #[inline(always)]
    pub fn n_cehigh(self) -> &'a mut W {
        self.variant(GPIO44INTD_A::NCEHIGH)
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
#[doc = "GPIO44 output configuration.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum GPIO44OUTCFG_A {
    #[doc = "0: FNCSEL = 0x3 - Output disabled value."]
    DIS = 0,
    #[doc = "1: FNCSEL = 0x3 - Output is push-pull value."]
    PUSHPULL = 1,
    #[doc = "2: FNCSEL = 0x3 - Output is open drain value."]
    OD = 2,
    #[doc = "3: FNCSEL = 0x3 - Output is tri-state value."]
    TS = 3,
}
impl From<GPIO44OUTCFG_A> for u8 {
    #[inline(always)]
    fn from(variant: GPIO44OUTCFG_A) -> Self {
        variant as _
    }
}
#[doc = "Field `GPIO44OUTCFG` reader - GPIO44 output configuration."]
pub struct GPIO44OUTCFG_R(crate::FieldReader<u8, GPIO44OUTCFG_A>);
impl GPIO44OUTCFG_R {
    pub(crate) fn new(bits: u8) -> Self {
        GPIO44OUTCFG_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO44OUTCFG_A {
        match self.bits {
            0 => GPIO44OUTCFG_A::DIS,
            1 => GPIO44OUTCFG_A::PUSHPULL,
            2 => GPIO44OUTCFG_A::OD,
            3 => GPIO44OUTCFG_A::TS,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        **self == GPIO44OUTCFG_A::DIS
    }
    #[doc = "Checks if the value of the field is `PUSHPULL`"]
    #[inline(always)]
    pub fn is_pushpull(&self) -> bool {
        **self == GPIO44OUTCFG_A::PUSHPULL
    }
    #[doc = "Checks if the value of the field is `OD`"]
    #[inline(always)]
    pub fn is_od(&self) -> bool {
        **self == GPIO44OUTCFG_A::OD
    }
    #[doc = "Checks if the value of the field is `TS`"]
    #[inline(always)]
    pub fn is_ts(&self) -> bool {
        **self == GPIO44OUTCFG_A::TS
    }
}
impl core::ops::Deref for GPIO44OUTCFG_R {
    type Target = crate::FieldReader<u8, GPIO44OUTCFG_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIO44OUTCFG` writer - GPIO44 output configuration."]
pub struct GPIO44OUTCFG_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO44OUTCFG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIO44OUTCFG_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "FNCSEL = 0x3 - Output disabled value."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(GPIO44OUTCFG_A::DIS)
    }
    #[doc = "FNCSEL = 0x3 - Output is push-pull value."]
    #[inline(always)]
    pub fn pushpull(self) -> &'a mut W {
        self.variant(GPIO44OUTCFG_A::PUSHPULL)
    }
    #[doc = "FNCSEL = 0x3 - Output is open drain value."]
    #[inline(always)]
    pub fn od(self) -> &'a mut W {
        self.variant(GPIO44OUTCFG_A::OD)
    }
    #[doc = "FNCSEL = 0x3 - Output is tri-state value."]
    #[inline(always)]
    pub fn ts(self) -> &'a mut W {
        self.variant(GPIO44OUTCFG_A::TS)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 17)) | ((value as u32 & 0x03) << 17);
        self.w
    }
}
#[doc = "GPIO44 input enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO44INCFG_A {
    #[doc = "0: Read the GPIO pin data value."]
    READ = 0,
    #[doc = "1: INTD = 0 - Readback will always be zero value."]
    RDZERO = 1,
}
impl From<GPIO44INCFG_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO44INCFG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIO44INCFG` reader - GPIO44 input enable."]
pub struct GPIO44INCFG_R(crate::FieldReader<bool, GPIO44INCFG_A>);
impl GPIO44INCFG_R {
    pub(crate) fn new(bits: bool) -> Self {
        GPIO44INCFG_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO44INCFG_A {
        match self.bits {
            false => GPIO44INCFG_A::READ,
            true => GPIO44INCFG_A::RDZERO,
        }
    }
    #[doc = "Checks if the value of the field is `READ`"]
    #[inline(always)]
    pub fn is_read(&self) -> bool {
        **self == GPIO44INCFG_A::READ
    }
    #[doc = "Checks if the value of the field is `RDZERO`"]
    #[inline(always)]
    pub fn is_rdzero(&self) -> bool {
        **self == GPIO44INCFG_A::RDZERO
    }
}
impl core::ops::Deref for GPIO44INCFG_R {
    type Target = crate::FieldReader<bool, GPIO44INCFG_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIO44INCFG` writer - GPIO44 input enable."]
pub struct GPIO44INCFG_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO44INCFG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIO44INCFG_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Read the GPIO pin data value."]
    #[inline(always)]
    pub fn read(self) -> &'a mut W {
        self.variant(GPIO44INCFG_A::READ)
    }
    #[doc = "INTD = 0 - Readback will always be zero value."]
    #[inline(always)]
    pub fn rdzero(self) -> &'a mut W {
        self.variant(GPIO44INCFG_A::RDZERO)
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
#[doc = "GPIO43 interrupt direction.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO43INTD_A {
    #[doc = "0: FNCSEL = 0x1 - nCE polarity active low value."]
    NCELOW = 0,
    #[doc = "1: FNCSEL = 0x1 - nCE polarity active high value."]
    NCEHIGH = 1,
}
impl From<GPIO43INTD_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO43INTD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIO43INTD` reader - GPIO43 interrupt direction."]
pub struct GPIO43INTD_R(crate::FieldReader<bool, GPIO43INTD_A>);
impl GPIO43INTD_R {
    pub(crate) fn new(bits: bool) -> Self {
        GPIO43INTD_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO43INTD_A {
        match self.bits {
            false => GPIO43INTD_A::NCELOW,
            true => GPIO43INTD_A::NCEHIGH,
        }
    }
    #[doc = "Checks if the value of the field is `NCELOW`"]
    #[inline(always)]
    pub fn is_n_celow(&self) -> bool {
        **self == GPIO43INTD_A::NCELOW
    }
    #[doc = "Checks if the value of the field is `NCEHIGH`"]
    #[inline(always)]
    pub fn is_n_cehigh(&self) -> bool {
        **self == GPIO43INTD_A::NCEHIGH
    }
}
impl core::ops::Deref for GPIO43INTD_R {
    type Target = crate::FieldReader<bool, GPIO43INTD_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIO43INTD` writer - GPIO43 interrupt direction."]
pub struct GPIO43INTD_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO43INTD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIO43INTD_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "FNCSEL = 0x1 - nCE polarity active low value."]
    #[inline(always)]
    pub fn n_celow(self) -> &'a mut W {
        self.variant(GPIO43INTD_A::NCELOW)
    }
    #[doc = "FNCSEL = 0x1 - nCE polarity active high value."]
    #[inline(always)]
    pub fn n_cehigh(self) -> &'a mut W {
        self.variant(GPIO43INTD_A::NCEHIGH)
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
#[doc = "GPIO43 output configuration.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum GPIO43OUTCFG_A {
    #[doc = "0: FNCSEL = 0x3 - Output disabled value."]
    DIS = 0,
    #[doc = "1: FNCSEL = 0x3 - Output is push-pull value."]
    PUSHPULL = 1,
    #[doc = "2: FNCSEL = 0x3 - Output is open drain value."]
    OD = 2,
    #[doc = "3: FNCSEL = 0x3 - Output is tri-state value."]
    TS = 3,
}
impl From<GPIO43OUTCFG_A> for u8 {
    #[inline(always)]
    fn from(variant: GPIO43OUTCFG_A) -> Self {
        variant as _
    }
}
#[doc = "Field `GPIO43OUTCFG` reader - GPIO43 output configuration."]
pub struct GPIO43OUTCFG_R(crate::FieldReader<u8, GPIO43OUTCFG_A>);
impl GPIO43OUTCFG_R {
    pub(crate) fn new(bits: u8) -> Self {
        GPIO43OUTCFG_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO43OUTCFG_A {
        match self.bits {
            0 => GPIO43OUTCFG_A::DIS,
            1 => GPIO43OUTCFG_A::PUSHPULL,
            2 => GPIO43OUTCFG_A::OD,
            3 => GPIO43OUTCFG_A::TS,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        **self == GPIO43OUTCFG_A::DIS
    }
    #[doc = "Checks if the value of the field is `PUSHPULL`"]
    #[inline(always)]
    pub fn is_pushpull(&self) -> bool {
        **self == GPIO43OUTCFG_A::PUSHPULL
    }
    #[doc = "Checks if the value of the field is `OD`"]
    #[inline(always)]
    pub fn is_od(&self) -> bool {
        **self == GPIO43OUTCFG_A::OD
    }
    #[doc = "Checks if the value of the field is `TS`"]
    #[inline(always)]
    pub fn is_ts(&self) -> bool {
        **self == GPIO43OUTCFG_A::TS
    }
}
impl core::ops::Deref for GPIO43OUTCFG_R {
    type Target = crate::FieldReader<u8, GPIO43OUTCFG_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIO43OUTCFG` writer - GPIO43 output configuration."]
pub struct GPIO43OUTCFG_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO43OUTCFG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIO43OUTCFG_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "FNCSEL = 0x3 - Output disabled value."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(GPIO43OUTCFG_A::DIS)
    }
    #[doc = "FNCSEL = 0x3 - Output is push-pull value."]
    #[inline(always)]
    pub fn pushpull(self) -> &'a mut W {
        self.variant(GPIO43OUTCFG_A::PUSHPULL)
    }
    #[doc = "FNCSEL = 0x3 - Output is open drain value."]
    #[inline(always)]
    pub fn od(self) -> &'a mut W {
        self.variant(GPIO43OUTCFG_A::OD)
    }
    #[doc = "FNCSEL = 0x3 - Output is tri-state value."]
    #[inline(always)]
    pub fn ts(self) -> &'a mut W {
        self.variant(GPIO43OUTCFG_A::TS)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 13)) | ((value as u32 & 0x03) << 13);
        self.w
    }
}
#[doc = "GPIO43 input enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO43INCFG_A {
    #[doc = "0: Read the GPIO pin data value."]
    READ = 0,
    #[doc = "1: INTD = 0 - Readback will always be zero value."]
    RDZERO = 1,
}
impl From<GPIO43INCFG_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO43INCFG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIO43INCFG` reader - GPIO43 input enable."]
pub struct GPIO43INCFG_R(crate::FieldReader<bool, GPIO43INCFG_A>);
impl GPIO43INCFG_R {
    pub(crate) fn new(bits: bool) -> Self {
        GPIO43INCFG_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO43INCFG_A {
        match self.bits {
            false => GPIO43INCFG_A::READ,
            true => GPIO43INCFG_A::RDZERO,
        }
    }
    #[doc = "Checks if the value of the field is `READ`"]
    #[inline(always)]
    pub fn is_read(&self) -> bool {
        **self == GPIO43INCFG_A::READ
    }
    #[doc = "Checks if the value of the field is `RDZERO`"]
    #[inline(always)]
    pub fn is_rdzero(&self) -> bool {
        **self == GPIO43INCFG_A::RDZERO
    }
}
impl core::ops::Deref for GPIO43INCFG_R {
    type Target = crate::FieldReader<bool, GPIO43INCFG_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIO43INCFG` writer - GPIO43 input enable."]
pub struct GPIO43INCFG_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO43INCFG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIO43INCFG_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Read the GPIO pin data value."]
    #[inline(always)]
    pub fn read(self) -> &'a mut W {
        self.variant(GPIO43INCFG_A::READ)
    }
    #[doc = "INTD = 0 - Readback will always be zero value."]
    #[inline(always)]
    pub fn rdzero(self) -> &'a mut W {
        self.variant(GPIO43INCFG_A::RDZERO)
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
#[doc = "GPIO42 interrupt direction.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO42INTD_A {
    #[doc = "0: FNCSEL = 0x1 - nCE polarity active low value."]
    NCELOW = 0,
    #[doc = "1: FNCSEL = 0x1 - nCE polarity active high value."]
    NCEHIGH = 1,
}
impl From<GPIO42INTD_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO42INTD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIO42INTD` reader - GPIO42 interrupt direction."]
pub struct GPIO42INTD_R(crate::FieldReader<bool, GPIO42INTD_A>);
impl GPIO42INTD_R {
    pub(crate) fn new(bits: bool) -> Self {
        GPIO42INTD_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO42INTD_A {
        match self.bits {
            false => GPIO42INTD_A::NCELOW,
            true => GPIO42INTD_A::NCEHIGH,
        }
    }
    #[doc = "Checks if the value of the field is `NCELOW`"]
    #[inline(always)]
    pub fn is_n_celow(&self) -> bool {
        **self == GPIO42INTD_A::NCELOW
    }
    #[doc = "Checks if the value of the field is `NCEHIGH`"]
    #[inline(always)]
    pub fn is_n_cehigh(&self) -> bool {
        **self == GPIO42INTD_A::NCEHIGH
    }
}
impl core::ops::Deref for GPIO42INTD_R {
    type Target = crate::FieldReader<bool, GPIO42INTD_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIO42INTD` writer - GPIO42 interrupt direction."]
pub struct GPIO42INTD_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO42INTD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIO42INTD_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "FNCSEL = 0x1 - nCE polarity active low value."]
    #[inline(always)]
    pub fn n_celow(self) -> &'a mut W {
        self.variant(GPIO42INTD_A::NCELOW)
    }
    #[doc = "FNCSEL = 0x1 - nCE polarity active high value."]
    #[inline(always)]
    pub fn n_cehigh(self) -> &'a mut W {
        self.variant(GPIO42INTD_A::NCEHIGH)
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
#[doc = "GPIO42 output configuration.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum GPIO42OUTCFG_A {
    #[doc = "0: FNCSEL = 0x3 - Output disabled value."]
    DIS = 0,
    #[doc = "1: FNCSEL = 0x3 - Output is push-pull value."]
    PUSHPULL = 1,
    #[doc = "2: FNCSEL = 0x3 - Output is open drain value."]
    OD = 2,
    #[doc = "3: FNCSEL = 0x3 - Output is tri-state value."]
    TS = 3,
}
impl From<GPIO42OUTCFG_A> for u8 {
    #[inline(always)]
    fn from(variant: GPIO42OUTCFG_A) -> Self {
        variant as _
    }
}
#[doc = "Field `GPIO42OUTCFG` reader - GPIO42 output configuration."]
pub struct GPIO42OUTCFG_R(crate::FieldReader<u8, GPIO42OUTCFG_A>);
impl GPIO42OUTCFG_R {
    pub(crate) fn new(bits: u8) -> Self {
        GPIO42OUTCFG_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO42OUTCFG_A {
        match self.bits {
            0 => GPIO42OUTCFG_A::DIS,
            1 => GPIO42OUTCFG_A::PUSHPULL,
            2 => GPIO42OUTCFG_A::OD,
            3 => GPIO42OUTCFG_A::TS,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        **self == GPIO42OUTCFG_A::DIS
    }
    #[doc = "Checks if the value of the field is `PUSHPULL`"]
    #[inline(always)]
    pub fn is_pushpull(&self) -> bool {
        **self == GPIO42OUTCFG_A::PUSHPULL
    }
    #[doc = "Checks if the value of the field is `OD`"]
    #[inline(always)]
    pub fn is_od(&self) -> bool {
        **self == GPIO42OUTCFG_A::OD
    }
    #[doc = "Checks if the value of the field is `TS`"]
    #[inline(always)]
    pub fn is_ts(&self) -> bool {
        **self == GPIO42OUTCFG_A::TS
    }
}
impl core::ops::Deref for GPIO42OUTCFG_R {
    type Target = crate::FieldReader<u8, GPIO42OUTCFG_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIO42OUTCFG` writer - GPIO42 output configuration."]
pub struct GPIO42OUTCFG_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO42OUTCFG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIO42OUTCFG_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "FNCSEL = 0x3 - Output disabled value."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(GPIO42OUTCFG_A::DIS)
    }
    #[doc = "FNCSEL = 0x3 - Output is push-pull value."]
    #[inline(always)]
    pub fn pushpull(self) -> &'a mut W {
        self.variant(GPIO42OUTCFG_A::PUSHPULL)
    }
    #[doc = "FNCSEL = 0x3 - Output is open drain value."]
    #[inline(always)]
    pub fn od(self) -> &'a mut W {
        self.variant(GPIO42OUTCFG_A::OD)
    }
    #[doc = "FNCSEL = 0x3 - Output is tri-state value."]
    #[inline(always)]
    pub fn ts(self) -> &'a mut W {
        self.variant(GPIO42OUTCFG_A::TS)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 9)) | ((value as u32 & 0x03) << 9);
        self.w
    }
}
#[doc = "GPIO42 input enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO42INCFG_A {
    #[doc = "0: Read the GPIO pin data value."]
    READ = 0,
    #[doc = "1: INTD = 0 - Readback will always be zero value."]
    RDZERO = 1,
}
impl From<GPIO42INCFG_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO42INCFG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIO42INCFG` reader - GPIO42 input enable."]
pub struct GPIO42INCFG_R(crate::FieldReader<bool, GPIO42INCFG_A>);
impl GPIO42INCFG_R {
    pub(crate) fn new(bits: bool) -> Self {
        GPIO42INCFG_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO42INCFG_A {
        match self.bits {
            false => GPIO42INCFG_A::READ,
            true => GPIO42INCFG_A::RDZERO,
        }
    }
    #[doc = "Checks if the value of the field is `READ`"]
    #[inline(always)]
    pub fn is_read(&self) -> bool {
        **self == GPIO42INCFG_A::READ
    }
    #[doc = "Checks if the value of the field is `RDZERO`"]
    #[inline(always)]
    pub fn is_rdzero(&self) -> bool {
        **self == GPIO42INCFG_A::RDZERO
    }
}
impl core::ops::Deref for GPIO42INCFG_R {
    type Target = crate::FieldReader<bool, GPIO42INCFG_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIO42INCFG` writer - GPIO42 input enable."]
pub struct GPIO42INCFG_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO42INCFG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIO42INCFG_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Read the GPIO pin data value."]
    #[inline(always)]
    pub fn read(self) -> &'a mut W {
        self.variant(GPIO42INCFG_A::READ)
    }
    #[doc = "INTD = 0 - Readback will always be zero value."]
    #[inline(always)]
    pub fn rdzero(self) -> &'a mut W {
        self.variant(GPIO42INCFG_A::RDZERO)
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
#[doc = "GPIO41 interrupt direction.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO41INTD_A {
    #[doc = "0: FNCSEL = 0x0 - nCE polarity active low value."]
    NCELOW = 0,
    #[doc = "1: FNCSEL = 0x0 - nCE polarity active high value."]
    NCEHIGH = 1,
}
impl From<GPIO41INTD_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO41INTD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIO41INTD` reader - GPIO41 interrupt direction."]
pub struct GPIO41INTD_R(crate::FieldReader<bool, GPIO41INTD_A>);
impl GPIO41INTD_R {
    pub(crate) fn new(bits: bool) -> Self {
        GPIO41INTD_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO41INTD_A {
        match self.bits {
            false => GPIO41INTD_A::NCELOW,
            true => GPIO41INTD_A::NCEHIGH,
        }
    }
    #[doc = "Checks if the value of the field is `NCELOW`"]
    #[inline(always)]
    pub fn is_n_celow(&self) -> bool {
        **self == GPIO41INTD_A::NCELOW
    }
    #[doc = "Checks if the value of the field is `NCEHIGH`"]
    #[inline(always)]
    pub fn is_n_cehigh(&self) -> bool {
        **self == GPIO41INTD_A::NCEHIGH
    }
}
impl core::ops::Deref for GPIO41INTD_R {
    type Target = crate::FieldReader<bool, GPIO41INTD_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIO41INTD` writer - GPIO41 interrupt direction."]
pub struct GPIO41INTD_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO41INTD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIO41INTD_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "FNCSEL = 0x0 - nCE polarity active low value."]
    #[inline(always)]
    pub fn n_celow(self) -> &'a mut W {
        self.variant(GPIO41INTD_A::NCELOW)
    }
    #[doc = "FNCSEL = 0x0 - nCE polarity active high value."]
    #[inline(always)]
    pub fn n_cehigh(self) -> &'a mut W {
        self.variant(GPIO41INTD_A::NCEHIGH)
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
#[doc = "GPIO41 output configuration.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum GPIO41OUTCFG_A {
    #[doc = "0: FNCSEL = 0x3 - Output disabled value."]
    DIS = 0,
    #[doc = "1: FNCSEL = 0x3 - Output is push-pull value."]
    PUSHPULL = 1,
    #[doc = "2: FNCSEL = 0x3 - Output is open drain value."]
    OD = 2,
    #[doc = "3: FNCSEL = 0x3 - Output is tri-state value."]
    TS = 3,
}
impl From<GPIO41OUTCFG_A> for u8 {
    #[inline(always)]
    fn from(variant: GPIO41OUTCFG_A) -> Self {
        variant as _
    }
}
#[doc = "Field `GPIO41OUTCFG` reader - GPIO41 output configuration."]
pub struct GPIO41OUTCFG_R(crate::FieldReader<u8, GPIO41OUTCFG_A>);
impl GPIO41OUTCFG_R {
    pub(crate) fn new(bits: u8) -> Self {
        GPIO41OUTCFG_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO41OUTCFG_A {
        match self.bits {
            0 => GPIO41OUTCFG_A::DIS,
            1 => GPIO41OUTCFG_A::PUSHPULL,
            2 => GPIO41OUTCFG_A::OD,
            3 => GPIO41OUTCFG_A::TS,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        **self == GPIO41OUTCFG_A::DIS
    }
    #[doc = "Checks if the value of the field is `PUSHPULL`"]
    #[inline(always)]
    pub fn is_pushpull(&self) -> bool {
        **self == GPIO41OUTCFG_A::PUSHPULL
    }
    #[doc = "Checks if the value of the field is `OD`"]
    #[inline(always)]
    pub fn is_od(&self) -> bool {
        **self == GPIO41OUTCFG_A::OD
    }
    #[doc = "Checks if the value of the field is `TS`"]
    #[inline(always)]
    pub fn is_ts(&self) -> bool {
        **self == GPIO41OUTCFG_A::TS
    }
}
impl core::ops::Deref for GPIO41OUTCFG_R {
    type Target = crate::FieldReader<u8, GPIO41OUTCFG_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIO41OUTCFG` writer - GPIO41 output configuration."]
pub struct GPIO41OUTCFG_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO41OUTCFG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIO41OUTCFG_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "FNCSEL = 0x3 - Output disabled value."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(GPIO41OUTCFG_A::DIS)
    }
    #[doc = "FNCSEL = 0x3 - Output is push-pull value."]
    #[inline(always)]
    pub fn pushpull(self) -> &'a mut W {
        self.variant(GPIO41OUTCFG_A::PUSHPULL)
    }
    #[doc = "FNCSEL = 0x3 - Output is open drain value."]
    #[inline(always)]
    pub fn od(self) -> &'a mut W {
        self.variant(GPIO41OUTCFG_A::OD)
    }
    #[doc = "FNCSEL = 0x3 - Output is tri-state value."]
    #[inline(always)]
    pub fn ts(self) -> &'a mut W {
        self.variant(GPIO41OUTCFG_A::TS)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 5)) | ((value as u32 & 0x03) << 5);
        self.w
    }
}
#[doc = "GPIO41 input enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO41INCFG_A {
    #[doc = "0: Read the GPIO pin data value."]
    READ = 0,
    #[doc = "1: INTD = 0 - Readback will always be zero value."]
    RDZERO = 1,
}
impl From<GPIO41INCFG_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO41INCFG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIO41INCFG` reader - GPIO41 input enable."]
pub struct GPIO41INCFG_R(crate::FieldReader<bool, GPIO41INCFG_A>);
impl GPIO41INCFG_R {
    pub(crate) fn new(bits: bool) -> Self {
        GPIO41INCFG_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO41INCFG_A {
        match self.bits {
            false => GPIO41INCFG_A::READ,
            true => GPIO41INCFG_A::RDZERO,
        }
    }
    #[doc = "Checks if the value of the field is `READ`"]
    #[inline(always)]
    pub fn is_read(&self) -> bool {
        **self == GPIO41INCFG_A::READ
    }
    #[doc = "Checks if the value of the field is `RDZERO`"]
    #[inline(always)]
    pub fn is_rdzero(&self) -> bool {
        **self == GPIO41INCFG_A::RDZERO
    }
}
impl core::ops::Deref for GPIO41INCFG_R {
    type Target = crate::FieldReader<bool, GPIO41INCFG_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIO41INCFG` writer - GPIO41 input enable."]
pub struct GPIO41INCFG_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO41INCFG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIO41INCFG_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Read the GPIO pin data value."]
    #[inline(always)]
    pub fn read(self) -> &'a mut W {
        self.variant(GPIO41INCFG_A::READ)
    }
    #[doc = "INTD = 0 - Readback will always be zero value."]
    #[inline(always)]
    pub fn rdzero(self) -> &'a mut W {
        self.variant(GPIO41INCFG_A::RDZERO)
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
#[doc = "GPIO40 interrupt direction.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO40INTD_A {
    #[doc = "0: INCFG = 1 - No interrupt on GPIO transition value."]
    INTDIS = 0,
    #[doc = "1: INCFG = 1 - Interrupt on either low to high or high to low GPIO transition value."]
    INTBOTH = 1,
}
impl From<GPIO40INTD_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO40INTD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIO40INTD` reader - GPIO40 interrupt direction."]
pub struct GPIO40INTD_R(crate::FieldReader<bool, GPIO40INTD_A>);
impl GPIO40INTD_R {
    pub(crate) fn new(bits: bool) -> Self {
        GPIO40INTD_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO40INTD_A {
        match self.bits {
            false => GPIO40INTD_A::INTDIS,
            true => GPIO40INTD_A::INTBOTH,
        }
    }
    #[doc = "Checks if the value of the field is `INTDIS`"]
    #[inline(always)]
    pub fn is_intdis(&self) -> bool {
        **self == GPIO40INTD_A::INTDIS
    }
    #[doc = "Checks if the value of the field is `INTBOTH`"]
    #[inline(always)]
    pub fn is_intboth(&self) -> bool {
        **self == GPIO40INTD_A::INTBOTH
    }
}
impl core::ops::Deref for GPIO40INTD_R {
    type Target = crate::FieldReader<bool, GPIO40INTD_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIO40INTD` writer - GPIO40 interrupt direction."]
pub struct GPIO40INTD_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO40INTD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIO40INTD_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "INCFG = 1 - No interrupt on GPIO transition value."]
    #[inline(always)]
    pub fn intdis(self) -> &'a mut W {
        self.variant(GPIO40INTD_A::INTDIS)
    }
    #[doc = "INCFG = 1 - Interrupt on either low to high or high to low GPIO transition value."]
    #[inline(always)]
    pub fn intboth(self) -> &'a mut W {
        self.variant(GPIO40INTD_A::INTBOTH)
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
#[doc = "GPIO40 output configuration.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum GPIO40OUTCFG_A {
    #[doc = "0: FNCSEL = 0x3 - Output disabled value."]
    DIS = 0,
    #[doc = "1: FNCSEL = 0x3 - Output is push-pull value."]
    PUSHPULL = 1,
    #[doc = "2: FNCSEL = 0x3 - Output is open drain value."]
    OD = 2,
    #[doc = "3: FNCSEL = 0x3 - Output is tri-state value."]
    TS = 3,
}
impl From<GPIO40OUTCFG_A> for u8 {
    #[inline(always)]
    fn from(variant: GPIO40OUTCFG_A) -> Self {
        variant as _
    }
}
#[doc = "Field `GPIO40OUTCFG` reader - GPIO40 output configuration."]
pub struct GPIO40OUTCFG_R(crate::FieldReader<u8, GPIO40OUTCFG_A>);
impl GPIO40OUTCFG_R {
    pub(crate) fn new(bits: u8) -> Self {
        GPIO40OUTCFG_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO40OUTCFG_A {
        match self.bits {
            0 => GPIO40OUTCFG_A::DIS,
            1 => GPIO40OUTCFG_A::PUSHPULL,
            2 => GPIO40OUTCFG_A::OD,
            3 => GPIO40OUTCFG_A::TS,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        **self == GPIO40OUTCFG_A::DIS
    }
    #[doc = "Checks if the value of the field is `PUSHPULL`"]
    #[inline(always)]
    pub fn is_pushpull(&self) -> bool {
        **self == GPIO40OUTCFG_A::PUSHPULL
    }
    #[doc = "Checks if the value of the field is `OD`"]
    #[inline(always)]
    pub fn is_od(&self) -> bool {
        **self == GPIO40OUTCFG_A::OD
    }
    #[doc = "Checks if the value of the field is `TS`"]
    #[inline(always)]
    pub fn is_ts(&self) -> bool {
        **self == GPIO40OUTCFG_A::TS
    }
}
impl core::ops::Deref for GPIO40OUTCFG_R {
    type Target = crate::FieldReader<u8, GPIO40OUTCFG_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIO40OUTCFG` writer - GPIO40 output configuration."]
pub struct GPIO40OUTCFG_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO40OUTCFG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIO40OUTCFG_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "FNCSEL = 0x3 - Output disabled value."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(GPIO40OUTCFG_A::DIS)
    }
    #[doc = "FNCSEL = 0x3 - Output is push-pull value."]
    #[inline(always)]
    pub fn pushpull(self) -> &'a mut W {
        self.variant(GPIO40OUTCFG_A::PUSHPULL)
    }
    #[doc = "FNCSEL = 0x3 - Output is open drain value."]
    #[inline(always)]
    pub fn od(self) -> &'a mut W {
        self.variant(GPIO40OUTCFG_A::OD)
    }
    #[doc = "FNCSEL = 0x3 - Output is tri-state value."]
    #[inline(always)]
    pub fn ts(self) -> &'a mut W {
        self.variant(GPIO40OUTCFG_A::TS)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 1)) | ((value as u32 & 0x03) << 1);
        self.w
    }
}
#[doc = "GPIO40 input enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO40INCFG_A {
    #[doc = "0: Read the GPIO pin data value."]
    READ = 0,
    #[doc = "1: INTD = 0 - Readback will always be zero value."]
    RDZERO = 1,
}
impl From<GPIO40INCFG_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO40INCFG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIO40INCFG` reader - GPIO40 input enable."]
pub struct GPIO40INCFG_R(crate::FieldReader<bool, GPIO40INCFG_A>);
impl GPIO40INCFG_R {
    pub(crate) fn new(bits: bool) -> Self {
        GPIO40INCFG_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO40INCFG_A {
        match self.bits {
            false => GPIO40INCFG_A::READ,
            true => GPIO40INCFG_A::RDZERO,
        }
    }
    #[doc = "Checks if the value of the field is `READ`"]
    #[inline(always)]
    pub fn is_read(&self) -> bool {
        **self == GPIO40INCFG_A::READ
    }
    #[doc = "Checks if the value of the field is `RDZERO`"]
    #[inline(always)]
    pub fn is_rdzero(&self) -> bool {
        **self == GPIO40INCFG_A::RDZERO
    }
}
impl core::ops::Deref for GPIO40INCFG_R {
    type Target = crate::FieldReader<bool, GPIO40INCFG_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIO40INCFG` writer - GPIO40 input enable."]
pub struct GPIO40INCFG_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO40INCFG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIO40INCFG_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Read the GPIO pin data value."]
    #[inline(always)]
    pub fn read(self) -> &'a mut W {
        self.variant(GPIO40INCFG_A::READ)
    }
    #[doc = "INTD = 0 - Readback will always be zero value."]
    #[inline(always)]
    pub fn rdzero(self) -> &'a mut W {
        self.variant(GPIO40INCFG_A::RDZERO)
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
    #[doc = "Bit 31 - GPIO47 interrupt direction."]
    #[inline(always)]
    pub fn gpio47intd(&self) -> GPIO47INTD_R {
        GPIO47INTD_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bits 29:30 - GPIO47 output configuration."]
    #[inline(always)]
    pub fn gpio47outcfg(&self) -> GPIO47OUTCFG_R {
        GPIO47OUTCFG_R::new(((self.bits >> 29) & 0x03) as u8)
    }
    #[doc = "Bit 28 - GPIO47 input enable."]
    #[inline(always)]
    pub fn gpio47incfg(&self) -> GPIO47INCFG_R {
        GPIO47INCFG_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 27 - GPIO46 interrupt direction."]
    #[inline(always)]
    pub fn gpio46intd(&self) -> GPIO46INTD_R {
        GPIO46INTD_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bits 25:26 - GPIO46 output configuration."]
    #[inline(always)]
    pub fn gpio46outcfg(&self) -> GPIO46OUTCFG_R {
        GPIO46OUTCFG_R::new(((self.bits >> 25) & 0x03) as u8)
    }
    #[doc = "Bit 24 - GPIO46 input enable."]
    #[inline(always)]
    pub fn gpio46incfg(&self) -> GPIO46INCFG_R {
        GPIO46INCFG_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 23 - GPIO45 interrupt direction."]
    #[inline(always)]
    pub fn gpio45intd(&self) -> GPIO45INTD_R {
        GPIO45INTD_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bits 21:22 - GPIO45 output configuration."]
    #[inline(always)]
    pub fn gpio45outcfg(&self) -> GPIO45OUTCFG_R {
        GPIO45OUTCFG_R::new(((self.bits >> 21) & 0x03) as u8)
    }
    #[doc = "Bit 20 - GPIO45 input enable."]
    #[inline(always)]
    pub fn gpio45incfg(&self) -> GPIO45INCFG_R {
        GPIO45INCFG_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 19 - GPIO44 interrupt direction."]
    #[inline(always)]
    pub fn gpio44intd(&self) -> GPIO44INTD_R {
        GPIO44INTD_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bits 17:18 - GPIO44 output configuration."]
    #[inline(always)]
    pub fn gpio44outcfg(&self) -> GPIO44OUTCFG_R {
        GPIO44OUTCFG_R::new(((self.bits >> 17) & 0x03) as u8)
    }
    #[doc = "Bit 16 - GPIO44 input enable."]
    #[inline(always)]
    pub fn gpio44incfg(&self) -> GPIO44INCFG_R {
        GPIO44INCFG_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 15 - GPIO43 interrupt direction."]
    #[inline(always)]
    pub fn gpio43intd(&self) -> GPIO43INTD_R {
        GPIO43INTD_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bits 13:14 - GPIO43 output configuration."]
    #[inline(always)]
    pub fn gpio43outcfg(&self) -> GPIO43OUTCFG_R {
        GPIO43OUTCFG_R::new(((self.bits >> 13) & 0x03) as u8)
    }
    #[doc = "Bit 12 - GPIO43 input enable."]
    #[inline(always)]
    pub fn gpio43incfg(&self) -> GPIO43INCFG_R {
        GPIO43INCFG_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11 - GPIO42 interrupt direction."]
    #[inline(always)]
    pub fn gpio42intd(&self) -> GPIO42INTD_R {
        GPIO42INTD_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bits 9:10 - GPIO42 output configuration."]
    #[inline(always)]
    pub fn gpio42outcfg(&self) -> GPIO42OUTCFG_R {
        GPIO42OUTCFG_R::new(((self.bits >> 9) & 0x03) as u8)
    }
    #[doc = "Bit 8 - GPIO42 input enable."]
    #[inline(always)]
    pub fn gpio42incfg(&self) -> GPIO42INCFG_R {
        GPIO42INCFG_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7 - GPIO41 interrupt direction."]
    #[inline(always)]
    pub fn gpio41intd(&self) -> GPIO41INTD_R {
        GPIO41INTD_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 5:6 - GPIO41 output configuration."]
    #[inline(always)]
    pub fn gpio41outcfg(&self) -> GPIO41OUTCFG_R {
        GPIO41OUTCFG_R::new(((self.bits >> 5) & 0x03) as u8)
    }
    #[doc = "Bit 4 - GPIO41 input enable."]
    #[inline(always)]
    pub fn gpio41incfg(&self) -> GPIO41INCFG_R {
        GPIO41INCFG_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - GPIO40 interrupt direction."]
    #[inline(always)]
    pub fn gpio40intd(&self) -> GPIO40INTD_R {
        GPIO40INTD_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bits 1:2 - GPIO40 output configuration."]
    #[inline(always)]
    pub fn gpio40outcfg(&self) -> GPIO40OUTCFG_R {
        GPIO40OUTCFG_R::new(((self.bits >> 1) & 0x03) as u8)
    }
    #[doc = "Bit 0 - GPIO40 input enable."]
    #[inline(always)]
    pub fn gpio40incfg(&self) -> GPIO40INCFG_R {
        GPIO40INCFG_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 31 - GPIO47 interrupt direction."]
    #[inline(always)]
    pub fn gpio47intd(&mut self) -> GPIO47INTD_W {
        GPIO47INTD_W { w: self }
    }
    #[doc = "Bits 29:30 - GPIO47 output configuration."]
    #[inline(always)]
    pub fn gpio47outcfg(&mut self) -> GPIO47OUTCFG_W {
        GPIO47OUTCFG_W { w: self }
    }
    #[doc = "Bit 28 - GPIO47 input enable."]
    #[inline(always)]
    pub fn gpio47incfg(&mut self) -> GPIO47INCFG_W {
        GPIO47INCFG_W { w: self }
    }
    #[doc = "Bit 27 - GPIO46 interrupt direction."]
    #[inline(always)]
    pub fn gpio46intd(&mut self) -> GPIO46INTD_W {
        GPIO46INTD_W { w: self }
    }
    #[doc = "Bits 25:26 - GPIO46 output configuration."]
    #[inline(always)]
    pub fn gpio46outcfg(&mut self) -> GPIO46OUTCFG_W {
        GPIO46OUTCFG_W { w: self }
    }
    #[doc = "Bit 24 - GPIO46 input enable."]
    #[inline(always)]
    pub fn gpio46incfg(&mut self) -> GPIO46INCFG_W {
        GPIO46INCFG_W { w: self }
    }
    #[doc = "Bit 23 - GPIO45 interrupt direction."]
    #[inline(always)]
    pub fn gpio45intd(&mut self) -> GPIO45INTD_W {
        GPIO45INTD_W { w: self }
    }
    #[doc = "Bits 21:22 - GPIO45 output configuration."]
    #[inline(always)]
    pub fn gpio45outcfg(&mut self) -> GPIO45OUTCFG_W {
        GPIO45OUTCFG_W { w: self }
    }
    #[doc = "Bit 20 - GPIO45 input enable."]
    #[inline(always)]
    pub fn gpio45incfg(&mut self) -> GPIO45INCFG_W {
        GPIO45INCFG_W { w: self }
    }
    #[doc = "Bit 19 - GPIO44 interrupt direction."]
    #[inline(always)]
    pub fn gpio44intd(&mut self) -> GPIO44INTD_W {
        GPIO44INTD_W { w: self }
    }
    #[doc = "Bits 17:18 - GPIO44 output configuration."]
    #[inline(always)]
    pub fn gpio44outcfg(&mut self) -> GPIO44OUTCFG_W {
        GPIO44OUTCFG_W { w: self }
    }
    #[doc = "Bit 16 - GPIO44 input enable."]
    #[inline(always)]
    pub fn gpio44incfg(&mut self) -> GPIO44INCFG_W {
        GPIO44INCFG_W { w: self }
    }
    #[doc = "Bit 15 - GPIO43 interrupt direction."]
    #[inline(always)]
    pub fn gpio43intd(&mut self) -> GPIO43INTD_W {
        GPIO43INTD_W { w: self }
    }
    #[doc = "Bits 13:14 - GPIO43 output configuration."]
    #[inline(always)]
    pub fn gpio43outcfg(&mut self) -> GPIO43OUTCFG_W {
        GPIO43OUTCFG_W { w: self }
    }
    #[doc = "Bit 12 - GPIO43 input enable."]
    #[inline(always)]
    pub fn gpio43incfg(&mut self) -> GPIO43INCFG_W {
        GPIO43INCFG_W { w: self }
    }
    #[doc = "Bit 11 - GPIO42 interrupt direction."]
    #[inline(always)]
    pub fn gpio42intd(&mut self) -> GPIO42INTD_W {
        GPIO42INTD_W { w: self }
    }
    #[doc = "Bits 9:10 - GPIO42 output configuration."]
    #[inline(always)]
    pub fn gpio42outcfg(&mut self) -> GPIO42OUTCFG_W {
        GPIO42OUTCFG_W { w: self }
    }
    #[doc = "Bit 8 - GPIO42 input enable."]
    #[inline(always)]
    pub fn gpio42incfg(&mut self) -> GPIO42INCFG_W {
        GPIO42INCFG_W { w: self }
    }
    #[doc = "Bit 7 - GPIO41 interrupt direction."]
    #[inline(always)]
    pub fn gpio41intd(&mut self) -> GPIO41INTD_W {
        GPIO41INTD_W { w: self }
    }
    #[doc = "Bits 5:6 - GPIO41 output configuration."]
    #[inline(always)]
    pub fn gpio41outcfg(&mut self) -> GPIO41OUTCFG_W {
        GPIO41OUTCFG_W { w: self }
    }
    #[doc = "Bit 4 - GPIO41 input enable."]
    #[inline(always)]
    pub fn gpio41incfg(&mut self) -> GPIO41INCFG_W {
        GPIO41INCFG_W { w: self }
    }
    #[doc = "Bit 3 - GPIO40 interrupt direction."]
    #[inline(always)]
    pub fn gpio40intd(&mut self) -> GPIO40INTD_W {
        GPIO40INTD_W { w: self }
    }
    #[doc = "Bits 1:2 - GPIO40 output configuration."]
    #[inline(always)]
    pub fn gpio40outcfg(&mut self) -> GPIO40OUTCFG_W {
        GPIO40OUTCFG_W { w: self }
    }
    #[doc = "Bit 0 - GPIO40 input enable."]
    #[inline(always)]
    pub fn gpio40incfg(&mut self) -> GPIO40INCFG_W {
        GPIO40INCFG_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPIO Configuration Register F (Pads 40 -47)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cfgf](index.html) module"]
pub struct CFGF_SPEC;
impl crate::RegisterSpec for CFGF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cfgf::R](R) reader structure"]
impl crate::Readable for CFGF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cfgf::W](W) writer structure"]
impl crate::Writable for CFGF_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CFGF to value 0"]
impl crate::Resettable for CFGF_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
