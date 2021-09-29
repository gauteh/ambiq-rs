#[doc = "Register `STMINTSTAT` reader"]
pub struct R(crate::R<STMINTSTAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STMINTSTAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STMINTSTAT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STMINTSTAT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `STMINTSTAT` writer"]
pub struct W(crate::W<STMINTSTAT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<STMINTSTAT_SPEC>;
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
impl From<crate::W<STMINTSTAT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<STMINTSTAT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "CAPTURE register D has grabbed the value in the counter\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CAPTURED_A {
    #[doc = "1: Capture D interrupt status bit was set. value."]
    CAPD_INT = 1,
}
impl From<CAPTURED_A> for bool {
    #[inline(always)]
    fn from(variant: CAPTURED_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CAPTURED` reader - CAPTURE register D has grabbed the value in the counter"]
pub struct CAPTURED_R(crate::FieldReader<bool, CAPTURED_A>);
impl CAPTURED_R {
    pub(crate) fn new(bits: bool) -> Self {
        CAPTURED_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CAPTURED_A> {
        match self.bits {
            true => Some(CAPTURED_A::CAPD_INT),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `CAPD_INT`"]
    #[inline(always)]
    pub fn is_capd_int(&self) -> bool {
        **self == CAPTURED_A::CAPD_INT
    }
}
impl core::ops::Deref for CAPTURED_R {
    type Target = crate::FieldReader<bool, CAPTURED_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CAPTURED` writer - CAPTURE register D has grabbed the value in the counter"]
pub struct CAPTURED_W<'a> {
    w: &'a mut W,
}
impl<'a> CAPTURED_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CAPTURED_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Capture D interrupt status bit was set. value."]
    #[inline(always)]
    pub fn capd_int(self) -> &'a mut W {
        self.variant(CAPTURED_A::CAPD_INT)
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
#[doc = "CAPTURE register C has grabbed the value in the counter\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CAPTUREC_A {
    #[doc = "1: CAPTURE C interrupt status bit was set. value."]
    CAPC_INT = 1,
}
impl From<CAPTUREC_A> for bool {
    #[inline(always)]
    fn from(variant: CAPTUREC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CAPTUREC` reader - CAPTURE register C has grabbed the value in the counter"]
pub struct CAPTUREC_R(crate::FieldReader<bool, CAPTUREC_A>);
impl CAPTUREC_R {
    pub(crate) fn new(bits: bool) -> Self {
        CAPTUREC_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CAPTUREC_A> {
        match self.bits {
            true => Some(CAPTUREC_A::CAPC_INT),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `CAPC_INT`"]
    #[inline(always)]
    pub fn is_capc_int(&self) -> bool {
        **self == CAPTUREC_A::CAPC_INT
    }
}
impl core::ops::Deref for CAPTUREC_R {
    type Target = crate::FieldReader<bool, CAPTUREC_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CAPTUREC` writer - CAPTURE register C has grabbed the value in the counter"]
pub struct CAPTUREC_W<'a> {
    w: &'a mut W,
}
impl<'a> CAPTUREC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CAPTUREC_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "CAPTURE C interrupt status bit was set. value."]
    #[inline(always)]
    pub fn capc_int(self) -> &'a mut W {
        self.variant(CAPTUREC_A::CAPC_INT)
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
#[doc = "CAPTURE register B has grabbed the value in the counter\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CAPTUREB_A {
    #[doc = "1: CAPTURE B interrupt status bit was set. value."]
    CAPB_INT = 1,
}
impl From<CAPTUREB_A> for bool {
    #[inline(always)]
    fn from(variant: CAPTUREB_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CAPTUREB` reader - CAPTURE register B has grabbed the value in the counter"]
pub struct CAPTUREB_R(crate::FieldReader<bool, CAPTUREB_A>);
impl CAPTUREB_R {
    pub(crate) fn new(bits: bool) -> Self {
        CAPTUREB_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CAPTUREB_A> {
        match self.bits {
            true => Some(CAPTUREB_A::CAPB_INT),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `CAPB_INT`"]
    #[inline(always)]
    pub fn is_capb_int(&self) -> bool {
        **self == CAPTUREB_A::CAPB_INT
    }
}
impl core::ops::Deref for CAPTUREB_R {
    type Target = crate::FieldReader<bool, CAPTUREB_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CAPTUREB` writer - CAPTURE register B has grabbed the value in the counter"]
pub struct CAPTUREB_W<'a> {
    w: &'a mut W,
}
impl<'a> CAPTUREB_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CAPTUREB_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "CAPTURE B interrupt status bit was set. value."]
    #[inline(always)]
    pub fn capb_int(self) -> &'a mut W {
        self.variant(CAPTUREB_A::CAPB_INT)
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
#[doc = "CAPTURE register A has grabbed the value in the counter\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CAPTUREA_A {
    #[doc = "1: CAPTURE A interrupt status bit was set. value."]
    CAPA_INT = 1,
}
impl From<CAPTUREA_A> for bool {
    #[inline(always)]
    fn from(variant: CAPTUREA_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CAPTUREA` reader - CAPTURE register A has grabbed the value in the counter"]
pub struct CAPTUREA_R(crate::FieldReader<bool, CAPTUREA_A>);
impl CAPTUREA_R {
    pub(crate) fn new(bits: bool) -> Self {
        CAPTUREA_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CAPTUREA_A> {
        match self.bits {
            true => Some(CAPTUREA_A::CAPA_INT),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `CAPA_INT`"]
    #[inline(always)]
    pub fn is_capa_int(&self) -> bool {
        **self == CAPTUREA_A::CAPA_INT
    }
}
impl core::ops::Deref for CAPTUREA_R {
    type Target = crate::FieldReader<bool, CAPTUREA_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CAPTUREA` writer - CAPTURE register A has grabbed the value in the counter"]
pub struct CAPTUREA_W<'a> {
    w: &'a mut W,
}
impl<'a> CAPTUREA_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CAPTUREA_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "CAPTURE A interrupt status bit was set. value."]
    #[inline(always)]
    pub fn capa_int(self) -> &'a mut W {
        self.variant(CAPTUREA_A::CAPA_INT)
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
#[doc = "COUNTER over flowed from 0xFFFFFFFF back to 0x00000000.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OVERFLOW_A {
    #[doc = "1: Overflow interrupt status bit was set. value."]
    OFLOW_INT = 1,
}
impl From<OVERFLOW_A> for bool {
    #[inline(always)]
    fn from(variant: OVERFLOW_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OVERFLOW` reader - COUNTER over flowed from 0xFFFFFFFF back to 0x00000000."]
pub struct OVERFLOW_R(crate::FieldReader<bool, OVERFLOW_A>);
impl OVERFLOW_R {
    pub(crate) fn new(bits: bool) -> Self {
        OVERFLOW_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<OVERFLOW_A> {
        match self.bits {
            true => Some(OVERFLOW_A::OFLOW_INT),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `OFLOW_INT`"]
    #[inline(always)]
    pub fn is_oflow_int(&self) -> bool {
        **self == OVERFLOW_A::OFLOW_INT
    }
}
impl core::ops::Deref for OVERFLOW_R {
    type Target = crate::FieldReader<bool, OVERFLOW_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OVERFLOW` writer - COUNTER over flowed from 0xFFFFFFFF back to 0x00000000."]
pub struct OVERFLOW_W<'a> {
    w: &'a mut W,
}
impl<'a> OVERFLOW_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OVERFLOW_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Overflow interrupt status bit was set. value."]
    #[inline(always)]
    pub fn oflow_int(self) -> &'a mut W {
        self.variant(OVERFLOW_A::OFLOW_INT)
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
#[doc = "COUNTER is greater than or equal to COMPARE register H.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum COMPAREH_A {
    #[doc = "1: COUNTER greater than or equal to COMPARE register. value."]
    COMPARED = 1,
}
impl From<COMPAREH_A> for bool {
    #[inline(always)]
    fn from(variant: COMPAREH_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `COMPAREH` reader - COUNTER is greater than or equal to COMPARE register H."]
pub struct COMPAREH_R(crate::FieldReader<bool, COMPAREH_A>);
impl COMPAREH_R {
    pub(crate) fn new(bits: bool) -> Self {
        COMPAREH_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<COMPAREH_A> {
        match self.bits {
            true => Some(COMPAREH_A::COMPARED),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `COMPARED`"]
    #[inline(always)]
    pub fn is_compared(&self) -> bool {
        **self == COMPAREH_A::COMPARED
    }
}
impl core::ops::Deref for COMPAREH_R {
    type Target = crate::FieldReader<bool, COMPAREH_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `COMPAREH` writer - COUNTER is greater than or equal to COMPARE register H."]
pub struct COMPAREH_W<'a> {
    w: &'a mut W,
}
impl<'a> COMPAREH_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: COMPAREH_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "COUNTER greater than or equal to COMPARE register. value."]
    #[inline(always)]
    pub fn compared(self) -> &'a mut W {
        self.variant(COMPAREH_A::COMPARED)
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
#[doc = "COUNTER is greater than or equal to COMPARE register G.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum COMPAREG_A {
    #[doc = "1: COUNTER greater than or equal to COMPARE register. value."]
    COMPARED = 1,
}
impl From<COMPAREG_A> for bool {
    #[inline(always)]
    fn from(variant: COMPAREG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `COMPAREG` reader - COUNTER is greater than or equal to COMPARE register G."]
pub struct COMPAREG_R(crate::FieldReader<bool, COMPAREG_A>);
impl COMPAREG_R {
    pub(crate) fn new(bits: bool) -> Self {
        COMPAREG_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<COMPAREG_A> {
        match self.bits {
            true => Some(COMPAREG_A::COMPARED),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `COMPARED`"]
    #[inline(always)]
    pub fn is_compared(&self) -> bool {
        **self == COMPAREG_A::COMPARED
    }
}
impl core::ops::Deref for COMPAREG_R {
    type Target = crate::FieldReader<bool, COMPAREG_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `COMPAREG` writer - COUNTER is greater than or equal to COMPARE register G."]
pub struct COMPAREG_W<'a> {
    w: &'a mut W,
}
impl<'a> COMPAREG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: COMPAREG_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "COUNTER greater than or equal to COMPARE register. value."]
    #[inline(always)]
    pub fn compared(self) -> &'a mut W {
        self.variant(COMPAREG_A::COMPARED)
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
#[doc = "COUNTER is greater than or equal to COMPARE register F.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum COMPAREF_A {
    #[doc = "1: COUNTER greater than or equal to COMPARE register. value."]
    COMPARED = 1,
}
impl From<COMPAREF_A> for bool {
    #[inline(always)]
    fn from(variant: COMPAREF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `COMPAREF` reader - COUNTER is greater than or equal to COMPARE register F."]
pub struct COMPAREF_R(crate::FieldReader<bool, COMPAREF_A>);
impl COMPAREF_R {
    pub(crate) fn new(bits: bool) -> Self {
        COMPAREF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<COMPAREF_A> {
        match self.bits {
            true => Some(COMPAREF_A::COMPARED),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `COMPARED`"]
    #[inline(always)]
    pub fn is_compared(&self) -> bool {
        **self == COMPAREF_A::COMPARED
    }
}
impl core::ops::Deref for COMPAREF_R {
    type Target = crate::FieldReader<bool, COMPAREF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `COMPAREF` writer - COUNTER is greater than or equal to COMPARE register F."]
pub struct COMPAREF_W<'a> {
    w: &'a mut W,
}
impl<'a> COMPAREF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: COMPAREF_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "COUNTER greater than or equal to COMPARE register. value."]
    #[inline(always)]
    pub fn compared(self) -> &'a mut W {
        self.variant(COMPAREF_A::COMPARED)
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
#[doc = "COUNTER is greater than or equal to COMPARE register E.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum COMPAREE_A {
    #[doc = "1: COUNTER greater than or equal to COMPARE register. value."]
    COMPARED = 1,
}
impl From<COMPAREE_A> for bool {
    #[inline(always)]
    fn from(variant: COMPAREE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `COMPAREE` reader - COUNTER is greater than or equal to COMPARE register E."]
pub struct COMPAREE_R(crate::FieldReader<bool, COMPAREE_A>);
impl COMPAREE_R {
    pub(crate) fn new(bits: bool) -> Self {
        COMPAREE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<COMPAREE_A> {
        match self.bits {
            true => Some(COMPAREE_A::COMPARED),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `COMPARED`"]
    #[inline(always)]
    pub fn is_compared(&self) -> bool {
        **self == COMPAREE_A::COMPARED
    }
}
impl core::ops::Deref for COMPAREE_R {
    type Target = crate::FieldReader<bool, COMPAREE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `COMPAREE` writer - COUNTER is greater than or equal to COMPARE register E."]
pub struct COMPAREE_W<'a> {
    w: &'a mut W,
}
impl<'a> COMPAREE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: COMPAREE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "COUNTER greater than or equal to COMPARE register. value."]
    #[inline(always)]
    pub fn compared(self) -> &'a mut W {
        self.variant(COMPAREE_A::COMPARED)
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
#[doc = "COUNTER is greater than or equal to COMPARE register D.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum COMPARED_A {
    #[doc = "1: COUNTER greater than or equal to COMPARE register. value."]
    COMPARED = 1,
}
impl From<COMPARED_A> for bool {
    #[inline(always)]
    fn from(variant: COMPARED_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `COMPARED` reader - COUNTER is greater than or equal to COMPARE register D."]
pub struct COMPARED_R(crate::FieldReader<bool, COMPARED_A>);
impl COMPARED_R {
    pub(crate) fn new(bits: bool) -> Self {
        COMPARED_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<COMPARED_A> {
        match self.bits {
            true => Some(COMPARED_A::COMPARED),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `COMPARED`"]
    #[inline(always)]
    pub fn is_compared(&self) -> bool {
        **self == COMPARED_A::COMPARED
    }
}
impl core::ops::Deref for COMPARED_R {
    type Target = crate::FieldReader<bool, COMPARED_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `COMPARED` writer - COUNTER is greater than or equal to COMPARE register D."]
pub struct COMPARED_W<'a> {
    w: &'a mut W,
}
impl<'a> COMPARED_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: COMPARED_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "COUNTER greater than or equal to COMPARE register. value."]
    #[inline(always)]
    pub fn compared(self) -> &'a mut W {
        self.variant(COMPARED_A::COMPARED)
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
#[doc = "COUNTER is greater than or equal to COMPARE register C.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum COMPAREC_A {
    #[doc = "1: COUNTER greater than or equal to COMPARE register. value."]
    COMPARED = 1,
}
impl From<COMPAREC_A> for bool {
    #[inline(always)]
    fn from(variant: COMPAREC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `COMPAREC` reader - COUNTER is greater than or equal to COMPARE register C."]
pub struct COMPAREC_R(crate::FieldReader<bool, COMPAREC_A>);
impl COMPAREC_R {
    pub(crate) fn new(bits: bool) -> Self {
        COMPAREC_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<COMPAREC_A> {
        match self.bits {
            true => Some(COMPAREC_A::COMPARED),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `COMPARED`"]
    #[inline(always)]
    pub fn is_compared(&self) -> bool {
        **self == COMPAREC_A::COMPARED
    }
}
impl core::ops::Deref for COMPAREC_R {
    type Target = crate::FieldReader<bool, COMPAREC_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `COMPAREC` writer - COUNTER is greater than or equal to COMPARE register C."]
pub struct COMPAREC_W<'a> {
    w: &'a mut W,
}
impl<'a> COMPAREC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: COMPAREC_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "COUNTER greater than or equal to COMPARE register. value."]
    #[inline(always)]
    pub fn compared(self) -> &'a mut W {
        self.variant(COMPAREC_A::COMPARED)
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
#[doc = "COUNTER is greater than or equal to COMPARE register B.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum COMPAREB_A {
    #[doc = "1: COUNTER greater than or equal to COMPARE register. value."]
    COMPARED = 1,
}
impl From<COMPAREB_A> for bool {
    #[inline(always)]
    fn from(variant: COMPAREB_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `COMPAREB` reader - COUNTER is greater than or equal to COMPARE register B."]
pub struct COMPAREB_R(crate::FieldReader<bool, COMPAREB_A>);
impl COMPAREB_R {
    pub(crate) fn new(bits: bool) -> Self {
        COMPAREB_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<COMPAREB_A> {
        match self.bits {
            true => Some(COMPAREB_A::COMPARED),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `COMPARED`"]
    #[inline(always)]
    pub fn is_compared(&self) -> bool {
        **self == COMPAREB_A::COMPARED
    }
}
impl core::ops::Deref for COMPAREB_R {
    type Target = crate::FieldReader<bool, COMPAREB_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `COMPAREB` writer - COUNTER is greater than or equal to COMPARE register B."]
pub struct COMPAREB_W<'a> {
    w: &'a mut W,
}
impl<'a> COMPAREB_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: COMPAREB_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "COUNTER greater than or equal to COMPARE register. value."]
    #[inline(always)]
    pub fn compared(self) -> &'a mut W {
        self.variant(COMPAREB_A::COMPARED)
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
#[doc = "COUNTER is greater than or equal to COMPARE register A.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum COMPAREA_A {
    #[doc = "1: COUNTER greater than or equal to COMPARE register. value."]
    COMPARED = 1,
}
impl From<COMPAREA_A> for bool {
    #[inline(always)]
    fn from(variant: COMPAREA_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `COMPAREA` reader - COUNTER is greater than or equal to COMPARE register A."]
pub struct COMPAREA_R(crate::FieldReader<bool, COMPAREA_A>);
impl COMPAREA_R {
    pub(crate) fn new(bits: bool) -> Self {
        COMPAREA_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<COMPAREA_A> {
        match self.bits {
            true => Some(COMPAREA_A::COMPARED),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `COMPARED`"]
    #[inline(always)]
    pub fn is_compared(&self) -> bool {
        **self == COMPAREA_A::COMPARED
    }
}
impl core::ops::Deref for COMPAREA_R {
    type Target = crate::FieldReader<bool, COMPAREA_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `COMPAREA` writer - COUNTER is greater than or equal to COMPARE register A."]
pub struct COMPAREA_W<'a> {
    w: &'a mut W,
}
impl<'a> COMPAREA_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: COMPAREA_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "COUNTER greater than or equal to COMPARE register. value."]
    #[inline(always)]
    pub fn compared(self) -> &'a mut W {
        self.variant(COMPAREA_A::COMPARED)
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
    #[doc = "Bit 12 - CAPTURE register D has grabbed the value in the counter"]
    #[inline(always)]
    pub fn captured(&self) -> CAPTURED_R {
        CAPTURED_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11 - CAPTURE register C has grabbed the value in the counter"]
    #[inline(always)]
    pub fn capturec(&self) -> CAPTUREC_R {
        CAPTUREC_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10 - CAPTURE register B has grabbed the value in the counter"]
    #[inline(always)]
    pub fn captureb(&self) -> CAPTUREB_R {
        CAPTUREB_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - CAPTURE register A has grabbed the value in the counter"]
    #[inline(always)]
    pub fn capturea(&self) -> CAPTUREA_R {
        CAPTUREA_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - COUNTER over flowed from 0xFFFFFFFF back to 0x00000000."]
    #[inline(always)]
    pub fn overflow(&self) -> OVERFLOW_R {
        OVERFLOW_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7 - COUNTER is greater than or equal to COMPARE register H."]
    #[inline(always)]
    pub fn compareh(&self) -> COMPAREH_R {
        COMPAREH_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - COUNTER is greater than or equal to COMPARE register G."]
    #[inline(always)]
    pub fn compareg(&self) -> COMPAREG_R {
        COMPAREG_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - COUNTER is greater than or equal to COMPARE register F."]
    #[inline(always)]
    pub fn comparef(&self) -> COMPAREF_R {
        COMPAREF_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - COUNTER is greater than or equal to COMPARE register E."]
    #[inline(always)]
    pub fn comparee(&self) -> COMPAREE_R {
        COMPAREE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - COUNTER is greater than or equal to COMPARE register D."]
    #[inline(always)]
    pub fn compared(&self) -> COMPARED_R {
        COMPARED_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - COUNTER is greater than or equal to COMPARE register C."]
    #[inline(always)]
    pub fn comparec(&self) -> COMPAREC_R {
        COMPAREC_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - COUNTER is greater than or equal to COMPARE register B."]
    #[inline(always)]
    pub fn compareb(&self) -> COMPAREB_R {
        COMPAREB_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - COUNTER is greater than or equal to COMPARE register A."]
    #[inline(always)]
    pub fn comparea(&self) -> COMPAREA_R {
        COMPAREA_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 12 - CAPTURE register D has grabbed the value in the counter"]
    #[inline(always)]
    pub fn captured(&mut self) -> CAPTURED_W {
        CAPTURED_W { w: self }
    }
    #[doc = "Bit 11 - CAPTURE register C has grabbed the value in the counter"]
    #[inline(always)]
    pub fn capturec(&mut self) -> CAPTUREC_W {
        CAPTUREC_W { w: self }
    }
    #[doc = "Bit 10 - CAPTURE register B has grabbed the value in the counter"]
    #[inline(always)]
    pub fn captureb(&mut self) -> CAPTUREB_W {
        CAPTUREB_W { w: self }
    }
    #[doc = "Bit 9 - CAPTURE register A has grabbed the value in the counter"]
    #[inline(always)]
    pub fn capturea(&mut self) -> CAPTUREA_W {
        CAPTUREA_W { w: self }
    }
    #[doc = "Bit 8 - COUNTER over flowed from 0xFFFFFFFF back to 0x00000000."]
    #[inline(always)]
    pub fn overflow(&mut self) -> OVERFLOW_W {
        OVERFLOW_W { w: self }
    }
    #[doc = "Bit 7 - COUNTER is greater than or equal to COMPARE register H."]
    #[inline(always)]
    pub fn compareh(&mut self) -> COMPAREH_W {
        COMPAREH_W { w: self }
    }
    #[doc = "Bit 6 - COUNTER is greater than or equal to COMPARE register G."]
    #[inline(always)]
    pub fn compareg(&mut self) -> COMPAREG_W {
        COMPAREG_W { w: self }
    }
    #[doc = "Bit 5 - COUNTER is greater than or equal to COMPARE register F."]
    #[inline(always)]
    pub fn comparef(&mut self) -> COMPAREF_W {
        COMPAREF_W { w: self }
    }
    #[doc = "Bit 4 - COUNTER is greater than or equal to COMPARE register E."]
    #[inline(always)]
    pub fn comparee(&mut self) -> COMPAREE_W {
        COMPAREE_W { w: self }
    }
    #[doc = "Bit 3 - COUNTER is greater than or equal to COMPARE register D."]
    #[inline(always)]
    pub fn compared(&mut self) -> COMPARED_W {
        COMPARED_W { w: self }
    }
    #[doc = "Bit 2 - COUNTER is greater than or equal to COMPARE register C."]
    #[inline(always)]
    pub fn comparec(&mut self) -> COMPAREC_W {
        COMPAREC_W { w: self }
    }
    #[doc = "Bit 1 - COUNTER is greater than or equal to COMPARE register B."]
    #[inline(always)]
    pub fn compareb(&mut self) -> COMPAREB_W {
        COMPAREB_W { w: self }
    }
    #[doc = "Bit 0 - COUNTER is greater than or equal to COMPARE register A."]
    #[inline(always)]
    pub fn comparea(&mut self) -> COMPAREA_W {
        COMPAREA_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "STIMER Interrupt registers: Status\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [stmintstat](index.html) module"]
pub struct STMINTSTAT_SPEC;
impl crate::RegisterSpec for STMINTSTAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [stmintstat::R](R) reader structure"]
impl crate::Readable for STMINTSTAT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [stmintstat::W](W) writer structure"]
impl crate::Writable for STMINTSTAT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets STMINTSTAT to value 0"]
impl crate::Resettable for STMINTSTAT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
