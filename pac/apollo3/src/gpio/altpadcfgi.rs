#[doc = "Register `ALTPADCFGI` reader"]
pub struct R(crate::R<ALTPADCFGI_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ALTPADCFGI_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ALTPADCFGI_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ALTPADCFGI_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ALTPADCFGI` writer"]
pub struct W(crate::W<ALTPADCFGI_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ALTPADCFGI_SPEC>;
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
impl From<crate::W<ALTPADCFGI_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ALTPADCFGI_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Pad 35 slew rate selection.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD35_SR_A {
    #[doc = "1: Enables Slew rate control on pad value."]
    SR_EN = 1,
}
impl From<PAD35_SR_A> for bool {
    #[inline(always)]
    fn from(variant: PAD35_SR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PAD35_SR` reader - Pad 35 slew rate selection."]
pub struct PAD35_SR_R(crate::FieldReader<bool, PAD35_SR_A>);
impl PAD35_SR_R {
    pub(crate) fn new(bits: bool) -> Self {
        PAD35_SR_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PAD35_SR_A> {
        match self.bits {
            true => Some(PAD35_SR_A::SR_EN),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `SR_EN`"]
    #[inline(always)]
    pub fn is_sr_en(&self) -> bool {
        **self == PAD35_SR_A::SR_EN
    }
}
impl core::ops::Deref for PAD35_SR_R {
    type Target = crate::FieldReader<bool, PAD35_SR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PAD35_SR` writer - Pad 35 slew rate selection."]
pub struct PAD35_SR_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD35_SR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD35_SR_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Enables Slew rate control on pad value."]
    #[inline(always)]
    pub fn sr_en(self) -> &'a mut W {
        self.variant(PAD35_SR_A::SR_EN)
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
#[doc = "Field `PAD35_DS1` reader - Pad 35 high order drive strength selection. Used in conjunction with PAD35STRNG field to set the pad drive strength."]
pub struct PAD35_DS1_R(crate::FieldReader<bool, bool>);
impl PAD35_DS1_R {
    pub(crate) fn new(bits: bool) -> Self {
        PAD35_DS1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PAD35_DS1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PAD35_DS1` writer - Pad 35 high order drive strength selection. Used in conjunction with PAD35STRNG field to set the pad drive strength."]
pub struct PAD35_DS1_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD35_DS1_W<'a> {
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
#[doc = "Pad 34 slew rate selection.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD34_SR_A {
    #[doc = "1: Enables Slew rate control on pad value."]
    SR_EN = 1,
}
impl From<PAD34_SR_A> for bool {
    #[inline(always)]
    fn from(variant: PAD34_SR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PAD34_SR` reader - Pad 34 slew rate selection."]
pub struct PAD34_SR_R(crate::FieldReader<bool, PAD34_SR_A>);
impl PAD34_SR_R {
    pub(crate) fn new(bits: bool) -> Self {
        PAD34_SR_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PAD34_SR_A> {
        match self.bits {
            true => Some(PAD34_SR_A::SR_EN),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `SR_EN`"]
    #[inline(always)]
    pub fn is_sr_en(&self) -> bool {
        **self == PAD34_SR_A::SR_EN
    }
}
impl core::ops::Deref for PAD34_SR_R {
    type Target = crate::FieldReader<bool, PAD34_SR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PAD34_SR` writer - Pad 34 slew rate selection."]
pub struct PAD34_SR_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD34_SR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD34_SR_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Enables Slew rate control on pad value."]
    #[inline(always)]
    pub fn sr_en(self) -> &'a mut W {
        self.variant(PAD34_SR_A::SR_EN)
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
#[doc = "Field `PAD34_DS1` reader - Pad 34 high order drive strength selection. Used in conjunction with PAD34STRNG field to set the pad drive strength."]
pub struct PAD34_DS1_R(crate::FieldReader<bool, bool>);
impl PAD34_DS1_R {
    pub(crate) fn new(bits: bool) -> Self {
        PAD34_DS1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PAD34_DS1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PAD34_DS1` writer - Pad 34 high order drive strength selection. Used in conjunction with PAD34STRNG field to set the pad drive strength."]
pub struct PAD34_DS1_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD34_DS1_W<'a> {
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
#[doc = "Pad 33 slew rate selection.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD33_SR_A {
    #[doc = "1: Enables Slew rate control on pad value."]
    SR_EN = 1,
}
impl From<PAD33_SR_A> for bool {
    #[inline(always)]
    fn from(variant: PAD33_SR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PAD33_SR` reader - Pad 33 slew rate selection."]
pub struct PAD33_SR_R(crate::FieldReader<bool, PAD33_SR_A>);
impl PAD33_SR_R {
    pub(crate) fn new(bits: bool) -> Self {
        PAD33_SR_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PAD33_SR_A> {
        match self.bits {
            true => Some(PAD33_SR_A::SR_EN),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `SR_EN`"]
    #[inline(always)]
    pub fn is_sr_en(&self) -> bool {
        **self == PAD33_SR_A::SR_EN
    }
}
impl core::ops::Deref for PAD33_SR_R {
    type Target = crate::FieldReader<bool, PAD33_SR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PAD33_SR` writer - Pad 33 slew rate selection."]
pub struct PAD33_SR_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD33_SR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD33_SR_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Enables Slew rate control on pad value."]
    #[inline(always)]
    pub fn sr_en(self) -> &'a mut W {
        self.variant(PAD33_SR_A::SR_EN)
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
#[doc = "Field `PAD33_DS1` reader - Pad 33 high order drive strength selection. Used in conjunction with PAD33STRNG field to set the pad drive strength."]
pub struct PAD33_DS1_R(crate::FieldReader<bool, bool>);
impl PAD33_DS1_R {
    pub(crate) fn new(bits: bool) -> Self {
        PAD33_DS1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PAD33_DS1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PAD33_DS1` writer - Pad 33 high order drive strength selection. Used in conjunction with PAD33STRNG field to set the pad drive strength."]
pub struct PAD33_DS1_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD33_DS1_W<'a> {
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
#[doc = "Pad 32 slew rate selection.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD32_SR_A {
    #[doc = "1: Enables Slew rate control on pad value."]
    SR_EN = 1,
}
impl From<PAD32_SR_A> for bool {
    #[inline(always)]
    fn from(variant: PAD32_SR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PAD32_SR` reader - Pad 32 slew rate selection."]
pub struct PAD32_SR_R(crate::FieldReader<bool, PAD32_SR_A>);
impl PAD32_SR_R {
    pub(crate) fn new(bits: bool) -> Self {
        PAD32_SR_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PAD32_SR_A> {
        match self.bits {
            true => Some(PAD32_SR_A::SR_EN),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `SR_EN`"]
    #[inline(always)]
    pub fn is_sr_en(&self) -> bool {
        **self == PAD32_SR_A::SR_EN
    }
}
impl core::ops::Deref for PAD32_SR_R {
    type Target = crate::FieldReader<bool, PAD32_SR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PAD32_SR` writer - Pad 32 slew rate selection."]
pub struct PAD32_SR_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD32_SR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD32_SR_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Enables Slew rate control on pad value."]
    #[inline(always)]
    pub fn sr_en(self) -> &'a mut W {
        self.variant(PAD32_SR_A::SR_EN)
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
#[doc = "Field `PAD32_DS1` reader - Pad 32 high order drive strength selection. Used in conjunction with PAD32STRNG field to set the pad drive strength."]
pub struct PAD32_DS1_R(crate::FieldReader<bool, bool>);
impl PAD32_DS1_R {
    pub(crate) fn new(bits: bool) -> Self {
        PAD32_DS1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PAD32_DS1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PAD32_DS1` writer - Pad 32 high order drive strength selection. Used in conjunction with PAD32STRNG field to set the pad drive strength."]
pub struct PAD32_DS1_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD32_DS1_W<'a> {
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
    #[doc = "Bit 28 - Pad 35 slew rate selection."]
    #[inline(always)]
    pub fn pad35_sr(&self) -> PAD35_SR_R {
        PAD35_SR_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Pad 35 high order drive strength selection. Used in conjunction with PAD35STRNG field to set the pad drive strength."]
    #[inline(always)]
    pub fn pad35_ds1(&self) -> PAD35_DS1_R {
        PAD35_DS1_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Pad 34 slew rate selection."]
    #[inline(always)]
    pub fn pad34_sr(&self) -> PAD34_SR_R {
        PAD34_SR_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Pad 34 high order drive strength selection. Used in conjunction with PAD34STRNG field to set the pad drive strength."]
    #[inline(always)]
    pub fn pad34_ds1(&self) -> PAD34_DS1_R {
        PAD34_DS1_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Pad 33 slew rate selection."]
    #[inline(always)]
    pub fn pad33_sr(&self) -> PAD33_SR_R {
        PAD33_SR_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Pad 33 high order drive strength selection. Used in conjunction with PAD33STRNG field to set the pad drive strength."]
    #[inline(always)]
    pub fn pad33_ds1(&self) -> PAD33_DS1_R {
        PAD33_DS1_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Pad 32 slew rate selection."]
    #[inline(always)]
    pub fn pad32_sr(&self) -> PAD32_SR_R {
        PAD32_SR_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Pad 32 high order drive strength selection. Used in conjunction with PAD32STRNG field to set the pad drive strength."]
    #[inline(always)]
    pub fn pad32_ds1(&self) -> PAD32_DS1_R {
        PAD32_DS1_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 28 - Pad 35 slew rate selection."]
    #[inline(always)]
    pub fn pad35_sr(&mut self) -> PAD35_SR_W {
        PAD35_SR_W { w: self }
    }
    #[doc = "Bit 24 - Pad 35 high order drive strength selection. Used in conjunction with PAD35STRNG field to set the pad drive strength."]
    #[inline(always)]
    pub fn pad35_ds1(&mut self) -> PAD35_DS1_W {
        PAD35_DS1_W { w: self }
    }
    #[doc = "Bit 20 - Pad 34 slew rate selection."]
    #[inline(always)]
    pub fn pad34_sr(&mut self) -> PAD34_SR_W {
        PAD34_SR_W { w: self }
    }
    #[doc = "Bit 16 - Pad 34 high order drive strength selection. Used in conjunction with PAD34STRNG field to set the pad drive strength."]
    #[inline(always)]
    pub fn pad34_ds1(&mut self) -> PAD34_DS1_W {
        PAD34_DS1_W { w: self }
    }
    #[doc = "Bit 12 - Pad 33 slew rate selection."]
    #[inline(always)]
    pub fn pad33_sr(&mut self) -> PAD33_SR_W {
        PAD33_SR_W { w: self }
    }
    #[doc = "Bit 8 - Pad 33 high order drive strength selection. Used in conjunction with PAD33STRNG field to set the pad drive strength."]
    #[inline(always)]
    pub fn pad33_ds1(&mut self) -> PAD33_DS1_W {
        PAD33_DS1_W { w: self }
    }
    #[doc = "Bit 4 - Pad 32 slew rate selection."]
    #[inline(always)]
    pub fn pad32_sr(&mut self) -> PAD32_SR_W {
        PAD32_SR_W { w: self }
    }
    #[doc = "Bit 0 - Pad 32 high order drive strength selection. Used in conjunction with PAD32STRNG field to set the pad drive strength."]
    #[inline(always)]
    pub fn pad32_ds1(&mut self) -> PAD32_DS1_W {
        PAD32_DS1_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Alternate Pad Configuration reg8 (Pads 35,34,33,32)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [altpadcfgi](index.html) module"]
pub struct ALTPADCFGI_SPEC;
impl crate::RegisterSpec for ALTPADCFGI_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [altpadcfgi::R](R) reader structure"]
impl crate::Readable for ALTPADCFGI_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [altpadcfgi::W](W) writer structure"]
impl crate::Writable for ALTPADCFGI_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ALTPADCFGI to value 0"]
impl crate::Resettable for ALTPADCFGI_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
