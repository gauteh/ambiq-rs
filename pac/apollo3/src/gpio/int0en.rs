#[doc = "Register `INT0EN` reader"]
pub struct R(crate::R<INT0EN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INT0EN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INT0EN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INT0EN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INT0EN` writer"]
pub struct W(crate::W<INT0EN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INT0EN_SPEC>;
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
impl From<crate::W<INT0EN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INT0EN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `GPIO31` reader - GPIO31 interrupt."]
pub struct GPIO31_R(crate::FieldReader<bool, bool>);
impl GPIO31_R {
    pub(crate) fn new(bits: bool) -> Self {
        GPIO31_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPIO31_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIO31` writer - GPIO31 interrupt."]
pub struct GPIO31_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO31_W<'a> {
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
#[doc = "Field `GPIO30` reader - GPIO30 interrupt."]
pub struct GPIO30_R(crate::FieldReader<bool, bool>);
impl GPIO30_R {
    pub(crate) fn new(bits: bool) -> Self {
        GPIO30_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPIO30_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIO30` writer - GPIO30 interrupt."]
pub struct GPIO30_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO30_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | ((value as u32 & 0x01) << 30);
        self.w
    }
}
#[doc = "Field `GPIO29` reader - GPIO29 interrupt."]
pub struct GPIO29_R(crate::FieldReader<bool, bool>);
impl GPIO29_R {
    pub(crate) fn new(bits: bool) -> Self {
        GPIO29_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPIO29_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIO29` writer - GPIO29 interrupt."]
pub struct GPIO29_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO29_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 29)) | ((value as u32 & 0x01) << 29);
        self.w
    }
}
#[doc = "Field `GPIO28` reader - GPIO28 interrupt."]
pub struct GPIO28_R(crate::FieldReader<bool, bool>);
impl GPIO28_R {
    pub(crate) fn new(bits: bool) -> Self {
        GPIO28_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPIO28_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIO28` writer - GPIO28 interrupt."]
pub struct GPIO28_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO28_W<'a> {
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
#[doc = "Field `GPIO27` reader - GPIO27 interrupt."]
pub struct GPIO27_R(crate::FieldReader<bool, bool>);
impl GPIO27_R {
    pub(crate) fn new(bits: bool) -> Self {
        GPIO27_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPIO27_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIO27` writer - GPIO27 interrupt."]
pub struct GPIO27_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO27_W<'a> {
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
#[doc = "Field `GPIO26` reader - GPIO26 interrupt."]
pub struct GPIO26_R(crate::FieldReader<bool, bool>);
impl GPIO26_R {
    pub(crate) fn new(bits: bool) -> Self {
        GPIO26_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPIO26_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIO26` writer - GPIO26 interrupt."]
pub struct GPIO26_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO26_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 26)) | ((value as u32 & 0x01) << 26);
        self.w
    }
}
#[doc = "Field `GPIO25` reader - GPIO25 interrupt."]
pub struct GPIO25_R(crate::FieldReader<bool, bool>);
impl GPIO25_R {
    pub(crate) fn new(bits: bool) -> Self {
        GPIO25_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPIO25_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIO25` writer - GPIO25 interrupt."]
pub struct GPIO25_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO25_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | ((value as u32 & 0x01) << 25);
        self.w
    }
}
#[doc = "Field `GPIO24` reader - GPIO24 interrupt."]
pub struct GPIO24_R(crate::FieldReader<bool, bool>);
impl GPIO24_R {
    pub(crate) fn new(bits: bool) -> Self {
        GPIO24_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPIO24_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIO24` writer - GPIO24 interrupt."]
pub struct GPIO24_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO24_W<'a> {
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
#[doc = "Field `GPIO23` reader - GPIO23 interrupt."]
pub struct GPIO23_R(crate::FieldReader<bool, bool>);
impl GPIO23_R {
    pub(crate) fn new(bits: bool) -> Self {
        GPIO23_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPIO23_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIO23` writer - GPIO23 interrupt."]
pub struct GPIO23_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO23_W<'a> {
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
#[doc = "Field `GPIO22` reader - GPIO22 interrupt."]
pub struct GPIO22_R(crate::FieldReader<bool, bool>);
impl GPIO22_R {
    pub(crate) fn new(bits: bool) -> Self {
        GPIO22_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPIO22_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIO22` writer - GPIO22 interrupt."]
pub struct GPIO22_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO22_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | ((value as u32 & 0x01) << 22);
        self.w
    }
}
#[doc = "Field `GPIO21` reader - GPIO21 interrupt."]
pub struct GPIO21_R(crate::FieldReader<bool, bool>);
impl GPIO21_R {
    pub(crate) fn new(bits: bool) -> Self {
        GPIO21_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPIO21_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIO21` writer - GPIO21 interrupt."]
pub struct GPIO21_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO21_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | ((value as u32 & 0x01) << 21);
        self.w
    }
}
#[doc = "Field `GPIO20` reader - GPIO20 interrupt."]
pub struct GPIO20_R(crate::FieldReader<bool, bool>);
impl GPIO20_R {
    pub(crate) fn new(bits: bool) -> Self {
        GPIO20_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPIO20_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIO20` writer - GPIO20 interrupt."]
pub struct GPIO20_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO20_W<'a> {
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
#[doc = "Field `GPIO19` reader - GPIO19 interrupt."]
pub struct GPIO19_R(crate::FieldReader<bool, bool>);
impl GPIO19_R {
    pub(crate) fn new(bits: bool) -> Self {
        GPIO19_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPIO19_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIO19` writer - GPIO19 interrupt."]
pub struct GPIO19_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO19_W<'a> {
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
#[doc = "Field `GPIO18` reader - GPIO18interrupt."]
pub struct GPIO18_R(crate::FieldReader<bool, bool>);
impl GPIO18_R {
    pub(crate) fn new(bits: bool) -> Self {
        GPIO18_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPIO18_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIO18` writer - GPIO18interrupt."]
pub struct GPIO18_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO18_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | ((value as u32 & 0x01) << 18);
        self.w
    }
}
#[doc = "Field `GPIO17` reader - GPIO17 interrupt."]
pub struct GPIO17_R(crate::FieldReader<bool, bool>);
impl GPIO17_R {
    pub(crate) fn new(bits: bool) -> Self {
        GPIO17_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPIO17_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIO17` writer - GPIO17 interrupt."]
pub struct GPIO17_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO17_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | ((value as u32 & 0x01) << 17);
        self.w
    }
}
#[doc = "Field `GPIO16` reader - GPIO16 interrupt."]
pub struct GPIO16_R(crate::FieldReader<bool, bool>);
impl GPIO16_R {
    pub(crate) fn new(bits: bool) -> Self {
        GPIO16_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPIO16_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIO16` writer - GPIO16 interrupt."]
pub struct GPIO16_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO16_W<'a> {
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
#[doc = "Field `GPIO15` reader - GPIO15 interrupt."]
pub struct GPIO15_R(crate::FieldReader<bool, bool>);
impl GPIO15_R {
    pub(crate) fn new(bits: bool) -> Self {
        GPIO15_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPIO15_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIO15` writer - GPIO15 interrupt."]
pub struct GPIO15_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO15_W<'a> {
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
#[doc = "Field `GPIO14` reader - GPIO14 interrupt."]
pub struct GPIO14_R(crate::FieldReader<bool, bool>);
impl GPIO14_R {
    pub(crate) fn new(bits: bool) -> Self {
        GPIO14_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPIO14_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIO14` writer - GPIO14 interrupt."]
pub struct GPIO14_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO14_W<'a> {
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
#[doc = "Field `GPIO13` reader - GPIO13 interrupt."]
pub struct GPIO13_R(crate::FieldReader<bool, bool>);
impl GPIO13_R {
    pub(crate) fn new(bits: bool) -> Self {
        GPIO13_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPIO13_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIO13` writer - GPIO13 interrupt."]
pub struct GPIO13_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO13_W<'a> {
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
#[doc = "Field `GPIO12` reader - GPIO12 interrupt."]
pub struct GPIO12_R(crate::FieldReader<bool, bool>);
impl GPIO12_R {
    pub(crate) fn new(bits: bool) -> Self {
        GPIO12_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPIO12_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIO12` writer - GPIO12 interrupt."]
pub struct GPIO12_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO12_W<'a> {
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
#[doc = "Field `GPIO11` reader - GPIO11 interrupt."]
pub struct GPIO11_R(crate::FieldReader<bool, bool>);
impl GPIO11_R {
    pub(crate) fn new(bits: bool) -> Self {
        GPIO11_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPIO11_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIO11` writer - GPIO11 interrupt."]
pub struct GPIO11_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO11_W<'a> {
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
#[doc = "Field `GPIO10` reader - GPIO10 interrupt."]
pub struct GPIO10_R(crate::FieldReader<bool, bool>);
impl GPIO10_R {
    pub(crate) fn new(bits: bool) -> Self {
        GPIO10_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPIO10_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIO10` writer - GPIO10 interrupt."]
pub struct GPIO10_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO10_W<'a> {
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
#[doc = "Field `GPIO9` reader - GPIO9 interrupt."]
pub struct GPIO9_R(crate::FieldReader<bool, bool>);
impl GPIO9_R {
    pub(crate) fn new(bits: bool) -> Self {
        GPIO9_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPIO9_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIO9` writer - GPIO9 interrupt."]
pub struct GPIO9_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO9_W<'a> {
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
#[doc = "Field `GPIO8` reader - GPIO8 interrupt."]
pub struct GPIO8_R(crate::FieldReader<bool, bool>);
impl GPIO8_R {
    pub(crate) fn new(bits: bool) -> Self {
        GPIO8_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPIO8_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIO8` writer - GPIO8 interrupt."]
pub struct GPIO8_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO8_W<'a> {
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
#[doc = "Field `GPIO7` reader - GPIO7 interrupt."]
pub struct GPIO7_R(crate::FieldReader<bool, bool>);
impl GPIO7_R {
    pub(crate) fn new(bits: bool) -> Self {
        GPIO7_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPIO7_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIO7` writer - GPIO7 interrupt."]
pub struct GPIO7_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO7_W<'a> {
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
#[doc = "Field `GPIO6` reader - GPIO6 interrupt."]
pub struct GPIO6_R(crate::FieldReader<bool, bool>);
impl GPIO6_R {
    pub(crate) fn new(bits: bool) -> Self {
        GPIO6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPIO6_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIO6` writer - GPIO6 interrupt."]
pub struct GPIO6_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO6_W<'a> {
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
#[doc = "Field `GPIO5` reader - GPIO5 interrupt."]
pub struct GPIO5_R(crate::FieldReader<bool, bool>);
impl GPIO5_R {
    pub(crate) fn new(bits: bool) -> Self {
        GPIO5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPIO5_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIO5` writer - GPIO5 interrupt."]
pub struct GPIO5_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO5_W<'a> {
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
#[doc = "Field `GPIO4` reader - GPIO4 interrupt."]
pub struct GPIO4_R(crate::FieldReader<bool, bool>);
impl GPIO4_R {
    pub(crate) fn new(bits: bool) -> Self {
        GPIO4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPIO4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIO4` writer - GPIO4 interrupt."]
pub struct GPIO4_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO4_W<'a> {
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
#[doc = "Field `GPIO3` reader - GPIO3 interrupt."]
pub struct GPIO3_R(crate::FieldReader<bool, bool>);
impl GPIO3_R {
    pub(crate) fn new(bits: bool) -> Self {
        GPIO3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPIO3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIO3` writer - GPIO3 interrupt."]
pub struct GPIO3_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO3_W<'a> {
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
#[doc = "Field `GPIO2` reader - GPIO2 interrupt."]
pub struct GPIO2_R(crate::FieldReader<bool, bool>);
impl GPIO2_R {
    pub(crate) fn new(bits: bool) -> Self {
        GPIO2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPIO2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIO2` writer - GPIO2 interrupt."]
pub struct GPIO2_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO2_W<'a> {
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
#[doc = "Field `GPIO1` reader - GPIO1 interrupt."]
pub struct GPIO1_R(crate::FieldReader<bool, bool>);
impl GPIO1_R {
    pub(crate) fn new(bits: bool) -> Self {
        GPIO1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPIO1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIO1` writer - GPIO1 interrupt."]
pub struct GPIO1_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO1_W<'a> {
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
#[doc = "Field `GPIO0` reader - GPIO0 interrupt."]
pub struct GPIO0_R(crate::FieldReader<bool, bool>);
impl GPIO0_R {
    pub(crate) fn new(bits: bool) -> Self {
        GPIO0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPIO0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIO0` writer - GPIO0 interrupt."]
pub struct GPIO0_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO0_W<'a> {
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
    #[doc = "Bit 31 - GPIO31 interrupt."]
    #[inline(always)]
    pub fn gpio31(&self) -> GPIO31_R {
        GPIO31_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bit 30 - GPIO30 interrupt."]
    #[inline(always)]
    pub fn gpio30(&self) -> GPIO30_R {
        GPIO30_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 29 - GPIO29 interrupt."]
    #[inline(always)]
    pub fn gpio29(&self) -> GPIO29_R {
        GPIO29_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 28 - GPIO28 interrupt."]
    #[inline(always)]
    pub fn gpio28(&self) -> GPIO28_R {
        GPIO28_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 27 - GPIO27 interrupt."]
    #[inline(always)]
    pub fn gpio27(&self) -> GPIO27_R {
        GPIO27_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 26 - GPIO26 interrupt."]
    #[inline(always)]
    pub fn gpio26(&self) -> GPIO26_R {
        GPIO26_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 25 - GPIO25 interrupt."]
    #[inline(always)]
    pub fn gpio25(&self) -> GPIO25_R {
        GPIO25_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 24 - GPIO24 interrupt."]
    #[inline(always)]
    pub fn gpio24(&self) -> GPIO24_R {
        GPIO24_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 23 - GPIO23 interrupt."]
    #[inline(always)]
    pub fn gpio23(&self) -> GPIO23_R {
        GPIO23_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 22 - GPIO22 interrupt."]
    #[inline(always)]
    pub fn gpio22(&self) -> GPIO22_R {
        GPIO22_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 21 - GPIO21 interrupt."]
    #[inline(always)]
    pub fn gpio21(&self) -> GPIO21_R {
        GPIO21_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 20 - GPIO20 interrupt."]
    #[inline(always)]
    pub fn gpio20(&self) -> GPIO20_R {
        GPIO20_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 19 - GPIO19 interrupt."]
    #[inline(always)]
    pub fn gpio19(&self) -> GPIO19_R {
        GPIO19_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 18 - GPIO18interrupt."]
    #[inline(always)]
    pub fn gpio18(&self) -> GPIO18_R {
        GPIO18_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 17 - GPIO17 interrupt."]
    #[inline(always)]
    pub fn gpio17(&self) -> GPIO17_R {
        GPIO17_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 16 - GPIO16 interrupt."]
    #[inline(always)]
    pub fn gpio16(&self) -> GPIO16_R {
        GPIO16_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 15 - GPIO15 interrupt."]
    #[inline(always)]
    pub fn gpio15(&self) -> GPIO15_R {
        GPIO15_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 14 - GPIO14 interrupt."]
    #[inline(always)]
    pub fn gpio14(&self) -> GPIO14_R {
        GPIO14_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 13 - GPIO13 interrupt."]
    #[inline(always)]
    pub fn gpio13(&self) -> GPIO13_R {
        GPIO13_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12 - GPIO12 interrupt."]
    #[inline(always)]
    pub fn gpio12(&self) -> GPIO12_R {
        GPIO12_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11 - GPIO11 interrupt."]
    #[inline(always)]
    pub fn gpio11(&self) -> GPIO11_R {
        GPIO11_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10 - GPIO10 interrupt."]
    #[inline(always)]
    pub fn gpio10(&self) -> GPIO10_R {
        GPIO10_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - GPIO9 interrupt."]
    #[inline(always)]
    pub fn gpio9(&self) -> GPIO9_R {
        GPIO9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - GPIO8 interrupt."]
    #[inline(always)]
    pub fn gpio8(&self) -> GPIO8_R {
        GPIO8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7 - GPIO7 interrupt."]
    #[inline(always)]
    pub fn gpio7(&self) -> GPIO7_R {
        GPIO7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - GPIO6 interrupt."]
    #[inline(always)]
    pub fn gpio6(&self) -> GPIO6_R {
        GPIO6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - GPIO5 interrupt."]
    #[inline(always)]
    pub fn gpio5(&self) -> GPIO5_R {
        GPIO5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - GPIO4 interrupt."]
    #[inline(always)]
    pub fn gpio4(&self) -> GPIO4_R {
        GPIO4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - GPIO3 interrupt."]
    #[inline(always)]
    pub fn gpio3(&self) -> GPIO3_R {
        GPIO3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - GPIO2 interrupt."]
    #[inline(always)]
    pub fn gpio2(&self) -> GPIO2_R {
        GPIO2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - GPIO1 interrupt."]
    #[inline(always)]
    pub fn gpio1(&self) -> GPIO1_R {
        GPIO1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - GPIO0 interrupt."]
    #[inline(always)]
    pub fn gpio0(&self) -> GPIO0_R {
        GPIO0_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 31 - GPIO31 interrupt."]
    #[inline(always)]
    pub fn gpio31(&mut self) -> GPIO31_W {
        GPIO31_W { w: self }
    }
    #[doc = "Bit 30 - GPIO30 interrupt."]
    #[inline(always)]
    pub fn gpio30(&mut self) -> GPIO30_W {
        GPIO30_W { w: self }
    }
    #[doc = "Bit 29 - GPIO29 interrupt."]
    #[inline(always)]
    pub fn gpio29(&mut self) -> GPIO29_W {
        GPIO29_W { w: self }
    }
    #[doc = "Bit 28 - GPIO28 interrupt."]
    #[inline(always)]
    pub fn gpio28(&mut self) -> GPIO28_W {
        GPIO28_W { w: self }
    }
    #[doc = "Bit 27 - GPIO27 interrupt."]
    #[inline(always)]
    pub fn gpio27(&mut self) -> GPIO27_W {
        GPIO27_W { w: self }
    }
    #[doc = "Bit 26 - GPIO26 interrupt."]
    #[inline(always)]
    pub fn gpio26(&mut self) -> GPIO26_W {
        GPIO26_W { w: self }
    }
    #[doc = "Bit 25 - GPIO25 interrupt."]
    #[inline(always)]
    pub fn gpio25(&mut self) -> GPIO25_W {
        GPIO25_W { w: self }
    }
    #[doc = "Bit 24 - GPIO24 interrupt."]
    #[inline(always)]
    pub fn gpio24(&mut self) -> GPIO24_W {
        GPIO24_W { w: self }
    }
    #[doc = "Bit 23 - GPIO23 interrupt."]
    #[inline(always)]
    pub fn gpio23(&mut self) -> GPIO23_W {
        GPIO23_W { w: self }
    }
    #[doc = "Bit 22 - GPIO22 interrupt."]
    #[inline(always)]
    pub fn gpio22(&mut self) -> GPIO22_W {
        GPIO22_W { w: self }
    }
    #[doc = "Bit 21 - GPIO21 interrupt."]
    #[inline(always)]
    pub fn gpio21(&mut self) -> GPIO21_W {
        GPIO21_W { w: self }
    }
    #[doc = "Bit 20 - GPIO20 interrupt."]
    #[inline(always)]
    pub fn gpio20(&mut self) -> GPIO20_W {
        GPIO20_W { w: self }
    }
    #[doc = "Bit 19 - GPIO19 interrupt."]
    #[inline(always)]
    pub fn gpio19(&mut self) -> GPIO19_W {
        GPIO19_W { w: self }
    }
    #[doc = "Bit 18 - GPIO18interrupt."]
    #[inline(always)]
    pub fn gpio18(&mut self) -> GPIO18_W {
        GPIO18_W { w: self }
    }
    #[doc = "Bit 17 - GPIO17 interrupt."]
    #[inline(always)]
    pub fn gpio17(&mut self) -> GPIO17_W {
        GPIO17_W { w: self }
    }
    #[doc = "Bit 16 - GPIO16 interrupt."]
    #[inline(always)]
    pub fn gpio16(&mut self) -> GPIO16_W {
        GPIO16_W { w: self }
    }
    #[doc = "Bit 15 - GPIO15 interrupt."]
    #[inline(always)]
    pub fn gpio15(&mut self) -> GPIO15_W {
        GPIO15_W { w: self }
    }
    #[doc = "Bit 14 - GPIO14 interrupt."]
    #[inline(always)]
    pub fn gpio14(&mut self) -> GPIO14_W {
        GPIO14_W { w: self }
    }
    #[doc = "Bit 13 - GPIO13 interrupt."]
    #[inline(always)]
    pub fn gpio13(&mut self) -> GPIO13_W {
        GPIO13_W { w: self }
    }
    #[doc = "Bit 12 - GPIO12 interrupt."]
    #[inline(always)]
    pub fn gpio12(&mut self) -> GPIO12_W {
        GPIO12_W { w: self }
    }
    #[doc = "Bit 11 - GPIO11 interrupt."]
    #[inline(always)]
    pub fn gpio11(&mut self) -> GPIO11_W {
        GPIO11_W { w: self }
    }
    #[doc = "Bit 10 - GPIO10 interrupt."]
    #[inline(always)]
    pub fn gpio10(&mut self) -> GPIO10_W {
        GPIO10_W { w: self }
    }
    #[doc = "Bit 9 - GPIO9 interrupt."]
    #[inline(always)]
    pub fn gpio9(&mut self) -> GPIO9_W {
        GPIO9_W { w: self }
    }
    #[doc = "Bit 8 - GPIO8 interrupt."]
    #[inline(always)]
    pub fn gpio8(&mut self) -> GPIO8_W {
        GPIO8_W { w: self }
    }
    #[doc = "Bit 7 - GPIO7 interrupt."]
    #[inline(always)]
    pub fn gpio7(&mut self) -> GPIO7_W {
        GPIO7_W { w: self }
    }
    #[doc = "Bit 6 - GPIO6 interrupt."]
    #[inline(always)]
    pub fn gpio6(&mut self) -> GPIO6_W {
        GPIO6_W { w: self }
    }
    #[doc = "Bit 5 - GPIO5 interrupt."]
    #[inline(always)]
    pub fn gpio5(&mut self) -> GPIO5_W {
        GPIO5_W { w: self }
    }
    #[doc = "Bit 4 - GPIO4 interrupt."]
    #[inline(always)]
    pub fn gpio4(&mut self) -> GPIO4_W {
        GPIO4_W { w: self }
    }
    #[doc = "Bit 3 - GPIO3 interrupt."]
    #[inline(always)]
    pub fn gpio3(&mut self) -> GPIO3_W {
        GPIO3_W { w: self }
    }
    #[doc = "Bit 2 - GPIO2 interrupt."]
    #[inline(always)]
    pub fn gpio2(&mut self) -> GPIO2_W {
        GPIO2_W { w: self }
    }
    #[doc = "Bit 1 - GPIO1 interrupt."]
    #[inline(always)]
    pub fn gpio1(&mut self) -> GPIO1_W {
        GPIO1_W { w: self }
    }
    #[doc = "Bit 0 - GPIO0 interrupt."]
    #[inline(always)]
    pub fn gpio0(&mut self) -> GPIO0_W {
        GPIO0_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPIO Interrupt Registers 31-0: Enable\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [int0en](index.html) module"]
pub struct INT0EN_SPEC;
impl crate::RegisterSpec for INT0EN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [int0en::R](R) reader structure"]
impl crate::Readable for INT0EN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [int0en::W](W) writer structure"]
impl crate::Writable for INT0EN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets INT0EN to value 0"]
impl crate::Resettable for INT0EN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
