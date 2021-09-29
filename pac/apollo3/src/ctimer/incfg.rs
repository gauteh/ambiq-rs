#[doc = "Register `INCFG` reader"]
pub struct R(crate::R<INCFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INCFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INCFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INCFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INCFG` writer"]
pub struct W(crate::W<INCFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INCFG_SPEC>;
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
impl From<crate::W<INCFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INCFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "CTIMER B7 input configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CFGB7_A {
    #[doc = "1: Input is CT31 value."]
    CT31 = 1,
    #[doc = "0: Input is CT30 value."]
    CT30 = 0,
}
impl From<CFGB7_A> for bool {
    #[inline(always)]
    fn from(variant: CFGB7_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CFGB7` reader - CTIMER B7 input configuration"]
pub struct CFGB7_R(crate::FieldReader<bool, CFGB7_A>);
impl CFGB7_R {
    pub(crate) fn new(bits: bool) -> Self {
        CFGB7_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CFGB7_A {
        match self.bits {
            true => CFGB7_A::CT31,
            false => CFGB7_A::CT30,
        }
    }
    #[doc = "Checks if the value of the field is `CT31`"]
    #[inline(always)]
    pub fn is_ct31(&self) -> bool {
        **self == CFGB7_A::CT31
    }
    #[doc = "Checks if the value of the field is `CT30`"]
    #[inline(always)]
    pub fn is_ct30(&self) -> bool {
        **self == CFGB7_A::CT30
    }
}
impl core::ops::Deref for CFGB7_R {
    type Target = crate::FieldReader<bool, CFGB7_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CFGB7` writer - CTIMER B7 input configuration"]
pub struct CFGB7_W<'a> {
    w: &'a mut W,
}
impl<'a> CFGB7_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CFGB7_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Input is CT31 value."]
    #[inline(always)]
    pub fn ct31(self) -> &'a mut W {
        self.variant(CFGB7_A::CT31)
    }
    #[doc = "Input is CT30 value."]
    #[inline(always)]
    pub fn ct30(self) -> &'a mut W {
        self.variant(CFGB7_A::CT30)
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
#[doc = "CTIMER A7 input configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CFGA7_A {
    #[doc = "1: Input is CT29 value."]
    CT29 = 1,
    #[doc = "0: Input is CT28 value."]
    CT28 = 0,
}
impl From<CFGA7_A> for bool {
    #[inline(always)]
    fn from(variant: CFGA7_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CFGA7` reader - CTIMER A7 input configuration"]
pub struct CFGA7_R(crate::FieldReader<bool, CFGA7_A>);
impl CFGA7_R {
    pub(crate) fn new(bits: bool) -> Self {
        CFGA7_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CFGA7_A {
        match self.bits {
            true => CFGA7_A::CT29,
            false => CFGA7_A::CT28,
        }
    }
    #[doc = "Checks if the value of the field is `CT29`"]
    #[inline(always)]
    pub fn is_ct29(&self) -> bool {
        **self == CFGA7_A::CT29
    }
    #[doc = "Checks if the value of the field is `CT28`"]
    #[inline(always)]
    pub fn is_ct28(&self) -> bool {
        **self == CFGA7_A::CT28
    }
}
impl core::ops::Deref for CFGA7_R {
    type Target = crate::FieldReader<bool, CFGA7_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CFGA7` writer - CTIMER A7 input configuration"]
pub struct CFGA7_W<'a> {
    w: &'a mut W,
}
impl<'a> CFGA7_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CFGA7_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Input is CT29 value."]
    #[inline(always)]
    pub fn ct29(self) -> &'a mut W {
        self.variant(CFGA7_A::CT29)
    }
    #[doc = "Input is CT28 value."]
    #[inline(always)]
    pub fn ct28(self) -> &'a mut W {
        self.variant(CFGA7_A::CT28)
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | ((value as u32 & 0x01) << 14);
        self.w
    }
}
#[doc = "CTIMER B6 input configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CFGB6_A {
    #[doc = "1: Input is CT27 value."]
    CT27 = 1,
    #[doc = "0: Input is CT26 value."]
    CT26 = 0,
}
impl From<CFGB6_A> for bool {
    #[inline(always)]
    fn from(variant: CFGB6_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CFGB6` reader - CTIMER B6 input configuration"]
pub struct CFGB6_R(crate::FieldReader<bool, CFGB6_A>);
impl CFGB6_R {
    pub(crate) fn new(bits: bool) -> Self {
        CFGB6_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CFGB6_A {
        match self.bits {
            true => CFGB6_A::CT27,
            false => CFGB6_A::CT26,
        }
    }
    #[doc = "Checks if the value of the field is `CT27`"]
    #[inline(always)]
    pub fn is_ct27(&self) -> bool {
        **self == CFGB6_A::CT27
    }
    #[doc = "Checks if the value of the field is `CT26`"]
    #[inline(always)]
    pub fn is_ct26(&self) -> bool {
        **self == CFGB6_A::CT26
    }
}
impl core::ops::Deref for CFGB6_R {
    type Target = crate::FieldReader<bool, CFGB6_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CFGB6` writer - CTIMER B6 input configuration"]
pub struct CFGB6_W<'a> {
    w: &'a mut W,
}
impl<'a> CFGB6_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CFGB6_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Input is CT27 value."]
    #[inline(always)]
    pub fn ct27(self) -> &'a mut W {
        self.variant(CFGB6_A::CT27)
    }
    #[doc = "Input is CT26 value."]
    #[inline(always)]
    pub fn ct26(self) -> &'a mut W {
        self.variant(CFGB6_A::CT26)
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | ((value as u32 & 0x01) << 13);
        self.w
    }
}
#[doc = "CTIMER A6 input configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CFGA6_A {
    #[doc = "1: Input is CT25 value."]
    CT25 = 1,
    #[doc = "0: Input is CT24 value."]
    CT24 = 0,
}
impl From<CFGA6_A> for bool {
    #[inline(always)]
    fn from(variant: CFGA6_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CFGA6` reader - CTIMER A6 input configuration"]
pub struct CFGA6_R(crate::FieldReader<bool, CFGA6_A>);
impl CFGA6_R {
    pub(crate) fn new(bits: bool) -> Self {
        CFGA6_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CFGA6_A {
        match self.bits {
            true => CFGA6_A::CT25,
            false => CFGA6_A::CT24,
        }
    }
    #[doc = "Checks if the value of the field is `CT25`"]
    #[inline(always)]
    pub fn is_ct25(&self) -> bool {
        **self == CFGA6_A::CT25
    }
    #[doc = "Checks if the value of the field is `CT24`"]
    #[inline(always)]
    pub fn is_ct24(&self) -> bool {
        **self == CFGA6_A::CT24
    }
}
impl core::ops::Deref for CFGA6_R {
    type Target = crate::FieldReader<bool, CFGA6_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CFGA6` writer - CTIMER A6 input configuration"]
pub struct CFGA6_W<'a> {
    w: &'a mut W,
}
impl<'a> CFGA6_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CFGA6_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Input is CT25 value."]
    #[inline(always)]
    pub fn ct25(self) -> &'a mut W {
        self.variant(CFGA6_A::CT25)
    }
    #[doc = "Input is CT24 value."]
    #[inline(always)]
    pub fn ct24(self) -> &'a mut W {
        self.variant(CFGA6_A::CT24)
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
#[doc = "CTIMER B5 input configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CFGB5_A {
    #[doc = "1: Input is CT23 value."]
    CT23 = 1,
    #[doc = "0: Input is CT22 value."]
    CT22 = 0,
}
impl From<CFGB5_A> for bool {
    #[inline(always)]
    fn from(variant: CFGB5_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CFGB5` reader - CTIMER B5 input configuration"]
pub struct CFGB5_R(crate::FieldReader<bool, CFGB5_A>);
impl CFGB5_R {
    pub(crate) fn new(bits: bool) -> Self {
        CFGB5_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CFGB5_A {
        match self.bits {
            true => CFGB5_A::CT23,
            false => CFGB5_A::CT22,
        }
    }
    #[doc = "Checks if the value of the field is `CT23`"]
    #[inline(always)]
    pub fn is_ct23(&self) -> bool {
        **self == CFGB5_A::CT23
    }
    #[doc = "Checks if the value of the field is `CT22`"]
    #[inline(always)]
    pub fn is_ct22(&self) -> bool {
        **self == CFGB5_A::CT22
    }
}
impl core::ops::Deref for CFGB5_R {
    type Target = crate::FieldReader<bool, CFGB5_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CFGB5` writer - CTIMER B5 input configuration"]
pub struct CFGB5_W<'a> {
    w: &'a mut W,
}
impl<'a> CFGB5_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CFGB5_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Input is CT23 value."]
    #[inline(always)]
    pub fn ct23(self) -> &'a mut W {
        self.variant(CFGB5_A::CT23)
    }
    #[doc = "Input is CT22 value."]
    #[inline(always)]
    pub fn ct22(self) -> &'a mut W {
        self.variant(CFGB5_A::CT22)
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
#[doc = "CTIMER A5 input configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CFGA5_A {
    #[doc = "1: Input is CT21 value."]
    CT21 = 1,
    #[doc = "0: Input is CT20 value."]
    CT20 = 0,
}
impl From<CFGA5_A> for bool {
    #[inline(always)]
    fn from(variant: CFGA5_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CFGA5` reader - CTIMER A5 input configuration"]
pub struct CFGA5_R(crate::FieldReader<bool, CFGA5_A>);
impl CFGA5_R {
    pub(crate) fn new(bits: bool) -> Self {
        CFGA5_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CFGA5_A {
        match self.bits {
            true => CFGA5_A::CT21,
            false => CFGA5_A::CT20,
        }
    }
    #[doc = "Checks if the value of the field is `CT21`"]
    #[inline(always)]
    pub fn is_ct21(&self) -> bool {
        **self == CFGA5_A::CT21
    }
    #[doc = "Checks if the value of the field is `CT20`"]
    #[inline(always)]
    pub fn is_ct20(&self) -> bool {
        **self == CFGA5_A::CT20
    }
}
impl core::ops::Deref for CFGA5_R {
    type Target = crate::FieldReader<bool, CFGA5_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CFGA5` writer - CTIMER A5 input configuration"]
pub struct CFGA5_W<'a> {
    w: &'a mut W,
}
impl<'a> CFGA5_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CFGA5_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Input is CT21 value."]
    #[inline(always)]
    pub fn ct21(self) -> &'a mut W {
        self.variant(CFGA5_A::CT21)
    }
    #[doc = "Input is CT20 value."]
    #[inline(always)]
    pub fn ct20(self) -> &'a mut W {
        self.variant(CFGA5_A::CT20)
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
#[doc = "CTIMER B4 input configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CFGB4_A {
    #[doc = "1: Input is CT19 value."]
    CT19 = 1,
    #[doc = "0: Input is CT18 value."]
    CT18 = 0,
}
impl From<CFGB4_A> for bool {
    #[inline(always)]
    fn from(variant: CFGB4_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CFGB4` reader - CTIMER B4 input configuration"]
pub struct CFGB4_R(crate::FieldReader<bool, CFGB4_A>);
impl CFGB4_R {
    pub(crate) fn new(bits: bool) -> Self {
        CFGB4_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CFGB4_A {
        match self.bits {
            true => CFGB4_A::CT19,
            false => CFGB4_A::CT18,
        }
    }
    #[doc = "Checks if the value of the field is `CT19`"]
    #[inline(always)]
    pub fn is_ct19(&self) -> bool {
        **self == CFGB4_A::CT19
    }
    #[doc = "Checks if the value of the field is `CT18`"]
    #[inline(always)]
    pub fn is_ct18(&self) -> bool {
        **self == CFGB4_A::CT18
    }
}
impl core::ops::Deref for CFGB4_R {
    type Target = crate::FieldReader<bool, CFGB4_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CFGB4` writer - CTIMER B4 input configuration"]
pub struct CFGB4_W<'a> {
    w: &'a mut W,
}
impl<'a> CFGB4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CFGB4_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Input is CT19 value."]
    #[inline(always)]
    pub fn ct19(self) -> &'a mut W {
        self.variant(CFGB4_A::CT19)
    }
    #[doc = "Input is CT18 value."]
    #[inline(always)]
    pub fn ct18(self) -> &'a mut W {
        self.variant(CFGB4_A::CT18)
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
#[doc = "CTIMER A4 input configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CFGA4_A {
    #[doc = "1: Input is CT17 value."]
    CT17 = 1,
    #[doc = "0: Input is CT16 value."]
    CT16 = 0,
}
impl From<CFGA4_A> for bool {
    #[inline(always)]
    fn from(variant: CFGA4_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CFGA4` reader - CTIMER A4 input configuration"]
pub struct CFGA4_R(crate::FieldReader<bool, CFGA4_A>);
impl CFGA4_R {
    pub(crate) fn new(bits: bool) -> Self {
        CFGA4_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CFGA4_A {
        match self.bits {
            true => CFGA4_A::CT17,
            false => CFGA4_A::CT16,
        }
    }
    #[doc = "Checks if the value of the field is `CT17`"]
    #[inline(always)]
    pub fn is_ct17(&self) -> bool {
        **self == CFGA4_A::CT17
    }
    #[doc = "Checks if the value of the field is `CT16`"]
    #[inline(always)]
    pub fn is_ct16(&self) -> bool {
        **self == CFGA4_A::CT16
    }
}
impl core::ops::Deref for CFGA4_R {
    type Target = crate::FieldReader<bool, CFGA4_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CFGA4` writer - CTIMER A4 input configuration"]
pub struct CFGA4_W<'a> {
    w: &'a mut W,
}
impl<'a> CFGA4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CFGA4_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Input is CT17 value."]
    #[inline(always)]
    pub fn ct17(self) -> &'a mut W {
        self.variant(CFGA4_A::CT17)
    }
    #[doc = "Input is CT16 value."]
    #[inline(always)]
    pub fn ct16(self) -> &'a mut W {
        self.variant(CFGA4_A::CT16)
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
#[doc = "CTIMER B3 input configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CFGB3_A {
    #[doc = "1: Input is CT15 value."]
    CT15 = 1,
    #[doc = "0: Input is CT14 value."]
    CT14 = 0,
}
impl From<CFGB3_A> for bool {
    #[inline(always)]
    fn from(variant: CFGB3_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CFGB3` reader - CTIMER B3 input configuration"]
pub struct CFGB3_R(crate::FieldReader<bool, CFGB3_A>);
impl CFGB3_R {
    pub(crate) fn new(bits: bool) -> Self {
        CFGB3_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CFGB3_A {
        match self.bits {
            true => CFGB3_A::CT15,
            false => CFGB3_A::CT14,
        }
    }
    #[doc = "Checks if the value of the field is `CT15`"]
    #[inline(always)]
    pub fn is_ct15(&self) -> bool {
        **self == CFGB3_A::CT15
    }
    #[doc = "Checks if the value of the field is `CT14`"]
    #[inline(always)]
    pub fn is_ct14(&self) -> bool {
        **self == CFGB3_A::CT14
    }
}
impl core::ops::Deref for CFGB3_R {
    type Target = crate::FieldReader<bool, CFGB3_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CFGB3` writer - CTIMER B3 input configuration"]
pub struct CFGB3_W<'a> {
    w: &'a mut W,
}
impl<'a> CFGB3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CFGB3_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Input is CT15 value."]
    #[inline(always)]
    pub fn ct15(self) -> &'a mut W {
        self.variant(CFGB3_A::CT15)
    }
    #[doc = "Input is CT14 value."]
    #[inline(always)]
    pub fn ct14(self) -> &'a mut W {
        self.variant(CFGB3_A::CT14)
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
#[doc = "CTIMER A3 input configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CFGA3_A {
    #[doc = "1: Input is CT13 value."]
    CT13 = 1,
    #[doc = "0: Input is CT12 value."]
    CT12 = 0,
}
impl From<CFGA3_A> for bool {
    #[inline(always)]
    fn from(variant: CFGA3_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CFGA3` reader - CTIMER A3 input configuration"]
pub struct CFGA3_R(crate::FieldReader<bool, CFGA3_A>);
impl CFGA3_R {
    pub(crate) fn new(bits: bool) -> Self {
        CFGA3_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CFGA3_A {
        match self.bits {
            true => CFGA3_A::CT13,
            false => CFGA3_A::CT12,
        }
    }
    #[doc = "Checks if the value of the field is `CT13`"]
    #[inline(always)]
    pub fn is_ct13(&self) -> bool {
        **self == CFGA3_A::CT13
    }
    #[doc = "Checks if the value of the field is `CT12`"]
    #[inline(always)]
    pub fn is_ct12(&self) -> bool {
        **self == CFGA3_A::CT12
    }
}
impl core::ops::Deref for CFGA3_R {
    type Target = crate::FieldReader<bool, CFGA3_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CFGA3` writer - CTIMER A3 input configuration"]
pub struct CFGA3_W<'a> {
    w: &'a mut W,
}
impl<'a> CFGA3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CFGA3_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Input is CT13 value."]
    #[inline(always)]
    pub fn ct13(self) -> &'a mut W {
        self.variant(CFGA3_A::CT13)
    }
    #[doc = "Input is CT12 value."]
    #[inline(always)]
    pub fn ct12(self) -> &'a mut W {
        self.variant(CFGA3_A::CT12)
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | ((value as u32 & 0x01) << 6);
        self.w
    }
}
#[doc = "CTIMER B2 input configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CFGB2_A {
    #[doc = "1: Input is CT11 value."]
    CT11 = 1,
    #[doc = "0: Input is CT10 value."]
    CT10 = 0,
}
impl From<CFGB2_A> for bool {
    #[inline(always)]
    fn from(variant: CFGB2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CFGB2` reader - CTIMER B2 input configuration"]
pub struct CFGB2_R(crate::FieldReader<bool, CFGB2_A>);
impl CFGB2_R {
    pub(crate) fn new(bits: bool) -> Self {
        CFGB2_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CFGB2_A {
        match self.bits {
            true => CFGB2_A::CT11,
            false => CFGB2_A::CT10,
        }
    }
    #[doc = "Checks if the value of the field is `CT11`"]
    #[inline(always)]
    pub fn is_ct11(&self) -> bool {
        **self == CFGB2_A::CT11
    }
    #[doc = "Checks if the value of the field is `CT10`"]
    #[inline(always)]
    pub fn is_ct10(&self) -> bool {
        **self == CFGB2_A::CT10
    }
}
impl core::ops::Deref for CFGB2_R {
    type Target = crate::FieldReader<bool, CFGB2_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CFGB2` writer - CTIMER B2 input configuration"]
pub struct CFGB2_W<'a> {
    w: &'a mut W,
}
impl<'a> CFGB2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CFGB2_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Input is CT11 value."]
    #[inline(always)]
    pub fn ct11(self) -> &'a mut W {
        self.variant(CFGB2_A::CT11)
    }
    #[doc = "Input is CT10 value."]
    #[inline(always)]
    pub fn ct10(self) -> &'a mut W {
        self.variant(CFGB2_A::CT10)
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u32 & 0x01) << 5);
        self.w
    }
}
#[doc = "CTIMER A2 input configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CFGA2_A {
    #[doc = "1: Input is CT9 value."]
    CT9 = 1,
    #[doc = "0: Input is CT8 value."]
    CT8 = 0,
}
impl From<CFGA2_A> for bool {
    #[inline(always)]
    fn from(variant: CFGA2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CFGA2` reader - CTIMER A2 input configuration"]
pub struct CFGA2_R(crate::FieldReader<bool, CFGA2_A>);
impl CFGA2_R {
    pub(crate) fn new(bits: bool) -> Self {
        CFGA2_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CFGA2_A {
        match self.bits {
            true => CFGA2_A::CT9,
            false => CFGA2_A::CT8,
        }
    }
    #[doc = "Checks if the value of the field is `CT9`"]
    #[inline(always)]
    pub fn is_ct9(&self) -> bool {
        **self == CFGA2_A::CT9
    }
    #[doc = "Checks if the value of the field is `CT8`"]
    #[inline(always)]
    pub fn is_ct8(&self) -> bool {
        **self == CFGA2_A::CT8
    }
}
impl core::ops::Deref for CFGA2_R {
    type Target = crate::FieldReader<bool, CFGA2_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CFGA2` writer - CTIMER A2 input configuration"]
pub struct CFGA2_W<'a> {
    w: &'a mut W,
}
impl<'a> CFGA2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CFGA2_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Input is CT9 value."]
    #[inline(always)]
    pub fn ct9(self) -> &'a mut W {
        self.variant(CFGA2_A::CT9)
    }
    #[doc = "Input is CT8 value."]
    #[inline(always)]
    pub fn ct8(self) -> &'a mut W {
        self.variant(CFGA2_A::CT8)
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
#[doc = "CTIMER B1 input configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CFGB1_A {
    #[doc = "1: Input is CT7 value."]
    CT7 = 1,
    #[doc = "0: Input is CT6 value."]
    CT6 = 0,
}
impl From<CFGB1_A> for bool {
    #[inline(always)]
    fn from(variant: CFGB1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CFGB1` reader - CTIMER B1 input configuration"]
pub struct CFGB1_R(crate::FieldReader<bool, CFGB1_A>);
impl CFGB1_R {
    pub(crate) fn new(bits: bool) -> Self {
        CFGB1_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CFGB1_A {
        match self.bits {
            true => CFGB1_A::CT7,
            false => CFGB1_A::CT6,
        }
    }
    #[doc = "Checks if the value of the field is `CT7`"]
    #[inline(always)]
    pub fn is_ct7(&self) -> bool {
        **self == CFGB1_A::CT7
    }
    #[doc = "Checks if the value of the field is `CT6`"]
    #[inline(always)]
    pub fn is_ct6(&self) -> bool {
        **self == CFGB1_A::CT6
    }
}
impl core::ops::Deref for CFGB1_R {
    type Target = crate::FieldReader<bool, CFGB1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CFGB1` writer - CTIMER B1 input configuration"]
pub struct CFGB1_W<'a> {
    w: &'a mut W,
}
impl<'a> CFGB1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CFGB1_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Input is CT7 value."]
    #[inline(always)]
    pub fn ct7(self) -> &'a mut W {
        self.variant(CFGB1_A::CT7)
    }
    #[doc = "Input is CT6 value."]
    #[inline(always)]
    pub fn ct6(self) -> &'a mut W {
        self.variant(CFGB1_A::CT6)
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
#[doc = "CTIMER A1 input configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CFGA1_A {
    #[doc = "1: Input is CT5 value."]
    CT5 = 1,
    #[doc = "0: Input is CT4 value."]
    CT4 = 0,
}
impl From<CFGA1_A> for bool {
    #[inline(always)]
    fn from(variant: CFGA1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CFGA1` reader - CTIMER A1 input configuration"]
pub struct CFGA1_R(crate::FieldReader<bool, CFGA1_A>);
impl CFGA1_R {
    pub(crate) fn new(bits: bool) -> Self {
        CFGA1_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CFGA1_A {
        match self.bits {
            true => CFGA1_A::CT5,
            false => CFGA1_A::CT4,
        }
    }
    #[doc = "Checks if the value of the field is `CT5`"]
    #[inline(always)]
    pub fn is_ct5(&self) -> bool {
        **self == CFGA1_A::CT5
    }
    #[doc = "Checks if the value of the field is `CT4`"]
    #[inline(always)]
    pub fn is_ct4(&self) -> bool {
        **self == CFGA1_A::CT4
    }
}
impl core::ops::Deref for CFGA1_R {
    type Target = crate::FieldReader<bool, CFGA1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CFGA1` writer - CTIMER A1 input configuration"]
pub struct CFGA1_W<'a> {
    w: &'a mut W,
}
impl<'a> CFGA1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CFGA1_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Input is CT5 value."]
    #[inline(always)]
    pub fn ct5(self) -> &'a mut W {
        self.variant(CFGA1_A::CT5)
    }
    #[doc = "Input is CT4 value."]
    #[inline(always)]
    pub fn ct4(self) -> &'a mut W {
        self.variant(CFGA1_A::CT4)
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
#[doc = "CTIMER B0 input configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CFGB0_A {
    #[doc = "1: Input is CT3 value."]
    CT3 = 1,
    #[doc = "0: Input is CT2 value."]
    CT2 = 0,
}
impl From<CFGB0_A> for bool {
    #[inline(always)]
    fn from(variant: CFGB0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CFGB0` reader - CTIMER B0 input configuration"]
pub struct CFGB0_R(crate::FieldReader<bool, CFGB0_A>);
impl CFGB0_R {
    pub(crate) fn new(bits: bool) -> Self {
        CFGB0_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CFGB0_A {
        match self.bits {
            true => CFGB0_A::CT3,
            false => CFGB0_A::CT2,
        }
    }
    #[doc = "Checks if the value of the field is `CT3`"]
    #[inline(always)]
    pub fn is_ct3(&self) -> bool {
        **self == CFGB0_A::CT3
    }
    #[doc = "Checks if the value of the field is `CT2`"]
    #[inline(always)]
    pub fn is_ct2(&self) -> bool {
        **self == CFGB0_A::CT2
    }
}
impl core::ops::Deref for CFGB0_R {
    type Target = crate::FieldReader<bool, CFGB0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CFGB0` writer - CTIMER B0 input configuration"]
pub struct CFGB0_W<'a> {
    w: &'a mut W,
}
impl<'a> CFGB0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CFGB0_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Input is CT3 value."]
    #[inline(always)]
    pub fn ct3(self) -> &'a mut W {
        self.variant(CFGB0_A::CT3)
    }
    #[doc = "Input is CT2 value."]
    #[inline(always)]
    pub fn ct2(self) -> &'a mut W {
        self.variant(CFGB0_A::CT2)
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
#[doc = "CTIMER A0 input configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CFGA0_A {
    #[doc = "1: Input is CT1 value."]
    CT1 = 1,
    #[doc = "0: Input is CT0 value."]
    CT0 = 0,
}
impl From<CFGA0_A> for bool {
    #[inline(always)]
    fn from(variant: CFGA0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CFGA0` reader - CTIMER A0 input configuration"]
pub struct CFGA0_R(crate::FieldReader<bool, CFGA0_A>);
impl CFGA0_R {
    pub(crate) fn new(bits: bool) -> Self {
        CFGA0_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CFGA0_A {
        match self.bits {
            true => CFGA0_A::CT1,
            false => CFGA0_A::CT0,
        }
    }
    #[doc = "Checks if the value of the field is `CT1`"]
    #[inline(always)]
    pub fn is_ct1(&self) -> bool {
        **self == CFGA0_A::CT1
    }
    #[doc = "Checks if the value of the field is `CT0`"]
    #[inline(always)]
    pub fn is_ct0(&self) -> bool {
        **self == CFGA0_A::CT0
    }
}
impl core::ops::Deref for CFGA0_R {
    type Target = crate::FieldReader<bool, CFGA0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CFGA0` writer - CTIMER A0 input configuration"]
pub struct CFGA0_W<'a> {
    w: &'a mut W,
}
impl<'a> CFGA0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CFGA0_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Input is CT1 value."]
    #[inline(always)]
    pub fn ct1(self) -> &'a mut W {
        self.variant(CFGA0_A::CT1)
    }
    #[doc = "Input is CT0 value."]
    #[inline(always)]
    pub fn ct0(self) -> &'a mut W {
        self.variant(CFGA0_A::CT0)
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
    #[doc = "Bit 15 - CTIMER B7 input configuration"]
    #[inline(always)]
    pub fn cfgb7(&self) -> CFGB7_R {
        CFGB7_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 14 - CTIMER A7 input configuration"]
    #[inline(always)]
    pub fn cfga7(&self) -> CFGA7_R {
        CFGA7_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 13 - CTIMER B6 input configuration"]
    #[inline(always)]
    pub fn cfgb6(&self) -> CFGB6_R {
        CFGB6_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12 - CTIMER A6 input configuration"]
    #[inline(always)]
    pub fn cfga6(&self) -> CFGA6_R {
        CFGA6_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11 - CTIMER B5 input configuration"]
    #[inline(always)]
    pub fn cfgb5(&self) -> CFGB5_R {
        CFGB5_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10 - CTIMER A5 input configuration"]
    #[inline(always)]
    pub fn cfga5(&self) -> CFGA5_R {
        CFGA5_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - CTIMER B4 input configuration"]
    #[inline(always)]
    pub fn cfgb4(&self) -> CFGB4_R {
        CFGB4_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - CTIMER A4 input configuration"]
    #[inline(always)]
    pub fn cfga4(&self) -> CFGA4_R {
        CFGA4_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7 - CTIMER B3 input configuration"]
    #[inline(always)]
    pub fn cfgb3(&self) -> CFGB3_R {
        CFGB3_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - CTIMER A3 input configuration"]
    #[inline(always)]
    pub fn cfga3(&self) -> CFGA3_R {
        CFGA3_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - CTIMER B2 input configuration"]
    #[inline(always)]
    pub fn cfgb2(&self) -> CFGB2_R {
        CFGB2_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - CTIMER A2 input configuration"]
    #[inline(always)]
    pub fn cfga2(&self) -> CFGA2_R {
        CFGA2_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - CTIMER B1 input configuration"]
    #[inline(always)]
    pub fn cfgb1(&self) -> CFGB1_R {
        CFGB1_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - CTIMER A1 input configuration"]
    #[inline(always)]
    pub fn cfga1(&self) -> CFGA1_R {
        CFGA1_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - CTIMER B0 input configuration"]
    #[inline(always)]
    pub fn cfgb0(&self) -> CFGB0_R {
        CFGB0_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - CTIMER A0 input configuration"]
    #[inline(always)]
    pub fn cfga0(&self) -> CFGA0_R {
        CFGA0_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 15 - CTIMER B7 input configuration"]
    #[inline(always)]
    pub fn cfgb7(&mut self) -> CFGB7_W {
        CFGB7_W { w: self }
    }
    #[doc = "Bit 14 - CTIMER A7 input configuration"]
    #[inline(always)]
    pub fn cfga7(&mut self) -> CFGA7_W {
        CFGA7_W { w: self }
    }
    #[doc = "Bit 13 - CTIMER B6 input configuration"]
    #[inline(always)]
    pub fn cfgb6(&mut self) -> CFGB6_W {
        CFGB6_W { w: self }
    }
    #[doc = "Bit 12 - CTIMER A6 input configuration"]
    #[inline(always)]
    pub fn cfga6(&mut self) -> CFGA6_W {
        CFGA6_W { w: self }
    }
    #[doc = "Bit 11 - CTIMER B5 input configuration"]
    #[inline(always)]
    pub fn cfgb5(&mut self) -> CFGB5_W {
        CFGB5_W { w: self }
    }
    #[doc = "Bit 10 - CTIMER A5 input configuration"]
    #[inline(always)]
    pub fn cfga5(&mut self) -> CFGA5_W {
        CFGA5_W { w: self }
    }
    #[doc = "Bit 9 - CTIMER B4 input configuration"]
    #[inline(always)]
    pub fn cfgb4(&mut self) -> CFGB4_W {
        CFGB4_W { w: self }
    }
    #[doc = "Bit 8 - CTIMER A4 input configuration"]
    #[inline(always)]
    pub fn cfga4(&mut self) -> CFGA4_W {
        CFGA4_W { w: self }
    }
    #[doc = "Bit 7 - CTIMER B3 input configuration"]
    #[inline(always)]
    pub fn cfgb3(&mut self) -> CFGB3_W {
        CFGB3_W { w: self }
    }
    #[doc = "Bit 6 - CTIMER A3 input configuration"]
    #[inline(always)]
    pub fn cfga3(&mut self) -> CFGA3_W {
        CFGA3_W { w: self }
    }
    #[doc = "Bit 5 - CTIMER B2 input configuration"]
    #[inline(always)]
    pub fn cfgb2(&mut self) -> CFGB2_W {
        CFGB2_W { w: self }
    }
    #[doc = "Bit 4 - CTIMER A2 input configuration"]
    #[inline(always)]
    pub fn cfga2(&mut self) -> CFGA2_W {
        CFGA2_W { w: self }
    }
    #[doc = "Bit 3 - CTIMER B1 input configuration"]
    #[inline(always)]
    pub fn cfgb1(&mut self) -> CFGB1_W {
        CFGB1_W { w: self }
    }
    #[doc = "Bit 2 - CTIMER A1 input configuration"]
    #[inline(always)]
    pub fn cfga1(&mut self) -> CFGA1_W {
        CFGA1_W { w: self }
    }
    #[doc = "Bit 1 - CTIMER B0 input configuration"]
    #[inline(always)]
    pub fn cfgb0(&mut self) -> CFGB0_W {
        CFGB0_W { w: self }
    }
    #[doc = "Bit 0 - CTIMER A0 input configuration"]
    #[inline(always)]
    pub fn cfga0(&mut self) -> CFGA0_W {
        CFGA0_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Counter/Timer Input Config\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [incfg](index.html) module"]
pub struct INCFG_SPEC;
impl crate::RegisterSpec for INCFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [incfg::R](R) reader structure"]
impl crate::Readable for INCFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [incfg::W](W) writer structure"]
impl crate::Writable for INCFG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets INCFG to value 0"]
impl crate::Resettable for INCFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
