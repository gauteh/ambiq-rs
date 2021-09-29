#[doc = "Register `ALTPADCFGK` reader"]
pub struct R(crate::R<ALTPADCFGK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ALTPADCFGK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ALTPADCFGK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ALTPADCFGK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ALTPADCFGK` writer"]
pub struct W(crate::W<ALTPADCFGK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ALTPADCFGK_SPEC>;
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
impl From<crate::W<ALTPADCFGK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ALTPADCFGK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Pad 43 slew rate selection.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD43_SR_A {
    #[doc = "1: Enables Slew rate control on pad value."]
    SR_EN = 1,
}
impl From<PAD43_SR_A> for bool {
    #[inline(always)]
    fn from(variant: PAD43_SR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PAD43_SR` reader - Pad 43 slew rate selection."]
pub struct PAD43_SR_R(crate::FieldReader<bool, PAD43_SR_A>);
impl PAD43_SR_R {
    pub(crate) fn new(bits: bool) -> Self {
        PAD43_SR_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PAD43_SR_A> {
        match self.bits {
            true => Some(PAD43_SR_A::SR_EN),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `SR_EN`"]
    #[inline(always)]
    pub fn is_sr_en(&self) -> bool {
        **self == PAD43_SR_A::SR_EN
    }
}
impl core::ops::Deref for PAD43_SR_R {
    type Target = crate::FieldReader<bool, PAD43_SR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PAD43_SR` writer - Pad 43 slew rate selection."]
pub struct PAD43_SR_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD43_SR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD43_SR_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Enables Slew rate control on pad value."]
    #[inline(always)]
    pub fn sr_en(self) -> &'a mut W {
        self.variant(PAD43_SR_A::SR_EN)
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
#[doc = "Field `PAD43_DS1` reader - Pad 43 high order drive strength selection. Used in conjunction with PAD43STRNG field to set the pad drive strength."]
pub struct PAD43_DS1_R(crate::FieldReader<bool, bool>);
impl PAD43_DS1_R {
    pub(crate) fn new(bits: bool) -> Self {
        PAD43_DS1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PAD43_DS1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PAD43_DS1` writer - Pad 43 high order drive strength selection. Used in conjunction with PAD43STRNG field to set the pad drive strength."]
pub struct PAD43_DS1_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD43_DS1_W<'a> {
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
#[doc = "Pad 42 slew rate selection.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD42_SR_A {
    #[doc = "1: Enables Slew rate control on pad value."]
    SR_EN = 1,
}
impl From<PAD42_SR_A> for bool {
    #[inline(always)]
    fn from(variant: PAD42_SR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PAD42_SR` reader - Pad 42 slew rate selection."]
pub struct PAD42_SR_R(crate::FieldReader<bool, PAD42_SR_A>);
impl PAD42_SR_R {
    pub(crate) fn new(bits: bool) -> Self {
        PAD42_SR_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PAD42_SR_A> {
        match self.bits {
            true => Some(PAD42_SR_A::SR_EN),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `SR_EN`"]
    #[inline(always)]
    pub fn is_sr_en(&self) -> bool {
        **self == PAD42_SR_A::SR_EN
    }
}
impl core::ops::Deref for PAD42_SR_R {
    type Target = crate::FieldReader<bool, PAD42_SR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PAD42_SR` writer - Pad 42 slew rate selection."]
pub struct PAD42_SR_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD42_SR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD42_SR_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Enables Slew rate control on pad value."]
    #[inline(always)]
    pub fn sr_en(self) -> &'a mut W {
        self.variant(PAD42_SR_A::SR_EN)
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
#[doc = "Field `PAD42_DS1` reader - Pad 42 high order drive strength selection. Used in conjunction with PAD42STRNG field to set the pad drive strength."]
pub struct PAD42_DS1_R(crate::FieldReader<bool, bool>);
impl PAD42_DS1_R {
    pub(crate) fn new(bits: bool) -> Self {
        PAD42_DS1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PAD42_DS1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PAD42_DS1` writer - Pad 42 high order drive strength selection. Used in conjunction with PAD42STRNG field to set the pad drive strength."]
pub struct PAD42_DS1_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD42_DS1_W<'a> {
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
#[doc = "Pad 41 slew rate selection.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD41_SR_A {
    #[doc = "1: Enables Slew rate control on pad value."]
    SR_EN = 1,
}
impl From<PAD41_SR_A> for bool {
    #[inline(always)]
    fn from(variant: PAD41_SR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PAD41_SR` reader - Pad 41 slew rate selection."]
pub struct PAD41_SR_R(crate::FieldReader<bool, PAD41_SR_A>);
impl PAD41_SR_R {
    pub(crate) fn new(bits: bool) -> Self {
        PAD41_SR_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PAD41_SR_A> {
        match self.bits {
            true => Some(PAD41_SR_A::SR_EN),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `SR_EN`"]
    #[inline(always)]
    pub fn is_sr_en(&self) -> bool {
        **self == PAD41_SR_A::SR_EN
    }
}
impl core::ops::Deref for PAD41_SR_R {
    type Target = crate::FieldReader<bool, PAD41_SR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PAD41_SR` writer - Pad 41 slew rate selection."]
pub struct PAD41_SR_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD41_SR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD41_SR_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Enables Slew rate control on pad value."]
    #[inline(always)]
    pub fn sr_en(self) -> &'a mut W {
        self.variant(PAD41_SR_A::SR_EN)
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
#[doc = "Field `PAD41_DS1` reader - Pad 41 high order drive strength selection. Used in conjunction with PAD41STRNG field to set the pad drive strength."]
pub struct PAD41_DS1_R(crate::FieldReader<bool, bool>);
impl PAD41_DS1_R {
    pub(crate) fn new(bits: bool) -> Self {
        PAD41_DS1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PAD41_DS1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PAD41_DS1` writer - Pad 41 high order drive strength selection. Used in conjunction with PAD41STRNG field to set the pad drive strength."]
pub struct PAD41_DS1_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD41_DS1_W<'a> {
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
#[doc = "Pad 40 slew rate selection.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD40_SR_A {
    #[doc = "1: Enables Slew rate control on pad value."]
    SR_EN = 1,
}
impl From<PAD40_SR_A> for bool {
    #[inline(always)]
    fn from(variant: PAD40_SR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PAD40_SR` reader - Pad 40 slew rate selection."]
pub struct PAD40_SR_R(crate::FieldReader<bool, PAD40_SR_A>);
impl PAD40_SR_R {
    pub(crate) fn new(bits: bool) -> Self {
        PAD40_SR_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PAD40_SR_A> {
        match self.bits {
            true => Some(PAD40_SR_A::SR_EN),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `SR_EN`"]
    #[inline(always)]
    pub fn is_sr_en(&self) -> bool {
        **self == PAD40_SR_A::SR_EN
    }
}
impl core::ops::Deref for PAD40_SR_R {
    type Target = crate::FieldReader<bool, PAD40_SR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PAD40_SR` writer - Pad 40 slew rate selection."]
pub struct PAD40_SR_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD40_SR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD40_SR_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Enables Slew rate control on pad value."]
    #[inline(always)]
    pub fn sr_en(self) -> &'a mut W {
        self.variant(PAD40_SR_A::SR_EN)
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
#[doc = "Field `PAD40_DS1` reader - Pad 40 high order drive strength selection. Used in conjunction with PAD40STRNG field to set the pad drive strength."]
pub struct PAD40_DS1_R(crate::FieldReader<bool, bool>);
impl PAD40_DS1_R {
    pub(crate) fn new(bits: bool) -> Self {
        PAD40_DS1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PAD40_DS1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PAD40_DS1` writer - Pad 40 high order drive strength selection. Used in conjunction with PAD40STRNG field to set the pad drive strength."]
pub struct PAD40_DS1_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD40_DS1_W<'a> {
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
    #[doc = "Bit 28 - Pad 43 slew rate selection."]
    #[inline(always)]
    pub fn pad43_sr(&self) -> PAD43_SR_R {
        PAD43_SR_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Pad 43 high order drive strength selection. Used in conjunction with PAD43STRNG field to set the pad drive strength."]
    #[inline(always)]
    pub fn pad43_ds1(&self) -> PAD43_DS1_R {
        PAD43_DS1_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Pad 42 slew rate selection."]
    #[inline(always)]
    pub fn pad42_sr(&self) -> PAD42_SR_R {
        PAD42_SR_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Pad 42 high order drive strength selection. Used in conjunction with PAD42STRNG field to set the pad drive strength."]
    #[inline(always)]
    pub fn pad42_ds1(&self) -> PAD42_DS1_R {
        PAD42_DS1_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Pad 41 slew rate selection."]
    #[inline(always)]
    pub fn pad41_sr(&self) -> PAD41_SR_R {
        PAD41_SR_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Pad 41 high order drive strength selection. Used in conjunction with PAD41STRNG field to set the pad drive strength."]
    #[inline(always)]
    pub fn pad41_ds1(&self) -> PAD41_DS1_R {
        PAD41_DS1_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Pad 40 slew rate selection."]
    #[inline(always)]
    pub fn pad40_sr(&self) -> PAD40_SR_R {
        PAD40_SR_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Pad 40 high order drive strength selection. Used in conjunction with PAD40STRNG field to set the pad drive strength."]
    #[inline(always)]
    pub fn pad40_ds1(&self) -> PAD40_DS1_R {
        PAD40_DS1_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 28 - Pad 43 slew rate selection."]
    #[inline(always)]
    pub fn pad43_sr(&mut self) -> PAD43_SR_W {
        PAD43_SR_W { w: self }
    }
    #[doc = "Bit 24 - Pad 43 high order drive strength selection. Used in conjunction with PAD43STRNG field to set the pad drive strength."]
    #[inline(always)]
    pub fn pad43_ds1(&mut self) -> PAD43_DS1_W {
        PAD43_DS1_W { w: self }
    }
    #[doc = "Bit 20 - Pad 42 slew rate selection."]
    #[inline(always)]
    pub fn pad42_sr(&mut self) -> PAD42_SR_W {
        PAD42_SR_W { w: self }
    }
    #[doc = "Bit 16 - Pad 42 high order drive strength selection. Used in conjunction with PAD42STRNG field to set the pad drive strength."]
    #[inline(always)]
    pub fn pad42_ds1(&mut self) -> PAD42_DS1_W {
        PAD42_DS1_W { w: self }
    }
    #[doc = "Bit 12 - Pad 41 slew rate selection."]
    #[inline(always)]
    pub fn pad41_sr(&mut self) -> PAD41_SR_W {
        PAD41_SR_W { w: self }
    }
    #[doc = "Bit 8 - Pad 41 high order drive strength selection. Used in conjunction with PAD41STRNG field to set the pad drive strength."]
    #[inline(always)]
    pub fn pad41_ds1(&mut self) -> PAD41_DS1_W {
        PAD41_DS1_W { w: self }
    }
    #[doc = "Bit 4 - Pad 40 slew rate selection."]
    #[inline(always)]
    pub fn pad40_sr(&mut self) -> PAD40_SR_W {
        PAD40_SR_W { w: self }
    }
    #[doc = "Bit 0 - Pad 40 high order drive strength selection. Used in conjunction with PAD40STRNG field to set the pad drive strength."]
    #[inline(always)]
    pub fn pad40_ds1(&mut self) -> PAD40_DS1_W {
        PAD40_DS1_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Alternate Pad Configuration reg10 (Pads 43,42,41,40)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [altpadcfgk](index.html) module"]
pub struct ALTPADCFGK_SPEC;
impl crate::RegisterSpec for ALTPADCFGK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [altpadcfgk::R](R) reader structure"]
impl crate::Readable for ALTPADCFGK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [altpadcfgk::W](W) writer structure"]
impl crate::Writable for ALTPADCFGK_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ALTPADCFGK to value 0"]
impl crate::Resettable for ALTPADCFGK_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
