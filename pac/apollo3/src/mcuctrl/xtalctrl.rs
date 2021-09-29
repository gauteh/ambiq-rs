#[doc = "Register `XTALCTRL` reader"]
pub struct R(crate::R<XTALCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<XTALCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<XTALCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<XTALCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `XTALCTRL` writer"]
pub struct W(crate::W<XTALCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<XTALCTRL_SPEC>;
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
impl From<crate::W<XTALCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<XTALCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `XTALICOMPTRIM` reader - XTAL ICOMP trim"]
pub struct XTALICOMPTRIM_R(crate::FieldReader<u8, u8>);
impl XTALICOMPTRIM_R {
    pub(crate) fn new(bits: u8) -> Self {
        XTALICOMPTRIM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for XTALICOMPTRIM_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `XTALICOMPTRIM` writer - XTAL ICOMP trim"]
pub struct XTALICOMPTRIM_W<'a> {
    w: &'a mut W,
}
impl<'a> XTALICOMPTRIM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | ((value as u32 & 0x03) << 8);
        self.w
    }
}
#[doc = "Field `XTALIBUFTRIM` reader - XTAL IBUFF trim"]
pub struct XTALIBUFTRIM_R(crate::FieldReader<u8, u8>);
impl XTALIBUFTRIM_R {
    pub(crate) fn new(bits: u8) -> Self {
        XTALIBUFTRIM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for XTALIBUFTRIM_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `XTALIBUFTRIM` writer - XTAL IBUFF trim"]
pub struct XTALIBUFTRIM_W<'a> {
    w: &'a mut W,
}
impl<'a> XTALIBUFTRIM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | ((value as u32 & 0x03) << 6);
        self.w
    }
}
#[doc = "XTAL Power down on brown out.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWDBODXTAL_A {
    #[doc = "0: Power up xtal on BOD value."]
    PWRUPBOD = 0,
    #[doc = "1: Power down XTAL on BOD. value."]
    PWRDNBOD = 1,
}
impl From<PWDBODXTAL_A> for bool {
    #[inline(always)]
    fn from(variant: PWDBODXTAL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PWDBODXTAL` reader - XTAL Power down on brown out."]
pub struct PWDBODXTAL_R(crate::FieldReader<bool, PWDBODXTAL_A>);
impl PWDBODXTAL_R {
    pub(crate) fn new(bits: bool) -> Self {
        PWDBODXTAL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PWDBODXTAL_A {
        match self.bits {
            false => PWDBODXTAL_A::PWRUPBOD,
            true => PWDBODXTAL_A::PWRDNBOD,
        }
    }
    #[doc = "Checks if the value of the field is `PWRUPBOD`"]
    #[inline(always)]
    pub fn is_pwrupbod(&self) -> bool {
        **self == PWDBODXTAL_A::PWRUPBOD
    }
    #[doc = "Checks if the value of the field is `PWRDNBOD`"]
    #[inline(always)]
    pub fn is_pwrdnbod(&self) -> bool {
        **self == PWDBODXTAL_A::PWRDNBOD
    }
}
impl core::ops::Deref for PWDBODXTAL_R {
    type Target = crate::FieldReader<bool, PWDBODXTAL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PWDBODXTAL` writer - XTAL Power down on brown out."]
pub struct PWDBODXTAL_W<'a> {
    w: &'a mut W,
}
impl<'a> PWDBODXTAL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PWDBODXTAL_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Power up xtal on BOD value."]
    #[inline(always)]
    pub fn pwrupbod(self) -> &'a mut W {
        self.variant(PWDBODXTAL_A::PWRUPBOD)
    }
    #[doc = "Power down XTAL on BOD. value."]
    #[inline(always)]
    pub fn pwrdnbod(self) -> &'a mut W {
        self.variant(PWDBODXTAL_A::PWRDNBOD)
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
#[doc = "XTAL Oscillator Power Down Comparator.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PDNBCMPRXTAL_A {
    #[doc = "1: Power up XTAL oscillator comparator. value."]
    PWRUPCOMP = 1,
    #[doc = "0: Power down XTAL oscillator comparator. value."]
    PWRDNCOMP = 0,
}
impl From<PDNBCMPRXTAL_A> for bool {
    #[inline(always)]
    fn from(variant: PDNBCMPRXTAL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PDNBCMPRXTAL` reader - XTAL Oscillator Power Down Comparator."]
pub struct PDNBCMPRXTAL_R(crate::FieldReader<bool, PDNBCMPRXTAL_A>);
impl PDNBCMPRXTAL_R {
    pub(crate) fn new(bits: bool) -> Self {
        PDNBCMPRXTAL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PDNBCMPRXTAL_A {
        match self.bits {
            true => PDNBCMPRXTAL_A::PWRUPCOMP,
            false => PDNBCMPRXTAL_A::PWRDNCOMP,
        }
    }
    #[doc = "Checks if the value of the field is `PWRUPCOMP`"]
    #[inline(always)]
    pub fn is_pwrupcomp(&self) -> bool {
        **self == PDNBCMPRXTAL_A::PWRUPCOMP
    }
    #[doc = "Checks if the value of the field is `PWRDNCOMP`"]
    #[inline(always)]
    pub fn is_pwrdncomp(&self) -> bool {
        **self == PDNBCMPRXTAL_A::PWRDNCOMP
    }
}
impl core::ops::Deref for PDNBCMPRXTAL_R {
    type Target = crate::FieldReader<bool, PDNBCMPRXTAL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PDNBCMPRXTAL` writer - XTAL Oscillator Power Down Comparator."]
pub struct PDNBCMPRXTAL_W<'a> {
    w: &'a mut W,
}
impl<'a> PDNBCMPRXTAL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PDNBCMPRXTAL_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Power up XTAL oscillator comparator. value."]
    #[inline(always)]
    pub fn pwrupcomp(self) -> &'a mut W {
        self.variant(PDNBCMPRXTAL_A::PWRUPCOMP)
    }
    #[doc = "Power down XTAL oscillator comparator. value."]
    #[inline(always)]
    pub fn pwrdncomp(self) -> &'a mut W {
        self.variant(PDNBCMPRXTAL_A::PWRDNCOMP)
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
#[doc = "XTAL Oscillator Power Down Core.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PDNBCOREXTAL_A {
    #[doc = "1: Power up XTAL oscillator core. value."]
    PWRUPCORE = 1,
    #[doc = "0: Power down XTAL oscillator core. value."]
    PWRDNCORE = 0,
}
impl From<PDNBCOREXTAL_A> for bool {
    #[inline(always)]
    fn from(variant: PDNBCOREXTAL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PDNBCOREXTAL` reader - XTAL Oscillator Power Down Core."]
pub struct PDNBCOREXTAL_R(crate::FieldReader<bool, PDNBCOREXTAL_A>);
impl PDNBCOREXTAL_R {
    pub(crate) fn new(bits: bool) -> Self {
        PDNBCOREXTAL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PDNBCOREXTAL_A {
        match self.bits {
            true => PDNBCOREXTAL_A::PWRUPCORE,
            false => PDNBCOREXTAL_A::PWRDNCORE,
        }
    }
    #[doc = "Checks if the value of the field is `PWRUPCORE`"]
    #[inline(always)]
    pub fn is_pwrupcore(&self) -> bool {
        **self == PDNBCOREXTAL_A::PWRUPCORE
    }
    #[doc = "Checks if the value of the field is `PWRDNCORE`"]
    #[inline(always)]
    pub fn is_pwrdncore(&self) -> bool {
        **self == PDNBCOREXTAL_A::PWRDNCORE
    }
}
impl core::ops::Deref for PDNBCOREXTAL_R {
    type Target = crate::FieldReader<bool, PDNBCOREXTAL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PDNBCOREXTAL` writer - XTAL Oscillator Power Down Core."]
pub struct PDNBCOREXTAL_W<'a> {
    w: &'a mut W,
}
impl<'a> PDNBCOREXTAL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PDNBCOREXTAL_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Power up XTAL oscillator core. value."]
    #[inline(always)]
    pub fn pwrupcore(self) -> &'a mut W {
        self.variant(PDNBCOREXTAL_A::PWRUPCORE)
    }
    #[doc = "Power down XTAL oscillator core. value."]
    #[inline(always)]
    pub fn pwrdncore(self) -> &'a mut W {
        self.variant(PDNBCOREXTAL_A::PWRDNCORE)
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
#[doc = "XTAL Oscillator Bypass Comparator.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BYPCMPRXTAL_A {
    #[doc = "0: Use the XTAL oscillator comparator. value."]
    USECOMP = 0,
    #[doc = "1: Bypass the XTAL oscillator comparator. value."]
    BYPCOMP = 1,
}
impl From<BYPCMPRXTAL_A> for bool {
    #[inline(always)]
    fn from(variant: BYPCMPRXTAL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BYPCMPRXTAL` reader - XTAL Oscillator Bypass Comparator."]
pub struct BYPCMPRXTAL_R(crate::FieldReader<bool, BYPCMPRXTAL_A>);
impl BYPCMPRXTAL_R {
    pub(crate) fn new(bits: bool) -> Self {
        BYPCMPRXTAL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BYPCMPRXTAL_A {
        match self.bits {
            false => BYPCMPRXTAL_A::USECOMP,
            true => BYPCMPRXTAL_A::BYPCOMP,
        }
    }
    #[doc = "Checks if the value of the field is `USECOMP`"]
    #[inline(always)]
    pub fn is_usecomp(&self) -> bool {
        **self == BYPCMPRXTAL_A::USECOMP
    }
    #[doc = "Checks if the value of the field is `BYPCOMP`"]
    #[inline(always)]
    pub fn is_bypcomp(&self) -> bool {
        **self == BYPCMPRXTAL_A::BYPCOMP
    }
}
impl core::ops::Deref for BYPCMPRXTAL_R {
    type Target = crate::FieldReader<bool, BYPCMPRXTAL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BYPCMPRXTAL` writer - XTAL Oscillator Bypass Comparator."]
pub struct BYPCMPRXTAL_W<'a> {
    w: &'a mut W,
}
impl<'a> BYPCMPRXTAL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BYPCMPRXTAL_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Use the XTAL oscillator comparator. value."]
    #[inline(always)]
    pub fn usecomp(self) -> &'a mut W {
        self.variant(BYPCMPRXTAL_A::USECOMP)
    }
    #[doc = "Bypass the XTAL oscillator comparator. value."]
    #[inline(always)]
    pub fn bypcomp(self) -> &'a mut W {
        self.variant(BYPCMPRXTAL_A::BYPCOMP)
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
#[doc = "XTAL Oscillator Disable Feedback.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FDBKDSBLXTAL_A {
    #[doc = "0: Enable XTAL oscillator comparator. value."]
    EN = 0,
    #[doc = "1: Disable XTAL oscillator comparator. value."]
    DIS = 1,
}
impl From<FDBKDSBLXTAL_A> for bool {
    #[inline(always)]
    fn from(variant: FDBKDSBLXTAL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FDBKDSBLXTAL` reader - XTAL Oscillator Disable Feedback."]
pub struct FDBKDSBLXTAL_R(crate::FieldReader<bool, FDBKDSBLXTAL_A>);
impl FDBKDSBLXTAL_R {
    pub(crate) fn new(bits: bool) -> Self {
        FDBKDSBLXTAL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FDBKDSBLXTAL_A {
        match self.bits {
            false => FDBKDSBLXTAL_A::EN,
            true => FDBKDSBLXTAL_A::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        **self == FDBKDSBLXTAL_A::EN
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        **self == FDBKDSBLXTAL_A::DIS
    }
}
impl core::ops::Deref for FDBKDSBLXTAL_R {
    type Target = crate::FieldReader<bool, FDBKDSBLXTAL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FDBKDSBLXTAL` writer - XTAL Oscillator Disable Feedback."]
pub struct FDBKDSBLXTAL_W<'a> {
    w: &'a mut W,
}
impl<'a> FDBKDSBLXTAL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FDBKDSBLXTAL_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Enable XTAL oscillator comparator. value."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(FDBKDSBLXTAL_A::EN)
    }
    #[doc = "Disable XTAL oscillator comparator. value."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(FDBKDSBLXTAL_A::DIS)
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
#[doc = "XTAL Software Override Enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum XTALSWE_A {
    #[doc = "0: XTAL Software Override Disable. value."]
    OVERRIDE_DIS = 0,
    #[doc = "1: XTAL Software Override Enable. value."]
    OVERRIDE_EN = 1,
}
impl From<XTALSWE_A> for bool {
    #[inline(always)]
    fn from(variant: XTALSWE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `XTALSWE` reader - XTAL Software Override Enable."]
pub struct XTALSWE_R(crate::FieldReader<bool, XTALSWE_A>);
impl XTALSWE_R {
    pub(crate) fn new(bits: bool) -> Self {
        XTALSWE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> XTALSWE_A {
        match self.bits {
            false => XTALSWE_A::OVERRIDE_DIS,
            true => XTALSWE_A::OVERRIDE_EN,
        }
    }
    #[doc = "Checks if the value of the field is `OVERRIDE_DIS`"]
    #[inline(always)]
    pub fn is_override_dis(&self) -> bool {
        **self == XTALSWE_A::OVERRIDE_DIS
    }
    #[doc = "Checks if the value of the field is `OVERRIDE_EN`"]
    #[inline(always)]
    pub fn is_override_en(&self) -> bool {
        **self == XTALSWE_A::OVERRIDE_EN
    }
}
impl core::ops::Deref for XTALSWE_R {
    type Target = crate::FieldReader<bool, XTALSWE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `XTALSWE` writer - XTAL Software Override Enable."]
pub struct XTALSWE_W<'a> {
    w: &'a mut W,
}
impl<'a> XTALSWE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: XTALSWE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "XTAL Software Override Disable. value."]
    #[inline(always)]
    pub fn override_dis(self) -> &'a mut W {
        self.variant(XTALSWE_A::OVERRIDE_DIS)
    }
    #[doc = "XTAL Software Override Enable. value."]
    #[inline(always)]
    pub fn override_en(self) -> &'a mut W {
        self.variant(XTALSWE_A::OVERRIDE_EN)
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
    #[doc = "Bits 8:9 - XTAL ICOMP trim"]
    #[inline(always)]
    pub fn xtalicomptrim(&self) -> XTALICOMPTRIM_R {
        XTALICOMPTRIM_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 6:7 - XTAL IBUFF trim"]
    #[inline(always)]
    pub fn xtalibuftrim(&self) -> XTALIBUFTRIM_R {
        XTALIBUFTRIM_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bit 5 - XTAL Power down on brown out."]
    #[inline(always)]
    pub fn pwdbodxtal(&self) -> PWDBODXTAL_R {
        PWDBODXTAL_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - XTAL Oscillator Power Down Comparator."]
    #[inline(always)]
    pub fn pdnbcmprxtal(&self) -> PDNBCMPRXTAL_R {
        PDNBCMPRXTAL_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - XTAL Oscillator Power Down Core."]
    #[inline(always)]
    pub fn pdnbcorextal(&self) -> PDNBCOREXTAL_R {
        PDNBCOREXTAL_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - XTAL Oscillator Bypass Comparator."]
    #[inline(always)]
    pub fn bypcmprxtal(&self) -> BYPCMPRXTAL_R {
        BYPCMPRXTAL_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - XTAL Oscillator Disable Feedback."]
    #[inline(always)]
    pub fn fdbkdsblxtal(&self) -> FDBKDSBLXTAL_R {
        FDBKDSBLXTAL_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - XTAL Software Override Enable."]
    #[inline(always)]
    pub fn xtalswe(&self) -> XTALSWE_R {
        XTALSWE_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 8:9 - XTAL ICOMP trim"]
    #[inline(always)]
    pub fn xtalicomptrim(&mut self) -> XTALICOMPTRIM_W {
        XTALICOMPTRIM_W { w: self }
    }
    #[doc = "Bits 6:7 - XTAL IBUFF trim"]
    #[inline(always)]
    pub fn xtalibuftrim(&mut self) -> XTALIBUFTRIM_W {
        XTALIBUFTRIM_W { w: self }
    }
    #[doc = "Bit 5 - XTAL Power down on brown out."]
    #[inline(always)]
    pub fn pwdbodxtal(&mut self) -> PWDBODXTAL_W {
        PWDBODXTAL_W { w: self }
    }
    #[doc = "Bit 4 - XTAL Oscillator Power Down Comparator."]
    #[inline(always)]
    pub fn pdnbcmprxtal(&mut self) -> PDNBCMPRXTAL_W {
        PDNBCMPRXTAL_W { w: self }
    }
    #[doc = "Bit 3 - XTAL Oscillator Power Down Core."]
    #[inline(always)]
    pub fn pdnbcorextal(&mut self) -> PDNBCOREXTAL_W {
        PDNBCOREXTAL_W { w: self }
    }
    #[doc = "Bit 2 - XTAL Oscillator Bypass Comparator."]
    #[inline(always)]
    pub fn bypcmprxtal(&mut self) -> BYPCMPRXTAL_W {
        BYPCMPRXTAL_W { w: self }
    }
    #[doc = "Bit 1 - XTAL Oscillator Disable Feedback."]
    #[inline(always)]
    pub fn fdbkdsblxtal(&mut self) -> FDBKDSBLXTAL_W {
        FDBKDSBLXTAL_W { w: self }
    }
    #[doc = "Bit 0 - XTAL Software Override Enable."]
    #[inline(always)]
    pub fn xtalswe(&mut self) -> XTALSWE_W {
        XTALSWE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "XTAL Oscillator Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [xtalctrl](index.html) module"]
pub struct XTALCTRL_SPEC;
impl crate::RegisterSpec for XTALCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [xtalctrl::R](R) reader structure"]
impl crate::Readable for XTALCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [xtalctrl::W](W) writer structure"]
impl crate::Writable for XTALCTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets XTALCTRL to value 0x0358"]
impl crate::Resettable for XTALCTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0358
    }
}
