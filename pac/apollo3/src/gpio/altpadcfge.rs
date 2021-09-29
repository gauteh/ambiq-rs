#[doc = "Register `ALTPADCFGE` reader"]
pub struct R(crate::R<ALTPADCFGE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ALTPADCFGE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ALTPADCFGE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ALTPADCFGE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ALTPADCFGE` writer"]
pub struct W(crate::W<ALTPADCFGE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ALTPADCFGE_SPEC>;
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
impl From<crate::W<ALTPADCFGE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ALTPADCFGE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Pad 19 slew rate selection.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD19_SR_A {
    #[doc = "1: Enables Slew rate control on pad value."]
    SR_EN = 1,
}
impl From<PAD19_SR_A> for bool {
    #[inline(always)]
    fn from(variant: PAD19_SR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PAD19_SR` reader - Pad 19 slew rate selection."]
pub struct PAD19_SR_R(crate::FieldReader<bool, PAD19_SR_A>);
impl PAD19_SR_R {
    pub(crate) fn new(bits: bool) -> Self {
        PAD19_SR_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PAD19_SR_A> {
        match self.bits {
            true => Some(PAD19_SR_A::SR_EN),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `SR_EN`"]
    #[inline(always)]
    pub fn is_sr_en(&self) -> bool {
        **self == PAD19_SR_A::SR_EN
    }
}
impl core::ops::Deref for PAD19_SR_R {
    type Target = crate::FieldReader<bool, PAD19_SR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PAD19_SR` writer - Pad 19 slew rate selection."]
pub struct PAD19_SR_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD19_SR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD19_SR_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Enables Slew rate control on pad value."]
    #[inline(always)]
    pub fn sr_en(self) -> &'a mut W {
        self.variant(PAD19_SR_A::SR_EN)
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
#[doc = "Field `PAD19_DS1` reader - Pad 19 high order drive strength selection. Used in conjunction with PAD19STRNG field to set the pad drive strength."]
pub struct PAD19_DS1_R(crate::FieldReader<bool, bool>);
impl PAD19_DS1_R {
    pub(crate) fn new(bits: bool) -> Self {
        PAD19_DS1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PAD19_DS1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PAD19_DS1` writer - Pad 19 high order drive strength selection. Used in conjunction with PAD19STRNG field to set the pad drive strength."]
pub struct PAD19_DS1_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD19_DS1_W<'a> {
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
#[doc = "Pad 18 slew rate selection.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD18_SR_A {
    #[doc = "1: Enables Slew rate control on pad value."]
    SR_EN = 1,
}
impl From<PAD18_SR_A> for bool {
    #[inline(always)]
    fn from(variant: PAD18_SR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PAD18_SR` reader - Pad 18 slew rate selection."]
pub struct PAD18_SR_R(crate::FieldReader<bool, PAD18_SR_A>);
impl PAD18_SR_R {
    pub(crate) fn new(bits: bool) -> Self {
        PAD18_SR_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PAD18_SR_A> {
        match self.bits {
            true => Some(PAD18_SR_A::SR_EN),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `SR_EN`"]
    #[inline(always)]
    pub fn is_sr_en(&self) -> bool {
        **self == PAD18_SR_A::SR_EN
    }
}
impl core::ops::Deref for PAD18_SR_R {
    type Target = crate::FieldReader<bool, PAD18_SR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PAD18_SR` writer - Pad 18 slew rate selection."]
pub struct PAD18_SR_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD18_SR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD18_SR_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Enables Slew rate control on pad value."]
    #[inline(always)]
    pub fn sr_en(self) -> &'a mut W {
        self.variant(PAD18_SR_A::SR_EN)
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
#[doc = "Field `PAD18_DS1` reader - Pad 18 high order drive strength selection. Used in conjunction with PAD18STRNG field to set the pad drive strength."]
pub struct PAD18_DS1_R(crate::FieldReader<bool, bool>);
impl PAD18_DS1_R {
    pub(crate) fn new(bits: bool) -> Self {
        PAD18_DS1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PAD18_DS1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PAD18_DS1` writer - Pad 18 high order drive strength selection. Used in conjunction with PAD18STRNG field to set the pad drive strength."]
pub struct PAD18_DS1_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD18_DS1_W<'a> {
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
#[doc = "Pad 17 slew rate selection.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD17_SR_A {
    #[doc = "1: Enables Slew rate control on pad value."]
    SR_EN = 1,
}
impl From<PAD17_SR_A> for bool {
    #[inline(always)]
    fn from(variant: PAD17_SR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PAD17_SR` reader - Pad 17 slew rate selection."]
pub struct PAD17_SR_R(crate::FieldReader<bool, PAD17_SR_A>);
impl PAD17_SR_R {
    pub(crate) fn new(bits: bool) -> Self {
        PAD17_SR_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PAD17_SR_A> {
        match self.bits {
            true => Some(PAD17_SR_A::SR_EN),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `SR_EN`"]
    #[inline(always)]
    pub fn is_sr_en(&self) -> bool {
        **self == PAD17_SR_A::SR_EN
    }
}
impl core::ops::Deref for PAD17_SR_R {
    type Target = crate::FieldReader<bool, PAD17_SR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PAD17_SR` writer - Pad 17 slew rate selection."]
pub struct PAD17_SR_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD17_SR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD17_SR_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Enables Slew rate control on pad value."]
    #[inline(always)]
    pub fn sr_en(self) -> &'a mut W {
        self.variant(PAD17_SR_A::SR_EN)
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
#[doc = "Field `PAD17_DS1` reader - Pad 17 high order drive strength selection. Used in conjunction with PAD17STRNG field to set the pad drive strength."]
pub struct PAD17_DS1_R(crate::FieldReader<bool, bool>);
impl PAD17_DS1_R {
    pub(crate) fn new(bits: bool) -> Self {
        PAD17_DS1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PAD17_DS1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PAD17_DS1` writer - Pad 17 high order drive strength selection. Used in conjunction with PAD17STRNG field to set the pad drive strength."]
pub struct PAD17_DS1_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD17_DS1_W<'a> {
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
#[doc = "Pad 16 slew rate selection.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD16_SR_A {
    #[doc = "1: Enables Slew rate control on pad value."]
    SR_EN = 1,
}
impl From<PAD16_SR_A> for bool {
    #[inline(always)]
    fn from(variant: PAD16_SR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PAD16_SR` reader - Pad 16 slew rate selection."]
pub struct PAD16_SR_R(crate::FieldReader<bool, PAD16_SR_A>);
impl PAD16_SR_R {
    pub(crate) fn new(bits: bool) -> Self {
        PAD16_SR_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PAD16_SR_A> {
        match self.bits {
            true => Some(PAD16_SR_A::SR_EN),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `SR_EN`"]
    #[inline(always)]
    pub fn is_sr_en(&self) -> bool {
        **self == PAD16_SR_A::SR_EN
    }
}
impl core::ops::Deref for PAD16_SR_R {
    type Target = crate::FieldReader<bool, PAD16_SR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PAD16_SR` writer - Pad 16 slew rate selection."]
pub struct PAD16_SR_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD16_SR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD16_SR_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Enables Slew rate control on pad value."]
    #[inline(always)]
    pub fn sr_en(self) -> &'a mut W {
        self.variant(PAD16_SR_A::SR_EN)
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
#[doc = "Field `PAD16_DS1` reader - Pad 16 high order drive strength selection. Used in conjunction with PAD16STRNG field to set the pad drive strength."]
pub struct PAD16_DS1_R(crate::FieldReader<bool, bool>);
impl PAD16_DS1_R {
    pub(crate) fn new(bits: bool) -> Self {
        PAD16_DS1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PAD16_DS1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PAD16_DS1` writer - Pad 16 high order drive strength selection. Used in conjunction with PAD16STRNG field to set the pad drive strength."]
pub struct PAD16_DS1_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD16_DS1_W<'a> {
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
    #[doc = "Bit 28 - Pad 19 slew rate selection."]
    #[inline(always)]
    pub fn pad19_sr(&self) -> PAD19_SR_R {
        PAD19_SR_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Pad 19 high order drive strength selection. Used in conjunction with PAD19STRNG field to set the pad drive strength."]
    #[inline(always)]
    pub fn pad19_ds1(&self) -> PAD19_DS1_R {
        PAD19_DS1_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Pad 18 slew rate selection."]
    #[inline(always)]
    pub fn pad18_sr(&self) -> PAD18_SR_R {
        PAD18_SR_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Pad 18 high order drive strength selection. Used in conjunction with PAD18STRNG field to set the pad drive strength."]
    #[inline(always)]
    pub fn pad18_ds1(&self) -> PAD18_DS1_R {
        PAD18_DS1_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Pad 17 slew rate selection."]
    #[inline(always)]
    pub fn pad17_sr(&self) -> PAD17_SR_R {
        PAD17_SR_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Pad 17 high order drive strength selection. Used in conjunction with PAD17STRNG field to set the pad drive strength."]
    #[inline(always)]
    pub fn pad17_ds1(&self) -> PAD17_DS1_R {
        PAD17_DS1_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Pad 16 slew rate selection."]
    #[inline(always)]
    pub fn pad16_sr(&self) -> PAD16_SR_R {
        PAD16_SR_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Pad 16 high order drive strength selection. Used in conjunction with PAD16STRNG field to set the pad drive strength."]
    #[inline(always)]
    pub fn pad16_ds1(&self) -> PAD16_DS1_R {
        PAD16_DS1_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 28 - Pad 19 slew rate selection."]
    #[inline(always)]
    pub fn pad19_sr(&mut self) -> PAD19_SR_W {
        PAD19_SR_W { w: self }
    }
    #[doc = "Bit 24 - Pad 19 high order drive strength selection. Used in conjunction with PAD19STRNG field to set the pad drive strength."]
    #[inline(always)]
    pub fn pad19_ds1(&mut self) -> PAD19_DS1_W {
        PAD19_DS1_W { w: self }
    }
    #[doc = "Bit 20 - Pad 18 slew rate selection."]
    #[inline(always)]
    pub fn pad18_sr(&mut self) -> PAD18_SR_W {
        PAD18_SR_W { w: self }
    }
    #[doc = "Bit 16 - Pad 18 high order drive strength selection. Used in conjunction with PAD18STRNG field to set the pad drive strength."]
    #[inline(always)]
    pub fn pad18_ds1(&mut self) -> PAD18_DS1_W {
        PAD18_DS1_W { w: self }
    }
    #[doc = "Bit 12 - Pad 17 slew rate selection."]
    #[inline(always)]
    pub fn pad17_sr(&mut self) -> PAD17_SR_W {
        PAD17_SR_W { w: self }
    }
    #[doc = "Bit 8 - Pad 17 high order drive strength selection. Used in conjunction with PAD17STRNG field to set the pad drive strength."]
    #[inline(always)]
    pub fn pad17_ds1(&mut self) -> PAD17_DS1_W {
        PAD17_DS1_W { w: self }
    }
    #[doc = "Bit 4 - Pad 16 slew rate selection."]
    #[inline(always)]
    pub fn pad16_sr(&mut self) -> PAD16_SR_W {
        PAD16_SR_W { w: self }
    }
    #[doc = "Bit 0 - Pad 16 high order drive strength selection. Used in conjunction with PAD16STRNG field to set the pad drive strength."]
    #[inline(always)]
    pub fn pad16_ds1(&mut self) -> PAD16_DS1_W {
        PAD16_DS1_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Alternate Pad Configuration reg4 (Pads 19,18,17,16)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [altpadcfge](index.html) module"]
pub struct ALTPADCFGE_SPEC;
impl crate::RegisterSpec for ALTPADCFGE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [altpadcfge::R](R) reader structure"]
impl crate::Readable for ALTPADCFGE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [altpadcfge::W](W) writer structure"]
impl crate::Writable for ALTPADCFGE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ALTPADCFGE to value 0"]
impl crate::Resettable for ALTPADCFGE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
