#[doc = "Register `DEVPWREN` reader"]
pub struct R(crate::R<DEVPWREN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DEVPWREN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DEVPWREN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DEVPWREN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DEVPWREN` writer"]
pub struct W(crate::W<DEVPWREN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DEVPWREN_SPEC>;
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
impl From<crate::W<DEVPWREN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DEVPWREN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Power up BLE controller\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWRBLEL_A {
    #[doc = "1: Power up BLE controller value."]
    EN = 1,
    #[doc = "0: Power down BLE controller value."]
    DIS = 0,
}
impl From<PWRBLEL_A> for bool {
    #[inline(always)]
    fn from(variant: PWRBLEL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PWRBLEL` reader - Power up BLE controller"]
pub struct PWRBLEL_R(crate::FieldReader<bool, PWRBLEL_A>);
impl PWRBLEL_R {
    pub(crate) fn new(bits: bool) -> Self {
        PWRBLEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PWRBLEL_A {
        match self.bits {
            true => PWRBLEL_A::EN,
            false => PWRBLEL_A::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        **self == PWRBLEL_A::EN
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        **self == PWRBLEL_A::DIS
    }
}
impl core::ops::Deref for PWRBLEL_R {
    type Target = crate::FieldReader<bool, PWRBLEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PWRBLEL` writer - Power up BLE controller"]
pub struct PWRBLEL_W<'a> {
    w: &'a mut W,
}
impl<'a> PWRBLEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PWRBLEL_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Power up BLE controller value."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(PWRBLEL_A::EN)
    }
    #[doc = "Power down BLE controller value."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(PWRBLEL_A::DIS)
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
#[doc = "Power up PDM block\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWRPDM_A {
    #[doc = "1: Power up PDM value."]
    EN = 1,
    #[doc = "0: Power down PDM value."]
    DIS = 0,
}
impl From<PWRPDM_A> for bool {
    #[inline(always)]
    fn from(variant: PWRPDM_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PWRPDM` reader - Power up PDM block"]
pub struct PWRPDM_R(crate::FieldReader<bool, PWRPDM_A>);
impl PWRPDM_R {
    pub(crate) fn new(bits: bool) -> Self {
        PWRPDM_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PWRPDM_A {
        match self.bits {
            true => PWRPDM_A::EN,
            false => PWRPDM_A::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        **self == PWRPDM_A::EN
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        **self == PWRPDM_A::DIS
    }
}
impl core::ops::Deref for PWRPDM_R {
    type Target = crate::FieldReader<bool, PWRPDM_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PWRPDM` writer - Power up PDM block"]
pub struct PWRPDM_W<'a> {
    w: &'a mut W,
}
impl<'a> PWRPDM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PWRPDM_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Power up PDM value."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(PWRPDM_A::EN)
    }
    #[doc = "Power down PDM value."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(PWRPDM_A::DIS)
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
#[doc = "Power up MSPI Controller\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWRMSPI_A {
    #[doc = "1: Power up MSPI value."]
    EN = 1,
    #[doc = "0: Power down MSPI value."]
    DIS = 0,
}
impl From<PWRMSPI_A> for bool {
    #[inline(always)]
    fn from(variant: PWRMSPI_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PWRMSPI` reader - Power up MSPI Controller"]
pub struct PWRMSPI_R(crate::FieldReader<bool, PWRMSPI_A>);
impl PWRMSPI_R {
    pub(crate) fn new(bits: bool) -> Self {
        PWRMSPI_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PWRMSPI_A {
        match self.bits {
            true => PWRMSPI_A::EN,
            false => PWRMSPI_A::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        **self == PWRMSPI_A::EN
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        **self == PWRMSPI_A::DIS
    }
}
impl core::ops::Deref for PWRMSPI_R {
    type Target = crate::FieldReader<bool, PWRMSPI_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PWRMSPI` writer - Power up MSPI Controller"]
pub struct PWRMSPI_W<'a> {
    w: &'a mut W,
}
impl<'a> PWRMSPI_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PWRMSPI_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Power up MSPI value."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(PWRMSPI_A::EN)
    }
    #[doc = "Power down MSPI value."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(PWRMSPI_A::DIS)
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
#[doc = "Power up SCARD Controller\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWRSCARD_A {
    #[doc = "1: Power up SCARD value."]
    EN = 1,
    #[doc = "0: Power down SCARD value."]
    DIS = 0,
}
impl From<PWRSCARD_A> for bool {
    #[inline(always)]
    fn from(variant: PWRSCARD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PWRSCARD` reader - Power up SCARD Controller"]
pub struct PWRSCARD_R(crate::FieldReader<bool, PWRSCARD_A>);
impl PWRSCARD_R {
    pub(crate) fn new(bits: bool) -> Self {
        PWRSCARD_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PWRSCARD_A {
        match self.bits {
            true => PWRSCARD_A::EN,
            false => PWRSCARD_A::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        **self == PWRSCARD_A::EN
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        **self == PWRSCARD_A::DIS
    }
}
impl core::ops::Deref for PWRSCARD_R {
    type Target = crate::FieldReader<bool, PWRSCARD_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PWRSCARD` writer - Power up SCARD Controller"]
pub struct PWRSCARD_W<'a> {
    w: &'a mut W,
}
impl<'a> PWRSCARD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PWRSCARD_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Power up SCARD value."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(PWRSCARD_A::EN)
    }
    #[doc = "Power down SCARD value."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(PWRSCARD_A::DIS)
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
#[doc = "Power up ADC Digital Controller\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWRADC_A {
    #[doc = "1: Power up ADC value."]
    EN = 1,
    #[doc = "0: Power Down ADC value."]
    DIS = 0,
}
impl From<PWRADC_A> for bool {
    #[inline(always)]
    fn from(variant: PWRADC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PWRADC` reader - Power up ADC Digital Controller"]
pub struct PWRADC_R(crate::FieldReader<bool, PWRADC_A>);
impl PWRADC_R {
    pub(crate) fn new(bits: bool) -> Self {
        PWRADC_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PWRADC_A {
        match self.bits {
            true => PWRADC_A::EN,
            false => PWRADC_A::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        **self == PWRADC_A::EN
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        **self == PWRADC_A::DIS
    }
}
impl core::ops::Deref for PWRADC_R {
    type Target = crate::FieldReader<bool, PWRADC_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PWRADC` writer - Power up ADC Digital Controller"]
pub struct PWRADC_W<'a> {
    w: &'a mut W,
}
impl<'a> PWRADC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PWRADC_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Power up ADC value."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(PWRADC_A::EN)
    }
    #[doc = "Power Down ADC value."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(PWRADC_A::DIS)
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
#[doc = "Power up UART Controller 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWRUART1_A {
    #[doc = "1: Power up UART 1 value."]
    EN = 1,
    #[doc = "0: Power down UART 1 value."]
    DIS = 0,
}
impl From<PWRUART1_A> for bool {
    #[inline(always)]
    fn from(variant: PWRUART1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PWRUART1` reader - Power up UART Controller 1"]
pub struct PWRUART1_R(crate::FieldReader<bool, PWRUART1_A>);
impl PWRUART1_R {
    pub(crate) fn new(bits: bool) -> Self {
        PWRUART1_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PWRUART1_A {
        match self.bits {
            true => PWRUART1_A::EN,
            false => PWRUART1_A::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        **self == PWRUART1_A::EN
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        **self == PWRUART1_A::DIS
    }
}
impl core::ops::Deref for PWRUART1_R {
    type Target = crate::FieldReader<bool, PWRUART1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PWRUART1` writer - Power up UART Controller 1"]
pub struct PWRUART1_W<'a> {
    w: &'a mut W,
}
impl<'a> PWRUART1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PWRUART1_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Power up UART 1 value."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(PWRUART1_A::EN)
    }
    #[doc = "Power down UART 1 value."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(PWRUART1_A::DIS)
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
#[doc = "Power up UART Controller 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWRUART0_A {
    #[doc = "1: Power up UART 0 value."]
    EN = 1,
    #[doc = "0: Power down UART 0 value."]
    DIS = 0,
}
impl From<PWRUART0_A> for bool {
    #[inline(always)]
    fn from(variant: PWRUART0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PWRUART0` reader - Power up UART Controller 0"]
pub struct PWRUART0_R(crate::FieldReader<bool, PWRUART0_A>);
impl PWRUART0_R {
    pub(crate) fn new(bits: bool) -> Self {
        PWRUART0_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PWRUART0_A {
        match self.bits {
            true => PWRUART0_A::EN,
            false => PWRUART0_A::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        **self == PWRUART0_A::EN
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        **self == PWRUART0_A::DIS
    }
}
impl core::ops::Deref for PWRUART0_R {
    type Target = crate::FieldReader<bool, PWRUART0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PWRUART0` writer - Power up UART Controller 0"]
pub struct PWRUART0_W<'a> {
    w: &'a mut W,
}
impl<'a> PWRUART0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PWRUART0_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Power up UART 0 value."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(PWRUART0_A::EN)
    }
    #[doc = "Power down UART 0 value."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(PWRUART0_A::DIS)
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
#[doc = "Power up IO Master 5\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWRIOM5_A {
    #[doc = "1: Power up IO Master 5 value."]
    EN = 1,
    #[doc = "0: Power down IO Master 5 value."]
    DIS = 0,
}
impl From<PWRIOM5_A> for bool {
    #[inline(always)]
    fn from(variant: PWRIOM5_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PWRIOM5` reader - Power up IO Master 5"]
pub struct PWRIOM5_R(crate::FieldReader<bool, PWRIOM5_A>);
impl PWRIOM5_R {
    pub(crate) fn new(bits: bool) -> Self {
        PWRIOM5_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PWRIOM5_A {
        match self.bits {
            true => PWRIOM5_A::EN,
            false => PWRIOM5_A::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        **self == PWRIOM5_A::EN
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        **self == PWRIOM5_A::DIS
    }
}
impl core::ops::Deref for PWRIOM5_R {
    type Target = crate::FieldReader<bool, PWRIOM5_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PWRIOM5` writer - Power up IO Master 5"]
pub struct PWRIOM5_W<'a> {
    w: &'a mut W,
}
impl<'a> PWRIOM5_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PWRIOM5_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Power up IO Master 5 value."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(PWRIOM5_A::EN)
    }
    #[doc = "Power down IO Master 5 value."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(PWRIOM5_A::DIS)
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
#[doc = "Power up IO Master 4\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWRIOM4_A {
    #[doc = "1: Power up IO Master 4 value."]
    EN = 1,
    #[doc = "0: Power down IO Master 4 value."]
    DIS = 0,
}
impl From<PWRIOM4_A> for bool {
    #[inline(always)]
    fn from(variant: PWRIOM4_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PWRIOM4` reader - Power up IO Master 4"]
pub struct PWRIOM4_R(crate::FieldReader<bool, PWRIOM4_A>);
impl PWRIOM4_R {
    pub(crate) fn new(bits: bool) -> Self {
        PWRIOM4_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PWRIOM4_A {
        match self.bits {
            true => PWRIOM4_A::EN,
            false => PWRIOM4_A::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        **self == PWRIOM4_A::EN
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        **self == PWRIOM4_A::DIS
    }
}
impl core::ops::Deref for PWRIOM4_R {
    type Target = crate::FieldReader<bool, PWRIOM4_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PWRIOM4` writer - Power up IO Master 4"]
pub struct PWRIOM4_W<'a> {
    w: &'a mut W,
}
impl<'a> PWRIOM4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PWRIOM4_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Power up IO Master 4 value."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(PWRIOM4_A::EN)
    }
    #[doc = "Power down IO Master 4 value."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(PWRIOM4_A::DIS)
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
#[doc = "Power up IO Master 3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWRIOM3_A {
    #[doc = "1: Power up IO Master 3 value."]
    EN = 1,
    #[doc = "0: Power down IO Master 3 value."]
    DIS = 0,
}
impl From<PWRIOM3_A> for bool {
    #[inline(always)]
    fn from(variant: PWRIOM3_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PWRIOM3` reader - Power up IO Master 3"]
pub struct PWRIOM3_R(crate::FieldReader<bool, PWRIOM3_A>);
impl PWRIOM3_R {
    pub(crate) fn new(bits: bool) -> Self {
        PWRIOM3_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PWRIOM3_A {
        match self.bits {
            true => PWRIOM3_A::EN,
            false => PWRIOM3_A::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        **self == PWRIOM3_A::EN
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        **self == PWRIOM3_A::DIS
    }
}
impl core::ops::Deref for PWRIOM3_R {
    type Target = crate::FieldReader<bool, PWRIOM3_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PWRIOM3` writer - Power up IO Master 3"]
pub struct PWRIOM3_W<'a> {
    w: &'a mut W,
}
impl<'a> PWRIOM3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PWRIOM3_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Power up IO Master 3 value."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(PWRIOM3_A::EN)
    }
    #[doc = "Power down IO Master 3 value."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(PWRIOM3_A::DIS)
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
#[doc = "Power up IO Master 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWRIOM2_A {
    #[doc = "1: Power up IO Master 2 value."]
    EN = 1,
    #[doc = "0: Power down IO Master 2 value."]
    DIS = 0,
}
impl From<PWRIOM2_A> for bool {
    #[inline(always)]
    fn from(variant: PWRIOM2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PWRIOM2` reader - Power up IO Master 2"]
pub struct PWRIOM2_R(crate::FieldReader<bool, PWRIOM2_A>);
impl PWRIOM2_R {
    pub(crate) fn new(bits: bool) -> Self {
        PWRIOM2_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PWRIOM2_A {
        match self.bits {
            true => PWRIOM2_A::EN,
            false => PWRIOM2_A::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        **self == PWRIOM2_A::EN
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        **self == PWRIOM2_A::DIS
    }
}
impl core::ops::Deref for PWRIOM2_R {
    type Target = crate::FieldReader<bool, PWRIOM2_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PWRIOM2` writer - Power up IO Master 2"]
pub struct PWRIOM2_W<'a> {
    w: &'a mut W,
}
impl<'a> PWRIOM2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PWRIOM2_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Power up IO Master 2 value."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(PWRIOM2_A::EN)
    }
    #[doc = "Power down IO Master 2 value."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(PWRIOM2_A::DIS)
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
#[doc = "Power up IO Master 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWRIOM1_A {
    #[doc = "1: Power up IO Master 1 value."]
    EN = 1,
    #[doc = "0: Power down IO Master 1 value."]
    DIS = 0,
}
impl From<PWRIOM1_A> for bool {
    #[inline(always)]
    fn from(variant: PWRIOM1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PWRIOM1` reader - Power up IO Master 1"]
pub struct PWRIOM1_R(crate::FieldReader<bool, PWRIOM1_A>);
impl PWRIOM1_R {
    pub(crate) fn new(bits: bool) -> Self {
        PWRIOM1_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PWRIOM1_A {
        match self.bits {
            true => PWRIOM1_A::EN,
            false => PWRIOM1_A::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        **self == PWRIOM1_A::EN
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        **self == PWRIOM1_A::DIS
    }
}
impl core::ops::Deref for PWRIOM1_R {
    type Target = crate::FieldReader<bool, PWRIOM1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PWRIOM1` writer - Power up IO Master 1"]
pub struct PWRIOM1_W<'a> {
    w: &'a mut W,
}
impl<'a> PWRIOM1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PWRIOM1_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Power up IO Master 1 value."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(PWRIOM1_A::EN)
    }
    #[doc = "Power down IO Master 1 value."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(PWRIOM1_A::DIS)
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
#[doc = "Power up IO Master 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWRIOM0_A {
    #[doc = "1: Power up IO Master 0 value."]
    EN = 1,
    #[doc = "0: Power down IO Master 0 value."]
    DIS = 0,
}
impl From<PWRIOM0_A> for bool {
    #[inline(always)]
    fn from(variant: PWRIOM0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PWRIOM0` reader - Power up IO Master 0"]
pub struct PWRIOM0_R(crate::FieldReader<bool, PWRIOM0_A>);
impl PWRIOM0_R {
    pub(crate) fn new(bits: bool) -> Self {
        PWRIOM0_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PWRIOM0_A {
        match self.bits {
            true => PWRIOM0_A::EN,
            false => PWRIOM0_A::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        **self == PWRIOM0_A::EN
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        **self == PWRIOM0_A::DIS
    }
}
impl core::ops::Deref for PWRIOM0_R {
    type Target = crate::FieldReader<bool, PWRIOM0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PWRIOM0` writer - Power up IO Master 0"]
pub struct PWRIOM0_W<'a> {
    w: &'a mut W,
}
impl<'a> PWRIOM0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PWRIOM0_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Power up IO Master 0 value."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(PWRIOM0_A::EN)
    }
    #[doc = "Power down IO Master 0 value."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(PWRIOM0_A::DIS)
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
#[doc = "Power up IO Slave\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWRIOS_A {
    #[doc = "1: Power up IO slave value."]
    EN = 1,
    #[doc = "0: Power down IO slave value."]
    DIS = 0,
}
impl From<PWRIOS_A> for bool {
    #[inline(always)]
    fn from(variant: PWRIOS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PWRIOS` reader - Power up IO Slave"]
pub struct PWRIOS_R(crate::FieldReader<bool, PWRIOS_A>);
impl PWRIOS_R {
    pub(crate) fn new(bits: bool) -> Self {
        PWRIOS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PWRIOS_A {
        match self.bits {
            true => PWRIOS_A::EN,
            false => PWRIOS_A::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        **self == PWRIOS_A::EN
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        **self == PWRIOS_A::DIS
    }
}
impl core::ops::Deref for PWRIOS_R {
    type Target = crate::FieldReader<bool, PWRIOS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PWRIOS` writer - Power up IO Slave"]
pub struct PWRIOS_W<'a> {
    w: &'a mut W,
}
impl<'a> PWRIOS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PWRIOS_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Power up IO slave value."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(PWRIOS_A::EN)
    }
    #[doc = "Power down IO slave value."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(PWRIOS_A::DIS)
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
    #[doc = "Bit 13 - Power up BLE controller"]
    #[inline(always)]
    pub fn pwrblel(&self) -> PWRBLEL_R {
        PWRBLEL_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Power up PDM block"]
    #[inline(always)]
    pub fn pwrpdm(&self) -> PWRPDM_R {
        PWRPDM_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Power up MSPI Controller"]
    #[inline(always)]
    pub fn pwrmspi(&self) -> PWRMSPI_R {
        PWRMSPI_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Power up SCARD Controller"]
    #[inline(always)]
    pub fn pwrscard(&self) -> PWRSCARD_R {
        PWRSCARD_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Power up ADC Digital Controller"]
    #[inline(always)]
    pub fn pwradc(&self) -> PWRADC_R {
        PWRADC_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Power up UART Controller 1"]
    #[inline(always)]
    pub fn pwruart1(&self) -> PWRUART1_R {
        PWRUART1_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Power up UART Controller 0"]
    #[inline(always)]
    pub fn pwruart0(&self) -> PWRUART0_R {
        PWRUART0_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Power up IO Master 5"]
    #[inline(always)]
    pub fn pwriom5(&self) -> PWRIOM5_R {
        PWRIOM5_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Power up IO Master 4"]
    #[inline(always)]
    pub fn pwriom4(&self) -> PWRIOM4_R {
        PWRIOM4_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Power up IO Master 3"]
    #[inline(always)]
    pub fn pwriom3(&self) -> PWRIOM3_R {
        PWRIOM3_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Power up IO Master 2"]
    #[inline(always)]
    pub fn pwriom2(&self) -> PWRIOM2_R {
        PWRIOM2_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Power up IO Master 1"]
    #[inline(always)]
    pub fn pwriom1(&self) -> PWRIOM1_R {
        PWRIOM1_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - Power up IO Master 0"]
    #[inline(always)]
    pub fn pwriom0(&self) -> PWRIOM0_R {
        PWRIOM0_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Power up IO Slave"]
    #[inline(always)]
    pub fn pwrios(&self) -> PWRIOS_R {
        PWRIOS_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 13 - Power up BLE controller"]
    #[inline(always)]
    pub fn pwrblel(&mut self) -> PWRBLEL_W {
        PWRBLEL_W { w: self }
    }
    #[doc = "Bit 12 - Power up PDM block"]
    #[inline(always)]
    pub fn pwrpdm(&mut self) -> PWRPDM_W {
        PWRPDM_W { w: self }
    }
    #[doc = "Bit 11 - Power up MSPI Controller"]
    #[inline(always)]
    pub fn pwrmspi(&mut self) -> PWRMSPI_W {
        PWRMSPI_W { w: self }
    }
    #[doc = "Bit 10 - Power up SCARD Controller"]
    #[inline(always)]
    pub fn pwrscard(&mut self) -> PWRSCARD_W {
        PWRSCARD_W { w: self }
    }
    #[doc = "Bit 9 - Power up ADC Digital Controller"]
    #[inline(always)]
    pub fn pwradc(&mut self) -> PWRADC_W {
        PWRADC_W { w: self }
    }
    #[doc = "Bit 8 - Power up UART Controller 1"]
    #[inline(always)]
    pub fn pwruart1(&mut self) -> PWRUART1_W {
        PWRUART1_W { w: self }
    }
    #[doc = "Bit 7 - Power up UART Controller 0"]
    #[inline(always)]
    pub fn pwruart0(&mut self) -> PWRUART0_W {
        PWRUART0_W { w: self }
    }
    #[doc = "Bit 6 - Power up IO Master 5"]
    #[inline(always)]
    pub fn pwriom5(&mut self) -> PWRIOM5_W {
        PWRIOM5_W { w: self }
    }
    #[doc = "Bit 5 - Power up IO Master 4"]
    #[inline(always)]
    pub fn pwriom4(&mut self) -> PWRIOM4_W {
        PWRIOM4_W { w: self }
    }
    #[doc = "Bit 4 - Power up IO Master 3"]
    #[inline(always)]
    pub fn pwriom3(&mut self) -> PWRIOM3_W {
        PWRIOM3_W { w: self }
    }
    #[doc = "Bit 3 - Power up IO Master 2"]
    #[inline(always)]
    pub fn pwriom2(&mut self) -> PWRIOM2_W {
        PWRIOM2_W { w: self }
    }
    #[doc = "Bit 2 - Power up IO Master 1"]
    #[inline(always)]
    pub fn pwriom1(&mut self) -> PWRIOM1_W {
        PWRIOM1_W { w: self }
    }
    #[doc = "Bit 1 - Power up IO Master 0"]
    #[inline(always)]
    pub fn pwriom0(&mut self) -> PWRIOM0_W {
        PWRIOM0_W { w: self }
    }
    #[doc = "Bit 0 - Power up IO Slave"]
    #[inline(always)]
    pub fn pwrios(&mut self) -> PWRIOS_W {
        PWRIOS_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Device Power Enables\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [devpwren](index.html) module"]
pub struct DEVPWREN_SPEC;
impl crate::RegisterSpec for DEVPWREN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [devpwren::R](R) reader structure"]
impl crate::Readable for DEVPWREN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [devpwren::W](W) writer structure"]
impl crate::Writable for DEVPWREN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DEVPWREN to value 0"]
impl crate::Resettable for DEVPWREN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
