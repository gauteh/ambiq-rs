#[doc = "Register `INT1SET` reader"]
pub struct R(crate::R<INT1SET_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INT1SET_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INT1SET_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INT1SET_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INT1SET` writer"]
pub struct W(crate::W<INT1SET_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INT1SET_SPEC>;
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
impl From<crate::W<INT1SET_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INT1SET_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `GPIO49` reader - GPIO49 interrupt."]
pub struct GPIO49_R(crate::FieldReader<bool, bool>);
impl GPIO49_R {
    pub(crate) fn new(bits: bool) -> Self {
        GPIO49_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPIO49_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIO49` writer - GPIO49 interrupt."]
pub struct GPIO49_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO49_W<'a> {
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
#[doc = "Field `GPIO48` reader - GPIO48 interrupt."]
pub struct GPIO48_R(crate::FieldReader<bool, bool>);
impl GPIO48_R {
    pub(crate) fn new(bits: bool) -> Self {
        GPIO48_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPIO48_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIO48` writer - GPIO48 interrupt."]
pub struct GPIO48_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO48_W<'a> {
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
#[doc = "Field `GPIO47` reader - GPIO47 interrupt."]
pub struct GPIO47_R(crate::FieldReader<bool, bool>);
impl GPIO47_R {
    pub(crate) fn new(bits: bool) -> Self {
        GPIO47_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPIO47_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIO47` writer - GPIO47 interrupt."]
pub struct GPIO47_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO47_W<'a> {
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
#[doc = "Field `GPIO46` reader - GPIO46 interrupt."]
pub struct GPIO46_R(crate::FieldReader<bool, bool>);
impl GPIO46_R {
    pub(crate) fn new(bits: bool) -> Self {
        GPIO46_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPIO46_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIO46` writer - GPIO46 interrupt."]
pub struct GPIO46_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO46_W<'a> {
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
#[doc = "Field `GPIO45` reader - GPIO45 interrupt."]
pub struct GPIO45_R(crate::FieldReader<bool, bool>);
impl GPIO45_R {
    pub(crate) fn new(bits: bool) -> Self {
        GPIO45_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPIO45_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIO45` writer - GPIO45 interrupt."]
pub struct GPIO45_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO45_W<'a> {
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
#[doc = "Field `GPIO44` reader - GPIO44 interrupt."]
pub struct GPIO44_R(crate::FieldReader<bool, bool>);
impl GPIO44_R {
    pub(crate) fn new(bits: bool) -> Self {
        GPIO44_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPIO44_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIO44` writer - GPIO44 interrupt."]
pub struct GPIO44_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO44_W<'a> {
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
#[doc = "Field `GPIO43` reader - GPIO43 interrupt."]
pub struct GPIO43_R(crate::FieldReader<bool, bool>);
impl GPIO43_R {
    pub(crate) fn new(bits: bool) -> Self {
        GPIO43_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPIO43_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIO43` writer - GPIO43 interrupt."]
pub struct GPIO43_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO43_W<'a> {
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
#[doc = "Field `GPIO42` reader - GPIO42 interrupt."]
pub struct GPIO42_R(crate::FieldReader<bool, bool>);
impl GPIO42_R {
    pub(crate) fn new(bits: bool) -> Self {
        GPIO42_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPIO42_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIO42` writer - GPIO42 interrupt."]
pub struct GPIO42_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO42_W<'a> {
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
#[doc = "Field `GPIO41` reader - GPIO41 interrupt."]
pub struct GPIO41_R(crate::FieldReader<bool, bool>);
impl GPIO41_R {
    pub(crate) fn new(bits: bool) -> Self {
        GPIO41_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPIO41_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIO41` writer - GPIO41 interrupt."]
pub struct GPIO41_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO41_W<'a> {
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
#[doc = "Field `GPIO40` reader - GPIO40 interrupt."]
pub struct GPIO40_R(crate::FieldReader<bool, bool>);
impl GPIO40_R {
    pub(crate) fn new(bits: bool) -> Self {
        GPIO40_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPIO40_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIO40` writer - GPIO40 interrupt."]
pub struct GPIO40_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO40_W<'a> {
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
#[doc = "Field `GPIO39` reader - GPIO39 interrupt."]
pub struct GPIO39_R(crate::FieldReader<bool, bool>);
impl GPIO39_R {
    pub(crate) fn new(bits: bool) -> Self {
        GPIO39_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPIO39_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIO39` writer - GPIO39 interrupt."]
pub struct GPIO39_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO39_W<'a> {
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
#[doc = "Field `GPIO38` reader - GPIO38 interrupt."]
pub struct GPIO38_R(crate::FieldReader<bool, bool>);
impl GPIO38_R {
    pub(crate) fn new(bits: bool) -> Self {
        GPIO38_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPIO38_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIO38` writer - GPIO38 interrupt."]
pub struct GPIO38_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO38_W<'a> {
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
#[doc = "Field `GPIO37` reader - GPIO37 interrupt."]
pub struct GPIO37_R(crate::FieldReader<bool, bool>);
impl GPIO37_R {
    pub(crate) fn new(bits: bool) -> Self {
        GPIO37_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPIO37_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIO37` writer - GPIO37 interrupt."]
pub struct GPIO37_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO37_W<'a> {
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
#[doc = "Field `GPIO36` reader - GPIO36 interrupt."]
pub struct GPIO36_R(crate::FieldReader<bool, bool>);
impl GPIO36_R {
    pub(crate) fn new(bits: bool) -> Self {
        GPIO36_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPIO36_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIO36` writer - GPIO36 interrupt."]
pub struct GPIO36_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO36_W<'a> {
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
#[doc = "Field `GPIO35` reader - GPIO35 interrupt."]
pub struct GPIO35_R(crate::FieldReader<bool, bool>);
impl GPIO35_R {
    pub(crate) fn new(bits: bool) -> Self {
        GPIO35_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPIO35_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIO35` writer - GPIO35 interrupt."]
pub struct GPIO35_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO35_W<'a> {
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
#[doc = "Field `GPIO34` reader - GPIO34 interrupt."]
pub struct GPIO34_R(crate::FieldReader<bool, bool>);
impl GPIO34_R {
    pub(crate) fn new(bits: bool) -> Self {
        GPIO34_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPIO34_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIO34` writer - GPIO34 interrupt."]
pub struct GPIO34_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO34_W<'a> {
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
#[doc = "Field `GPIO33` reader - GPIO33 interrupt."]
pub struct GPIO33_R(crate::FieldReader<bool, bool>);
impl GPIO33_R {
    pub(crate) fn new(bits: bool) -> Self {
        GPIO33_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPIO33_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIO33` writer - GPIO33 interrupt."]
pub struct GPIO33_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO33_W<'a> {
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
#[doc = "Field `GPIO32` reader - GPIO32 interrupt."]
pub struct GPIO32_R(crate::FieldReader<bool, bool>);
impl GPIO32_R {
    pub(crate) fn new(bits: bool) -> Self {
        GPIO32_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPIO32_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIO32` writer - GPIO32 interrupt."]
pub struct GPIO32_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO32_W<'a> {
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
    #[doc = "Bit 17 - GPIO49 interrupt."]
    #[inline(always)]
    pub fn gpio49(&self) -> GPIO49_R {
        GPIO49_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 16 - GPIO48 interrupt."]
    #[inline(always)]
    pub fn gpio48(&self) -> GPIO48_R {
        GPIO48_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 15 - GPIO47 interrupt."]
    #[inline(always)]
    pub fn gpio47(&self) -> GPIO47_R {
        GPIO47_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 14 - GPIO46 interrupt."]
    #[inline(always)]
    pub fn gpio46(&self) -> GPIO46_R {
        GPIO46_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 13 - GPIO45 interrupt."]
    #[inline(always)]
    pub fn gpio45(&self) -> GPIO45_R {
        GPIO45_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12 - GPIO44 interrupt."]
    #[inline(always)]
    pub fn gpio44(&self) -> GPIO44_R {
        GPIO44_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11 - GPIO43 interrupt."]
    #[inline(always)]
    pub fn gpio43(&self) -> GPIO43_R {
        GPIO43_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10 - GPIO42 interrupt."]
    #[inline(always)]
    pub fn gpio42(&self) -> GPIO42_R {
        GPIO42_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - GPIO41 interrupt."]
    #[inline(always)]
    pub fn gpio41(&self) -> GPIO41_R {
        GPIO41_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - GPIO40 interrupt."]
    #[inline(always)]
    pub fn gpio40(&self) -> GPIO40_R {
        GPIO40_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7 - GPIO39 interrupt."]
    #[inline(always)]
    pub fn gpio39(&self) -> GPIO39_R {
        GPIO39_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - GPIO38 interrupt."]
    #[inline(always)]
    pub fn gpio38(&self) -> GPIO38_R {
        GPIO38_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - GPIO37 interrupt."]
    #[inline(always)]
    pub fn gpio37(&self) -> GPIO37_R {
        GPIO37_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - GPIO36 interrupt."]
    #[inline(always)]
    pub fn gpio36(&self) -> GPIO36_R {
        GPIO36_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - GPIO35 interrupt."]
    #[inline(always)]
    pub fn gpio35(&self) -> GPIO35_R {
        GPIO35_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - GPIO34 interrupt."]
    #[inline(always)]
    pub fn gpio34(&self) -> GPIO34_R {
        GPIO34_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - GPIO33 interrupt."]
    #[inline(always)]
    pub fn gpio33(&self) -> GPIO33_R {
        GPIO33_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - GPIO32 interrupt."]
    #[inline(always)]
    pub fn gpio32(&self) -> GPIO32_R {
        GPIO32_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 17 - GPIO49 interrupt."]
    #[inline(always)]
    pub fn gpio49(&mut self) -> GPIO49_W {
        GPIO49_W { w: self }
    }
    #[doc = "Bit 16 - GPIO48 interrupt."]
    #[inline(always)]
    pub fn gpio48(&mut self) -> GPIO48_W {
        GPIO48_W { w: self }
    }
    #[doc = "Bit 15 - GPIO47 interrupt."]
    #[inline(always)]
    pub fn gpio47(&mut self) -> GPIO47_W {
        GPIO47_W { w: self }
    }
    #[doc = "Bit 14 - GPIO46 interrupt."]
    #[inline(always)]
    pub fn gpio46(&mut self) -> GPIO46_W {
        GPIO46_W { w: self }
    }
    #[doc = "Bit 13 - GPIO45 interrupt."]
    #[inline(always)]
    pub fn gpio45(&mut self) -> GPIO45_W {
        GPIO45_W { w: self }
    }
    #[doc = "Bit 12 - GPIO44 interrupt."]
    #[inline(always)]
    pub fn gpio44(&mut self) -> GPIO44_W {
        GPIO44_W { w: self }
    }
    #[doc = "Bit 11 - GPIO43 interrupt."]
    #[inline(always)]
    pub fn gpio43(&mut self) -> GPIO43_W {
        GPIO43_W { w: self }
    }
    #[doc = "Bit 10 - GPIO42 interrupt."]
    #[inline(always)]
    pub fn gpio42(&mut self) -> GPIO42_W {
        GPIO42_W { w: self }
    }
    #[doc = "Bit 9 - GPIO41 interrupt."]
    #[inline(always)]
    pub fn gpio41(&mut self) -> GPIO41_W {
        GPIO41_W { w: self }
    }
    #[doc = "Bit 8 - GPIO40 interrupt."]
    #[inline(always)]
    pub fn gpio40(&mut self) -> GPIO40_W {
        GPIO40_W { w: self }
    }
    #[doc = "Bit 7 - GPIO39 interrupt."]
    #[inline(always)]
    pub fn gpio39(&mut self) -> GPIO39_W {
        GPIO39_W { w: self }
    }
    #[doc = "Bit 6 - GPIO38 interrupt."]
    #[inline(always)]
    pub fn gpio38(&mut self) -> GPIO38_W {
        GPIO38_W { w: self }
    }
    #[doc = "Bit 5 - GPIO37 interrupt."]
    #[inline(always)]
    pub fn gpio37(&mut self) -> GPIO37_W {
        GPIO37_W { w: self }
    }
    #[doc = "Bit 4 - GPIO36 interrupt."]
    #[inline(always)]
    pub fn gpio36(&mut self) -> GPIO36_W {
        GPIO36_W { w: self }
    }
    #[doc = "Bit 3 - GPIO35 interrupt."]
    #[inline(always)]
    pub fn gpio35(&mut self) -> GPIO35_W {
        GPIO35_W { w: self }
    }
    #[doc = "Bit 2 - GPIO34 interrupt."]
    #[inline(always)]
    pub fn gpio34(&mut self) -> GPIO34_W {
        GPIO34_W { w: self }
    }
    #[doc = "Bit 1 - GPIO33 interrupt."]
    #[inline(always)]
    pub fn gpio33(&mut self) -> GPIO33_W {
        GPIO33_W { w: self }
    }
    #[doc = "Bit 0 - GPIO32 interrupt."]
    #[inline(always)]
    pub fn gpio32(&mut self) -> GPIO32_W {
        GPIO32_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPIO Interrupt Registers 49-32: Set\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [int1set](index.html) module"]
pub struct INT1SET_SPEC;
impl crate::RegisterSpec for INT1SET_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [int1set::R](R) reader structure"]
impl crate::Readable for INT1SET_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [int1set::W](W) writer structure"]
impl crate::Writable for INT1SET_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets INT1SET to value 0"]
impl crate::Resettable for INT1SET_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
