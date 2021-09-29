#[doc = "Register `CFGE` reader"]
pub struct R(crate::R<CFGE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CFGE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CFGE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CFGE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CFGE` writer"]
pub struct W(crate::W<CFGE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CFGE_SPEC>;
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
impl From<crate::W<CFGE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CFGE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "GPIO39 interrupt direction.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO39INTD_A {
    #[doc = "0: INCFG = 1 - No interrupt on GPIO transition value."]
    INTDIS = 0,
    #[doc = "1: INCFG = 1 - Interrupt on either low to high or high to low GPIO transition value."]
    INTBOTH = 1,
}
impl From<GPIO39INTD_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO39INTD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIO39INTD` reader - GPIO39 interrupt direction."]
pub struct GPIO39INTD_R(crate::FieldReader<bool, GPIO39INTD_A>);
impl GPIO39INTD_R {
    pub(crate) fn new(bits: bool) -> Self {
        GPIO39INTD_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO39INTD_A {
        match self.bits {
            false => GPIO39INTD_A::INTDIS,
            true => GPIO39INTD_A::INTBOTH,
        }
    }
    #[doc = "Checks if the value of the field is `INTDIS`"]
    #[inline(always)]
    pub fn is_intdis(&self) -> bool {
        **self == GPIO39INTD_A::INTDIS
    }
    #[doc = "Checks if the value of the field is `INTBOTH`"]
    #[inline(always)]
    pub fn is_intboth(&self) -> bool {
        **self == GPIO39INTD_A::INTBOTH
    }
}
impl core::ops::Deref for GPIO39INTD_R {
    type Target = crate::FieldReader<bool, GPIO39INTD_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIO39INTD` writer - GPIO39 interrupt direction."]
pub struct GPIO39INTD_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO39INTD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIO39INTD_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "INCFG = 1 - No interrupt on GPIO transition value."]
    #[inline(always)]
    pub fn intdis(self) -> &'a mut W {
        self.variant(GPIO39INTD_A::INTDIS)
    }
    #[doc = "INCFG = 1 - Interrupt on either low to high or high to low GPIO transition value."]
    #[inline(always)]
    pub fn intboth(self) -> &'a mut W {
        self.variant(GPIO39INTD_A::INTBOTH)
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
#[doc = "GPIO39 output configuration.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum GPIO39OUTCFG_A {
    #[doc = "0: FNCSEL = 0x3 - Output disabled value."]
    DIS = 0,
    #[doc = "1: FNCSEL = 0x3 - Output is push-pull value."]
    PUSHPULL = 1,
    #[doc = "2: FNCSEL = 0x3 - Output is open drain value."]
    OD = 2,
    #[doc = "3: FNCSEL = 0x3 - Output is tri-state value."]
    TS = 3,
}
impl From<GPIO39OUTCFG_A> for u8 {
    #[inline(always)]
    fn from(variant: GPIO39OUTCFG_A) -> Self {
        variant as _
    }
}
#[doc = "Field `GPIO39OUTCFG` reader - GPIO39 output configuration."]
pub struct GPIO39OUTCFG_R(crate::FieldReader<u8, GPIO39OUTCFG_A>);
impl GPIO39OUTCFG_R {
    pub(crate) fn new(bits: u8) -> Self {
        GPIO39OUTCFG_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO39OUTCFG_A {
        match self.bits {
            0 => GPIO39OUTCFG_A::DIS,
            1 => GPIO39OUTCFG_A::PUSHPULL,
            2 => GPIO39OUTCFG_A::OD,
            3 => GPIO39OUTCFG_A::TS,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        **self == GPIO39OUTCFG_A::DIS
    }
    #[doc = "Checks if the value of the field is `PUSHPULL`"]
    #[inline(always)]
    pub fn is_pushpull(&self) -> bool {
        **self == GPIO39OUTCFG_A::PUSHPULL
    }
    #[doc = "Checks if the value of the field is `OD`"]
    #[inline(always)]
    pub fn is_od(&self) -> bool {
        **self == GPIO39OUTCFG_A::OD
    }
    #[doc = "Checks if the value of the field is `TS`"]
    #[inline(always)]
    pub fn is_ts(&self) -> bool {
        **self == GPIO39OUTCFG_A::TS
    }
}
impl core::ops::Deref for GPIO39OUTCFG_R {
    type Target = crate::FieldReader<u8, GPIO39OUTCFG_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIO39OUTCFG` writer - GPIO39 output configuration."]
pub struct GPIO39OUTCFG_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO39OUTCFG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIO39OUTCFG_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "FNCSEL = 0x3 - Output disabled value."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(GPIO39OUTCFG_A::DIS)
    }
    #[doc = "FNCSEL = 0x3 - Output is push-pull value."]
    #[inline(always)]
    pub fn pushpull(self) -> &'a mut W {
        self.variant(GPIO39OUTCFG_A::PUSHPULL)
    }
    #[doc = "FNCSEL = 0x3 - Output is open drain value."]
    #[inline(always)]
    pub fn od(self) -> &'a mut W {
        self.variant(GPIO39OUTCFG_A::OD)
    }
    #[doc = "FNCSEL = 0x3 - Output is tri-state value."]
    #[inline(always)]
    pub fn ts(self) -> &'a mut W {
        self.variant(GPIO39OUTCFG_A::TS)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 29)) | ((value as u32 & 0x03) << 29);
        self.w
    }
}
#[doc = "GPIO39 input enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO39INCFG_A {
    #[doc = "0: Read the GPIO pin data value."]
    READ = 0,
    #[doc = "1: INTD = 0 - Readback will always be zero value."]
    RDZERO = 1,
}
impl From<GPIO39INCFG_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO39INCFG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIO39INCFG` reader - GPIO39 input enable."]
pub struct GPIO39INCFG_R(crate::FieldReader<bool, GPIO39INCFG_A>);
impl GPIO39INCFG_R {
    pub(crate) fn new(bits: bool) -> Self {
        GPIO39INCFG_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO39INCFG_A {
        match self.bits {
            false => GPIO39INCFG_A::READ,
            true => GPIO39INCFG_A::RDZERO,
        }
    }
    #[doc = "Checks if the value of the field is `READ`"]
    #[inline(always)]
    pub fn is_read(&self) -> bool {
        **self == GPIO39INCFG_A::READ
    }
    #[doc = "Checks if the value of the field is `RDZERO`"]
    #[inline(always)]
    pub fn is_rdzero(&self) -> bool {
        **self == GPIO39INCFG_A::RDZERO
    }
}
impl core::ops::Deref for GPIO39INCFG_R {
    type Target = crate::FieldReader<bool, GPIO39INCFG_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIO39INCFG` writer - GPIO39 input enable."]
pub struct GPIO39INCFG_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO39INCFG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIO39INCFG_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Read the GPIO pin data value."]
    #[inline(always)]
    pub fn read(self) -> &'a mut W {
        self.variant(GPIO39INCFG_A::READ)
    }
    #[doc = "INTD = 0 - Readback will always be zero value."]
    #[inline(always)]
    pub fn rdzero(self) -> &'a mut W {
        self.variant(GPIO39INCFG_A::RDZERO)
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
#[doc = "GPIO38 interrupt direction.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO38INTD_A {
    #[doc = "0: FNCSEL = 0x1 - nCE polarity active low value."]
    NCELOW = 0,
    #[doc = "1: FNCSEL = 0x1 - nCE polarity active high value."]
    NCEHIGH = 1,
}
impl From<GPIO38INTD_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO38INTD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIO38INTD` reader - GPIO38 interrupt direction."]
pub struct GPIO38INTD_R(crate::FieldReader<bool, GPIO38INTD_A>);
impl GPIO38INTD_R {
    pub(crate) fn new(bits: bool) -> Self {
        GPIO38INTD_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO38INTD_A {
        match self.bits {
            false => GPIO38INTD_A::NCELOW,
            true => GPIO38INTD_A::NCEHIGH,
        }
    }
    #[doc = "Checks if the value of the field is `NCELOW`"]
    #[inline(always)]
    pub fn is_n_celow(&self) -> bool {
        **self == GPIO38INTD_A::NCELOW
    }
    #[doc = "Checks if the value of the field is `NCEHIGH`"]
    #[inline(always)]
    pub fn is_n_cehigh(&self) -> bool {
        **self == GPIO38INTD_A::NCEHIGH
    }
}
impl core::ops::Deref for GPIO38INTD_R {
    type Target = crate::FieldReader<bool, GPIO38INTD_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIO38INTD` writer - GPIO38 interrupt direction."]
pub struct GPIO38INTD_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO38INTD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIO38INTD_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "FNCSEL = 0x1 - nCE polarity active low value."]
    #[inline(always)]
    pub fn n_celow(self) -> &'a mut W {
        self.variant(GPIO38INTD_A::NCELOW)
    }
    #[doc = "FNCSEL = 0x1 - nCE polarity active high value."]
    #[inline(always)]
    pub fn n_cehigh(self) -> &'a mut W {
        self.variant(GPIO38INTD_A::NCEHIGH)
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
#[doc = "GPIO38 output configuration.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum GPIO38OUTCFG_A {
    #[doc = "0: FNCSEL = 0x3 - Output disabled value."]
    DIS = 0,
    #[doc = "1: FNCSEL = 0x3 - Output is push-pull value."]
    PUSHPULL = 1,
    #[doc = "2: FNCSEL = 0x3 - Output is open drain value."]
    OD = 2,
    #[doc = "3: FNCSEL = 0x3 - Output is tri-state value."]
    TS = 3,
}
impl From<GPIO38OUTCFG_A> for u8 {
    #[inline(always)]
    fn from(variant: GPIO38OUTCFG_A) -> Self {
        variant as _
    }
}
#[doc = "Field `GPIO38OUTCFG` reader - GPIO38 output configuration."]
pub struct GPIO38OUTCFG_R(crate::FieldReader<u8, GPIO38OUTCFG_A>);
impl GPIO38OUTCFG_R {
    pub(crate) fn new(bits: u8) -> Self {
        GPIO38OUTCFG_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO38OUTCFG_A {
        match self.bits {
            0 => GPIO38OUTCFG_A::DIS,
            1 => GPIO38OUTCFG_A::PUSHPULL,
            2 => GPIO38OUTCFG_A::OD,
            3 => GPIO38OUTCFG_A::TS,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        **self == GPIO38OUTCFG_A::DIS
    }
    #[doc = "Checks if the value of the field is `PUSHPULL`"]
    #[inline(always)]
    pub fn is_pushpull(&self) -> bool {
        **self == GPIO38OUTCFG_A::PUSHPULL
    }
    #[doc = "Checks if the value of the field is `OD`"]
    #[inline(always)]
    pub fn is_od(&self) -> bool {
        **self == GPIO38OUTCFG_A::OD
    }
    #[doc = "Checks if the value of the field is `TS`"]
    #[inline(always)]
    pub fn is_ts(&self) -> bool {
        **self == GPIO38OUTCFG_A::TS
    }
}
impl core::ops::Deref for GPIO38OUTCFG_R {
    type Target = crate::FieldReader<u8, GPIO38OUTCFG_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIO38OUTCFG` writer - GPIO38 output configuration."]
pub struct GPIO38OUTCFG_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO38OUTCFG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIO38OUTCFG_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "FNCSEL = 0x3 - Output disabled value."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(GPIO38OUTCFG_A::DIS)
    }
    #[doc = "FNCSEL = 0x3 - Output is push-pull value."]
    #[inline(always)]
    pub fn pushpull(self) -> &'a mut W {
        self.variant(GPIO38OUTCFG_A::PUSHPULL)
    }
    #[doc = "FNCSEL = 0x3 - Output is open drain value."]
    #[inline(always)]
    pub fn od(self) -> &'a mut W {
        self.variant(GPIO38OUTCFG_A::OD)
    }
    #[doc = "FNCSEL = 0x3 - Output is tri-state value."]
    #[inline(always)]
    pub fn ts(self) -> &'a mut W {
        self.variant(GPIO38OUTCFG_A::TS)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 25)) | ((value as u32 & 0x03) << 25);
        self.w
    }
}
#[doc = "GPIO38 input enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO38INCFG_A {
    #[doc = "0: Read the GPIO pin data value."]
    READ = 0,
    #[doc = "1: INTD = 0 - Readback will always be zero value."]
    RDZERO = 1,
}
impl From<GPIO38INCFG_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO38INCFG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIO38INCFG` reader - GPIO38 input enable."]
pub struct GPIO38INCFG_R(crate::FieldReader<bool, GPIO38INCFG_A>);
impl GPIO38INCFG_R {
    pub(crate) fn new(bits: bool) -> Self {
        GPIO38INCFG_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO38INCFG_A {
        match self.bits {
            false => GPIO38INCFG_A::READ,
            true => GPIO38INCFG_A::RDZERO,
        }
    }
    #[doc = "Checks if the value of the field is `READ`"]
    #[inline(always)]
    pub fn is_read(&self) -> bool {
        **self == GPIO38INCFG_A::READ
    }
    #[doc = "Checks if the value of the field is `RDZERO`"]
    #[inline(always)]
    pub fn is_rdzero(&self) -> bool {
        **self == GPIO38INCFG_A::RDZERO
    }
}
impl core::ops::Deref for GPIO38INCFG_R {
    type Target = crate::FieldReader<bool, GPIO38INCFG_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIO38INCFG` writer - GPIO38 input enable."]
pub struct GPIO38INCFG_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO38INCFG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIO38INCFG_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Read the GPIO pin data value."]
    #[inline(always)]
    pub fn read(self) -> &'a mut W {
        self.variant(GPIO38INCFG_A::READ)
    }
    #[doc = "INTD = 0 - Readback will always be zero value."]
    #[inline(always)]
    pub fn rdzero(self) -> &'a mut W {
        self.variant(GPIO38INCFG_A::RDZERO)
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
#[doc = "GPIO37 interrupt direction.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO37INTD_A {
    #[doc = "0: FNCSEL = 0x1 - nCE polarity active low value."]
    NCELOW = 0,
    #[doc = "1: FNCSEL = 0x1 - nCE polarity active high value."]
    NCEHIGH = 1,
}
impl From<GPIO37INTD_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO37INTD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIO37INTD` reader - GPIO37 interrupt direction."]
pub struct GPIO37INTD_R(crate::FieldReader<bool, GPIO37INTD_A>);
impl GPIO37INTD_R {
    pub(crate) fn new(bits: bool) -> Self {
        GPIO37INTD_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO37INTD_A {
        match self.bits {
            false => GPIO37INTD_A::NCELOW,
            true => GPIO37INTD_A::NCEHIGH,
        }
    }
    #[doc = "Checks if the value of the field is `NCELOW`"]
    #[inline(always)]
    pub fn is_n_celow(&self) -> bool {
        **self == GPIO37INTD_A::NCELOW
    }
    #[doc = "Checks if the value of the field is `NCEHIGH`"]
    #[inline(always)]
    pub fn is_n_cehigh(&self) -> bool {
        **self == GPIO37INTD_A::NCEHIGH
    }
}
impl core::ops::Deref for GPIO37INTD_R {
    type Target = crate::FieldReader<bool, GPIO37INTD_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIO37INTD` writer - GPIO37 interrupt direction."]
pub struct GPIO37INTD_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO37INTD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIO37INTD_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "FNCSEL = 0x1 - nCE polarity active low value."]
    #[inline(always)]
    pub fn n_celow(self) -> &'a mut W {
        self.variant(GPIO37INTD_A::NCELOW)
    }
    #[doc = "FNCSEL = 0x1 - nCE polarity active high value."]
    #[inline(always)]
    pub fn n_cehigh(self) -> &'a mut W {
        self.variant(GPIO37INTD_A::NCEHIGH)
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
#[doc = "GPIO37 output configuration.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum GPIO37OUTCFG_A {
    #[doc = "0: FNCSEL = 0x3 - Output disabled value."]
    DIS = 0,
    #[doc = "1: FNCSEL = 0x3 - Output is push-pull value."]
    PUSHPULL = 1,
    #[doc = "2: FNCSEL = 0x3 - Output is open drain value."]
    OD = 2,
    #[doc = "3: FNCSEL = 0x3 - Output is tri-state value."]
    TS = 3,
}
impl From<GPIO37OUTCFG_A> for u8 {
    #[inline(always)]
    fn from(variant: GPIO37OUTCFG_A) -> Self {
        variant as _
    }
}
#[doc = "Field `GPIO37OUTCFG` reader - GPIO37 output configuration."]
pub struct GPIO37OUTCFG_R(crate::FieldReader<u8, GPIO37OUTCFG_A>);
impl GPIO37OUTCFG_R {
    pub(crate) fn new(bits: u8) -> Self {
        GPIO37OUTCFG_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO37OUTCFG_A {
        match self.bits {
            0 => GPIO37OUTCFG_A::DIS,
            1 => GPIO37OUTCFG_A::PUSHPULL,
            2 => GPIO37OUTCFG_A::OD,
            3 => GPIO37OUTCFG_A::TS,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        **self == GPIO37OUTCFG_A::DIS
    }
    #[doc = "Checks if the value of the field is `PUSHPULL`"]
    #[inline(always)]
    pub fn is_pushpull(&self) -> bool {
        **self == GPIO37OUTCFG_A::PUSHPULL
    }
    #[doc = "Checks if the value of the field is `OD`"]
    #[inline(always)]
    pub fn is_od(&self) -> bool {
        **self == GPIO37OUTCFG_A::OD
    }
    #[doc = "Checks if the value of the field is `TS`"]
    #[inline(always)]
    pub fn is_ts(&self) -> bool {
        **self == GPIO37OUTCFG_A::TS
    }
}
impl core::ops::Deref for GPIO37OUTCFG_R {
    type Target = crate::FieldReader<u8, GPIO37OUTCFG_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIO37OUTCFG` writer - GPIO37 output configuration."]
pub struct GPIO37OUTCFG_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO37OUTCFG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIO37OUTCFG_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "FNCSEL = 0x3 - Output disabled value."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(GPIO37OUTCFG_A::DIS)
    }
    #[doc = "FNCSEL = 0x3 - Output is push-pull value."]
    #[inline(always)]
    pub fn pushpull(self) -> &'a mut W {
        self.variant(GPIO37OUTCFG_A::PUSHPULL)
    }
    #[doc = "FNCSEL = 0x3 - Output is open drain value."]
    #[inline(always)]
    pub fn od(self) -> &'a mut W {
        self.variant(GPIO37OUTCFG_A::OD)
    }
    #[doc = "FNCSEL = 0x3 - Output is tri-state value."]
    #[inline(always)]
    pub fn ts(self) -> &'a mut W {
        self.variant(GPIO37OUTCFG_A::TS)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 21)) | ((value as u32 & 0x03) << 21);
        self.w
    }
}
#[doc = "GPIO37 input enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO37INCFG_A {
    #[doc = "0: Read the GPIO pin data value."]
    READ = 0,
    #[doc = "1: INTD = 0 - Readback will always be zero value."]
    RDZERO = 1,
}
impl From<GPIO37INCFG_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO37INCFG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIO37INCFG` reader - GPIO37 input enable."]
pub struct GPIO37INCFG_R(crate::FieldReader<bool, GPIO37INCFG_A>);
impl GPIO37INCFG_R {
    pub(crate) fn new(bits: bool) -> Self {
        GPIO37INCFG_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO37INCFG_A {
        match self.bits {
            false => GPIO37INCFG_A::READ,
            true => GPIO37INCFG_A::RDZERO,
        }
    }
    #[doc = "Checks if the value of the field is `READ`"]
    #[inline(always)]
    pub fn is_read(&self) -> bool {
        **self == GPIO37INCFG_A::READ
    }
    #[doc = "Checks if the value of the field is `RDZERO`"]
    #[inline(always)]
    pub fn is_rdzero(&self) -> bool {
        **self == GPIO37INCFG_A::RDZERO
    }
}
impl core::ops::Deref for GPIO37INCFG_R {
    type Target = crate::FieldReader<bool, GPIO37INCFG_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIO37INCFG` writer - GPIO37 input enable."]
pub struct GPIO37INCFG_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO37INCFG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIO37INCFG_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Read the GPIO pin data value."]
    #[inline(always)]
    pub fn read(self) -> &'a mut W {
        self.variant(GPIO37INCFG_A::READ)
    }
    #[doc = "INTD = 0 - Readback will always be zero value."]
    #[inline(always)]
    pub fn rdzero(self) -> &'a mut W {
        self.variant(GPIO37INCFG_A::RDZERO)
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
#[doc = "GPIO36 interrupt direction.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO36INTD_A {
    #[doc = "0: FNCSEL = 0x1 - nCE polarity active low value."]
    NCELOW = 0,
    #[doc = "1: FNCSEL = 0x1 - nCE polarity active high value."]
    NCEHIGH = 1,
}
impl From<GPIO36INTD_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO36INTD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIO36INTD` reader - GPIO36 interrupt direction."]
pub struct GPIO36INTD_R(crate::FieldReader<bool, GPIO36INTD_A>);
impl GPIO36INTD_R {
    pub(crate) fn new(bits: bool) -> Self {
        GPIO36INTD_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO36INTD_A {
        match self.bits {
            false => GPIO36INTD_A::NCELOW,
            true => GPIO36INTD_A::NCEHIGH,
        }
    }
    #[doc = "Checks if the value of the field is `NCELOW`"]
    #[inline(always)]
    pub fn is_n_celow(&self) -> bool {
        **self == GPIO36INTD_A::NCELOW
    }
    #[doc = "Checks if the value of the field is `NCEHIGH`"]
    #[inline(always)]
    pub fn is_n_cehigh(&self) -> bool {
        **self == GPIO36INTD_A::NCEHIGH
    }
}
impl core::ops::Deref for GPIO36INTD_R {
    type Target = crate::FieldReader<bool, GPIO36INTD_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIO36INTD` writer - GPIO36 interrupt direction."]
pub struct GPIO36INTD_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO36INTD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIO36INTD_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "FNCSEL = 0x1 - nCE polarity active low value."]
    #[inline(always)]
    pub fn n_celow(self) -> &'a mut W {
        self.variant(GPIO36INTD_A::NCELOW)
    }
    #[doc = "FNCSEL = 0x1 - nCE polarity active high value."]
    #[inline(always)]
    pub fn n_cehigh(self) -> &'a mut W {
        self.variant(GPIO36INTD_A::NCEHIGH)
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
#[doc = "GPIO36 output configuration.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum GPIO36OUTCFG_A {
    #[doc = "0: FNCSEL = 0x3 - Output disabled value."]
    DIS = 0,
    #[doc = "1: FNCSEL = 0x3 - Output is push-pull value."]
    PUSHPULL = 1,
    #[doc = "2: FNCSEL = 0x3 - Output is open drain value."]
    OD = 2,
    #[doc = "3: FNCSEL = 0x3 - Output is tri-state value."]
    TS = 3,
}
impl From<GPIO36OUTCFG_A> for u8 {
    #[inline(always)]
    fn from(variant: GPIO36OUTCFG_A) -> Self {
        variant as _
    }
}
#[doc = "Field `GPIO36OUTCFG` reader - GPIO36 output configuration."]
pub struct GPIO36OUTCFG_R(crate::FieldReader<u8, GPIO36OUTCFG_A>);
impl GPIO36OUTCFG_R {
    pub(crate) fn new(bits: u8) -> Self {
        GPIO36OUTCFG_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO36OUTCFG_A {
        match self.bits {
            0 => GPIO36OUTCFG_A::DIS,
            1 => GPIO36OUTCFG_A::PUSHPULL,
            2 => GPIO36OUTCFG_A::OD,
            3 => GPIO36OUTCFG_A::TS,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        **self == GPIO36OUTCFG_A::DIS
    }
    #[doc = "Checks if the value of the field is `PUSHPULL`"]
    #[inline(always)]
    pub fn is_pushpull(&self) -> bool {
        **self == GPIO36OUTCFG_A::PUSHPULL
    }
    #[doc = "Checks if the value of the field is `OD`"]
    #[inline(always)]
    pub fn is_od(&self) -> bool {
        **self == GPIO36OUTCFG_A::OD
    }
    #[doc = "Checks if the value of the field is `TS`"]
    #[inline(always)]
    pub fn is_ts(&self) -> bool {
        **self == GPIO36OUTCFG_A::TS
    }
}
impl core::ops::Deref for GPIO36OUTCFG_R {
    type Target = crate::FieldReader<u8, GPIO36OUTCFG_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIO36OUTCFG` writer - GPIO36 output configuration."]
pub struct GPIO36OUTCFG_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO36OUTCFG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIO36OUTCFG_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "FNCSEL = 0x3 - Output disabled value."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(GPIO36OUTCFG_A::DIS)
    }
    #[doc = "FNCSEL = 0x3 - Output is push-pull value."]
    #[inline(always)]
    pub fn pushpull(self) -> &'a mut W {
        self.variant(GPIO36OUTCFG_A::PUSHPULL)
    }
    #[doc = "FNCSEL = 0x3 - Output is open drain value."]
    #[inline(always)]
    pub fn od(self) -> &'a mut W {
        self.variant(GPIO36OUTCFG_A::OD)
    }
    #[doc = "FNCSEL = 0x3 - Output is tri-state value."]
    #[inline(always)]
    pub fn ts(self) -> &'a mut W {
        self.variant(GPIO36OUTCFG_A::TS)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 17)) | ((value as u32 & 0x03) << 17);
        self.w
    }
}
#[doc = "GPIO36 input enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO36INCFG_A {
    #[doc = "0: Read the GPIO pin data value."]
    READ = 0,
    #[doc = "1: INTD = 0 - Readback will always be zero value."]
    RDZERO = 1,
}
impl From<GPIO36INCFG_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO36INCFG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIO36INCFG` reader - GPIO36 input enable."]
pub struct GPIO36INCFG_R(crate::FieldReader<bool, GPIO36INCFG_A>);
impl GPIO36INCFG_R {
    pub(crate) fn new(bits: bool) -> Self {
        GPIO36INCFG_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO36INCFG_A {
        match self.bits {
            false => GPIO36INCFG_A::READ,
            true => GPIO36INCFG_A::RDZERO,
        }
    }
    #[doc = "Checks if the value of the field is `READ`"]
    #[inline(always)]
    pub fn is_read(&self) -> bool {
        **self == GPIO36INCFG_A::READ
    }
    #[doc = "Checks if the value of the field is `RDZERO`"]
    #[inline(always)]
    pub fn is_rdzero(&self) -> bool {
        **self == GPIO36INCFG_A::RDZERO
    }
}
impl core::ops::Deref for GPIO36INCFG_R {
    type Target = crate::FieldReader<bool, GPIO36INCFG_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIO36INCFG` writer - GPIO36 input enable."]
pub struct GPIO36INCFG_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO36INCFG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIO36INCFG_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Read the GPIO pin data value."]
    #[inline(always)]
    pub fn read(self) -> &'a mut W {
        self.variant(GPIO36INCFG_A::READ)
    }
    #[doc = "INTD = 0 - Readback will always be zero value."]
    #[inline(always)]
    pub fn rdzero(self) -> &'a mut W {
        self.variant(GPIO36INCFG_A::RDZERO)
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
#[doc = "GPIO35 interrupt direction.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO35INTD_A {
    #[doc = "0: FNCSEL = 0x1 - nCE polarity active low value."]
    NCELOW = 0,
    #[doc = "1: FNCSEL = 0x1 - nCE polarity active high value."]
    NCEHIGH = 1,
}
impl From<GPIO35INTD_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO35INTD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIO35INTD` reader - GPIO35 interrupt direction."]
pub struct GPIO35INTD_R(crate::FieldReader<bool, GPIO35INTD_A>);
impl GPIO35INTD_R {
    pub(crate) fn new(bits: bool) -> Self {
        GPIO35INTD_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO35INTD_A {
        match self.bits {
            false => GPIO35INTD_A::NCELOW,
            true => GPIO35INTD_A::NCEHIGH,
        }
    }
    #[doc = "Checks if the value of the field is `NCELOW`"]
    #[inline(always)]
    pub fn is_n_celow(&self) -> bool {
        **self == GPIO35INTD_A::NCELOW
    }
    #[doc = "Checks if the value of the field is `NCEHIGH`"]
    #[inline(always)]
    pub fn is_n_cehigh(&self) -> bool {
        **self == GPIO35INTD_A::NCEHIGH
    }
}
impl core::ops::Deref for GPIO35INTD_R {
    type Target = crate::FieldReader<bool, GPIO35INTD_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIO35INTD` writer - GPIO35 interrupt direction."]
pub struct GPIO35INTD_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO35INTD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIO35INTD_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "FNCSEL = 0x1 - nCE polarity active low value."]
    #[inline(always)]
    pub fn n_celow(self) -> &'a mut W {
        self.variant(GPIO35INTD_A::NCELOW)
    }
    #[doc = "FNCSEL = 0x1 - nCE polarity active high value."]
    #[inline(always)]
    pub fn n_cehigh(self) -> &'a mut W {
        self.variant(GPIO35INTD_A::NCEHIGH)
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
#[doc = "GPIO35 output configuration.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum GPIO35OUTCFG_A {
    #[doc = "0: FNCSEL = 0x3 - Output disabled value."]
    DIS = 0,
    #[doc = "1: FNCSEL = 0x3 - Output is push-pull value."]
    PUSHPULL = 1,
    #[doc = "2: FNCSEL = 0x3 - Output is open drain value."]
    OD = 2,
    #[doc = "3: FNCSEL = 0x3 - Output is tri-state value."]
    TS = 3,
}
impl From<GPIO35OUTCFG_A> for u8 {
    #[inline(always)]
    fn from(variant: GPIO35OUTCFG_A) -> Self {
        variant as _
    }
}
#[doc = "Field `GPIO35OUTCFG` reader - GPIO35 output configuration."]
pub struct GPIO35OUTCFG_R(crate::FieldReader<u8, GPIO35OUTCFG_A>);
impl GPIO35OUTCFG_R {
    pub(crate) fn new(bits: u8) -> Self {
        GPIO35OUTCFG_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO35OUTCFG_A {
        match self.bits {
            0 => GPIO35OUTCFG_A::DIS,
            1 => GPIO35OUTCFG_A::PUSHPULL,
            2 => GPIO35OUTCFG_A::OD,
            3 => GPIO35OUTCFG_A::TS,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        **self == GPIO35OUTCFG_A::DIS
    }
    #[doc = "Checks if the value of the field is `PUSHPULL`"]
    #[inline(always)]
    pub fn is_pushpull(&self) -> bool {
        **self == GPIO35OUTCFG_A::PUSHPULL
    }
    #[doc = "Checks if the value of the field is `OD`"]
    #[inline(always)]
    pub fn is_od(&self) -> bool {
        **self == GPIO35OUTCFG_A::OD
    }
    #[doc = "Checks if the value of the field is `TS`"]
    #[inline(always)]
    pub fn is_ts(&self) -> bool {
        **self == GPIO35OUTCFG_A::TS
    }
}
impl core::ops::Deref for GPIO35OUTCFG_R {
    type Target = crate::FieldReader<u8, GPIO35OUTCFG_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIO35OUTCFG` writer - GPIO35 output configuration."]
pub struct GPIO35OUTCFG_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO35OUTCFG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIO35OUTCFG_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "FNCSEL = 0x3 - Output disabled value."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(GPIO35OUTCFG_A::DIS)
    }
    #[doc = "FNCSEL = 0x3 - Output is push-pull value."]
    #[inline(always)]
    pub fn pushpull(self) -> &'a mut W {
        self.variant(GPIO35OUTCFG_A::PUSHPULL)
    }
    #[doc = "FNCSEL = 0x3 - Output is open drain value."]
    #[inline(always)]
    pub fn od(self) -> &'a mut W {
        self.variant(GPIO35OUTCFG_A::OD)
    }
    #[doc = "FNCSEL = 0x3 - Output is tri-state value."]
    #[inline(always)]
    pub fn ts(self) -> &'a mut W {
        self.variant(GPIO35OUTCFG_A::TS)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 13)) | ((value as u32 & 0x03) << 13);
        self.w
    }
}
#[doc = "GPIO35 input enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO35INCFG_A {
    #[doc = "0: Read the GPIO pin data value."]
    READ = 0,
    #[doc = "1: INTD = 0 - Readback will always be zero value."]
    RDZERO = 1,
}
impl From<GPIO35INCFG_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO35INCFG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIO35INCFG` reader - GPIO35 input enable."]
pub struct GPIO35INCFG_R(crate::FieldReader<bool, GPIO35INCFG_A>);
impl GPIO35INCFG_R {
    pub(crate) fn new(bits: bool) -> Self {
        GPIO35INCFG_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO35INCFG_A {
        match self.bits {
            false => GPIO35INCFG_A::READ,
            true => GPIO35INCFG_A::RDZERO,
        }
    }
    #[doc = "Checks if the value of the field is `READ`"]
    #[inline(always)]
    pub fn is_read(&self) -> bool {
        **self == GPIO35INCFG_A::READ
    }
    #[doc = "Checks if the value of the field is `RDZERO`"]
    #[inline(always)]
    pub fn is_rdzero(&self) -> bool {
        **self == GPIO35INCFG_A::RDZERO
    }
}
impl core::ops::Deref for GPIO35INCFG_R {
    type Target = crate::FieldReader<bool, GPIO35INCFG_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIO35INCFG` writer - GPIO35 input enable."]
pub struct GPIO35INCFG_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO35INCFG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIO35INCFG_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Read the GPIO pin data value."]
    #[inline(always)]
    pub fn read(self) -> &'a mut W {
        self.variant(GPIO35INCFG_A::READ)
    }
    #[doc = "INTD = 0 - Readback will always be zero value."]
    #[inline(always)]
    pub fn rdzero(self) -> &'a mut W {
        self.variant(GPIO35INCFG_A::RDZERO)
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
#[doc = "GPIO34 interrupt direction.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO34INTD_A {
    #[doc = "0: FNCSEL = 0x1 - nCE polarity active low value."]
    NCELOW = 0,
    #[doc = "1: FNCSEL = 0x1 - nCE polarity active high value."]
    NCEHIGH = 1,
}
impl From<GPIO34INTD_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO34INTD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIO34INTD` reader - GPIO34 interrupt direction."]
pub struct GPIO34INTD_R(crate::FieldReader<bool, GPIO34INTD_A>);
impl GPIO34INTD_R {
    pub(crate) fn new(bits: bool) -> Self {
        GPIO34INTD_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO34INTD_A {
        match self.bits {
            false => GPIO34INTD_A::NCELOW,
            true => GPIO34INTD_A::NCEHIGH,
        }
    }
    #[doc = "Checks if the value of the field is `NCELOW`"]
    #[inline(always)]
    pub fn is_n_celow(&self) -> bool {
        **self == GPIO34INTD_A::NCELOW
    }
    #[doc = "Checks if the value of the field is `NCEHIGH`"]
    #[inline(always)]
    pub fn is_n_cehigh(&self) -> bool {
        **self == GPIO34INTD_A::NCEHIGH
    }
}
impl core::ops::Deref for GPIO34INTD_R {
    type Target = crate::FieldReader<bool, GPIO34INTD_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIO34INTD` writer - GPIO34 interrupt direction."]
pub struct GPIO34INTD_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO34INTD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIO34INTD_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "FNCSEL = 0x1 - nCE polarity active low value."]
    #[inline(always)]
    pub fn n_celow(self) -> &'a mut W {
        self.variant(GPIO34INTD_A::NCELOW)
    }
    #[doc = "FNCSEL = 0x1 - nCE polarity active high value."]
    #[inline(always)]
    pub fn n_cehigh(self) -> &'a mut W {
        self.variant(GPIO34INTD_A::NCEHIGH)
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
#[doc = "GPIO34 output configuration.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum GPIO34OUTCFG_A {
    #[doc = "0: FNCSEL = 0x3 - Output disabled value."]
    DIS = 0,
    #[doc = "1: FNCSEL = 0x3 - Output is push-pull value."]
    PUSHPULL = 1,
    #[doc = "2: FNCSEL = 0x3 - Output is open drain value."]
    OD = 2,
    #[doc = "3: FNCSEL = 0x3 - Output is tri-state value."]
    TS = 3,
}
impl From<GPIO34OUTCFG_A> for u8 {
    #[inline(always)]
    fn from(variant: GPIO34OUTCFG_A) -> Self {
        variant as _
    }
}
#[doc = "Field `GPIO34OUTCFG` reader - GPIO34 output configuration."]
pub struct GPIO34OUTCFG_R(crate::FieldReader<u8, GPIO34OUTCFG_A>);
impl GPIO34OUTCFG_R {
    pub(crate) fn new(bits: u8) -> Self {
        GPIO34OUTCFG_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO34OUTCFG_A {
        match self.bits {
            0 => GPIO34OUTCFG_A::DIS,
            1 => GPIO34OUTCFG_A::PUSHPULL,
            2 => GPIO34OUTCFG_A::OD,
            3 => GPIO34OUTCFG_A::TS,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        **self == GPIO34OUTCFG_A::DIS
    }
    #[doc = "Checks if the value of the field is `PUSHPULL`"]
    #[inline(always)]
    pub fn is_pushpull(&self) -> bool {
        **self == GPIO34OUTCFG_A::PUSHPULL
    }
    #[doc = "Checks if the value of the field is `OD`"]
    #[inline(always)]
    pub fn is_od(&self) -> bool {
        **self == GPIO34OUTCFG_A::OD
    }
    #[doc = "Checks if the value of the field is `TS`"]
    #[inline(always)]
    pub fn is_ts(&self) -> bool {
        **self == GPIO34OUTCFG_A::TS
    }
}
impl core::ops::Deref for GPIO34OUTCFG_R {
    type Target = crate::FieldReader<u8, GPIO34OUTCFG_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIO34OUTCFG` writer - GPIO34 output configuration."]
pub struct GPIO34OUTCFG_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO34OUTCFG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIO34OUTCFG_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "FNCSEL = 0x3 - Output disabled value."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(GPIO34OUTCFG_A::DIS)
    }
    #[doc = "FNCSEL = 0x3 - Output is push-pull value."]
    #[inline(always)]
    pub fn pushpull(self) -> &'a mut W {
        self.variant(GPIO34OUTCFG_A::PUSHPULL)
    }
    #[doc = "FNCSEL = 0x3 - Output is open drain value."]
    #[inline(always)]
    pub fn od(self) -> &'a mut W {
        self.variant(GPIO34OUTCFG_A::OD)
    }
    #[doc = "FNCSEL = 0x3 - Output is tri-state value."]
    #[inline(always)]
    pub fn ts(self) -> &'a mut W {
        self.variant(GPIO34OUTCFG_A::TS)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 9)) | ((value as u32 & 0x03) << 9);
        self.w
    }
}
#[doc = "GPIO34 input enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO34INCFG_A {
    #[doc = "0: Read the GPIO pin data value."]
    READ = 0,
    #[doc = "1: INTD = 0 - Readback will always be zero value."]
    RDZERO = 1,
}
impl From<GPIO34INCFG_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO34INCFG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIO34INCFG` reader - GPIO34 input enable."]
pub struct GPIO34INCFG_R(crate::FieldReader<bool, GPIO34INCFG_A>);
impl GPIO34INCFG_R {
    pub(crate) fn new(bits: bool) -> Self {
        GPIO34INCFG_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO34INCFG_A {
        match self.bits {
            false => GPIO34INCFG_A::READ,
            true => GPIO34INCFG_A::RDZERO,
        }
    }
    #[doc = "Checks if the value of the field is `READ`"]
    #[inline(always)]
    pub fn is_read(&self) -> bool {
        **self == GPIO34INCFG_A::READ
    }
    #[doc = "Checks if the value of the field is `RDZERO`"]
    #[inline(always)]
    pub fn is_rdzero(&self) -> bool {
        **self == GPIO34INCFG_A::RDZERO
    }
}
impl core::ops::Deref for GPIO34INCFG_R {
    type Target = crate::FieldReader<bool, GPIO34INCFG_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIO34INCFG` writer - GPIO34 input enable."]
pub struct GPIO34INCFG_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO34INCFG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIO34INCFG_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Read the GPIO pin data value."]
    #[inline(always)]
    pub fn read(self) -> &'a mut W {
        self.variant(GPIO34INCFG_A::READ)
    }
    #[doc = "INTD = 0 - Readback will always be zero value."]
    #[inline(always)]
    pub fn rdzero(self) -> &'a mut W {
        self.variant(GPIO34INCFG_A::RDZERO)
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
#[doc = "GPIO33 interrupt direction.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO33INTD_A {
    #[doc = "0: FNCSEL = 0x1 - nCE polarity active low value."]
    NCELOW = 0,
    #[doc = "1: FNCSEL = 0x1 - nCE polarity active high value."]
    NCEHIGH = 1,
}
impl From<GPIO33INTD_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO33INTD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIO33INTD` reader - GPIO33 interrupt direction."]
pub struct GPIO33INTD_R(crate::FieldReader<bool, GPIO33INTD_A>);
impl GPIO33INTD_R {
    pub(crate) fn new(bits: bool) -> Self {
        GPIO33INTD_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO33INTD_A {
        match self.bits {
            false => GPIO33INTD_A::NCELOW,
            true => GPIO33INTD_A::NCEHIGH,
        }
    }
    #[doc = "Checks if the value of the field is `NCELOW`"]
    #[inline(always)]
    pub fn is_n_celow(&self) -> bool {
        **self == GPIO33INTD_A::NCELOW
    }
    #[doc = "Checks if the value of the field is `NCEHIGH`"]
    #[inline(always)]
    pub fn is_n_cehigh(&self) -> bool {
        **self == GPIO33INTD_A::NCEHIGH
    }
}
impl core::ops::Deref for GPIO33INTD_R {
    type Target = crate::FieldReader<bool, GPIO33INTD_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIO33INTD` writer - GPIO33 interrupt direction."]
pub struct GPIO33INTD_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO33INTD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIO33INTD_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "FNCSEL = 0x1 - nCE polarity active low value."]
    #[inline(always)]
    pub fn n_celow(self) -> &'a mut W {
        self.variant(GPIO33INTD_A::NCELOW)
    }
    #[doc = "FNCSEL = 0x1 - nCE polarity active high value."]
    #[inline(always)]
    pub fn n_cehigh(self) -> &'a mut W {
        self.variant(GPIO33INTD_A::NCEHIGH)
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
#[doc = "GPIO33 output configuration.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum GPIO33OUTCFG_A {
    #[doc = "0: FNCSEL = 0x3 - Output disabled value."]
    DIS = 0,
    #[doc = "1: FNCSEL = 0x3 - Output is push-pull value."]
    PUSHPULL = 1,
    #[doc = "2: FNCSEL = 0x3 - Output is open drain value."]
    OD = 2,
    #[doc = "3: FNCSEL = 0x3 - Output is tri-state value."]
    TS = 3,
}
impl From<GPIO33OUTCFG_A> for u8 {
    #[inline(always)]
    fn from(variant: GPIO33OUTCFG_A) -> Self {
        variant as _
    }
}
#[doc = "Field `GPIO33OUTCFG` reader - GPIO33 output configuration."]
pub struct GPIO33OUTCFG_R(crate::FieldReader<u8, GPIO33OUTCFG_A>);
impl GPIO33OUTCFG_R {
    pub(crate) fn new(bits: u8) -> Self {
        GPIO33OUTCFG_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO33OUTCFG_A {
        match self.bits {
            0 => GPIO33OUTCFG_A::DIS,
            1 => GPIO33OUTCFG_A::PUSHPULL,
            2 => GPIO33OUTCFG_A::OD,
            3 => GPIO33OUTCFG_A::TS,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        **self == GPIO33OUTCFG_A::DIS
    }
    #[doc = "Checks if the value of the field is `PUSHPULL`"]
    #[inline(always)]
    pub fn is_pushpull(&self) -> bool {
        **self == GPIO33OUTCFG_A::PUSHPULL
    }
    #[doc = "Checks if the value of the field is `OD`"]
    #[inline(always)]
    pub fn is_od(&self) -> bool {
        **self == GPIO33OUTCFG_A::OD
    }
    #[doc = "Checks if the value of the field is `TS`"]
    #[inline(always)]
    pub fn is_ts(&self) -> bool {
        **self == GPIO33OUTCFG_A::TS
    }
}
impl core::ops::Deref for GPIO33OUTCFG_R {
    type Target = crate::FieldReader<u8, GPIO33OUTCFG_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIO33OUTCFG` writer - GPIO33 output configuration."]
pub struct GPIO33OUTCFG_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO33OUTCFG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIO33OUTCFG_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "FNCSEL = 0x3 - Output disabled value."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(GPIO33OUTCFG_A::DIS)
    }
    #[doc = "FNCSEL = 0x3 - Output is push-pull value."]
    #[inline(always)]
    pub fn pushpull(self) -> &'a mut W {
        self.variant(GPIO33OUTCFG_A::PUSHPULL)
    }
    #[doc = "FNCSEL = 0x3 - Output is open drain value."]
    #[inline(always)]
    pub fn od(self) -> &'a mut W {
        self.variant(GPIO33OUTCFG_A::OD)
    }
    #[doc = "FNCSEL = 0x3 - Output is tri-state value."]
    #[inline(always)]
    pub fn ts(self) -> &'a mut W {
        self.variant(GPIO33OUTCFG_A::TS)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 5)) | ((value as u32 & 0x03) << 5);
        self.w
    }
}
#[doc = "GPIO33 input enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO33INCFG_A {
    #[doc = "0: Read the GPIO pin data value."]
    READ = 0,
    #[doc = "1: INTD = 0 - Readback will always be zero value."]
    RDZERO = 1,
}
impl From<GPIO33INCFG_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO33INCFG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIO33INCFG` reader - GPIO33 input enable."]
pub struct GPIO33INCFG_R(crate::FieldReader<bool, GPIO33INCFG_A>);
impl GPIO33INCFG_R {
    pub(crate) fn new(bits: bool) -> Self {
        GPIO33INCFG_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO33INCFG_A {
        match self.bits {
            false => GPIO33INCFG_A::READ,
            true => GPIO33INCFG_A::RDZERO,
        }
    }
    #[doc = "Checks if the value of the field is `READ`"]
    #[inline(always)]
    pub fn is_read(&self) -> bool {
        **self == GPIO33INCFG_A::READ
    }
    #[doc = "Checks if the value of the field is `RDZERO`"]
    #[inline(always)]
    pub fn is_rdzero(&self) -> bool {
        **self == GPIO33INCFG_A::RDZERO
    }
}
impl core::ops::Deref for GPIO33INCFG_R {
    type Target = crate::FieldReader<bool, GPIO33INCFG_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIO33INCFG` writer - GPIO33 input enable."]
pub struct GPIO33INCFG_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO33INCFG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIO33INCFG_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Read the GPIO pin data value."]
    #[inline(always)]
    pub fn read(self) -> &'a mut W {
        self.variant(GPIO33INCFG_A::READ)
    }
    #[doc = "INTD = 0 - Readback will always be zero value."]
    #[inline(always)]
    pub fn rdzero(self) -> &'a mut W {
        self.variant(GPIO33INCFG_A::RDZERO)
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
#[doc = "GPIO32 interrupt direction.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO32INTD_A {
    #[doc = "0: FNCSEL = 0x1 - nCE polarity active low value."]
    NCELOW = 0,
    #[doc = "1: FNCSEL = 0x1 - nCE polarity active high value."]
    NCEHIGH = 1,
}
impl From<GPIO32INTD_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO32INTD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIO32INTD` reader - GPIO32 interrupt direction."]
pub struct GPIO32INTD_R(crate::FieldReader<bool, GPIO32INTD_A>);
impl GPIO32INTD_R {
    pub(crate) fn new(bits: bool) -> Self {
        GPIO32INTD_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO32INTD_A {
        match self.bits {
            false => GPIO32INTD_A::NCELOW,
            true => GPIO32INTD_A::NCEHIGH,
        }
    }
    #[doc = "Checks if the value of the field is `NCELOW`"]
    #[inline(always)]
    pub fn is_n_celow(&self) -> bool {
        **self == GPIO32INTD_A::NCELOW
    }
    #[doc = "Checks if the value of the field is `NCEHIGH`"]
    #[inline(always)]
    pub fn is_n_cehigh(&self) -> bool {
        **self == GPIO32INTD_A::NCEHIGH
    }
}
impl core::ops::Deref for GPIO32INTD_R {
    type Target = crate::FieldReader<bool, GPIO32INTD_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIO32INTD` writer - GPIO32 interrupt direction."]
pub struct GPIO32INTD_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO32INTD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIO32INTD_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "FNCSEL = 0x1 - nCE polarity active low value."]
    #[inline(always)]
    pub fn n_celow(self) -> &'a mut W {
        self.variant(GPIO32INTD_A::NCELOW)
    }
    #[doc = "FNCSEL = 0x1 - nCE polarity active high value."]
    #[inline(always)]
    pub fn n_cehigh(self) -> &'a mut W {
        self.variant(GPIO32INTD_A::NCEHIGH)
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
#[doc = "GPIO32 output configuration.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum GPIO32OUTCFG_A {
    #[doc = "0: FNCSEL = 0x3 - Output disabled value."]
    DIS = 0,
    #[doc = "1: FNCSEL = 0x3 - Output is push-pull value."]
    PUSHPULL = 1,
    #[doc = "2: FNCSEL = 0x3 - Output is open drain value."]
    OD = 2,
    #[doc = "3: FNCSEL = 0x3 - Output is tri-state value."]
    TS = 3,
}
impl From<GPIO32OUTCFG_A> for u8 {
    #[inline(always)]
    fn from(variant: GPIO32OUTCFG_A) -> Self {
        variant as _
    }
}
#[doc = "Field `GPIO32OUTCFG` reader - GPIO32 output configuration."]
pub struct GPIO32OUTCFG_R(crate::FieldReader<u8, GPIO32OUTCFG_A>);
impl GPIO32OUTCFG_R {
    pub(crate) fn new(bits: u8) -> Self {
        GPIO32OUTCFG_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO32OUTCFG_A {
        match self.bits {
            0 => GPIO32OUTCFG_A::DIS,
            1 => GPIO32OUTCFG_A::PUSHPULL,
            2 => GPIO32OUTCFG_A::OD,
            3 => GPIO32OUTCFG_A::TS,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        **self == GPIO32OUTCFG_A::DIS
    }
    #[doc = "Checks if the value of the field is `PUSHPULL`"]
    #[inline(always)]
    pub fn is_pushpull(&self) -> bool {
        **self == GPIO32OUTCFG_A::PUSHPULL
    }
    #[doc = "Checks if the value of the field is `OD`"]
    #[inline(always)]
    pub fn is_od(&self) -> bool {
        **self == GPIO32OUTCFG_A::OD
    }
    #[doc = "Checks if the value of the field is `TS`"]
    #[inline(always)]
    pub fn is_ts(&self) -> bool {
        **self == GPIO32OUTCFG_A::TS
    }
}
impl core::ops::Deref for GPIO32OUTCFG_R {
    type Target = crate::FieldReader<u8, GPIO32OUTCFG_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIO32OUTCFG` writer - GPIO32 output configuration."]
pub struct GPIO32OUTCFG_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO32OUTCFG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIO32OUTCFG_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "FNCSEL = 0x3 - Output disabled value."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(GPIO32OUTCFG_A::DIS)
    }
    #[doc = "FNCSEL = 0x3 - Output is push-pull value."]
    #[inline(always)]
    pub fn pushpull(self) -> &'a mut W {
        self.variant(GPIO32OUTCFG_A::PUSHPULL)
    }
    #[doc = "FNCSEL = 0x3 - Output is open drain value."]
    #[inline(always)]
    pub fn od(self) -> &'a mut W {
        self.variant(GPIO32OUTCFG_A::OD)
    }
    #[doc = "FNCSEL = 0x3 - Output is tri-state value."]
    #[inline(always)]
    pub fn ts(self) -> &'a mut W {
        self.variant(GPIO32OUTCFG_A::TS)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 1)) | ((value as u32 & 0x03) << 1);
        self.w
    }
}
#[doc = "GPIO32 input enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO32INCFG_A {
    #[doc = "0: Read the GPIO pin data value."]
    READ = 0,
    #[doc = "1: INTD = 0 - Readback will always be zero value."]
    RDZERO = 1,
}
impl From<GPIO32INCFG_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO32INCFG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIO32INCFG` reader - GPIO32 input enable."]
pub struct GPIO32INCFG_R(crate::FieldReader<bool, GPIO32INCFG_A>);
impl GPIO32INCFG_R {
    pub(crate) fn new(bits: bool) -> Self {
        GPIO32INCFG_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO32INCFG_A {
        match self.bits {
            false => GPIO32INCFG_A::READ,
            true => GPIO32INCFG_A::RDZERO,
        }
    }
    #[doc = "Checks if the value of the field is `READ`"]
    #[inline(always)]
    pub fn is_read(&self) -> bool {
        **self == GPIO32INCFG_A::READ
    }
    #[doc = "Checks if the value of the field is `RDZERO`"]
    #[inline(always)]
    pub fn is_rdzero(&self) -> bool {
        **self == GPIO32INCFG_A::RDZERO
    }
}
impl core::ops::Deref for GPIO32INCFG_R {
    type Target = crate::FieldReader<bool, GPIO32INCFG_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIO32INCFG` writer - GPIO32 input enable."]
pub struct GPIO32INCFG_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO32INCFG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIO32INCFG_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Read the GPIO pin data value."]
    #[inline(always)]
    pub fn read(self) -> &'a mut W {
        self.variant(GPIO32INCFG_A::READ)
    }
    #[doc = "INTD = 0 - Readback will always be zero value."]
    #[inline(always)]
    pub fn rdzero(self) -> &'a mut W {
        self.variant(GPIO32INCFG_A::RDZERO)
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
    #[doc = "Bit 31 - GPIO39 interrupt direction."]
    #[inline(always)]
    pub fn gpio39intd(&self) -> GPIO39INTD_R {
        GPIO39INTD_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bits 29:30 - GPIO39 output configuration."]
    #[inline(always)]
    pub fn gpio39outcfg(&self) -> GPIO39OUTCFG_R {
        GPIO39OUTCFG_R::new(((self.bits >> 29) & 0x03) as u8)
    }
    #[doc = "Bit 28 - GPIO39 input enable."]
    #[inline(always)]
    pub fn gpio39incfg(&self) -> GPIO39INCFG_R {
        GPIO39INCFG_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 27 - GPIO38 interrupt direction."]
    #[inline(always)]
    pub fn gpio38intd(&self) -> GPIO38INTD_R {
        GPIO38INTD_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bits 25:26 - GPIO38 output configuration."]
    #[inline(always)]
    pub fn gpio38outcfg(&self) -> GPIO38OUTCFG_R {
        GPIO38OUTCFG_R::new(((self.bits >> 25) & 0x03) as u8)
    }
    #[doc = "Bit 24 - GPIO38 input enable."]
    #[inline(always)]
    pub fn gpio38incfg(&self) -> GPIO38INCFG_R {
        GPIO38INCFG_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 23 - GPIO37 interrupt direction."]
    #[inline(always)]
    pub fn gpio37intd(&self) -> GPIO37INTD_R {
        GPIO37INTD_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bits 21:22 - GPIO37 output configuration."]
    #[inline(always)]
    pub fn gpio37outcfg(&self) -> GPIO37OUTCFG_R {
        GPIO37OUTCFG_R::new(((self.bits >> 21) & 0x03) as u8)
    }
    #[doc = "Bit 20 - GPIO37 input enable."]
    #[inline(always)]
    pub fn gpio37incfg(&self) -> GPIO37INCFG_R {
        GPIO37INCFG_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 19 - GPIO36 interrupt direction."]
    #[inline(always)]
    pub fn gpio36intd(&self) -> GPIO36INTD_R {
        GPIO36INTD_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bits 17:18 - GPIO36 output configuration."]
    #[inline(always)]
    pub fn gpio36outcfg(&self) -> GPIO36OUTCFG_R {
        GPIO36OUTCFG_R::new(((self.bits >> 17) & 0x03) as u8)
    }
    #[doc = "Bit 16 - GPIO36 input enable."]
    #[inline(always)]
    pub fn gpio36incfg(&self) -> GPIO36INCFG_R {
        GPIO36INCFG_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 15 - GPIO35 interrupt direction."]
    #[inline(always)]
    pub fn gpio35intd(&self) -> GPIO35INTD_R {
        GPIO35INTD_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bits 13:14 - GPIO35 output configuration."]
    #[inline(always)]
    pub fn gpio35outcfg(&self) -> GPIO35OUTCFG_R {
        GPIO35OUTCFG_R::new(((self.bits >> 13) & 0x03) as u8)
    }
    #[doc = "Bit 12 - GPIO35 input enable."]
    #[inline(always)]
    pub fn gpio35incfg(&self) -> GPIO35INCFG_R {
        GPIO35INCFG_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11 - GPIO34 interrupt direction."]
    #[inline(always)]
    pub fn gpio34intd(&self) -> GPIO34INTD_R {
        GPIO34INTD_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bits 9:10 - GPIO34 output configuration."]
    #[inline(always)]
    pub fn gpio34outcfg(&self) -> GPIO34OUTCFG_R {
        GPIO34OUTCFG_R::new(((self.bits >> 9) & 0x03) as u8)
    }
    #[doc = "Bit 8 - GPIO34 input enable."]
    #[inline(always)]
    pub fn gpio34incfg(&self) -> GPIO34INCFG_R {
        GPIO34INCFG_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7 - GPIO33 interrupt direction."]
    #[inline(always)]
    pub fn gpio33intd(&self) -> GPIO33INTD_R {
        GPIO33INTD_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 5:6 - GPIO33 output configuration."]
    #[inline(always)]
    pub fn gpio33outcfg(&self) -> GPIO33OUTCFG_R {
        GPIO33OUTCFG_R::new(((self.bits >> 5) & 0x03) as u8)
    }
    #[doc = "Bit 4 - GPIO33 input enable."]
    #[inline(always)]
    pub fn gpio33incfg(&self) -> GPIO33INCFG_R {
        GPIO33INCFG_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - GPIO32 interrupt direction."]
    #[inline(always)]
    pub fn gpio32intd(&self) -> GPIO32INTD_R {
        GPIO32INTD_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bits 1:2 - GPIO32 output configuration."]
    #[inline(always)]
    pub fn gpio32outcfg(&self) -> GPIO32OUTCFG_R {
        GPIO32OUTCFG_R::new(((self.bits >> 1) & 0x03) as u8)
    }
    #[doc = "Bit 0 - GPIO32 input enable."]
    #[inline(always)]
    pub fn gpio32incfg(&self) -> GPIO32INCFG_R {
        GPIO32INCFG_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 31 - GPIO39 interrupt direction."]
    #[inline(always)]
    pub fn gpio39intd(&mut self) -> GPIO39INTD_W {
        GPIO39INTD_W { w: self }
    }
    #[doc = "Bits 29:30 - GPIO39 output configuration."]
    #[inline(always)]
    pub fn gpio39outcfg(&mut self) -> GPIO39OUTCFG_W {
        GPIO39OUTCFG_W { w: self }
    }
    #[doc = "Bit 28 - GPIO39 input enable."]
    #[inline(always)]
    pub fn gpio39incfg(&mut self) -> GPIO39INCFG_W {
        GPIO39INCFG_W { w: self }
    }
    #[doc = "Bit 27 - GPIO38 interrupt direction."]
    #[inline(always)]
    pub fn gpio38intd(&mut self) -> GPIO38INTD_W {
        GPIO38INTD_W { w: self }
    }
    #[doc = "Bits 25:26 - GPIO38 output configuration."]
    #[inline(always)]
    pub fn gpio38outcfg(&mut self) -> GPIO38OUTCFG_W {
        GPIO38OUTCFG_W { w: self }
    }
    #[doc = "Bit 24 - GPIO38 input enable."]
    #[inline(always)]
    pub fn gpio38incfg(&mut self) -> GPIO38INCFG_W {
        GPIO38INCFG_W { w: self }
    }
    #[doc = "Bit 23 - GPIO37 interrupt direction."]
    #[inline(always)]
    pub fn gpio37intd(&mut self) -> GPIO37INTD_W {
        GPIO37INTD_W { w: self }
    }
    #[doc = "Bits 21:22 - GPIO37 output configuration."]
    #[inline(always)]
    pub fn gpio37outcfg(&mut self) -> GPIO37OUTCFG_W {
        GPIO37OUTCFG_W { w: self }
    }
    #[doc = "Bit 20 - GPIO37 input enable."]
    #[inline(always)]
    pub fn gpio37incfg(&mut self) -> GPIO37INCFG_W {
        GPIO37INCFG_W { w: self }
    }
    #[doc = "Bit 19 - GPIO36 interrupt direction."]
    #[inline(always)]
    pub fn gpio36intd(&mut self) -> GPIO36INTD_W {
        GPIO36INTD_W { w: self }
    }
    #[doc = "Bits 17:18 - GPIO36 output configuration."]
    #[inline(always)]
    pub fn gpio36outcfg(&mut self) -> GPIO36OUTCFG_W {
        GPIO36OUTCFG_W { w: self }
    }
    #[doc = "Bit 16 - GPIO36 input enable."]
    #[inline(always)]
    pub fn gpio36incfg(&mut self) -> GPIO36INCFG_W {
        GPIO36INCFG_W { w: self }
    }
    #[doc = "Bit 15 - GPIO35 interrupt direction."]
    #[inline(always)]
    pub fn gpio35intd(&mut self) -> GPIO35INTD_W {
        GPIO35INTD_W { w: self }
    }
    #[doc = "Bits 13:14 - GPIO35 output configuration."]
    #[inline(always)]
    pub fn gpio35outcfg(&mut self) -> GPIO35OUTCFG_W {
        GPIO35OUTCFG_W { w: self }
    }
    #[doc = "Bit 12 - GPIO35 input enable."]
    #[inline(always)]
    pub fn gpio35incfg(&mut self) -> GPIO35INCFG_W {
        GPIO35INCFG_W { w: self }
    }
    #[doc = "Bit 11 - GPIO34 interrupt direction."]
    #[inline(always)]
    pub fn gpio34intd(&mut self) -> GPIO34INTD_W {
        GPIO34INTD_W { w: self }
    }
    #[doc = "Bits 9:10 - GPIO34 output configuration."]
    #[inline(always)]
    pub fn gpio34outcfg(&mut self) -> GPIO34OUTCFG_W {
        GPIO34OUTCFG_W { w: self }
    }
    #[doc = "Bit 8 - GPIO34 input enable."]
    #[inline(always)]
    pub fn gpio34incfg(&mut self) -> GPIO34INCFG_W {
        GPIO34INCFG_W { w: self }
    }
    #[doc = "Bit 7 - GPIO33 interrupt direction."]
    #[inline(always)]
    pub fn gpio33intd(&mut self) -> GPIO33INTD_W {
        GPIO33INTD_W { w: self }
    }
    #[doc = "Bits 5:6 - GPIO33 output configuration."]
    #[inline(always)]
    pub fn gpio33outcfg(&mut self) -> GPIO33OUTCFG_W {
        GPIO33OUTCFG_W { w: self }
    }
    #[doc = "Bit 4 - GPIO33 input enable."]
    #[inline(always)]
    pub fn gpio33incfg(&mut self) -> GPIO33INCFG_W {
        GPIO33INCFG_W { w: self }
    }
    #[doc = "Bit 3 - GPIO32 interrupt direction."]
    #[inline(always)]
    pub fn gpio32intd(&mut self) -> GPIO32INTD_W {
        GPIO32INTD_W { w: self }
    }
    #[doc = "Bits 1:2 - GPIO32 output configuration."]
    #[inline(always)]
    pub fn gpio32outcfg(&mut self) -> GPIO32OUTCFG_W {
        GPIO32OUTCFG_W { w: self }
    }
    #[doc = "Bit 0 - GPIO32 input enable."]
    #[inline(always)]
    pub fn gpio32incfg(&mut self) -> GPIO32INCFG_W {
        GPIO32INCFG_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPIO Configuration Register E (Pads 32-39)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cfge](index.html) module"]
pub struct CFGE_SPEC;
impl crate::RegisterSpec for CFGE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cfge::R](R) reader structure"]
impl crate::Readable for CFGE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cfge::W](W) writer structure"]
impl crate::Writable for CFGE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CFGE to value 0"]
impl crate::Resettable for CFGE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
