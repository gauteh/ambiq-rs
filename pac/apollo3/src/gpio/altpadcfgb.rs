#[doc = "Register `ALTPADCFGB` reader"]
pub struct R(crate::R<ALTPADCFGB_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ALTPADCFGB_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ALTPADCFGB_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ALTPADCFGB_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ALTPADCFGB` writer"]
pub struct W(crate::W<ALTPADCFGB_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ALTPADCFGB_SPEC>;
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
impl From<crate::W<ALTPADCFGB_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ALTPADCFGB_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Pad 7 slew rate selection.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD7_SR_A {
    #[doc = "1: Enables Slew rate control on pad value."]
    SR_EN = 1,
}
impl From<PAD7_SR_A> for bool {
    #[inline(always)]
    fn from(variant: PAD7_SR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PAD7_SR` reader - Pad 7 slew rate selection."]
pub struct PAD7_SR_R(crate::FieldReader<bool, PAD7_SR_A>);
impl PAD7_SR_R {
    pub(crate) fn new(bits: bool) -> Self {
        PAD7_SR_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PAD7_SR_A> {
        match self.bits {
            true => Some(PAD7_SR_A::SR_EN),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `SR_EN`"]
    #[inline(always)]
    pub fn is_sr_en(&self) -> bool {
        **self == PAD7_SR_A::SR_EN
    }
}
impl core::ops::Deref for PAD7_SR_R {
    type Target = crate::FieldReader<bool, PAD7_SR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PAD7_SR` writer - Pad 7 slew rate selection."]
pub struct PAD7_SR_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD7_SR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD7_SR_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Enables Slew rate control on pad value."]
    #[inline(always)]
    pub fn sr_en(self) -> &'a mut W {
        self.variant(PAD7_SR_A::SR_EN)
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
#[doc = "Field `PAD7_DS1` reader - Pad 7 high order drive strength selection. Used in conjunction with PAD7STRNG field to set the pad drive strength."]
pub struct PAD7_DS1_R(crate::FieldReader<bool, bool>);
impl PAD7_DS1_R {
    pub(crate) fn new(bits: bool) -> Self {
        PAD7_DS1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PAD7_DS1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PAD7_DS1` writer - Pad 7 high order drive strength selection. Used in conjunction with PAD7STRNG field to set the pad drive strength."]
pub struct PAD7_DS1_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD7_DS1_W<'a> {
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
#[doc = "Pad 6 slew rate selection.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD6_SR_A {
    #[doc = "1: Enables Slew rate control on pad value."]
    SR_EN = 1,
}
impl From<PAD6_SR_A> for bool {
    #[inline(always)]
    fn from(variant: PAD6_SR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PAD6_SR` reader - Pad 6 slew rate selection."]
pub struct PAD6_SR_R(crate::FieldReader<bool, PAD6_SR_A>);
impl PAD6_SR_R {
    pub(crate) fn new(bits: bool) -> Self {
        PAD6_SR_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PAD6_SR_A> {
        match self.bits {
            true => Some(PAD6_SR_A::SR_EN),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `SR_EN`"]
    #[inline(always)]
    pub fn is_sr_en(&self) -> bool {
        **self == PAD6_SR_A::SR_EN
    }
}
impl core::ops::Deref for PAD6_SR_R {
    type Target = crate::FieldReader<bool, PAD6_SR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PAD6_SR` writer - Pad 6 slew rate selection."]
pub struct PAD6_SR_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD6_SR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD6_SR_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Enables Slew rate control on pad value."]
    #[inline(always)]
    pub fn sr_en(self) -> &'a mut W {
        self.variant(PAD6_SR_A::SR_EN)
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
#[doc = "Field `PAD6_DS1` reader - Pad 6 high order drive strength selection. Used in conjunction with PAD6STRNG field to set the pad drive strength."]
pub struct PAD6_DS1_R(crate::FieldReader<bool, bool>);
impl PAD6_DS1_R {
    pub(crate) fn new(bits: bool) -> Self {
        PAD6_DS1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PAD6_DS1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PAD6_DS1` writer - Pad 6 high order drive strength selection. Used in conjunction with PAD6STRNG field to set the pad drive strength."]
pub struct PAD6_DS1_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD6_DS1_W<'a> {
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
#[doc = "Pad 5 slew rate selection.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD5_SR_A {
    #[doc = "1: Enables Slew rate control on pad value."]
    SR_EN = 1,
}
impl From<PAD5_SR_A> for bool {
    #[inline(always)]
    fn from(variant: PAD5_SR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PAD5_SR` reader - Pad 5 slew rate selection."]
pub struct PAD5_SR_R(crate::FieldReader<bool, PAD5_SR_A>);
impl PAD5_SR_R {
    pub(crate) fn new(bits: bool) -> Self {
        PAD5_SR_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PAD5_SR_A> {
        match self.bits {
            true => Some(PAD5_SR_A::SR_EN),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `SR_EN`"]
    #[inline(always)]
    pub fn is_sr_en(&self) -> bool {
        **self == PAD5_SR_A::SR_EN
    }
}
impl core::ops::Deref for PAD5_SR_R {
    type Target = crate::FieldReader<bool, PAD5_SR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PAD5_SR` writer - Pad 5 slew rate selection."]
pub struct PAD5_SR_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD5_SR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD5_SR_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Enables Slew rate control on pad value."]
    #[inline(always)]
    pub fn sr_en(self) -> &'a mut W {
        self.variant(PAD5_SR_A::SR_EN)
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
#[doc = "Field `PAD5_DS1` reader - Pad 5 high order drive strength selection. Used in conjunction with PAD5STRNG field to set the pad drive strength."]
pub struct PAD5_DS1_R(crate::FieldReader<bool, bool>);
impl PAD5_DS1_R {
    pub(crate) fn new(bits: bool) -> Self {
        PAD5_DS1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PAD5_DS1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PAD5_DS1` writer - Pad 5 high order drive strength selection. Used in conjunction with PAD5STRNG field to set the pad drive strength."]
pub struct PAD5_DS1_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD5_DS1_W<'a> {
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
#[doc = "Pad 4 slew rate selection.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD4_SR_A {
    #[doc = "1: Enables Slew rate control on pad value."]
    SR_EN = 1,
}
impl From<PAD4_SR_A> for bool {
    #[inline(always)]
    fn from(variant: PAD4_SR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PAD4_SR` reader - Pad 4 slew rate selection."]
pub struct PAD4_SR_R(crate::FieldReader<bool, PAD4_SR_A>);
impl PAD4_SR_R {
    pub(crate) fn new(bits: bool) -> Self {
        PAD4_SR_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PAD4_SR_A> {
        match self.bits {
            true => Some(PAD4_SR_A::SR_EN),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `SR_EN`"]
    #[inline(always)]
    pub fn is_sr_en(&self) -> bool {
        **self == PAD4_SR_A::SR_EN
    }
}
impl core::ops::Deref for PAD4_SR_R {
    type Target = crate::FieldReader<bool, PAD4_SR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PAD4_SR` writer - Pad 4 slew rate selection."]
pub struct PAD4_SR_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD4_SR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD4_SR_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Enables Slew rate control on pad value."]
    #[inline(always)]
    pub fn sr_en(self) -> &'a mut W {
        self.variant(PAD4_SR_A::SR_EN)
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
#[doc = "Field `PAD4_DS1` reader - Pad 4 high order drive strength selection. Used in conjunction with PAD4STRNG field to set the pad drive strength."]
pub struct PAD4_DS1_R(crate::FieldReader<bool, bool>);
impl PAD4_DS1_R {
    pub(crate) fn new(bits: bool) -> Self {
        PAD4_DS1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PAD4_DS1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PAD4_DS1` writer - Pad 4 high order drive strength selection. Used in conjunction with PAD4STRNG field to set the pad drive strength."]
pub struct PAD4_DS1_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD4_DS1_W<'a> {
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
    #[doc = "Bit 28 - Pad 7 slew rate selection."]
    #[inline(always)]
    pub fn pad7_sr(&self) -> PAD7_SR_R {
        PAD7_SR_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Pad 7 high order drive strength selection. Used in conjunction with PAD7STRNG field to set the pad drive strength."]
    #[inline(always)]
    pub fn pad7_ds1(&self) -> PAD7_DS1_R {
        PAD7_DS1_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Pad 6 slew rate selection."]
    #[inline(always)]
    pub fn pad6_sr(&self) -> PAD6_SR_R {
        PAD6_SR_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Pad 6 high order drive strength selection. Used in conjunction with PAD6STRNG field to set the pad drive strength."]
    #[inline(always)]
    pub fn pad6_ds1(&self) -> PAD6_DS1_R {
        PAD6_DS1_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Pad 5 slew rate selection."]
    #[inline(always)]
    pub fn pad5_sr(&self) -> PAD5_SR_R {
        PAD5_SR_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Pad 5 high order drive strength selection. Used in conjunction with PAD5STRNG field to set the pad drive strength."]
    #[inline(always)]
    pub fn pad5_ds1(&self) -> PAD5_DS1_R {
        PAD5_DS1_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Pad 4 slew rate selection."]
    #[inline(always)]
    pub fn pad4_sr(&self) -> PAD4_SR_R {
        PAD4_SR_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Pad 4 high order drive strength selection. Used in conjunction with PAD4STRNG field to set the pad drive strength."]
    #[inline(always)]
    pub fn pad4_ds1(&self) -> PAD4_DS1_R {
        PAD4_DS1_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 28 - Pad 7 slew rate selection."]
    #[inline(always)]
    pub fn pad7_sr(&mut self) -> PAD7_SR_W {
        PAD7_SR_W { w: self }
    }
    #[doc = "Bit 24 - Pad 7 high order drive strength selection. Used in conjunction with PAD7STRNG field to set the pad drive strength."]
    #[inline(always)]
    pub fn pad7_ds1(&mut self) -> PAD7_DS1_W {
        PAD7_DS1_W { w: self }
    }
    #[doc = "Bit 20 - Pad 6 slew rate selection."]
    #[inline(always)]
    pub fn pad6_sr(&mut self) -> PAD6_SR_W {
        PAD6_SR_W { w: self }
    }
    #[doc = "Bit 16 - Pad 6 high order drive strength selection. Used in conjunction with PAD6STRNG field to set the pad drive strength."]
    #[inline(always)]
    pub fn pad6_ds1(&mut self) -> PAD6_DS1_W {
        PAD6_DS1_W { w: self }
    }
    #[doc = "Bit 12 - Pad 5 slew rate selection."]
    #[inline(always)]
    pub fn pad5_sr(&mut self) -> PAD5_SR_W {
        PAD5_SR_W { w: self }
    }
    #[doc = "Bit 8 - Pad 5 high order drive strength selection. Used in conjunction with PAD5STRNG field to set the pad drive strength."]
    #[inline(always)]
    pub fn pad5_ds1(&mut self) -> PAD5_DS1_W {
        PAD5_DS1_W { w: self }
    }
    #[doc = "Bit 4 - Pad 4 slew rate selection."]
    #[inline(always)]
    pub fn pad4_sr(&mut self) -> PAD4_SR_W {
        PAD4_SR_W { w: self }
    }
    #[doc = "Bit 0 - Pad 4 high order drive strength selection. Used in conjunction with PAD4STRNG field to set the pad drive strength."]
    #[inline(always)]
    pub fn pad4_ds1(&mut self) -> PAD4_DS1_W {
        PAD4_DS1_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Alternate Pad Configuration reg1 (Pads 7,6,5,4)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [altpadcfgb](index.html) module"]
pub struct ALTPADCFGB_SPEC;
impl crate::RegisterSpec for ALTPADCFGB_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [altpadcfgb::R](R) reader structure"]
impl crate::Readable for ALTPADCFGB_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [altpadcfgb::W](W) writer structure"]
impl crate::Writable for ALTPADCFGB_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ALTPADCFGB to value 0"]
impl crate::Resettable for ALTPADCFGB_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
