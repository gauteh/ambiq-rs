#[doc = "Register `CFGC` reader"]
pub struct R(crate::R<CFGC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CFGC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CFGC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CFGC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CFGC` writer"]
pub struct W(crate::W<CFGC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CFGC_SPEC>;
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
impl From<crate::W<CFGC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CFGC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "GPIO23 interrupt direction.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO23INTD_A {
    #[doc = "0: FNCSEL = 0x1 - nCE polarity active low value."]
    NCELOW = 0,
    #[doc = "1: FNCSEL = 0x1 - nCE polarity active high value."]
    NCEHIGH = 1,
}
impl From<GPIO23INTD_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO23INTD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIO23INTD` reader - GPIO23 interrupt direction."]
pub struct GPIO23INTD_R(crate::FieldReader<bool, GPIO23INTD_A>);
impl GPIO23INTD_R {
    pub(crate) fn new(bits: bool) -> Self {
        GPIO23INTD_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO23INTD_A {
        match self.bits {
            false => GPIO23INTD_A::NCELOW,
            true => GPIO23INTD_A::NCEHIGH,
        }
    }
    #[doc = "Checks if the value of the field is `NCELOW`"]
    #[inline(always)]
    pub fn is_n_celow(&self) -> bool {
        **self == GPIO23INTD_A::NCELOW
    }
    #[doc = "Checks if the value of the field is `NCEHIGH`"]
    #[inline(always)]
    pub fn is_n_cehigh(&self) -> bool {
        **self == GPIO23INTD_A::NCEHIGH
    }
}
impl core::ops::Deref for GPIO23INTD_R {
    type Target = crate::FieldReader<bool, GPIO23INTD_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIO23INTD` writer - GPIO23 interrupt direction."]
pub struct GPIO23INTD_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO23INTD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIO23INTD_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "FNCSEL = 0x1 - nCE polarity active low value."]
    #[inline(always)]
    pub fn n_celow(self) -> &'a mut W {
        self.variant(GPIO23INTD_A::NCELOW)
    }
    #[doc = "FNCSEL = 0x1 - nCE polarity active high value."]
    #[inline(always)]
    pub fn n_cehigh(self) -> &'a mut W {
        self.variant(GPIO23INTD_A::NCEHIGH)
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
#[doc = "GPIO23 output configuration.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum GPIO23OUTCFG_A {
    #[doc = "0: FNCSEL = 0x3 - Output disabled value."]
    DIS = 0,
    #[doc = "1: FNCSEL = 0x3 - Output is push-pull value."]
    PUSHPULL = 1,
    #[doc = "2: FNCSEL = 0x3 - Output is open drain value."]
    OD = 2,
    #[doc = "3: FNCSEL = 0x3 - Output is tri-state value."]
    TS = 3,
}
impl From<GPIO23OUTCFG_A> for u8 {
    #[inline(always)]
    fn from(variant: GPIO23OUTCFG_A) -> Self {
        variant as _
    }
}
#[doc = "Field `GPIO23OUTCFG` reader - GPIO23 output configuration."]
pub struct GPIO23OUTCFG_R(crate::FieldReader<u8, GPIO23OUTCFG_A>);
impl GPIO23OUTCFG_R {
    pub(crate) fn new(bits: u8) -> Self {
        GPIO23OUTCFG_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO23OUTCFG_A {
        match self.bits {
            0 => GPIO23OUTCFG_A::DIS,
            1 => GPIO23OUTCFG_A::PUSHPULL,
            2 => GPIO23OUTCFG_A::OD,
            3 => GPIO23OUTCFG_A::TS,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        **self == GPIO23OUTCFG_A::DIS
    }
    #[doc = "Checks if the value of the field is `PUSHPULL`"]
    #[inline(always)]
    pub fn is_pushpull(&self) -> bool {
        **self == GPIO23OUTCFG_A::PUSHPULL
    }
    #[doc = "Checks if the value of the field is `OD`"]
    #[inline(always)]
    pub fn is_od(&self) -> bool {
        **self == GPIO23OUTCFG_A::OD
    }
    #[doc = "Checks if the value of the field is `TS`"]
    #[inline(always)]
    pub fn is_ts(&self) -> bool {
        **self == GPIO23OUTCFG_A::TS
    }
}
impl core::ops::Deref for GPIO23OUTCFG_R {
    type Target = crate::FieldReader<u8, GPIO23OUTCFG_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIO23OUTCFG` writer - GPIO23 output configuration."]
pub struct GPIO23OUTCFG_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO23OUTCFG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIO23OUTCFG_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "FNCSEL = 0x3 - Output disabled value."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(GPIO23OUTCFG_A::DIS)
    }
    #[doc = "FNCSEL = 0x3 - Output is push-pull value."]
    #[inline(always)]
    pub fn pushpull(self) -> &'a mut W {
        self.variant(GPIO23OUTCFG_A::PUSHPULL)
    }
    #[doc = "FNCSEL = 0x3 - Output is open drain value."]
    #[inline(always)]
    pub fn od(self) -> &'a mut W {
        self.variant(GPIO23OUTCFG_A::OD)
    }
    #[doc = "FNCSEL = 0x3 - Output is tri-state value."]
    #[inline(always)]
    pub fn ts(self) -> &'a mut W {
        self.variant(GPIO23OUTCFG_A::TS)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 29)) | ((value as u32 & 0x03) << 29);
        self.w
    }
}
#[doc = "GPIO23 input enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO23INCFG_A {
    #[doc = "0: Read the GPIO pin data value."]
    READ = 0,
    #[doc = "1: INTD = 0 - Readback will always be zero value."]
    RDZERO = 1,
}
impl From<GPIO23INCFG_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO23INCFG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIO23INCFG` reader - GPIO23 input enable."]
pub struct GPIO23INCFG_R(crate::FieldReader<bool, GPIO23INCFG_A>);
impl GPIO23INCFG_R {
    pub(crate) fn new(bits: bool) -> Self {
        GPIO23INCFG_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO23INCFG_A {
        match self.bits {
            false => GPIO23INCFG_A::READ,
            true => GPIO23INCFG_A::RDZERO,
        }
    }
    #[doc = "Checks if the value of the field is `READ`"]
    #[inline(always)]
    pub fn is_read(&self) -> bool {
        **self == GPIO23INCFG_A::READ
    }
    #[doc = "Checks if the value of the field is `RDZERO`"]
    #[inline(always)]
    pub fn is_rdzero(&self) -> bool {
        **self == GPIO23INCFG_A::RDZERO
    }
}
impl core::ops::Deref for GPIO23INCFG_R {
    type Target = crate::FieldReader<bool, GPIO23INCFG_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIO23INCFG` writer - GPIO23 input enable."]
pub struct GPIO23INCFG_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO23INCFG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIO23INCFG_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Read the GPIO pin data value."]
    #[inline(always)]
    pub fn read(self) -> &'a mut W {
        self.variant(GPIO23INCFG_A::READ)
    }
    #[doc = "INTD = 0 - Readback will always be zero value."]
    #[inline(always)]
    pub fn rdzero(self) -> &'a mut W {
        self.variant(GPIO23INCFG_A::RDZERO)
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
#[doc = "GPIO22 interrupt direction.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO22INTD_A {
    #[doc = "0: FNCSEL = 0x1 - nCE polarity active low value."]
    NCELOW = 0,
    #[doc = "1: FNCSEL = 0x1 - nCE polarity active high value."]
    NCEHIGH = 1,
}
impl From<GPIO22INTD_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO22INTD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIO22INTD` reader - GPIO22 interrupt direction."]
pub struct GPIO22INTD_R(crate::FieldReader<bool, GPIO22INTD_A>);
impl GPIO22INTD_R {
    pub(crate) fn new(bits: bool) -> Self {
        GPIO22INTD_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO22INTD_A {
        match self.bits {
            false => GPIO22INTD_A::NCELOW,
            true => GPIO22INTD_A::NCEHIGH,
        }
    }
    #[doc = "Checks if the value of the field is `NCELOW`"]
    #[inline(always)]
    pub fn is_n_celow(&self) -> bool {
        **self == GPIO22INTD_A::NCELOW
    }
    #[doc = "Checks if the value of the field is `NCEHIGH`"]
    #[inline(always)]
    pub fn is_n_cehigh(&self) -> bool {
        **self == GPIO22INTD_A::NCEHIGH
    }
}
impl core::ops::Deref for GPIO22INTD_R {
    type Target = crate::FieldReader<bool, GPIO22INTD_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIO22INTD` writer - GPIO22 interrupt direction."]
pub struct GPIO22INTD_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO22INTD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIO22INTD_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "FNCSEL = 0x1 - nCE polarity active low value."]
    #[inline(always)]
    pub fn n_celow(self) -> &'a mut W {
        self.variant(GPIO22INTD_A::NCELOW)
    }
    #[doc = "FNCSEL = 0x1 - nCE polarity active high value."]
    #[inline(always)]
    pub fn n_cehigh(self) -> &'a mut W {
        self.variant(GPIO22INTD_A::NCEHIGH)
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
#[doc = "GPIO22 output configuration.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum GPIO22OUTCFG_A {
    #[doc = "0: FNCSEL = 0x3 - Output disabled value."]
    DIS = 0,
    #[doc = "1: FNCSEL = 0x3 - Output is push-pull value."]
    PUSHPULL = 1,
    #[doc = "2: FNCSEL = 0x3 - Output is open drain value."]
    OD = 2,
    #[doc = "3: FNCSEL = 0x3 - Output is tri-state value."]
    TS = 3,
}
impl From<GPIO22OUTCFG_A> for u8 {
    #[inline(always)]
    fn from(variant: GPIO22OUTCFG_A) -> Self {
        variant as _
    }
}
#[doc = "Field `GPIO22OUTCFG` reader - GPIO22 output configuration."]
pub struct GPIO22OUTCFG_R(crate::FieldReader<u8, GPIO22OUTCFG_A>);
impl GPIO22OUTCFG_R {
    pub(crate) fn new(bits: u8) -> Self {
        GPIO22OUTCFG_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO22OUTCFG_A {
        match self.bits {
            0 => GPIO22OUTCFG_A::DIS,
            1 => GPIO22OUTCFG_A::PUSHPULL,
            2 => GPIO22OUTCFG_A::OD,
            3 => GPIO22OUTCFG_A::TS,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        **self == GPIO22OUTCFG_A::DIS
    }
    #[doc = "Checks if the value of the field is `PUSHPULL`"]
    #[inline(always)]
    pub fn is_pushpull(&self) -> bool {
        **self == GPIO22OUTCFG_A::PUSHPULL
    }
    #[doc = "Checks if the value of the field is `OD`"]
    #[inline(always)]
    pub fn is_od(&self) -> bool {
        **self == GPIO22OUTCFG_A::OD
    }
    #[doc = "Checks if the value of the field is `TS`"]
    #[inline(always)]
    pub fn is_ts(&self) -> bool {
        **self == GPIO22OUTCFG_A::TS
    }
}
impl core::ops::Deref for GPIO22OUTCFG_R {
    type Target = crate::FieldReader<u8, GPIO22OUTCFG_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIO22OUTCFG` writer - GPIO22 output configuration."]
pub struct GPIO22OUTCFG_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO22OUTCFG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIO22OUTCFG_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "FNCSEL = 0x3 - Output disabled value."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(GPIO22OUTCFG_A::DIS)
    }
    #[doc = "FNCSEL = 0x3 - Output is push-pull value."]
    #[inline(always)]
    pub fn pushpull(self) -> &'a mut W {
        self.variant(GPIO22OUTCFG_A::PUSHPULL)
    }
    #[doc = "FNCSEL = 0x3 - Output is open drain value."]
    #[inline(always)]
    pub fn od(self) -> &'a mut W {
        self.variant(GPIO22OUTCFG_A::OD)
    }
    #[doc = "FNCSEL = 0x3 - Output is tri-state value."]
    #[inline(always)]
    pub fn ts(self) -> &'a mut W {
        self.variant(GPIO22OUTCFG_A::TS)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 25)) | ((value as u32 & 0x03) << 25);
        self.w
    }
}
#[doc = "GPIO22 input enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO22INCFG_A {
    #[doc = "0: Read the GPIO pin data value."]
    READ = 0,
    #[doc = "1: INTD = 0 - Readback will always be zero value."]
    RDZERO = 1,
}
impl From<GPIO22INCFG_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO22INCFG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIO22INCFG` reader - GPIO22 input enable."]
pub struct GPIO22INCFG_R(crate::FieldReader<bool, GPIO22INCFG_A>);
impl GPIO22INCFG_R {
    pub(crate) fn new(bits: bool) -> Self {
        GPIO22INCFG_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO22INCFG_A {
        match self.bits {
            false => GPIO22INCFG_A::READ,
            true => GPIO22INCFG_A::RDZERO,
        }
    }
    #[doc = "Checks if the value of the field is `READ`"]
    #[inline(always)]
    pub fn is_read(&self) -> bool {
        **self == GPIO22INCFG_A::READ
    }
    #[doc = "Checks if the value of the field is `RDZERO`"]
    #[inline(always)]
    pub fn is_rdzero(&self) -> bool {
        **self == GPIO22INCFG_A::RDZERO
    }
}
impl core::ops::Deref for GPIO22INCFG_R {
    type Target = crate::FieldReader<bool, GPIO22INCFG_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIO22INCFG` writer - GPIO22 input enable."]
pub struct GPIO22INCFG_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO22INCFG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIO22INCFG_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Read the GPIO pin data value."]
    #[inline(always)]
    pub fn read(self) -> &'a mut W {
        self.variant(GPIO22INCFG_A::READ)
    }
    #[doc = "INTD = 0 - Readback will always be zero value."]
    #[inline(always)]
    pub fn rdzero(self) -> &'a mut W {
        self.variant(GPIO22INCFG_A::RDZERO)
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
#[doc = "GPIO21 interrupt direction.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO21INTD_A {
    #[doc = "0: FNCSEL = 0x1 - nCE polarity active low value."]
    NCELOW = 0,
    #[doc = "1: FNCSEL = 0x1 - nCE polarity active high value."]
    NCEHIGH = 1,
}
impl From<GPIO21INTD_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO21INTD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIO21INTD` reader - GPIO21 interrupt direction."]
pub struct GPIO21INTD_R(crate::FieldReader<bool, GPIO21INTD_A>);
impl GPIO21INTD_R {
    pub(crate) fn new(bits: bool) -> Self {
        GPIO21INTD_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO21INTD_A {
        match self.bits {
            false => GPIO21INTD_A::NCELOW,
            true => GPIO21INTD_A::NCEHIGH,
        }
    }
    #[doc = "Checks if the value of the field is `NCELOW`"]
    #[inline(always)]
    pub fn is_n_celow(&self) -> bool {
        **self == GPIO21INTD_A::NCELOW
    }
    #[doc = "Checks if the value of the field is `NCEHIGH`"]
    #[inline(always)]
    pub fn is_n_cehigh(&self) -> bool {
        **self == GPIO21INTD_A::NCEHIGH
    }
}
impl core::ops::Deref for GPIO21INTD_R {
    type Target = crate::FieldReader<bool, GPIO21INTD_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIO21INTD` writer - GPIO21 interrupt direction."]
pub struct GPIO21INTD_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO21INTD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIO21INTD_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "FNCSEL = 0x1 - nCE polarity active low value."]
    #[inline(always)]
    pub fn n_celow(self) -> &'a mut W {
        self.variant(GPIO21INTD_A::NCELOW)
    }
    #[doc = "FNCSEL = 0x1 - nCE polarity active high value."]
    #[inline(always)]
    pub fn n_cehigh(self) -> &'a mut W {
        self.variant(GPIO21INTD_A::NCEHIGH)
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
#[doc = "GPIO21 output configuration.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum GPIO21OUTCFG_A {
    #[doc = "0: FNCSEL = 0x3 - Output disabled value."]
    DIS = 0,
    #[doc = "1: FNCSEL = 0x3 - Output is push-pull value."]
    PUSHPULL = 1,
    #[doc = "2: FNCSEL = 0x3 - Output is open drain value."]
    OD = 2,
    #[doc = "3: FNCSEL = 0x3 - Output is tri-state value."]
    TS = 3,
}
impl From<GPIO21OUTCFG_A> for u8 {
    #[inline(always)]
    fn from(variant: GPIO21OUTCFG_A) -> Self {
        variant as _
    }
}
#[doc = "Field `GPIO21OUTCFG` reader - GPIO21 output configuration."]
pub struct GPIO21OUTCFG_R(crate::FieldReader<u8, GPIO21OUTCFG_A>);
impl GPIO21OUTCFG_R {
    pub(crate) fn new(bits: u8) -> Self {
        GPIO21OUTCFG_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO21OUTCFG_A {
        match self.bits {
            0 => GPIO21OUTCFG_A::DIS,
            1 => GPIO21OUTCFG_A::PUSHPULL,
            2 => GPIO21OUTCFG_A::OD,
            3 => GPIO21OUTCFG_A::TS,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        **self == GPIO21OUTCFG_A::DIS
    }
    #[doc = "Checks if the value of the field is `PUSHPULL`"]
    #[inline(always)]
    pub fn is_pushpull(&self) -> bool {
        **self == GPIO21OUTCFG_A::PUSHPULL
    }
    #[doc = "Checks if the value of the field is `OD`"]
    #[inline(always)]
    pub fn is_od(&self) -> bool {
        **self == GPIO21OUTCFG_A::OD
    }
    #[doc = "Checks if the value of the field is `TS`"]
    #[inline(always)]
    pub fn is_ts(&self) -> bool {
        **self == GPIO21OUTCFG_A::TS
    }
}
impl core::ops::Deref for GPIO21OUTCFG_R {
    type Target = crate::FieldReader<u8, GPIO21OUTCFG_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIO21OUTCFG` writer - GPIO21 output configuration."]
pub struct GPIO21OUTCFG_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO21OUTCFG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIO21OUTCFG_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "FNCSEL = 0x3 - Output disabled value."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(GPIO21OUTCFG_A::DIS)
    }
    #[doc = "FNCSEL = 0x3 - Output is push-pull value."]
    #[inline(always)]
    pub fn pushpull(self) -> &'a mut W {
        self.variant(GPIO21OUTCFG_A::PUSHPULL)
    }
    #[doc = "FNCSEL = 0x3 - Output is open drain value."]
    #[inline(always)]
    pub fn od(self) -> &'a mut W {
        self.variant(GPIO21OUTCFG_A::OD)
    }
    #[doc = "FNCSEL = 0x3 - Output is tri-state value."]
    #[inline(always)]
    pub fn ts(self) -> &'a mut W {
        self.variant(GPIO21OUTCFG_A::TS)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 21)) | ((value as u32 & 0x03) << 21);
        self.w
    }
}
#[doc = "GPIO21 input enable.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO21INCFG_A {
    #[doc = "0: Read the GPIO pin data value."]
    READ = 0,
    #[doc = "1: INTD = 0 - Readback will always be zero value."]
    RDZERO = 1,
}
impl From<GPIO21INCFG_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO21INCFG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIO21INCFG` reader - GPIO21 input enable."]
pub struct GPIO21INCFG_R(crate::FieldReader<bool, GPIO21INCFG_A>);
impl GPIO21INCFG_R {
    pub(crate) fn new(bits: bool) -> Self {
        GPIO21INCFG_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO21INCFG_A {
        match self.bits {
            false => GPIO21INCFG_A::READ,
            true => GPIO21INCFG_A::RDZERO,
        }
    }
    #[doc = "Checks if the value of the field is `READ`"]
    #[inline(always)]
    pub fn is_read(&self) -> bool {
        **self == GPIO21INCFG_A::READ
    }
    #[doc = "Checks if the value of the field is `RDZERO`"]
    #[inline(always)]
    pub fn is_rdzero(&self) -> bool {
        **self == GPIO21INCFG_A::RDZERO
    }
}
impl core::ops::Deref for GPIO21INCFG_R {
    type Target = crate::FieldReader<bool, GPIO21INCFG_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIO21INCFG` writer - GPIO21 input enable."]
pub struct GPIO21INCFG_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO21INCFG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIO21INCFG_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Read the GPIO pin data value."]
    #[inline(always)]
    pub fn read(self) -> &'a mut W {
        self.variant(GPIO21INCFG_A::READ)
    }
    #[doc = "INTD = 0 - Readback will always be zero value."]
    #[inline(always)]
    pub fn rdzero(self) -> &'a mut W {
        self.variant(GPIO21INCFG_A::RDZERO)
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
#[doc = "GPIO20 interrupt direction.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO20INTD_A {
    #[doc = "0: FNCSEL = 0x1 - nCE polarity active low value."]
    NCELOW = 0,
    #[doc = "1: FNCSEL = 0x1 - nCE polarity active high value."]
    NCEHIGH = 1,
}
impl From<GPIO20INTD_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO20INTD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIO20INTD` reader - GPIO20 interrupt direction."]
pub struct GPIO20INTD_R(crate::FieldReader<bool, GPIO20INTD_A>);
impl GPIO20INTD_R {
    pub(crate) fn new(bits: bool) -> Self {
        GPIO20INTD_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO20INTD_A {
        match self.bits {
            false => GPIO20INTD_A::NCELOW,
            true => GPIO20INTD_A::NCEHIGH,
        }
    }
    #[doc = "Checks if the value of the field is `NCELOW`"]
    #[inline(always)]
    pub fn is_n_celow(&self) -> bool {
        **self == GPIO20INTD_A::NCELOW
    }
    #[doc = "Checks if the value of the field is `NCEHIGH`"]
    #[inline(always)]
    pub fn is_n_cehigh(&self) -> bool {
        **self == GPIO20INTD_A::NCEHIGH
    }
}
impl core::ops::Deref for GPIO20INTD_R {
    type Target = crate::FieldReader<bool, GPIO20INTD_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIO20INTD` writer - GPIO20 interrupt direction."]
pub struct GPIO20INTD_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO20INTD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIO20INTD_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "FNCSEL = 0x1 - nCE polarity active low value."]
    #[inline(always)]
    pub fn n_celow(self) -> &'a mut W {
        self.variant(GPIO20INTD_A::NCELOW)
    }
    #[doc = "FNCSEL = 0x1 - nCE polarity active high value."]
    #[inline(always)]
    pub fn n_cehigh(self) -> &'a mut W {
        self.variant(GPIO20INTD_A::NCEHIGH)
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
#[doc = "GPIO20 output configuration.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum GPIO20OUTCFG_A {
    #[doc = "0: FNCSEL = 0x3 - Output disabled value."]
    DIS = 0,
    #[doc = "1: FNCSEL = 0x3 - Output is push-pull value."]
    PUSHPULL = 1,
    #[doc = "2: FNCSEL = 0x3 - Output is open drain value."]
    OD = 2,
    #[doc = "3: FNCSEL = 0x3 - Output is tri-state value."]
    TS = 3,
}
impl From<GPIO20OUTCFG_A> for u8 {
    #[inline(always)]
    fn from(variant: GPIO20OUTCFG_A) -> Self {
        variant as _
    }
}
#[doc = "Field `GPIO20OUTCFG` reader - GPIO20 output configuration."]
pub struct GPIO20OUTCFG_R(crate::FieldReader<u8, GPIO20OUTCFG_A>);
impl GPIO20OUTCFG_R {
    pub(crate) fn new(bits: u8) -> Self {
        GPIO20OUTCFG_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO20OUTCFG_A {
        match self.bits {
            0 => GPIO20OUTCFG_A::DIS,
            1 => GPIO20OUTCFG_A::PUSHPULL,
            2 => GPIO20OUTCFG_A::OD,
            3 => GPIO20OUTCFG_A::TS,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        **self == GPIO20OUTCFG_A::DIS
    }
    #[doc = "Checks if the value of the field is `PUSHPULL`"]
    #[inline(always)]
    pub fn is_pushpull(&self) -> bool {
        **self == GPIO20OUTCFG_A::PUSHPULL
    }
    #[doc = "Checks if the value of the field is `OD`"]
    #[inline(always)]
    pub fn is_od(&self) -> bool {
        **self == GPIO20OUTCFG_A::OD
    }
    #[doc = "Checks if the value of the field is `TS`"]
    #[inline(always)]
    pub fn is_ts(&self) -> bool {
        **self == GPIO20OUTCFG_A::TS
    }
}
impl core::ops::Deref for GPIO20OUTCFG_R {
    type Target = crate::FieldReader<u8, GPIO20OUTCFG_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIO20OUTCFG` writer - GPIO20 output configuration."]
pub struct GPIO20OUTCFG_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO20OUTCFG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIO20OUTCFG_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "FNCSEL = 0x3 - Output disabled value."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(GPIO20OUTCFG_A::DIS)
    }
    #[doc = "FNCSEL = 0x3 - Output is push-pull value."]
    #[inline(always)]
    pub fn pushpull(self) -> &'a mut W {
        self.variant(GPIO20OUTCFG_A::PUSHPULL)
    }
    #[doc = "FNCSEL = 0x3 - Output is open drain value."]
    #[inline(always)]
    pub fn od(self) -> &'a mut W {
        self.variant(GPIO20OUTCFG_A::OD)
    }
    #[doc = "FNCSEL = 0x3 - Output is tri-state value."]
    #[inline(always)]
    pub fn ts(self) -> &'a mut W {
        self.variant(GPIO20OUTCFG_A::TS)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 17)) | ((value as u32 & 0x03) << 17);
        self.w
    }
}
#[doc = "GPIO20 input enable.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO20INCFG_A {
    #[doc = "0: Read the GPIO pin data value."]
    READ = 0,
    #[doc = "1: INTD = 0 - Readback will always be zero value."]
    RDZERO = 1,
}
impl From<GPIO20INCFG_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO20INCFG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIO20INCFG` reader - GPIO20 input enable."]
pub struct GPIO20INCFG_R(crate::FieldReader<bool, GPIO20INCFG_A>);
impl GPIO20INCFG_R {
    pub(crate) fn new(bits: bool) -> Self {
        GPIO20INCFG_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO20INCFG_A {
        match self.bits {
            false => GPIO20INCFG_A::READ,
            true => GPIO20INCFG_A::RDZERO,
        }
    }
    #[doc = "Checks if the value of the field is `READ`"]
    #[inline(always)]
    pub fn is_read(&self) -> bool {
        **self == GPIO20INCFG_A::READ
    }
    #[doc = "Checks if the value of the field is `RDZERO`"]
    #[inline(always)]
    pub fn is_rdzero(&self) -> bool {
        **self == GPIO20INCFG_A::RDZERO
    }
}
impl core::ops::Deref for GPIO20INCFG_R {
    type Target = crate::FieldReader<bool, GPIO20INCFG_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIO20INCFG` writer - GPIO20 input enable."]
pub struct GPIO20INCFG_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO20INCFG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIO20INCFG_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Read the GPIO pin data value."]
    #[inline(always)]
    pub fn read(self) -> &'a mut W {
        self.variant(GPIO20INCFG_A::READ)
    }
    #[doc = "INTD = 0 - Readback will always be zero value."]
    #[inline(always)]
    pub fn rdzero(self) -> &'a mut W {
        self.variant(GPIO20INCFG_A::RDZERO)
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
#[doc = "GPIO19 interrupt direction.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO19INTD_A {
    #[doc = "0: FNCSEL = 0x1 - nCE polarity active low value."]
    NCELOW = 0,
    #[doc = "1: FNCSEL = 0x1 - nCE polarity active high value."]
    NCEHIGH = 1,
}
impl From<GPIO19INTD_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO19INTD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIO19INTD` reader - GPIO19 interrupt direction."]
pub struct GPIO19INTD_R(crate::FieldReader<bool, GPIO19INTD_A>);
impl GPIO19INTD_R {
    pub(crate) fn new(bits: bool) -> Self {
        GPIO19INTD_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO19INTD_A {
        match self.bits {
            false => GPIO19INTD_A::NCELOW,
            true => GPIO19INTD_A::NCEHIGH,
        }
    }
    #[doc = "Checks if the value of the field is `NCELOW`"]
    #[inline(always)]
    pub fn is_n_celow(&self) -> bool {
        **self == GPIO19INTD_A::NCELOW
    }
    #[doc = "Checks if the value of the field is `NCEHIGH`"]
    #[inline(always)]
    pub fn is_n_cehigh(&self) -> bool {
        **self == GPIO19INTD_A::NCEHIGH
    }
}
impl core::ops::Deref for GPIO19INTD_R {
    type Target = crate::FieldReader<bool, GPIO19INTD_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIO19INTD` writer - GPIO19 interrupt direction."]
pub struct GPIO19INTD_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO19INTD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIO19INTD_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "FNCSEL = 0x1 - nCE polarity active low value."]
    #[inline(always)]
    pub fn n_celow(self) -> &'a mut W {
        self.variant(GPIO19INTD_A::NCELOW)
    }
    #[doc = "FNCSEL = 0x1 - nCE polarity active high value."]
    #[inline(always)]
    pub fn n_cehigh(self) -> &'a mut W {
        self.variant(GPIO19INTD_A::NCEHIGH)
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
#[doc = "GPIO19 output configuration.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum GPIO19OUTCFG_A {
    #[doc = "0: FNCSEL = 0x3 - Output disabled value."]
    DIS = 0,
    #[doc = "1: FNCSEL = 0x3 - Output is push-pull value."]
    PUSHPULL = 1,
    #[doc = "2: FNCSEL = 0x3 - Output is open drain value."]
    OD = 2,
    #[doc = "3: FNCSEL = 0x3 - Output is tri-state value."]
    TS = 3,
}
impl From<GPIO19OUTCFG_A> for u8 {
    #[inline(always)]
    fn from(variant: GPIO19OUTCFG_A) -> Self {
        variant as _
    }
}
#[doc = "Field `GPIO19OUTCFG` reader - GPIO19 output configuration."]
pub struct GPIO19OUTCFG_R(crate::FieldReader<u8, GPIO19OUTCFG_A>);
impl GPIO19OUTCFG_R {
    pub(crate) fn new(bits: u8) -> Self {
        GPIO19OUTCFG_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO19OUTCFG_A {
        match self.bits {
            0 => GPIO19OUTCFG_A::DIS,
            1 => GPIO19OUTCFG_A::PUSHPULL,
            2 => GPIO19OUTCFG_A::OD,
            3 => GPIO19OUTCFG_A::TS,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        **self == GPIO19OUTCFG_A::DIS
    }
    #[doc = "Checks if the value of the field is `PUSHPULL`"]
    #[inline(always)]
    pub fn is_pushpull(&self) -> bool {
        **self == GPIO19OUTCFG_A::PUSHPULL
    }
    #[doc = "Checks if the value of the field is `OD`"]
    #[inline(always)]
    pub fn is_od(&self) -> bool {
        **self == GPIO19OUTCFG_A::OD
    }
    #[doc = "Checks if the value of the field is `TS`"]
    #[inline(always)]
    pub fn is_ts(&self) -> bool {
        **self == GPIO19OUTCFG_A::TS
    }
}
impl core::ops::Deref for GPIO19OUTCFG_R {
    type Target = crate::FieldReader<u8, GPIO19OUTCFG_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIO19OUTCFG` writer - GPIO19 output configuration."]
pub struct GPIO19OUTCFG_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO19OUTCFG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIO19OUTCFG_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "FNCSEL = 0x3 - Output disabled value."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(GPIO19OUTCFG_A::DIS)
    }
    #[doc = "FNCSEL = 0x3 - Output is push-pull value."]
    #[inline(always)]
    pub fn pushpull(self) -> &'a mut W {
        self.variant(GPIO19OUTCFG_A::PUSHPULL)
    }
    #[doc = "FNCSEL = 0x3 - Output is open drain value."]
    #[inline(always)]
    pub fn od(self) -> &'a mut W {
        self.variant(GPIO19OUTCFG_A::OD)
    }
    #[doc = "FNCSEL = 0x3 - Output is tri-state value."]
    #[inline(always)]
    pub fn ts(self) -> &'a mut W {
        self.variant(GPIO19OUTCFG_A::TS)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 13)) | ((value as u32 & 0x03) << 13);
        self.w
    }
}
#[doc = "GPIO19 input enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO19INCFG_A {
    #[doc = "0: Read the GPIO pin data value."]
    READ = 0,
    #[doc = "1: INTD = 0 - Readback will always be zero value."]
    RDZERO = 1,
}
impl From<GPIO19INCFG_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO19INCFG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIO19INCFG` reader - GPIO19 input enable."]
pub struct GPIO19INCFG_R(crate::FieldReader<bool, GPIO19INCFG_A>);
impl GPIO19INCFG_R {
    pub(crate) fn new(bits: bool) -> Self {
        GPIO19INCFG_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO19INCFG_A {
        match self.bits {
            false => GPIO19INCFG_A::READ,
            true => GPIO19INCFG_A::RDZERO,
        }
    }
    #[doc = "Checks if the value of the field is `READ`"]
    #[inline(always)]
    pub fn is_read(&self) -> bool {
        **self == GPIO19INCFG_A::READ
    }
    #[doc = "Checks if the value of the field is `RDZERO`"]
    #[inline(always)]
    pub fn is_rdzero(&self) -> bool {
        **self == GPIO19INCFG_A::RDZERO
    }
}
impl core::ops::Deref for GPIO19INCFG_R {
    type Target = crate::FieldReader<bool, GPIO19INCFG_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIO19INCFG` writer - GPIO19 input enable."]
pub struct GPIO19INCFG_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO19INCFG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIO19INCFG_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Read the GPIO pin data value."]
    #[inline(always)]
    pub fn read(self) -> &'a mut W {
        self.variant(GPIO19INCFG_A::READ)
    }
    #[doc = "INTD = 0 - Readback will always be zero value."]
    #[inline(always)]
    pub fn rdzero(self) -> &'a mut W {
        self.variant(GPIO19INCFG_A::RDZERO)
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
#[doc = "GPIO18 interrupt direction.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO18INTD_A {
    #[doc = "0: FNCSEL = 0x1 - nCE polarity active low value."]
    NCELOW = 0,
    #[doc = "1: FNCSEL = 0x1 - nCE polarity active high value."]
    NCEHIGH = 1,
}
impl From<GPIO18INTD_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO18INTD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIO18INTD` reader - GPIO18 interrupt direction."]
pub struct GPIO18INTD_R(crate::FieldReader<bool, GPIO18INTD_A>);
impl GPIO18INTD_R {
    pub(crate) fn new(bits: bool) -> Self {
        GPIO18INTD_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO18INTD_A {
        match self.bits {
            false => GPIO18INTD_A::NCELOW,
            true => GPIO18INTD_A::NCEHIGH,
        }
    }
    #[doc = "Checks if the value of the field is `NCELOW`"]
    #[inline(always)]
    pub fn is_n_celow(&self) -> bool {
        **self == GPIO18INTD_A::NCELOW
    }
    #[doc = "Checks if the value of the field is `NCEHIGH`"]
    #[inline(always)]
    pub fn is_n_cehigh(&self) -> bool {
        **self == GPIO18INTD_A::NCEHIGH
    }
}
impl core::ops::Deref for GPIO18INTD_R {
    type Target = crate::FieldReader<bool, GPIO18INTD_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIO18INTD` writer - GPIO18 interrupt direction."]
pub struct GPIO18INTD_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO18INTD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIO18INTD_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "FNCSEL = 0x1 - nCE polarity active low value."]
    #[inline(always)]
    pub fn n_celow(self) -> &'a mut W {
        self.variant(GPIO18INTD_A::NCELOW)
    }
    #[doc = "FNCSEL = 0x1 - nCE polarity active high value."]
    #[inline(always)]
    pub fn n_cehigh(self) -> &'a mut W {
        self.variant(GPIO18INTD_A::NCEHIGH)
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
#[doc = "GPIO18 output configuration.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum GPIO18OUTCFG_A {
    #[doc = "0: FNCSEL = 0x3 - Output disabled value."]
    DIS = 0,
    #[doc = "1: FNCSEL = 0x3 - Output is push-pull value."]
    PUSHPULL = 1,
    #[doc = "2: FNCSEL = 0x3 - Output is open drain value."]
    OD = 2,
    #[doc = "3: FNCSEL = 0x3 - Output is tri-state value."]
    TS = 3,
}
impl From<GPIO18OUTCFG_A> for u8 {
    #[inline(always)]
    fn from(variant: GPIO18OUTCFG_A) -> Self {
        variant as _
    }
}
#[doc = "Field `GPIO18OUTCFG` reader - GPIO18 output configuration."]
pub struct GPIO18OUTCFG_R(crate::FieldReader<u8, GPIO18OUTCFG_A>);
impl GPIO18OUTCFG_R {
    pub(crate) fn new(bits: u8) -> Self {
        GPIO18OUTCFG_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO18OUTCFG_A {
        match self.bits {
            0 => GPIO18OUTCFG_A::DIS,
            1 => GPIO18OUTCFG_A::PUSHPULL,
            2 => GPIO18OUTCFG_A::OD,
            3 => GPIO18OUTCFG_A::TS,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        **self == GPIO18OUTCFG_A::DIS
    }
    #[doc = "Checks if the value of the field is `PUSHPULL`"]
    #[inline(always)]
    pub fn is_pushpull(&self) -> bool {
        **self == GPIO18OUTCFG_A::PUSHPULL
    }
    #[doc = "Checks if the value of the field is `OD`"]
    #[inline(always)]
    pub fn is_od(&self) -> bool {
        **self == GPIO18OUTCFG_A::OD
    }
    #[doc = "Checks if the value of the field is `TS`"]
    #[inline(always)]
    pub fn is_ts(&self) -> bool {
        **self == GPIO18OUTCFG_A::TS
    }
}
impl core::ops::Deref for GPIO18OUTCFG_R {
    type Target = crate::FieldReader<u8, GPIO18OUTCFG_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIO18OUTCFG` writer - GPIO18 output configuration."]
pub struct GPIO18OUTCFG_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO18OUTCFG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIO18OUTCFG_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "FNCSEL = 0x3 - Output disabled value."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(GPIO18OUTCFG_A::DIS)
    }
    #[doc = "FNCSEL = 0x3 - Output is push-pull value."]
    #[inline(always)]
    pub fn pushpull(self) -> &'a mut W {
        self.variant(GPIO18OUTCFG_A::PUSHPULL)
    }
    #[doc = "FNCSEL = 0x3 - Output is open drain value."]
    #[inline(always)]
    pub fn od(self) -> &'a mut W {
        self.variant(GPIO18OUTCFG_A::OD)
    }
    #[doc = "FNCSEL = 0x3 - Output is tri-state value."]
    #[inline(always)]
    pub fn ts(self) -> &'a mut W {
        self.variant(GPIO18OUTCFG_A::TS)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 9)) | ((value as u32 & 0x03) << 9);
        self.w
    }
}
#[doc = "GPIO18 input enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO18INCFG_A {
    #[doc = "0: Read the GPIO pin data value."]
    READ = 0,
    #[doc = "1: INTD = 0 - Readback will always be zero value."]
    RDZERO = 1,
}
impl From<GPIO18INCFG_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO18INCFG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIO18INCFG` reader - GPIO18 input enable."]
pub struct GPIO18INCFG_R(crate::FieldReader<bool, GPIO18INCFG_A>);
impl GPIO18INCFG_R {
    pub(crate) fn new(bits: bool) -> Self {
        GPIO18INCFG_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO18INCFG_A {
        match self.bits {
            false => GPIO18INCFG_A::READ,
            true => GPIO18INCFG_A::RDZERO,
        }
    }
    #[doc = "Checks if the value of the field is `READ`"]
    #[inline(always)]
    pub fn is_read(&self) -> bool {
        **self == GPIO18INCFG_A::READ
    }
    #[doc = "Checks if the value of the field is `RDZERO`"]
    #[inline(always)]
    pub fn is_rdzero(&self) -> bool {
        **self == GPIO18INCFG_A::RDZERO
    }
}
impl core::ops::Deref for GPIO18INCFG_R {
    type Target = crate::FieldReader<bool, GPIO18INCFG_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIO18INCFG` writer - GPIO18 input enable."]
pub struct GPIO18INCFG_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO18INCFG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIO18INCFG_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Read the GPIO pin data value."]
    #[inline(always)]
    pub fn read(self) -> &'a mut W {
        self.variant(GPIO18INCFG_A::READ)
    }
    #[doc = "INTD = 0 - Readback will always be zero value."]
    #[inline(always)]
    pub fn rdzero(self) -> &'a mut W {
        self.variant(GPIO18INCFG_A::RDZERO)
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
#[doc = "GPIO17 interrupt direction.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO17INTD_A {
    #[doc = "0: FNCSEL = 0x1 - nCE polarity active low value."]
    NCELOW = 0,
    #[doc = "1: FNCSEL = 0x1 - nCE polarity active high value."]
    NCEHIGH = 1,
}
impl From<GPIO17INTD_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO17INTD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIO17INTD` reader - GPIO17 interrupt direction."]
pub struct GPIO17INTD_R(crate::FieldReader<bool, GPIO17INTD_A>);
impl GPIO17INTD_R {
    pub(crate) fn new(bits: bool) -> Self {
        GPIO17INTD_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO17INTD_A {
        match self.bits {
            false => GPIO17INTD_A::NCELOW,
            true => GPIO17INTD_A::NCEHIGH,
        }
    }
    #[doc = "Checks if the value of the field is `NCELOW`"]
    #[inline(always)]
    pub fn is_n_celow(&self) -> bool {
        **self == GPIO17INTD_A::NCELOW
    }
    #[doc = "Checks if the value of the field is `NCEHIGH`"]
    #[inline(always)]
    pub fn is_n_cehigh(&self) -> bool {
        **self == GPIO17INTD_A::NCEHIGH
    }
}
impl core::ops::Deref for GPIO17INTD_R {
    type Target = crate::FieldReader<bool, GPIO17INTD_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIO17INTD` writer - GPIO17 interrupt direction."]
pub struct GPIO17INTD_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO17INTD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIO17INTD_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "FNCSEL = 0x1 - nCE polarity active low value."]
    #[inline(always)]
    pub fn n_celow(self) -> &'a mut W {
        self.variant(GPIO17INTD_A::NCELOW)
    }
    #[doc = "FNCSEL = 0x1 - nCE polarity active high value."]
    #[inline(always)]
    pub fn n_cehigh(self) -> &'a mut W {
        self.variant(GPIO17INTD_A::NCEHIGH)
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
#[doc = "GPIO17 output configuration.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum GPIO17OUTCFG_A {
    #[doc = "0: FNCSEL = 0x3 - Output disabled value."]
    DIS = 0,
    #[doc = "1: FNCSEL = 0x3 - Output is push-pull value."]
    PUSHPULL = 1,
    #[doc = "2: FNCSEL = 0x3 - Output is open drain value."]
    OD = 2,
    #[doc = "3: FNCSEL = 0x3 - Output is tri-state value."]
    TS = 3,
}
impl From<GPIO17OUTCFG_A> for u8 {
    #[inline(always)]
    fn from(variant: GPIO17OUTCFG_A) -> Self {
        variant as _
    }
}
#[doc = "Field `GPIO17OUTCFG` reader - GPIO17 output configuration."]
pub struct GPIO17OUTCFG_R(crate::FieldReader<u8, GPIO17OUTCFG_A>);
impl GPIO17OUTCFG_R {
    pub(crate) fn new(bits: u8) -> Self {
        GPIO17OUTCFG_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO17OUTCFG_A {
        match self.bits {
            0 => GPIO17OUTCFG_A::DIS,
            1 => GPIO17OUTCFG_A::PUSHPULL,
            2 => GPIO17OUTCFG_A::OD,
            3 => GPIO17OUTCFG_A::TS,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        **self == GPIO17OUTCFG_A::DIS
    }
    #[doc = "Checks if the value of the field is `PUSHPULL`"]
    #[inline(always)]
    pub fn is_pushpull(&self) -> bool {
        **self == GPIO17OUTCFG_A::PUSHPULL
    }
    #[doc = "Checks if the value of the field is `OD`"]
    #[inline(always)]
    pub fn is_od(&self) -> bool {
        **self == GPIO17OUTCFG_A::OD
    }
    #[doc = "Checks if the value of the field is `TS`"]
    #[inline(always)]
    pub fn is_ts(&self) -> bool {
        **self == GPIO17OUTCFG_A::TS
    }
}
impl core::ops::Deref for GPIO17OUTCFG_R {
    type Target = crate::FieldReader<u8, GPIO17OUTCFG_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIO17OUTCFG` writer - GPIO17 output configuration."]
pub struct GPIO17OUTCFG_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO17OUTCFG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIO17OUTCFG_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "FNCSEL = 0x3 - Output disabled value."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(GPIO17OUTCFG_A::DIS)
    }
    #[doc = "FNCSEL = 0x3 - Output is push-pull value."]
    #[inline(always)]
    pub fn pushpull(self) -> &'a mut W {
        self.variant(GPIO17OUTCFG_A::PUSHPULL)
    }
    #[doc = "FNCSEL = 0x3 - Output is open drain value."]
    #[inline(always)]
    pub fn od(self) -> &'a mut W {
        self.variant(GPIO17OUTCFG_A::OD)
    }
    #[doc = "FNCSEL = 0x3 - Output is tri-state value."]
    #[inline(always)]
    pub fn ts(self) -> &'a mut W {
        self.variant(GPIO17OUTCFG_A::TS)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 5)) | ((value as u32 & 0x03) << 5);
        self.w
    }
}
#[doc = "GPIO17 input enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO17INCFG_A {
    #[doc = "0: Read the GPIO pin data value."]
    READ = 0,
    #[doc = "1: INTD = 0 - Readback will always be zero value."]
    RDZERO = 1,
}
impl From<GPIO17INCFG_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO17INCFG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIO17INCFG` reader - GPIO17 input enable."]
pub struct GPIO17INCFG_R(crate::FieldReader<bool, GPIO17INCFG_A>);
impl GPIO17INCFG_R {
    pub(crate) fn new(bits: bool) -> Self {
        GPIO17INCFG_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO17INCFG_A {
        match self.bits {
            false => GPIO17INCFG_A::READ,
            true => GPIO17INCFG_A::RDZERO,
        }
    }
    #[doc = "Checks if the value of the field is `READ`"]
    #[inline(always)]
    pub fn is_read(&self) -> bool {
        **self == GPIO17INCFG_A::READ
    }
    #[doc = "Checks if the value of the field is `RDZERO`"]
    #[inline(always)]
    pub fn is_rdzero(&self) -> bool {
        **self == GPIO17INCFG_A::RDZERO
    }
}
impl core::ops::Deref for GPIO17INCFG_R {
    type Target = crate::FieldReader<bool, GPIO17INCFG_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIO17INCFG` writer - GPIO17 input enable."]
pub struct GPIO17INCFG_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO17INCFG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIO17INCFG_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Read the GPIO pin data value."]
    #[inline(always)]
    pub fn read(self) -> &'a mut W {
        self.variant(GPIO17INCFG_A::READ)
    }
    #[doc = "INTD = 0 - Readback will always be zero value."]
    #[inline(always)]
    pub fn rdzero(self) -> &'a mut W {
        self.variant(GPIO17INCFG_A::RDZERO)
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
#[doc = "GPIO16 interrupt direction.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO16INTD_A {
    #[doc = "0: FNCSEL = 0x1 - nCE polarity active low value."]
    NCELOW = 0,
    #[doc = "1: FNCSEL = 0x1 - nCE polarity active high value."]
    NCEHIGH = 1,
}
impl From<GPIO16INTD_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO16INTD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIO16INTD` reader - GPIO16 interrupt direction."]
pub struct GPIO16INTD_R(crate::FieldReader<bool, GPIO16INTD_A>);
impl GPIO16INTD_R {
    pub(crate) fn new(bits: bool) -> Self {
        GPIO16INTD_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO16INTD_A {
        match self.bits {
            false => GPIO16INTD_A::NCELOW,
            true => GPIO16INTD_A::NCEHIGH,
        }
    }
    #[doc = "Checks if the value of the field is `NCELOW`"]
    #[inline(always)]
    pub fn is_n_celow(&self) -> bool {
        **self == GPIO16INTD_A::NCELOW
    }
    #[doc = "Checks if the value of the field is `NCEHIGH`"]
    #[inline(always)]
    pub fn is_n_cehigh(&self) -> bool {
        **self == GPIO16INTD_A::NCEHIGH
    }
}
impl core::ops::Deref for GPIO16INTD_R {
    type Target = crate::FieldReader<bool, GPIO16INTD_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIO16INTD` writer - GPIO16 interrupt direction."]
pub struct GPIO16INTD_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO16INTD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIO16INTD_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "FNCSEL = 0x1 - nCE polarity active low value."]
    #[inline(always)]
    pub fn n_celow(self) -> &'a mut W {
        self.variant(GPIO16INTD_A::NCELOW)
    }
    #[doc = "FNCSEL = 0x1 - nCE polarity active high value."]
    #[inline(always)]
    pub fn n_cehigh(self) -> &'a mut W {
        self.variant(GPIO16INTD_A::NCEHIGH)
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
#[doc = "GPIO16 output configuration.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum GPIO16OUTCFG_A {
    #[doc = "0: FNCSEL = 0x3 - Output disabled value."]
    DIS = 0,
    #[doc = "1: FNCSEL = 0x3 - Output is push-pull value."]
    PUSHPULL = 1,
    #[doc = "2: FNCSEL = 0x3 - Output is open drain value."]
    OD = 2,
    #[doc = "3: FNCSEL = 0x3 - Output is tri-state value."]
    TS = 3,
}
impl From<GPIO16OUTCFG_A> for u8 {
    #[inline(always)]
    fn from(variant: GPIO16OUTCFG_A) -> Self {
        variant as _
    }
}
#[doc = "Field `GPIO16OUTCFG` reader - GPIO16 output configuration."]
pub struct GPIO16OUTCFG_R(crate::FieldReader<u8, GPIO16OUTCFG_A>);
impl GPIO16OUTCFG_R {
    pub(crate) fn new(bits: u8) -> Self {
        GPIO16OUTCFG_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO16OUTCFG_A {
        match self.bits {
            0 => GPIO16OUTCFG_A::DIS,
            1 => GPIO16OUTCFG_A::PUSHPULL,
            2 => GPIO16OUTCFG_A::OD,
            3 => GPIO16OUTCFG_A::TS,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        **self == GPIO16OUTCFG_A::DIS
    }
    #[doc = "Checks if the value of the field is `PUSHPULL`"]
    #[inline(always)]
    pub fn is_pushpull(&self) -> bool {
        **self == GPIO16OUTCFG_A::PUSHPULL
    }
    #[doc = "Checks if the value of the field is `OD`"]
    #[inline(always)]
    pub fn is_od(&self) -> bool {
        **self == GPIO16OUTCFG_A::OD
    }
    #[doc = "Checks if the value of the field is `TS`"]
    #[inline(always)]
    pub fn is_ts(&self) -> bool {
        **self == GPIO16OUTCFG_A::TS
    }
}
impl core::ops::Deref for GPIO16OUTCFG_R {
    type Target = crate::FieldReader<u8, GPIO16OUTCFG_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIO16OUTCFG` writer - GPIO16 output configuration."]
pub struct GPIO16OUTCFG_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO16OUTCFG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIO16OUTCFG_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "FNCSEL = 0x3 - Output disabled value."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(GPIO16OUTCFG_A::DIS)
    }
    #[doc = "FNCSEL = 0x3 - Output is push-pull value."]
    #[inline(always)]
    pub fn pushpull(self) -> &'a mut W {
        self.variant(GPIO16OUTCFG_A::PUSHPULL)
    }
    #[doc = "FNCSEL = 0x3 - Output is open drain value."]
    #[inline(always)]
    pub fn od(self) -> &'a mut W {
        self.variant(GPIO16OUTCFG_A::OD)
    }
    #[doc = "FNCSEL = 0x3 - Output is tri-state value."]
    #[inline(always)]
    pub fn ts(self) -> &'a mut W {
        self.variant(GPIO16OUTCFG_A::TS)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 1)) | ((value as u32 & 0x03) << 1);
        self.w
    }
}
#[doc = "GPIO16 input enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO16INCFG_A {
    #[doc = "0: Read the GPIO pin data value."]
    READ = 0,
    #[doc = "1: INTD = 0 - Readback will always be zero value."]
    RDZERO = 1,
}
impl From<GPIO16INCFG_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO16INCFG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIO16INCFG` reader - GPIO16 input enable."]
pub struct GPIO16INCFG_R(crate::FieldReader<bool, GPIO16INCFG_A>);
impl GPIO16INCFG_R {
    pub(crate) fn new(bits: bool) -> Self {
        GPIO16INCFG_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO16INCFG_A {
        match self.bits {
            false => GPIO16INCFG_A::READ,
            true => GPIO16INCFG_A::RDZERO,
        }
    }
    #[doc = "Checks if the value of the field is `READ`"]
    #[inline(always)]
    pub fn is_read(&self) -> bool {
        **self == GPIO16INCFG_A::READ
    }
    #[doc = "Checks if the value of the field is `RDZERO`"]
    #[inline(always)]
    pub fn is_rdzero(&self) -> bool {
        **self == GPIO16INCFG_A::RDZERO
    }
}
impl core::ops::Deref for GPIO16INCFG_R {
    type Target = crate::FieldReader<bool, GPIO16INCFG_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIO16INCFG` writer - GPIO16 input enable."]
pub struct GPIO16INCFG_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO16INCFG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIO16INCFG_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Read the GPIO pin data value."]
    #[inline(always)]
    pub fn read(self) -> &'a mut W {
        self.variant(GPIO16INCFG_A::READ)
    }
    #[doc = "INTD = 0 - Readback will always be zero value."]
    #[inline(always)]
    pub fn rdzero(self) -> &'a mut W {
        self.variant(GPIO16INCFG_A::RDZERO)
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
    #[doc = "Bit 31 - GPIO23 interrupt direction."]
    #[inline(always)]
    pub fn gpio23intd(&self) -> GPIO23INTD_R {
        GPIO23INTD_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bits 29:30 - GPIO23 output configuration."]
    #[inline(always)]
    pub fn gpio23outcfg(&self) -> GPIO23OUTCFG_R {
        GPIO23OUTCFG_R::new(((self.bits >> 29) & 0x03) as u8)
    }
    #[doc = "Bit 28 - GPIO23 input enable."]
    #[inline(always)]
    pub fn gpio23incfg(&self) -> GPIO23INCFG_R {
        GPIO23INCFG_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 27 - GPIO22 interrupt direction."]
    #[inline(always)]
    pub fn gpio22intd(&self) -> GPIO22INTD_R {
        GPIO22INTD_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bits 25:26 - GPIO22 output configuration."]
    #[inline(always)]
    pub fn gpio22outcfg(&self) -> GPIO22OUTCFG_R {
        GPIO22OUTCFG_R::new(((self.bits >> 25) & 0x03) as u8)
    }
    #[doc = "Bit 24 - GPIO22 input enable."]
    #[inline(always)]
    pub fn gpio22incfg(&self) -> GPIO22INCFG_R {
        GPIO22INCFG_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 23 - GPIO21 interrupt direction."]
    #[inline(always)]
    pub fn gpio21intd(&self) -> GPIO21INTD_R {
        GPIO21INTD_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bits 21:22 - GPIO21 output configuration."]
    #[inline(always)]
    pub fn gpio21outcfg(&self) -> GPIO21OUTCFG_R {
        GPIO21OUTCFG_R::new(((self.bits >> 21) & 0x03) as u8)
    }
    #[doc = "Bit 20 - GPIO21 input enable."]
    #[inline(always)]
    pub fn gpio21incfg(&self) -> GPIO21INCFG_R {
        GPIO21INCFG_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 19 - GPIO20 interrupt direction."]
    #[inline(always)]
    pub fn gpio20intd(&self) -> GPIO20INTD_R {
        GPIO20INTD_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bits 17:18 - GPIO20 output configuration."]
    #[inline(always)]
    pub fn gpio20outcfg(&self) -> GPIO20OUTCFG_R {
        GPIO20OUTCFG_R::new(((self.bits >> 17) & 0x03) as u8)
    }
    #[doc = "Bit 16 - GPIO20 input enable."]
    #[inline(always)]
    pub fn gpio20incfg(&self) -> GPIO20INCFG_R {
        GPIO20INCFG_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 15 - GPIO19 interrupt direction."]
    #[inline(always)]
    pub fn gpio19intd(&self) -> GPIO19INTD_R {
        GPIO19INTD_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bits 13:14 - GPIO19 output configuration."]
    #[inline(always)]
    pub fn gpio19outcfg(&self) -> GPIO19OUTCFG_R {
        GPIO19OUTCFG_R::new(((self.bits >> 13) & 0x03) as u8)
    }
    #[doc = "Bit 12 - GPIO19 input enable."]
    #[inline(always)]
    pub fn gpio19incfg(&self) -> GPIO19INCFG_R {
        GPIO19INCFG_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11 - GPIO18 interrupt direction."]
    #[inline(always)]
    pub fn gpio18intd(&self) -> GPIO18INTD_R {
        GPIO18INTD_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bits 9:10 - GPIO18 output configuration."]
    #[inline(always)]
    pub fn gpio18outcfg(&self) -> GPIO18OUTCFG_R {
        GPIO18OUTCFG_R::new(((self.bits >> 9) & 0x03) as u8)
    }
    #[doc = "Bit 8 - GPIO18 input enable."]
    #[inline(always)]
    pub fn gpio18incfg(&self) -> GPIO18INCFG_R {
        GPIO18INCFG_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7 - GPIO17 interrupt direction."]
    #[inline(always)]
    pub fn gpio17intd(&self) -> GPIO17INTD_R {
        GPIO17INTD_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 5:6 - GPIO17 output configuration."]
    #[inline(always)]
    pub fn gpio17outcfg(&self) -> GPIO17OUTCFG_R {
        GPIO17OUTCFG_R::new(((self.bits >> 5) & 0x03) as u8)
    }
    #[doc = "Bit 4 - GPIO17 input enable."]
    #[inline(always)]
    pub fn gpio17incfg(&self) -> GPIO17INCFG_R {
        GPIO17INCFG_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - GPIO16 interrupt direction."]
    #[inline(always)]
    pub fn gpio16intd(&self) -> GPIO16INTD_R {
        GPIO16INTD_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bits 1:2 - GPIO16 output configuration."]
    #[inline(always)]
    pub fn gpio16outcfg(&self) -> GPIO16OUTCFG_R {
        GPIO16OUTCFG_R::new(((self.bits >> 1) & 0x03) as u8)
    }
    #[doc = "Bit 0 - GPIO16 input enable."]
    #[inline(always)]
    pub fn gpio16incfg(&self) -> GPIO16INCFG_R {
        GPIO16INCFG_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 31 - GPIO23 interrupt direction."]
    #[inline(always)]
    pub fn gpio23intd(&mut self) -> GPIO23INTD_W {
        GPIO23INTD_W { w: self }
    }
    #[doc = "Bits 29:30 - GPIO23 output configuration."]
    #[inline(always)]
    pub fn gpio23outcfg(&mut self) -> GPIO23OUTCFG_W {
        GPIO23OUTCFG_W { w: self }
    }
    #[doc = "Bit 28 - GPIO23 input enable."]
    #[inline(always)]
    pub fn gpio23incfg(&mut self) -> GPIO23INCFG_W {
        GPIO23INCFG_W { w: self }
    }
    #[doc = "Bit 27 - GPIO22 interrupt direction."]
    #[inline(always)]
    pub fn gpio22intd(&mut self) -> GPIO22INTD_W {
        GPIO22INTD_W { w: self }
    }
    #[doc = "Bits 25:26 - GPIO22 output configuration."]
    #[inline(always)]
    pub fn gpio22outcfg(&mut self) -> GPIO22OUTCFG_W {
        GPIO22OUTCFG_W { w: self }
    }
    #[doc = "Bit 24 - GPIO22 input enable."]
    #[inline(always)]
    pub fn gpio22incfg(&mut self) -> GPIO22INCFG_W {
        GPIO22INCFG_W { w: self }
    }
    #[doc = "Bit 23 - GPIO21 interrupt direction."]
    #[inline(always)]
    pub fn gpio21intd(&mut self) -> GPIO21INTD_W {
        GPIO21INTD_W { w: self }
    }
    #[doc = "Bits 21:22 - GPIO21 output configuration."]
    #[inline(always)]
    pub fn gpio21outcfg(&mut self) -> GPIO21OUTCFG_W {
        GPIO21OUTCFG_W { w: self }
    }
    #[doc = "Bit 20 - GPIO21 input enable."]
    #[inline(always)]
    pub fn gpio21incfg(&mut self) -> GPIO21INCFG_W {
        GPIO21INCFG_W { w: self }
    }
    #[doc = "Bit 19 - GPIO20 interrupt direction."]
    #[inline(always)]
    pub fn gpio20intd(&mut self) -> GPIO20INTD_W {
        GPIO20INTD_W { w: self }
    }
    #[doc = "Bits 17:18 - GPIO20 output configuration."]
    #[inline(always)]
    pub fn gpio20outcfg(&mut self) -> GPIO20OUTCFG_W {
        GPIO20OUTCFG_W { w: self }
    }
    #[doc = "Bit 16 - GPIO20 input enable."]
    #[inline(always)]
    pub fn gpio20incfg(&mut self) -> GPIO20INCFG_W {
        GPIO20INCFG_W { w: self }
    }
    #[doc = "Bit 15 - GPIO19 interrupt direction."]
    #[inline(always)]
    pub fn gpio19intd(&mut self) -> GPIO19INTD_W {
        GPIO19INTD_W { w: self }
    }
    #[doc = "Bits 13:14 - GPIO19 output configuration."]
    #[inline(always)]
    pub fn gpio19outcfg(&mut self) -> GPIO19OUTCFG_W {
        GPIO19OUTCFG_W { w: self }
    }
    #[doc = "Bit 12 - GPIO19 input enable."]
    #[inline(always)]
    pub fn gpio19incfg(&mut self) -> GPIO19INCFG_W {
        GPIO19INCFG_W { w: self }
    }
    #[doc = "Bit 11 - GPIO18 interrupt direction."]
    #[inline(always)]
    pub fn gpio18intd(&mut self) -> GPIO18INTD_W {
        GPIO18INTD_W { w: self }
    }
    #[doc = "Bits 9:10 - GPIO18 output configuration."]
    #[inline(always)]
    pub fn gpio18outcfg(&mut self) -> GPIO18OUTCFG_W {
        GPIO18OUTCFG_W { w: self }
    }
    #[doc = "Bit 8 - GPIO18 input enable."]
    #[inline(always)]
    pub fn gpio18incfg(&mut self) -> GPIO18INCFG_W {
        GPIO18INCFG_W { w: self }
    }
    #[doc = "Bit 7 - GPIO17 interrupt direction."]
    #[inline(always)]
    pub fn gpio17intd(&mut self) -> GPIO17INTD_W {
        GPIO17INTD_W { w: self }
    }
    #[doc = "Bits 5:6 - GPIO17 output configuration."]
    #[inline(always)]
    pub fn gpio17outcfg(&mut self) -> GPIO17OUTCFG_W {
        GPIO17OUTCFG_W { w: self }
    }
    #[doc = "Bit 4 - GPIO17 input enable."]
    #[inline(always)]
    pub fn gpio17incfg(&mut self) -> GPIO17INCFG_W {
        GPIO17INCFG_W { w: self }
    }
    #[doc = "Bit 3 - GPIO16 interrupt direction."]
    #[inline(always)]
    pub fn gpio16intd(&mut self) -> GPIO16INTD_W {
        GPIO16INTD_W { w: self }
    }
    #[doc = "Bits 1:2 - GPIO16 output configuration."]
    #[inline(always)]
    pub fn gpio16outcfg(&mut self) -> GPIO16OUTCFG_W {
        GPIO16OUTCFG_W { w: self }
    }
    #[doc = "Bit 0 - GPIO16 input enable."]
    #[inline(always)]
    pub fn gpio16incfg(&mut self) -> GPIO16INCFG_W {
        GPIO16INCFG_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPIO Configuration Register C (Pads 16-23)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cfgc](index.html) module"]
pub struct CFGC_SPEC;
impl crate::RegisterSpec for CFGC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cfgc::R](R) reader structure"]
impl crate::Readable for CFGC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cfgc::W](W) writer structure"]
impl crate::Writable for CFGC_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CFGC to value 0x0011_0000"]
impl crate::Resettable for CFGC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0011_0000
    }
}
