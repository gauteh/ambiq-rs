#[doc = "Register `ALTPADCFGG` reader"]
pub struct R(crate::R<ALTPADCFGG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ALTPADCFGG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ALTPADCFGG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ALTPADCFGG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ALTPADCFGG` writer"]
pub struct W(crate::W<ALTPADCFGG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ALTPADCFGG_SPEC>;
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
impl From<crate::W<ALTPADCFGG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ALTPADCFGG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Pad 27 slew rate selection.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD27_SR_A {
    #[doc = "1: Enables Slew rate control on pad value."]
    SR_EN = 1,
}
impl From<PAD27_SR_A> for bool {
    #[inline(always)]
    fn from(variant: PAD27_SR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PAD27_SR` reader - Pad 27 slew rate selection."]
pub struct PAD27_SR_R(crate::FieldReader<bool, PAD27_SR_A>);
impl PAD27_SR_R {
    pub(crate) fn new(bits: bool) -> Self {
        PAD27_SR_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PAD27_SR_A> {
        match self.bits {
            true => Some(PAD27_SR_A::SR_EN),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `SR_EN`"]
    #[inline(always)]
    pub fn is_sr_en(&self) -> bool {
        **self == PAD27_SR_A::SR_EN
    }
}
impl core::ops::Deref for PAD27_SR_R {
    type Target = crate::FieldReader<bool, PAD27_SR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PAD27_SR` writer - Pad 27 slew rate selection."]
pub struct PAD27_SR_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD27_SR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD27_SR_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Enables Slew rate control on pad value."]
    #[inline(always)]
    pub fn sr_en(self) -> &'a mut W {
        self.variant(PAD27_SR_A::SR_EN)
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
#[doc = "Field `PAD27_DS1` reader - Pad 27 high order drive strength selection. Used in conjunction with PAD27STRNG field to set the pad drive strength."]
pub struct PAD27_DS1_R(crate::FieldReader<bool, bool>);
impl PAD27_DS1_R {
    pub(crate) fn new(bits: bool) -> Self {
        PAD27_DS1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PAD27_DS1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PAD27_DS1` writer - Pad 27 high order drive strength selection. Used in conjunction with PAD27STRNG field to set the pad drive strength."]
pub struct PAD27_DS1_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD27_DS1_W<'a> {
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
#[doc = "Pad 26 slew rate selection.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD26_SR_A {
    #[doc = "1: Enables Slew rate control on pad value."]
    SR_EN = 1,
}
impl From<PAD26_SR_A> for bool {
    #[inline(always)]
    fn from(variant: PAD26_SR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PAD26_SR` reader - Pad 26 slew rate selection."]
pub struct PAD26_SR_R(crate::FieldReader<bool, PAD26_SR_A>);
impl PAD26_SR_R {
    pub(crate) fn new(bits: bool) -> Self {
        PAD26_SR_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PAD26_SR_A> {
        match self.bits {
            true => Some(PAD26_SR_A::SR_EN),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `SR_EN`"]
    #[inline(always)]
    pub fn is_sr_en(&self) -> bool {
        **self == PAD26_SR_A::SR_EN
    }
}
impl core::ops::Deref for PAD26_SR_R {
    type Target = crate::FieldReader<bool, PAD26_SR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PAD26_SR` writer - Pad 26 slew rate selection."]
pub struct PAD26_SR_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD26_SR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD26_SR_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Enables Slew rate control on pad value."]
    #[inline(always)]
    pub fn sr_en(self) -> &'a mut W {
        self.variant(PAD26_SR_A::SR_EN)
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
#[doc = "Field `PAD26_DS1` reader - Pad 26 high order drive strength selection. Used in conjunction with PAD26STRNG field to set the pad drive strength."]
pub struct PAD26_DS1_R(crate::FieldReader<bool, bool>);
impl PAD26_DS1_R {
    pub(crate) fn new(bits: bool) -> Self {
        PAD26_DS1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PAD26_DS1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PAD26_DS1` writer - Pad 26 high order drive strength selection. Used in conjunction with PAD26STRNG field to set the pad drive strength."]
pub struct PAD26_DS1_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD26_DS1_W<'a> {
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
#[doc = "Pad 25 slew rate selection.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD25_SR_A {
    #[doc = "1: Enables Slew rate control on pad value."]
    SR_EN = 1,
}
impl From<PAD25_SR_A> for bool {
    #[inline(always)]
    fn from(variant: PAD25_SR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PAD25_SR` reader - Pad 25 slew rate selection."]
pub struct PAD25_SR_R(crate::FieldReader<bool, PAD25_SR_A>);
impl PAD25_SR_R {
    pub(crate) fn new(bits: bool) -> Self {
        PAD25_SR_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PAD25_SR_A> {
        match self.bits {
            true => Some(PAD25_SR_A::SR_EN),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `SR_EN`"]
    #[inline(always)]
    pub fn is_sr_en(&self) -> bool {
        **self == PAD25_SR_A::SR_EN
    }
}
impl core::ops::Deref for PAD25_SR_R {
    type Target = crate::FieldReader<bool, PAD25_SR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PAD25_SR` writer - Pad 25 slew rate selection."]
pub struct PAD25_SR_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD25_SR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD25_SR_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Enables Slew rate control on pad value."]
    #[inline(always)]
    pub fn sr_en(self) -> &'a mut W {
        self.variant(PAD25_SR_A::SR_EN)
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
#[doc = "Field `PAD25_DS1` reader - Pad 25 high order drive strength selection. Used in conjunction with PAD25STRNG field to set the pad drive strength."]
pub struct PAD25_DS1_R(crate::FieldReader<bool, bool>);
impl PAD25_DS1_R {
    pub(crate) fn new(bits: bool) -> Self {
        PAD25_DS1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PAD25_DS1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PAD25_DS1` writer - Pad 25 high order drive strength selection. Used in conjunction with PAD25STRNG field to set the pad drive strength."]
pub struct PAD25_DS1_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD25_DS1_W<'a> {
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
#[doc = "Pad 24 slew rate selection.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD24_SR_A {
    #[doc = "1: Enables Slew rate control on pad value."]
    SR_EN = 1,
}
impl From<PAD24_SR_A> for bool {
    #[inline(always)]
    fn from(variant: PAD24_SR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PAD24_SR` reader - Pad 24 slew rate selection."]
pub struct PAD24_SR_R(crate::FieldReader<bool, PAD24_SR_A>);
impl PAD24_SR_R {
    pub(crate) fn new(bits: bool) -> Self {
        PAD24_SR_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PAD24_SR_A> {
        match self.bits {
            true => Some(PAD24_SR_A::SR_EN),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `SR_EN`"]
    #[inline(always)]
    pub fn is_sr_en(&self) -> bool {
        **self == PAD24_SR_A::SR_EN
    }
}
impl core::ops::Deref for PAD24_SR_R {
    type Target = crate::FieldReader<bool, PAD24_SR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PAD24_SR` writer - Pad 24 slew rate selection."]
pub struct PAD24_SR_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD24_SR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD24_SR_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Enables Slew rate control on pad value."]
    #[inline(always)]
    pub fn sr_en(self) -> &'a mut W {
        self.variant(PAD24_SR_A::SR_EN)
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
#[doc = "Field `PAD24_DS1` reader - Pad 24 high order drive strength selection. Used in conjunction with PAD24STRNG field to set the pad drive strength."]
pub struct PAD24_DS1_R(crate::FieldReader<bool, bool>);
impl PAD24_DS1_R {
    pub(crate) fn new(bits: bool) -> Self {
        PAD24_DS1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PAD24_DS1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PAD24_DS1` writer - Pad 24 high order drive strength selection. Used in conjunction with PAD24STRNG field to set the pad drive strength."]
pub struct PAD24_DS1_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD24_DS1_W<'a> {
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
    #[doc = "Bit 28 - Pad 27 slew rate selection."]
    #[inline(always)]
    pub fn pad27_sr(&self) -> PAD27_SR_R {
        PAD27_SR_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Pad 27 high order drive strength selection. Used in conjunction with PAD27STRNG field to set the pad drive strength."]
    #[inline(always)]
    pub fn pad27_ds1(&self) -> PAD27_DS1_R {
        PAD27_DS1_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Pad 26 slew rate selection."]
    #[inline(always)]
    pub fn pad26_sr(&self) -> PAD26_SR_R {
        PAD26_SR_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Pad 26 high order drive strength selection. Used in conjunction with PAD26STRNG field to set the pad drive strength."]
    #[inline(always)]
    pub fn pad26_ds1(&self) -> PAD26_DS1_R {
        PAD26_DS1_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Pad 25 slew rate selection."]
    #[inline(always)]
    pub fn pad25_sr(&self) -> PAD25_SR_R {
        PAD25_SR_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Pad 25 high order drive strength selection. Used in conjunction with PAD25STRNG field to set the pad drive strength."]
    #[inline(always)]
    pub fn pad25_ds1(&self) -> PAD25_DS1_R {
        PAD25_DS1_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Pad 24 slew rate selection."]
    #[inline(always)]
    pub fn pad24_sr(&self) -> PAD24_SR_R {
        PAD24_SR_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Pad 24 high order drive strength selection. Used in conjunction with PAD24STRNG field to set the pad drive strength."]
    #[inline(always)]
    pub fn pad24_ds1(&self) -> PAD24_DS1_R {
        PAD24_DS1_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 28 - Pad 27 slew rate selection."]
    #[inline(always)]
    pub fn pad27_sr(&mut self) -> PAD27_SR_W {
        PAD27_SR_W { w: self }
    }
    #[doc = "Bit 24 - Pad 27 high order drive strength selection. Used in conjunction with PAD27STRNG field to set the pad drive strength."]
    #[inline(always)]
    pub fn pad27_ds1(&mut self) -> PAD27_DS1_W {
        PAD27_DS1_W { w: self }
    }
    #[doc = "Bit 20 - Pad 26 slew rate selection."]
    #[inline(always)]
    pub fn pad26_sr(&mut self) -> PAD26_SR_W {
        PAD26_SR_W { w: self }
    }
    #[doc = "Bit 16 - Pad 26 high order drive strength selection. Used in conjunction with PAD26STRNG field to set the pad drive strength."]
    #[inline(always)]
    pub fn pad26_ds1(&mut self) -> PAD26_DS1_W {
        PAD26_DS1_W { w: self }
    }
    #[doc = "Bit 12 - Pad 25 slew rate selection."]
    #[inline(always)]
    pub fn pad25_sr(&mut self) -> PAD25_SR_W {
        PAD25_SR_W { w: self }
    }
    #[doc = "Bit 8 - Pad 25 high order drive strength selection. Used in conjunction with PAD25STRNG field to set the pad drive strength."]
    #[inline(always)]
    pub fn pad25_ds1(&mut self) -> PAD25_DS1_W {
        PAD25_DS1_W { w: self }
    }
    #[doc = "Bit 4 - Pad 24 slew rate selection."]
    #[inline(always)]
    pub fn pad24_sr(&mut self) -> PAD24_SR_W {
        PAD24_SR_W { w: self }
    }
    #[doc = "Bit 0 - Pad 24 high order drive strength selection. Used in conjunction with PAD24STRNG field to set the pad drive strength."]
    #[inline(always)]
    pub fn pad24_ds1(&mut self) -> PAD24_DS1_W {
        PAD24_DS1_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Alternate Pad Configuration reg6 (Pads 27,26,25,24)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [altpadcfgg](index.html) module"]
pub struct ALTPADCFGG_SPEC;
impl crate::RegisterSpec for ALTPADCFGG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [altpadcfgg::R](R) reader structure"]
impl crate::Readable for ALTPADCFGG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [altpadcfgg::W](W) writer structure"]
impl crate::Writable for ALTPADCFGG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ALTPADCFGG to value 0"]
impl crate::Resettable for ALTPADCFGG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
