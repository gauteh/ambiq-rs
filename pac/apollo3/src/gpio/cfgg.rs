#[doc = "Register `CFGG` reader"]
pub struct R(crate::R<CFGG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CFGG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CFGG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CFGG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CFGG` writer"]
pub struct W(crate::W<CFGG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CFGG_SPEC>;
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
impl From<crate::W<CFGG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CFGG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "GPIO49 interrupt direction.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO49INTD_A {
    #[doc = "0: FNCSEL = 0x1 - nCE polarity active low value."]
    NCELOW = 0,
    #[doc = "1: FNCSEL = 0x1 - nCE polarity active high value."]
    NCEHIGH = 1,
}
impl From<GPIO49INTD_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO49INTD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIO49INTD` reader - GPIO49 interrupt direction."]
pub struct GPIO49INTD_R(crate::FieldReader<bool, GPIO49INTD_A>);
impl GPIO49INTD_R {
    pub(crate) fn new(bits: bool) -> Self {
        GPIO49INTD_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO49INTD_A {
        match self.bits {
            false => GPIO49INTD_A::NCELOW,
            true => GPIO49INTD_A::NCEHIGH,
        }
    }
    #[doc = "Checks if the value of the field is `NCELOW`"]
    #[inline(always)]
    pub fn is_n_celow(&self) -> bool {
        **self == GPIO49INTD_A::NCELOW
    }
    #[doc = "Checks if the value of the field is `NCEHIGH`"]
    #[inline(always)]
    pub fn is_n_cehigh(&self) -> bool {
        **self == GPIO49INTD_A::NCEHIGH
    }
}
impl core::ops::Deref for GPIO49INTD_R {
    type Target = crate::FieldReader<bool, GPIO49INTD_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIO49INTD` writer - GPIO49 interrupt direction."]
pub struct GPIO49INTD_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO49INTD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIO49INTD_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "FNCSEL = 0x1 - nCE polarity active low value."]
    #[inline(always)]
    pub fn n_celow(self) -> &'a mut W {
        self.variant(GPIO49INTD_A::NCELOW)
    }
    #[doc = "FNCSEL = 0x1 - nCE polarity active high value."]
    #[inline(always)]
    pub fn n_cehigh(self) -> &'a mut W {
        self.variant(GPIO49INTD_A::NCEHIGH)
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
#[doc = "GPIO49 output configuration.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum GPIO49OUTCFG_A {
    #[doc = "0: FNCSEL = 0x3 - Output disabled value."]
    DIS = 0,
    #[doc = "1: FNCSEL = 0x3 - Output is push-pull value."]
    PUSHPULL = 1,
    #[doc = "2: FNCSEL = 0x3 - Output is open drain value."]
    OD = 2,
    #[doc = "3: FNCSEL = 0x3 - Output is tri-state value."]
    TS = 3,
}
impl From<GPIO49OUTCFG_A> for u8 {
    #[inline(always)]
    fn from(variant: GPIO49OUTCFG_A) -> Self {
        variant as _
    }
}
#[doc = "Field `GPIO49OUTCFG` reader - GPIO49 output configuration."]
pub struct GPIO49OUTCFG_R(crate::FieldReader<u8, GPIO49OUTCFG_A>);
impl GPIO49OUTCFG_R {
    pub(crate) fn new(bits: u8) -> Self {
        GPIO49OUTCFG_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO49OUTCFG_A {
        match self.bits {
            0 => GPIO49OUTCFG_A::DIS,
            1 => GPIO49OUTCFG_A::PUSHPULL,
            2 => GPIO49OUTCFG_A::OD,
            3 => GPIO49OUTCFG_A::TS,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        **self == GPIO49OUTCFG_A::DIS
    }
    #[doc = "Checks if the value of the field is `PUSHPULL`"]
    #[inline(always)]
    pub fn is_pushpull(&self) -> bool {
        **self == GPIO49OUTCFG_A::PUSHPULL
    }
    #[doc = "Checks if the value of the field is `OD`"]
    #[inline(always)]
    pub fn is_od(&self) -> bool {
        **self == GPIO49OUTCFG_A::OD
    }
    #[doc = "Checks if the value of the field is `TS`"]
    #[inline(always)]
    pub fn is_ts(&self) -> bool {
        **self == GPIO49OUTCFG_A::TS
    }
}
impl core::ops::Deref for GPIO49OUTCFG_R {
    type Target = crate::FieldReader<u8, GPIO49OUTCFG_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIO49OUTCFG` writer - GPIO49 output configuration."]
pub struct GPIO49OUTCFG_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO49OUTCFG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIO49OUTCFG_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "FNCSEL = 0x3 - Output disabled value."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(GPIO49OUTCFG_A::DIS)
    }
    #[doc = "FNCSEL = 0x3 - Output is push-pull value."]
    #[inline(always)]
    pub fn pushpull(self) -> &'a mut W {
        self.variant(GPIO49OUTCFG_A::PUSHPULL)
    }
    #[doc = "FNCSEL = 0x3 - Output is open drain value."]
    #[inline(always)]
    pub fn od(self) -> &'a mut W {
        self.variant(GPIO49OUTCFG_A::OD)
    }
    #[doc = "FNCSEL = 0x3 - Output is tri-state value."]
    #[inline(always)]
    pub fn ts(self) -> &'a mut W {
        self.variant(GPIO49OUTCFG_A::TS)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 5)) | ((value as u32 & 0x03) << 5);
        self.w
    }
}
#[doc = "GPIO49 input enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO49INCFG_A {
    #[doc = "0: Read the GPIO pin data value."]
    READ = 0,
    #[doc = "1: INTD = 0 - Readback will always be zero value."]
    RDZERO = 1,
}
impl From<GPIO49INCFG_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO49INCFG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIO49INCFG` reader - GPIO49 input enable."]
pub struct GPIO49INCFG_R(crate::FieldReader<bool, GPIO49INCFG_A>);
impl GPIO49INCFG_R {
    pub(crate) fn new(bits: bool) -> Self {
        GPIO49INCFG_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO49INCFG_A {
        match self.bits {
            false => GPIO49INCFG_A::READ,
            true => GPIO49INCFG_A::RDZERO,
        }
    }
    #[doc = "Checks if the value of the field is `READ`"]
    #[inline(always)]
    pub fn is_read(&self) -> bool {
        **self == GPIO49INCFG_A::READ
    }
    #[doc = "Checks if the value of the field is `RDZERO`"]
    #[inline(always)]
    pub fn is_rdzero(&self) -> bool {
        **self == GPIO49INCFG_A::RDZERO
    }
}
impl core::ops::Deref for GPIO49INCFG_R {
    type Target = crate::FieldReader<bool, GPIO49INCFG_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIO49INCFG` writer - GPIO49 input enable."]
pub struct GPIO49INCFG_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO49INCFG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIO49INCFG_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Read the GPIO pin data value."]
    #[inline(always)]
    pub fn read(self) -> &'a mut W {
        self.variant(GPIO49INCFG_A::READ)
    }
    #[doc = "INTD = 0 - Readback will always be zero value."]
    #[inline(always)]
    pub fn rdzero(self) -> &'a mut W {
        self.variant(GPIO49INCFG_A::RDZERO)
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
#[doc = "GPIO48 interrupt direction.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO48INTD_A {
    #[doc = "0: FNCSEL = 0x1 - nCE polarity active low value."]
    NCELOW = 0,
    #[doc = "1: FNCSEL = 0x1 - nCE polarity active high value."]
    NCEHIGH = 1,
}
impl From<GPIO48INTD_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO48INTD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIO48INTD` reader - GPIO48 interrupt direction."]
pub struct GPIO48INTD_R(crate::FieldReader<bool, GPIO48INTD_A>);
impl GPIO48INTD_R {
    pub(crate) fn new(bits: bool) -> Self {
        GPIO48INTD_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO48INTD_A {
        match self.bits {
            false => GPIO48INTD_A::NCELOW,
            true => GPIO48INTD_A::NCEHIGH,
        }
    }
    #[doc = "Checks if the value of the field is `NCELOW`"]
    #[inline(always)]
    pub fn is_n_celow(&self) -> bool {
        **self == GPIO48INTD_A::NCELOW
    }
    #[doc = "Checks if the value of the field is `NCEHIGH`"]
    #[inline(always)]
    pub fn is_n_cehigh(&self) -> bool {
        **self == GPIO48INTD_A::NCEHIGH
    }
}
impl core::ops::Deref for GPIO48INTD_R {
    type Target = crate::FieldReader<bool, GPIO48INTD_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIO48INTD` writer - GPIO48 interrupt direction."]
pub struct GPIO48INTD_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO48INTD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIO48INTD_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "FNCSEL = 0x1 - nCE polarity active low value."]
    #[inline(always)]
    pub fn n_celow(self) -> &'a mut W {
        self.variant(GPIO48INTD_A::NCELOW)
    }
    #[doc = "FNCSEL = 0x1 - nCE polarity active high value."]
    #[inline(always)]
    pub fn n_cehigh(self) -> &'a mut W {
        self.variant(GPIO48INTD_A::NCEHIGH)
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
#[doc = "GPIO48 output configuration.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum GPIO48OUTCFG_A {
    #[doc = "0: FNCSEL = 0x3 - Output disabled value."]
    DIS = 0,
    #[doc = "1: FNCSEL = 0x3 - Output is push-pull value."]
    PUSHPULL = 1,
    #[doc = "2: FNCSEL = 0x3 - Output is open drain value."]
    OD = 2,
    #[doc = "3: FNCSEL = 0x3 - Output is tri-state value."]
    TS = 3,
}
impl From<GPIO48OUTCFG_A> for u8 {
    #[inline(always)]
    fn from(variant: GPIO48OUTCFG_A) -> Self {
        variant as _
    }
}
#[doc = "Field `GPIO48OUTCFG` reader - GPIO48 output configuration."]
pub struct GPIO48OUTCFG_R(crate::FieldReader<u8, GPIO48OUTCFG_A>);
impl GPIO48OUTCFG_R {
    pub(crate) fn new(bits: u8) -> Self {
        GPIO48OUTCFG_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO48OUTCFG_A {
        match self.bits {
            0 => GPIO48OUTCFG_A::DIS,
            1 => GPIO48OUTCFG_A::PUSHPULL,
            2 => GPIO48OUTCFG_A::OD,
            3 => GPIO48OUTCFG_A::TS,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        **self == GPIO48OUTCFG_A::DIS
    }
    #[doc = "Checks if the value of the field is `PUSHPULL`"]
    #[inline(always)]
    pub fn is_pushpull(&self) -> bool {
        **self == GPIO48OUTCFG_A::PUSHPULL
    }
    #[doc = "Checks if the value of the field is `OD`"]
    #[inline(always)]
    pub fn is_od(&self) -> bool {
        **self == GPIO48OUTCFG_A::OD
    }
    #[doc = "Checks if the value of the field is `TS`"]
    #[inline(always)]
    pub fn is_ts(&self) -> bool {
        **self == GPIO48OUTCFG_A::TS
    }
}
impl core::ops::Deref for GPIO48OUTCFG_R {
    type Target = crate::FieldReader<u8, GPIO48OUTCFG_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIO48OUTCFG` writer - GPIO48 output configuration."]
pub struct GPIO48OUTCFG_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO48OUTCFG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIO48OUTCFG_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "FNCSEL = 0x3 - Output disabled value."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(GPIO48OUTCFG_A::DIS)
    }
    #[doc = "FNCSEL = 0x3 - Output is push-pull value."]
    #[inline(always)]
    pub fn pushpull(self) -> &'a mut W {
        self.variant(GPIO48OUTCFG_A::PUSHPULL)
    }
    #[doc = "FNCSEL = 0x3 - Output is open drain value."]
    #[inline(always)]
    pub fn od(self) -> &'a mut W {
        self.variant(GPIO48OUTCFG_A::OD)
    }
    #[doc = "FNCSEL = 0x3 - Output is tri-state value."]
    #[inline(always)]
    pub fn ts(self) -> &'a mut W {
        self.variant(GPIO48OUTCFG_A::TS)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 1)) | ((value as u32 & 0x03) << 1);
        self.w
    }
}
#[doc = "GPIO48 input enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO48INCFG_A {
    #[doc = "0: Read the GPIO pin data value."]
    READ = 0,
    #[doc = "1: INTD = 0 - Readback will always be zero value."]
    RDZERO = 1,
}
impl From<GPIO48INCFG_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO48INCFG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIO48INCFG` reader - GPIO48 input enable."]
pub struct GPIO48INCFG_R(crate::FieldReader<bool, GPIO48INCFG_A>);
impl GPIO48INCFG_R {
    pub(crate) fn new(bits: bool) -> Self {
        GPIO48INCFG_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO48INCFG_A {
        match self.bits {
            false => GPIO48INCFG_A::READ,
            true => GPIO48INCFG_A::RDZERO,
        }
    }
    #[doc = "Checks if the value of the field is `READ`"]
    #[inline(always)]
    pub fn is_read(&self) -> bool {
        **self == GPIO48INCFG_A::READ
    }
    #[doc = "Checks if the value of the field is `RDZERO`"]
    #[inline(always)]
    pub fn is_rdzero(&self) -> bool {
        **self == GPIO48INCFG_A::RDZERO
    }
}
impl core::ops::Deref for GPIO48INCFG_R {
    type Target = crate::FieldReader<bool, GPIO48INCFG_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIO48INCFG` writer - GPIO48 input enable."]
pub struct GPIO48INCFG_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO48INCFG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIO48INCFG_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Read the GPIO pin data value."]
    #[inline(always)]
    pub fn read(self) -> &'a mut W {
        self.variant(GPIO48INCFG_A::READ)
    }
    #[doc = "INTD = 0 - Readback will always be zero value."]
    #[inline(always)]
    pub fn rdzero(self) -> &'a mut W {
        self.variant(GPIO48INCFG_A::RDZERO)
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
    #[doc = "Bit 7 - GPIO49 interrupt direction."]
    #[inline(always)]
    pub fn gpio49intd(&self) -> GPIO49INTD_R {
        GPIO49INTD_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 5:6 - GPIO49 output configuration."]
    #[inline(always)]
    pub fn gpio49outcfg(&self) -> GPIO49OUTCFG_R {
        GPIO49OUTCFG_R::new(((self.bits >> 5) & 0x03) as u8)
    }
    #[doc = "Bit 4 - GPIO49 input enable."]
    #[inline(always)]
    pub fn gpio49incfg(&self) -> GPIO49INCFG_R {
        GPIO49INCFG_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - GPIO48 interrupt direction."]
    #[inline(always)]
    pub fn gpio48intd(&self) -> GPIO48INTD_R {
        GPIO48INTD_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bits 1:2 - GPIO48 output configuration."]
    #[inline(always)]
    pub fn gpio48outcfg(&self) -> GPIO48OUTCFG_R {
        GPIO48OUTCFG_R::new(((self.bits >> 1) & 0x03) as u8)
    }
    #[doc = "Bit 0 - GPIO48 input enable."]
    #[inline(always)]
    pub fn gpio48incfg(&self) -> GPIO48INCFG_R {
        GPIO48INCFG_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 7 - GPIO49 interrupt direction."]
    #[inline(always)]
    pub fn gpio49intd(&mut self) -> GPIO49INTD_W {
        GPIO49INTD_W { w: self }
    }
    #[doc = "Bits 5:6 - GPIO49 output configuration."]
    #[inline(always)]
    pub fn gpio49outcfg(&mut self) -> GPIO49OUTCFG_W {
        GPIO49OUTCFG_W { w: self }
    }
    #[doc = "Bit 4 - GPIO49 input enable."]
    #[inline(always)]
    pub fn gpio49incfg(&mut self) -> GPIO49INCFG_W {
        GPIO49INCFG_W { w: self }
    }
    #[doc = "Bit 3 - GPIO48 interrupt direction."]
    #[inline(always)]
    pub fn gpio48intd(&mut self) -> GPIO48INTD_W {
        GPIO48INTD_W { w: self }
    }
    #[doc = "Bits 1:2 - GPIO48 output configuration."]
    #[inline(always)]
    pub fn gpio48outcfg(&mut self) -> GPIO48OUTCFG_W {
        GPIO48OUTCFG_W { w: self }
    }
    #[doc = "Bit 0 - GPIO48 input enable."]
    #[inline(always)]
    pub fn gpio48incfg(&mut self) -> GPIO48INCFG_W {
        GPIO48INCFG_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPIO Configuration Register G (Pads 48-49)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cfgg](index.html) module"]
pub struct CFGG_SPEC;
impl crate::RegisterSpec for CFGG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cfgg::R](R) reader structure"]
impl crate::Readable for CFGG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cfgg::W](W) writer structure"]
impl crate::Writable for CFGG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CFGG to value 0"]
impl crate::Resettable for CFGG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
