#[doc = "Register `MEMPWRSTATUS` reader"]
pub struct R(crate::R<MEMPWRSTATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MEMPWRSTATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MEMPWRSTATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MEMPWRSTATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MEMPWRSTATUS` writer"]
pub struct W(crate::W<MEMPWRSTATUS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MEMPWRSTATUS_SPEC>;
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
impl From<crate::W<MEMPWRSTATUS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MEMPWRSTATUS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CACHEB2` reader - This bit is 1 if power is supplied to Cache Bank 2"]
pub struct CACHEB2_R(crate::FieldReader<bool, bool>);
impl CACHEB2_R {
    pub(crate) fn new(bits: bool) -> Self {
        CACHEB2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CACHEB2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CACHEB2` writer - This bit is 1 if power is supplied to Cache Bank 2"]
pub struct CACHEB2_W<'a> {
    w: &'a mut W,
}
impl<'a> CACHEB2_W<'a> {
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
#[doc = "Field `CACHEB0` reader - This bit is 1 if power is supplied to Cache Bank 0"]
pub struct CACHEB0_R(crate::FieldReader<bool, bool>);
impl CACHEB0_R {
    pub(crate) fn new(bits: bool) -> Self {
        CACHEB0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CACHEB0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CACHEB0` writer - This bit is 1 if power is supplied to Cache Bank 0"]
pub struct CACHEB0_W<'a> {
    w: &'a mut W,
}
impl<'a> CACHEB0_W<'a> {
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
#[doc = "Field `FLASH1` reader - This bit is 1 if power is supplied to FLASH 1"]
pub struct FLASH1_R(crate::FieldReader<bool, bool>);
impl FLASH1_R {
    pub(crate) fn new(bits: bool) -> Self {
        FLASH1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FLASH1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FLASH1` writer - This bit is 1 if power is supplied to FLASH 1"]
pub struct FLASH1_W<'a> {
    w: &'a mut W,
}
impl<'a> FLASH1_W<'a> {
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
#[doc = "Field `FLASH0` reader - This bit is 1 if power is supplied to FLASH 0"]
pub struct FLASH0_R(crate::FieldReader<bool, bool>);
impl FLASH0_R {
    pub(crate) fn new(bits: bool) -> Self {
        FLASH0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FLASH0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FLASH0` writer - This bit is 1 if power is supplied to FLASH 0"]
pub struct FLASH0_W<'a> {
    w: &'a mut W,
}
impl<'a> FLASH0_W<'a> {
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
#[doc = "Field `SRAM9` reader - This bit is 1 if power is supplied to SRAM GROUP9"]
pub struct SRAM9_R(crate::FieldReader<bool, bool>);
impl SRAM9_R {
    pub(crate) fn new(bits: bool) -> Self {
        SRAM9_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SRAM9_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SRAM9` writer - This bit is 1 if power is supplied to SRAM GROUP9"]
pub struct SRAM9_W<'a> {
    w: &'a mut W,
}
impl<'a> SRAM9_W<'a> {
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
#[doc = "Field `SRAM8` reader - This bit is 1 if power is supplied to SRAM GROUP8"]
pub struct SRAM8_R(crate::FieldReader<bool, bool>);
impl SRAM8_R {
    pub(crate) fn new(bits: bool) -> Self {
        SRAM8_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SRAM8_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SRAM8` writer - This bit is 1 if power is supplied to SRAM GROUP8"]
pub struct SRAM8_W<'a> {
    w: &'a mut W,
}
impl<'a> SRAM8_W<'a> {
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
#[doc = "Field `SRAM7` reader - This bit is 1 if power is supplied to SRAM GROUP7"]
pub struct SRAM7_R(crate::FieldReader<bool, bool>);
impl SRAM7_R {
    pub(crate) fn new(bits: bool) -> Self {
        SRAM7_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SRAM7_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SRAM7` writer - This bit is 1 if power is supplied to SRAM GROUP7"]
pub struct SRAM7_W<'a> {
    w: &'a mut W,
}
impl<'a> SRAM7_W<'a> {
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
#[doc = "Field `SRAM6` reader - This bit is 1 if power is supplied to SRAM GROUP6"]
pub struct SRAM6_R(crate::FieldReader<bool, bool>);
impl SRAM6_R {
    pub(crate) fn new(bits: bool) -> Self {
        SRAM6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SRAM6_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SRAM6` writer - This bit is 1 if power is supplied to SRAM GROUP6"]
pub struct SRAM6_W<'a> {
    w: &'a mut W,
}
impl<'a> SRAM6_W<'a> {
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
#[doc = "Field `SRAM5` reader - This bit is 1 if power is supplied to SRAM GROUP5"]
pub struct SRAM5_R(crate::FieldReader<bool, bool>);
impl SRAM5_R {
    pub(crate) fn new(bits: bool) -> Self {
        SRAM5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SRAM5_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SRAM5` writer - This bit is 1 if power is supplied to SRAM GROUP5"]
pub struct SRAM5_W<'a> {
    w: &'a mut W,
}
impl<'a> SRAM5_W<'a> {
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
#[doc = "Field `SRAM4` reader - This bit is 1 if power is supplied to SRAM GROUP4"]
pub struct SRAM4_R(crate::FieldReader<bool, bool>);
impl SRAM4_R {
    pub(crate) fn new(bits: bool) -> Self {
        SRAM4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SRAM4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SRAM4` writer - This bit is 1 if power is supplied to SRAM GROUP4"]
pub struct SRAM4_W<'a> {
    w: &'a mut W,
}
impl<'a> SRAM4_W<'a> {
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
#[doc = "Field `SRAM3` reader - This bit is 1 if power is supplied to SRAM GROUP3"]
pub struct SRAM3_R(crate::FieldReader<bool, bool>);
impl SRAM3_R {
    pub(crate) fn new(bits: bool) -> Self {
        SRAM3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SRAM3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SRAM3` writer - This bit is 1 if power is supplied to SRAM GROUP3"]
pub struct SRAM3_W<'a> {
    w: &'a mut W,
}
impl<'a> SRAM3_W<'a> {
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
#[doc = "Field `SRAM2` reader - This bit is 1 if power is supplied to SRAM GROUP2"]
pub struct SRAM2_R(crate::FieldReader<bool, bool>);
impl SRAM2_R {
    pub(crate) fn new(bits: bool) -> Self {
        SRAM2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SRAM2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SRAM2` writer - This bit is 1 if power is supplied to SRAM GROUP2"]
pub struct SRAM2_W<'a> {
    w: &'a mut W,
}
impl<'a> SRAM2_W<'a> {
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
#[doc = "Field `SRAM1` reader - This bit is 1 if power is supplied to SRAM GROUP1"]
pub struct SRAM1_R(crate::FieldReader<bool, bool>);
impl SRAM1_R {
    pub(crate) fn new(bits: bool) -> Self {
        SRAM1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SRAM1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SRAM1` writer - This bit is 1 if power is supplied to SRAM GROUP1"]
pub struct SRAM1_W<'a> {
    w: &'a mut W,
}
impl<'a> SRAM1_W<'a> {
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
#[doc = "Field `SRAM0` reader - This bit is 1 if power is supplied to SRAM GROUP0"]
pub struct SRAM0_R(crate::FieldReader<bool, bool>);
impl SRAM0_R {
    pub(crate) fn new(bits: bool) -> Self {
        SRAM0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SRAM0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SRAM0` writer - This bit is 1 if power is supplied to SRAM GROUP0"]
pub struct SRAM0_W<'a> {
    w: &'a mut W,
}
impl<'a> SRAM0_W<'a> {
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
#[doc = "Field `DTCM1` reader - This bit is 1 if power is supplied to DTCM GROUP1"]
pub struct DTCM1_R(crate::FieldReader<bool, bool>);
impl DTCM1_R {
    pub(crate) fn new(bits: bool) -> Self {
        DTCM1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DTCM1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DTCM1` writer - This bit is 1 if power is supplied to DTCM GROUP1"]
pub struct DTCM1_W<'a> {
    w: &'a mut W,
}
impl<'a> DTCM1_W<'a> {
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
#[doc = "Field `DTCM01` reader - This bit is 1 if power is supplied to DTCM GROUP0_1"]
pub struct DTCM01_R(crate::FieldReader<bool, bool>);
impl DTCM01_R {
    pub(crate) fn new(bits: bool) -> Self {
        DTCM01_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DTCM01_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DTCM01` writer - This bit is 1 if power is supplied to DTCM GROUP0_1"]
pub struct DTCM01_W<'a> {
    w: &'a mut W,
}
impl<'a> DTCM01_W<'a> {
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
#[doc = "Field `DTCM00` reader - This bit is 1 if power is supplied to DTCM GROUP0_0"]
pub struct DTCM00_R(crate::FieldReader<bool, bool>);
impl DTCM00_R {
    pub(crate) fn new(bits: bool) -> Self {
        DTCM00_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DTCM00_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DTCM00` writer - This bit is 1 if power is supplied to DTCM GROUP0_0"]
pub struct DTCM00_W<'a> {
    w: &'a mut W,
}
impl<'a> DTCM00_W<'a> {
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
    #[doc = "Bit 16 - This bit is 1 if power is supplied to Cache Bank 2"]
    #[inline(always)]
    pub fn cacheb2(&self) -> CACHEB2_R {
        CACHEB2_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 15 - This bit is 1 if power is supplied to Cache Bank 0"]
    #[inline(always)]
    pub fn cacheb0(&self) -> CACHEB0_R {
        CACHEB0_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 14 - This bit is 1 if power is supplied to FLASH 1"]
    #[inline(always)]
    pub fn flash1(&self) -> FLASH1_R {
        FLASH1_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 13 - This bit is 1 if power is supplied to FLASH 0"]
    #[inline(always)]
    pub fn flash0(&self) -> FLASH0_R {
        FLASH0_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12 - This bit is 1 if power is supplied to SRAM GROUP9"]
    #[inline(always)]
    pub fn sram9(&self) -> SRAM9_R {
        SRAM9_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11 - This bit is 1 if power is supplied to SRAM GROUP8"]
    #[inline(always)]
    pub fn sram8(&self) -> SRAM8_R {
        SRAM8_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10 - This bit is 1 if power is supplied to SRAM GROUP7"]
    #[inline(always)]
    pub fn sram7(&self) -> SRAM7_R {
        SRAM7_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - This bit is 1 if power is supplied to SRAM GROUP6"]
    #[inline(always)]
    pub fn sram6(&self) -> SRAM6_R {
        SRAM6_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - This bit is 1 if power is supplied to SRAM GROUP5"]
    #[inline(always)]
    pub fn sram5(&self) -> SRAM5_R {
        SRAM5_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7 - This bit is 1 if power is supplied to SRAM GROUP4"]
    #[inline(always)]
    pub fn sram4(&self) -> SRAM4_R {
        SRAM4_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - This bit is 1 if power is supplied to SRAM GROUP3"]
    #[inline(always)]
    pub fn sram3(&self) -> SRAM3_R {
        SRAM3_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - This bit is 1 if power is supplied to SRAM GROUP2"]
    #[inline(always)]
    pub fn sram2(&self) -> SRAM2_R {
        SRAM2_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - This bit is 1 if power is supplied to SRAM GROUP1"]
    #[inline(always)]
    pub fn sram1(&self) -> SRAM1_R {
        SRAM1_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - This bit is 1 if power is supplied to SRAM GROUP0"]
    #[inline(always)]
    pub fn sram0(&self) -> SRAM0_R {
        SRAM0_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - This bit is 1 if power is supplied to DTCM GROUP1"]
    #[inline(always)]
    pub fn dtcm1(&self) -> DTCM1_R {
        DTCM1_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - This bit is 1 if power is supplied to DTCM GROUP0_1"]
    #[inline(always)]
    pub fn dtcm01(&self) -> DTCM01_R {
        DTCM01_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - This bit is 1 if power is supplied to DTCM GROUP0_0"]
    #[inline(always)]
    pub fn dtcm00(&self) -> DTCM00_R {
        DTCM00_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 16 - This bit is 1 if power is supplied to Cache Bank 2"]
    #[inline(always)]
    pub fn cacheb2(&mut self) -> CACHEB2_W {
        CACHEB2_W { w: self }
    }
    #[doc = "Bit 15 - This bit is 1 if power is supplied to Cache Bank 0"]
    #[inline(always)]
    pub fn cacheb0(&mut self) -> CACHEB0_W {
        CACHEB0_W { w: self }
    }
    #[doc = "Bit 14 - This bit is 1 if power is supplied to FLASH 1"]
    #[inline(always)]
    pub fn flash1(&mut self) -> FLASH1_W {
        FLASH1_W { w: self }
    }
    #[doc = "Bit 13 - This bit is 1 if power is supplied to FLASH 0"]
    #[inline(always)]
    pub fn flash0(&mut self) -> FLASH0_W {
        FLASH0_W { w: self }
    }
    #[doc = "Bit 12 - This bit is 1 if power is supplied to SRAM GROUP9"]
    #[inline(always)]
    pub fn sram9(&mut self) -> SRAM9_W {
        SRAM9_W { w: self }
    }
    #[doc = "Bit 11 - This bit is 1 if power is supplied to SRAM GROUP8"]
    #[inline(always)]
    pub fn sram8(&mut self) -> SRAM8_W {
        SRAM8_W { w: self }
    }
    #[doc = "Bit 10 - This bit is 1 if power is supplied to SRAM GROUP7"]
    #[inline(always)]
    pub fn sram7(&mut self) -> SRAM7_W {
        SRAM7_W { w: self }
    }
    #[doc = "Bit 9 - This bit is 1 if power is supplied to SRAM GROUP6"]
    #[inline(always)]
    pub fn sram6(&mut self) -> SRAM6_W {
        SRAM6_W { w: self }
    }
    #[doc = "Bit 8 - This bit is 1 if power is supplied to SRAM GROUP5"]
    #[inline(always)]
    pub fn sram5(&mut self) -> SRAM5_W {
        SRAM5_W { w: self }
    }
    #[doc = "Bit 7 - This bit is 1 if power is supplied to SRAM GROUP4"]
    #[inline(always)]
    pub fn sram4(&mut self) -> SRAM4_W {
        SRAM4_W { w: self }
    }
    #[doc = "Bit 6 - This bit is 1 if power is supplied to SRAM GROUP3"]
    #[inline(always)]
    pub fn sram3(&mut self) -> SRAM3_W {
        SRAM3_W { w: self }
    }
    #[doc = "Bit 5 - This bit is 1 if power is supplied to SRAM GROUP2"]
    #[inline(always)]
    pub fn sram2(&mut self) -> SRAM2_W {
        SRAM2_W { w: self }
    }
    #[doc = "Bit 4 - This bit is 1 if power is supplied to SRAM GROUP1"]
    #[inline(always)]
    pub fn sram1(&mut self) -> SRAM1_W {
        SRAM1_W { w: self }
    }
    #[doc = "Bit 3 - This bit is 1 if power is supplied to SRAM GROUP0"]
    #[inline(always)]
    pub fn sram0(&mut self) -> SRAM0_W {
        SRAM0_W { w: self }
    }
    #[doc = "Bit 2 - This bit is 1 if power is supplied to DTCM GROUP1"]
    #[inline(always)]
    pub fn dtcm1(&mut self) -> DTCM1_W {
        DTCM1_W { w: self }
    }
    #[doc = "Bit 1 - This bit is 1 if power is supplied to DTCM GROUP0_1"]
    #[inline(always)]
    pub fn dtcm01(&mut self) -> DTCM01_W {
        DTCM01_W { w: self }
    }
    #[doc = "Bit 0 - This bit is 1 if power is supplied to DTCM GROUP0_0"]
    #[inline(always)]
    pub fn dtcm00(&mut self) -> DTCM00_W {
        DTCM00_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Mem Power ON Status\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mempwrstatus](index.html) module"]
pub struct MEMPWRSTATUS_SPEC;
impl crate::RegisterSpec for MEMPWRSTATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mempwrstatus::R](R) reader structure"]
impl crate::Readable for MEMPWRSTATUS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mempwrstatus::W](W) writer structure"]
impl crate::Writable for MEMPWRSTATUS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MEMPWRSTATUS to value 0x7fff"]
impl crate::Resettable for MEMPWRSTATUS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x7fff
    }
}
