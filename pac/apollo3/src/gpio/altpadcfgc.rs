#[doc = "Register `ALTPADCFGC` reader"]
pub struct R(crate::R<ALTPADCFGC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ALTPADCFGC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ALTPADCFGC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ALTPADCFGC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ALTPADCFGC` writer"]
pub struct W(crate::W<ALTPADCFGC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ALTPADCFGC_SPEC>;
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
impl From<crate::W<ALTPADCFGC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ALTPADCFGC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Pad 11 slew rate selection.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD11_SR_A {
    #[doc = "1: Enables Slew rate control on pad value."]
    SR_EN = 1,
}
impl From<PAD11_SR_A> for bool {
    #[inline(always)]
    fn from(variant: PAD11_SR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PAD11_SR` reader - Pad 11 slew rate selection."]
pub struct PAD11_SR_R(crate::FieldReader<bool, PAD11_SR_A>);
impl PAD11_SR_R {
    pub(crate) fn new(bits: bool) -> Self {
        PAD11_SR_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PAD11_SR_A> {
        match self.bits {
            true => Some(PAD11_SR_A::SR_EN),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `SR_EN`"]
    #[inline(always)]
    pub fn is_sr_en(&self) -> bool {
        **self == PAD11_SR_A::SR_EN
    }
}
impl core::ops::Deref for PAD11_SR_R {
    type Target = crate::FieldReader<bool, PAD11_SR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PAD11_SR` writer - Pad 11 slew rate selection."]
pub struct PAD11_SR_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD11_SR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD11_SR_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Enables Slew rate control on pad value."]
    #[inline(always)]
    pub fn sr_en(self) -> &'a mut W {
        self.variant(PAD11_SR_A::SR_EN)
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
#[doc = "Field `PAD11_DS1` reader - Pad 11 high order drive strength selection. Used in conjunction with PAD11STRNG field to set the pad drive strength."]
pub struct PAD11_DS1_R(crate::FieldReader<bool, bool>);
impl PAD11_DS1_R {
    pub(crate) fn new(bits: bool) -> Self {
        PAD11_DS1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PAD11_DS1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PAD11_DS1` writer - Pad 11 high order drive strength selection. Used in conjunction with PAD11STRNG field to set the pad drive strength."]
pub struct PAD11_DS1_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD11_DS1_W<'a> {
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
#[doc = "Pad 10 slew rate selection.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD10_SR_A {
    #[doc = "1: Enables Slew rate control on pad value."]
    SR_EN = 1,
}
impl From<PAD10_SR_A> for bool {
    #[inline(always)]
    fn from(variant: PAD10_SR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PAD10_SR` reader - Pad 10 slew rate selection."]
pub struct PAD10_SR_R(crate::FieldReader<bool, PAD10_SR_A>);
impl PAD10_SR_R {
    pub(crate) fn new(bits: bool) -> Self {
        PAD10_SR_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PAD10_SR_A> {
        match self.bits {
            true => Some(PAD10_SR_A::SR_EN),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `SR_EN`"]
    #[inline(always)]
    pub fn is_sr_en(&self) -> bool {
        **self == PAD10_SR_A::SR_EN
    }
}
impl core::ops::Deref for PAD10_SR_R {
    type Target = crate::FieldReader<bool, PAD10_SR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PAD10_SR` writer - Pad 10 slew rate selection."]
pub struct PAD10_SR_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD10_SR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD10_SR_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Enables Slew rate control on pad value."]
    #[inline(always)]
    pub fn sr_en(self) -> &'a mut W {
        self.variant(PAD10_SR_A::SR_EN)
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
#[doc = "Field `PAD10_DS1` reader - Pad 10 high order drive strength selection. Used in conjunction with PAD10STRNG field to set the pad drive strength."]
pub struct PAD10_DS1_R(crate::FieldReader<bool, bool>);
impl PAD10_DS1_R {
    pub(crate) fn new(bits: bool) -> Self {
        PAD10_DS1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PAD10_DS1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PAD10_DS1` writer - Pad 10 high order drive strength selection. Used in conjunction with PAD10STRNG field to set the pad drive strength."]
pub struct PAD10_DS1_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD10_DS1_W<'a> {
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
#[doc = "Pad 9 slew rate selection.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD9_SR_A {
    #[doc = "1: Enables Slew rate control on pad value."]
    SR_EN = 1,
}
impl From<PAD9_SR_A> for bool {
    #[inline(always)]
    fn from(variant: PAD9_SR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PAD9_SR` reader - Pad 9 slew rate selection."]
pub struct PAD9_SR_R(crate::FieldReader<bool, PAD9_SR_A>);
impl PAD9_SR_R {
    pub(crate) fn new(bits: bool) -> Self {
        PAD9_SR_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PAD9_SR_A> {
        match self.bits {
            true => Some(PAD9_SR_A::SR_EN),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `SR_EN`"]
    #[inline(always)]
    pub fn is_sr_en(&self) -> bool {
        **self == PAD9_SR_A::SR_EN
    }
}
impl core::ops::Deref for PAD9_SR_R {
    type Target = crate::FieldReader<bool, PAD9_SR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PAD9_SR` writer - Pad 9 slew rate selection."]
pub struct PAD9_SR_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD9_SR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD9_SR_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Enables Slew rate control on pad value."]
    #[inline(always)]
    pub fn sr_en(self) -> &'a mut W {
        self.variant(PAD9_SR_A::SR_EN)
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
#[doc = "Field `PAD9_DS1` reader - Pad 9 high order drive strength selection. Used in conjunction with PAD9STRNG field to set the pad drive strength."]
pub struct PAD9_DS1_R(crate::FieldReader<bool, bool>);
impl PAD9_DS1_R {
    pub(crate) fn new(bits: bool) -> Self {
        PAD9_DS1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PAD9_DS1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PAD9_DS1` writer - Pad 9 high order drive strength selection. Used in conjunction with PAD9STRNG field to set the pad drive strength."]
pub struct PAD9_DS1_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD9_DS1_W<'a> {
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
#[doc = "Pad 8 slew rate selection.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD8_SR_A {
    #[doc = "1: Enables Slew rate control on pad value."]
    SR_EN = 1,
}
impl From<PAD8_SR_A> for bool {
    #[inline(always)]
    fn from(variant: PAD8_SR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PAD8_SR` reader - Pad 8 slew rate selection."]
pub struct PAD8_SR_R(crate::FieldReader<bool, PAD8_SR_A>);
impl PAD8_SR_R {
    pub(crate) fn new(bits: bool) -> Self {
        PAD8_SR_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PAD8_SR_A> {
        match self.bits {
            true => Some(PAD8_SR_A::SR_EN),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `SR_EN`"]
    #[inline(always)]
    pub fn is_sr_en(&self) -> bool {
        **self == PAD8_SR_A::SR_EN
    }
}
impl core::ops::Deref for PAD8_SR_R {
    type Target = crate::FieldReader<bool, PAD8_SR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PAD8_SR` writer - Pad 8 slew rate selection."]
pub struct PAD8_SR_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD8_SR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD8_SR_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Enables Slew rate control on pad value."]
    #[inline(always)]
    pub fn sr_en(self) -> &'a mut W {
        self.variant(PAD8_SR_A::SR_EN)
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
#[doc = "Field `PAD8_DS1` reader - Pad 8 high order drive strength selection. Used in conjunction with PAD8STRNG field to set the pad drive strength."]
pub struct PAD8_DS1_R(crate::FieldReader<bool, bool>);
impl PAD8_DS1_R {
    pub(crate) fn new(bits: bool) -> Self {
        PAD8_DS1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PAD8_DS1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PAD8_DS1` writer - Pad 8 high order drive strength selection. Used in conjunction with PAD8STRNG field to set the pad drive strength."]
pub struct PAD8_DS1_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD8_DS1_W<'a> {
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
    #[doc = "Bit 28 - Pad 11 slew rate selection."]
    #[inline(always)]
    pub fn pad11_sr(&self) -> PAD11_SR_R {
        PAD11_SR_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Pad 11 high order drive strength selection. Used in conjunction with PAD11STRNG field to set the pad drive strength."]
    #[inline(always)]
    pub fn pad11_ds1(&self) -> PAD11_DS1_R {
        PAD11_DS1_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Pad 10 slew rate selection."]
    #[inline(always)]
    pub fn pad10_sr(&self) -> PAD10_SR_R {
        PAD10_SR_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Pad 10 high order drive strength selection. Used in conjunction with PAD10STRNG field to set the pad drive strength."]
    #[inline(always)]
    pub fn pad10_ds1(&self) -> PAD10_DS1_R {
        PAD10_DS1_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Pad 9 slew rate selection."]
    #[inline(always)]
    pub fn pad9_sr(&self) -> PAD9_SR_R {
        PAD9_SR_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Pad 9 high order drive strength selection. Used in conjunction with PAD9STRNG field to set the pad drive strength."]
    #[inline(always)]
    pub fn pad9_ds1(&self) -> PAD9_DS1_R {
        PAD9_DS1_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Pad 8 slew rate selection."]
    #[inline(always)]
    pub fn pad8_sr(&self) -> PAD8_SR_R {
        PAD8_SR_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Pad 8 high order drive strength selection. Used in conjunction with PAD8STRNG field to set the pad drive strength."]
    #[inline(always)]
    pub fn pad8_ds1(&self) -> PAD8_DS1_R {
        PAD8_DS1_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 28 - Pad 11 slew rate selection."]
    #[inline(always)]
    pub fn pad11_sr(&mut self) -> PAD11_SR_W {
        PAD11_SR_W { w: self }
    }
    #[doc = "Bit 24 - Pad 11 high order drive strength selection. Used in conjunction with PAD11STRNG field to set the pad drive strength."]
    #[inline(always)]
    pub fn pad11_ds1(&mut self) -> PAD11_DS1_W {
        PAD11_DS1_W { w: self }
    }
    #[doc = "Bit 20 - Pad 10 slew rate selection."]
    #[inline(always)]
    pub fn pad10_sr(&mut self) -> PAD10_SR_W {
        PAD10_SR_W { w: self }
    }
    #[doc = "Bit 16 - Pad 10 high order drive strength selection. Used in conjunction with PAD10STRNG field to set the pad drive strength."]
    #[inline(always)]
    pub fn pad10_ds1(&mut self) -> PAD10_DS1_W {
        PAD10_DS1_W { w: self }
    }
    #[doc = "Bit 12 - Pad 9 slew rate selection."]
    #[inline(always)]
    pub fn pad9_sr(&mut self) -> PAD9_SR_W {
        PAD9_SR_W { w: self }
    }
    #[doc = "Bit 8 - Pad 9 high order drive strength selection. Used in conjunction with PAD9STRNG field to set the pad drive strength."]
    #[inline(always)]
    pub fn pad9_ds1(&mut self) -> PAD9_DS1_W {
        PAD9_DS1_W { w: self }
    }
    #[doc = "Bit 4 - Pad 8 slew rate selection."]
    #[inline(always)]
    pub fn pad8_sr(&mut self) -> PAD8_SR_W {
        PAD8_SR_W { w: self }
    }
    #[doc = "Bit 0 - Pad 8 high order drive strength selection. Used in conjunction with PAD8STRNG field to set the pad drive strength."]
    #[inline(always)]
    pub fn pad8_ds1(&mut self) -> PAD8_DS1_W {
        PAD8_DS1_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Alternate Pad Configuration reg2 (Pads 11,10,9,8)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [altpadcfgc](index.html) module"]
pub struct ALTPADCFGC_SPEC;
impl crate::RegisterSpec for ALTPADCFGC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [altpadcfgc::R](R) reader structure"]
impl crate::Readable for ALTPADCFGC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [altpadcfgc::W](W) writer structure"]
impl crate::Writable for ALTPADCFGC_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ALTPADCFGC to value 0"]
impl crate::Resettable for ALTPADCFGC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
