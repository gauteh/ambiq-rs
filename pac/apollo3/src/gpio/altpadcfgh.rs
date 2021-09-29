#[doc = "Register `ALTPADCFGH` reader"]
pub struct R(crate::R<ALTPADCFGH_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ALTPADCFGH_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ALTPADCFGH_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ALTPADCFGH_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ALTPADCFGH` writer"]
pub struct W(crate::W<ALTPADCFGH_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ALTPADCFGH_SPEC>;
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
impl From<crate::W<ALTPADCFGH_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ALTPADCFGH_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Pad 31 slew rate selection.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD31_SR_A {
    #[doc = "1: Enables Slew rate control on pad value."]
    SR_EN = 1,
}
impl From<PAD31_SR_A> for bool {
    #[inline(always)]
    fn from(variant: PAD31_SR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PAD31_SR` reader - Pad 31 slew rate selection."]
pub struct PAD31_SR_R(crate::FieldReader<bool, PAD31_SR_A>);
impl PAD31_SR_R {
    pub(crate) fn new(bits: bool) -> Self {
        PAD31_SR_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PAD31_SR_A> {
        match self.bits {
            true => Some(PAD31_SR_A::SR_EN),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `SR_EN`"]
    #[inline(always)]
    pub fn is_sr_en(&self) -> bool {
        **self == PAD31_SR_A::SR_EN
    }
}
impl core::ops::Deref for PAD31_SR_R {
    type Target = crate::FieldReader<bool, PAD31_SR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PAD31_SR` writer - Pad 31 slew rate selection."]
pub struct PAD31_SR_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD31_SR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD31_SR_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Enables Slew rate control on pad value."]
    #[inline(always)]
    pub fn sr_en(self) -> &'a mut W {
        self.variant(PAD31_SR_A::SR_EN)
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
#[doc = "Field `PAD31_DS1` reader - Pad 31 high order drive strength selection. Used in conjunction with PAD31STRNG field to set the pad drive strength."]
pub struct PAD31_DS1_R(crate::FieldReader<bool, bool>);
impl PAD31_DS1_R {
    pub(crate) fn new(bits: bool) -> Self {
        PAD31_DS1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PAD31_DS1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PAD31_DS1` writer - Pad 31 high order drive strength selection. Used in conjunction with PAD31STRNG field to set the pad drive strength."]
pub struct PAD31_DS1_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD31_DS1_W<'a> {
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
#[doc = "Pad 30 slew rate selection.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD30_SR_A {
    #[doc = "1: Enables Slew rate control on pad value."]
    SR_EN = 1,
}
impl From<PAD30_SR_A> for bool {
    #[inline(always)]
    fn from(variant: PAD30_SR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PAD30_SR` reader - Pad 30 slew rate selection."]
pub struct PAD30_SR_R(crate::FieldReader<bool, PAD30_SR_A>);
impl PAD30_SR_R {
    pub(crate) fn new(bits: bool) -> Self {
        PAD30_SR_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PAD30_SR_A> {
        match self.bits {
            true => Some(PAD30_SR_A::SR_EN),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `SR_EN`"]
    #[inline(always)]
    pub fn is_sr_en(&self) -> bool {
        **self == PAD30_SR_A::SR_EN
    }
}
impl core::ops::Deref for PAD30_SR_R {
    type Target = crate::FieldReader<bool, PAD30_SR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PAD30_SR` writer - Pad 30 slew rate selection."]
pub struct PAD30_SR_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD30_SR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD30_SR_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Enables Slew rate control on pad value."]
    #[inline(always)]
    pub fn sr_en(self) -> &'a mut W {
        self.variant(PAD30_SR_A::SR_EN)
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
#[doc = "Field `PAD30_DS1` reader - Pad 30 high order drive strength selection. Used in conjunction with PAD30STRNG field to set the pad drive strength."]
pub struct PAD30_DS1_R(crate::FieldReader<bool, bool>);
impl PAD30_DS1_R {
    pub(crate) fn new(bits: bool) -> Self {
        PAD30_DS1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PAD30_DS1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PAD30_DS1` writer - Pad 30 high order drive strength selection. Used in conjunction with PAD30STRNG field to set the pad drive strength."]
pub struct PAD30_DS1_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD30_DS1_W<'a> {
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
#[doc = "Pad 29 slew rate selection.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD29_SR_A {
    #[doc = "1: Enables Slew rate control on pad value."]
    SR_EN = 1,
}
impl From<PAD29_SR_A> for bool {
    #[inline(always)]
    fn from(variant: PAD29_SR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PAD29_SR` reader - Pad 29 slew rate selection."]
pub struct PAD29_SR_R(crate::FieldReader<bool, PAD29_SR_A>);
impl PAD29_SR_R {
    pub(crate) fn new(bits: bool) -> Self {
        PAD29_SR_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PAD29_SR_A> {
        match self.bits {
            true => Some(PAD29_SR_A::SR_EN),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `SR_EN`"]
    #[inline(always)]
    pub fn is_sr_en(&self) -> bool {
        **self == PAD29_SR_A::SR_EN
    }
}
impl core::ops::Deref for PAD29_SR_R {
    type Target = crate::FieldReader<bool, PAD29_SR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PAD29_SR` writer - Pad 29 slew rate selection."]
pub struct PAD29_SR_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD29_SR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD29_SR_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Enables Slew rate control on pad value."]
    #[inline(always)]
    pub fn sr_en(self) -> &'a mut W {
        self.variant(PAD29_SR_A::SR_EN)
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
#[doc = "Field `PAD29_DS1` reader - Pad 29 high order drive strength selection. Used in conjunction with PAD29STRNG field to set the pad drive strength."]
pub struct PAD29_DS1_R(crate::FieldReader<bool, bool>);
impl PAD29_DS1_R {
    pub(crate) fn new(bits: bool) -> Self {
        PAD29_DS1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PAD29_DS1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PAD29_DS1` writer - Pad 29 high order drive strength selection. Used in conjunction with PAD29STRNG field to set the pad drive strength."]
pub struct PAD29_DS1_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD29_DS1_W<'a> {
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
#[doc = "Pad 28 slew rate selection.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD28_SR_A {
    #[doc = "1: Enables Slew rate control on pad value."]
    SR_EN = 1,
}
impl From<PAD28_SR_A> for bool {
    #[inline(always)]
    fn from(variant: PAD28_SR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PAD28_SR` reader - Pad 28 slew rate selection."]
pub struct PAD28_SR_R(crate::FieldReader<bool, PAD28_SR_A>);
impl PAD28_SR_R {
    pub(crate) fn new(bits: bool) -> Self {
        PAD28_SR_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PAD28_SR_A> {
        match self.bits {
            true => Some(PAD28_SR_A::SR_EN),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `SR_EN`"]
    #[inline(always)]
    pub fn is_sr_en(&self) -> bool {
        **self == PAD28_SR_A::SR_EN
    }
}
impl core::ops::Deref for PAD28_SR_R {
    type Target = crate::FieldReader<bool, PAD28_SR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PAD28_SR` writer - Pad 28 slew rate selection."]
pub struct PAD28_SR_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD28_SR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD28_SR_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Enables Slew rate control on pad value."]
    #[inline(always)]
    pub fn sr_en(self) -> &'a mut W {
        self.variant(PAD28_SR_A::SR_EN)
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
#[doc = "Field `PAD28_DS1` reader - Pad 28 high order drive strength selection. Used in conjunction with PAD28STRNG field to set the pad drive strength."]
pub struct PAD28_DS1_R(crate::FieldReader<bool, bool>);
impl PAD28_DS1_R {
    pub(crate) fn new(bits: bool) -> Self {
        PAD28_DS1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PAD28_DS1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PAD28_DS1` writer - Pad 28 high order drive strength selection. Used in conjunction with PAD28STRNG field to set the pad drive strength."]
pub struct PAD28_DS1_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD28_DS1_W<'a> {
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
    #[doc = "Bit 28 - Pad 31 slew rate selection."]
    #[inline(always)]
    pub fn pad31_sr(&self) -> PAD31_SR_R {
        PAD31_SR_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Pad 31 high order drive strength selection. Used in conjunction with PAD31STRNG field to set the pad drive strength."]
    #[inline(always)]
    pub fn pad31_ds1(&self) -> PAD31_DS1_R {
        PAD31_DS1_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Pad 30 slew rate selection."]
    #[inline(always)]
    pub fn pad30_sr(&self) -> PAD30_SR_R {
        PAD30_SR_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Pad 30 high order drive strength selection. Used in conjunction with PAD30STRNG field to set the pad drive strength."]
    #[inline(always)]
    pub fn pad30_ds1(&self) -> PAD30_DS1_R {
        PAD30_DS1_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Pad 29 slew rate selection."]
    #[inline(always)]
    pub fn pad29_sr(&self) -> PAD29_SR_R {
        PAD29_SR_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Pad 29 high order drive strength selection. Used in conjunction with PAD29STRNG field to set the pad drive strength."]
    #[inline(always)]
    pub fn pad29_ds1(&self) -> PAD29_DS1_R {
        PAD29_DS1_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Pad 28 slew rate selection."]
    #[inline(always)]
    pub fn pad28_sr(&self) -> PAD28_SR_R {
        PAD28_SR_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Pad 28 high order drive strength selection. Used in conjunction with PAD28STRNG field to set the pad drive strength."]
    #[inline(always)]
    pub fn pad28_ds1(&self) -> PAD28_DS1_R {
        PAD28_DS1_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 28 - Pad 31 slew rate selection."]
    #[inline(always)]
    pub fn pad31_sr(&mut self) -> PAD31_SR_W {
        PAD31_SR_W { w: self }
    }
    #[doc = "Bit 24 - Pad 31 high order drive strength selection. Used in conjunction with PAD31STRNG field to set the pad drive strength."]
    #[inline(always)]
    pub fn pad31_ds1(&mut self) -> PAD31_DS1_W {
        PAD31_DS1_W { w: self }
    }
    #[doc = "Bit 20 - Pad 30 slew rate selection."]
    #[inline(always)]
    pub fn pad30_sr(&mut self) -> PAD30_SR_W {
        PAD30_SR_W { w: self }
    }
    #[doc = "Bit 16 - Pad 30 high order drive strength selection. Used in conjunction with PAD30STRNG field to set the pad drive strength."]
    #[inline(always)]
    pub fn pad30_ds1(&mut self) -> PAD30_DS1_W {
        PAD30_DS1_W { w: self }
    }
    #[doc = "Bit 12 - Pad 29 slew rate selection."]
    #[inline(always)]
    pub fn pad29_sr(&mut self) -> PAD29_SR_W {
        PAD29_SR_W { w: self }
    }
    #[doc = "Bit 8 - Pad 29 high order drive strength selection. Used in conjunction with PAD29STRNG field to set the pad drive strength."]
    #[inline(always)]
    pub fn pad29_ds1(&mut self) -> PAD29_DS1_W {
        PAD29_DS1_W { w: self }
    }
    #[doc = "Bit 4 - Pad 28 slew rate selection."]
    #[inline(always)]
    pub fn pad28_sr(&mut self) -> PAD28_SR_W {
        PAD28_SR_W { w: self }
    }
    #[doc = "Bit 0 - Pad 28 high order drive strength selection. Used in conjunction with PAD28STRNG field to set the pad drive strength."]
    #[inline(always)]
    pub fn pad28_ds1(&mut self) -> PAD28_DS1_W {
        PAD28_DS1_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Alternate Pad Configuration reg7 (Pads 31,30,29,28)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [altpadcfgh](index.html) module"]
pub struct ALTPADCFGH_SPEC;
impl crate::RegisterSpec for ALTPADCFGH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [altpadcfgh::R](R) reader structure"]
impl crate::Readable for ALTPADCFGH_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [altpadcfgh::W](W) writer structure"]
impl crate::Writable for ALTPADCFGH_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ALTPADCFGH to value 0"]
impl crate::Resettable for ALTPADCFGH_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
