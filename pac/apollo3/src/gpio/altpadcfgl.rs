#[doc = "Register `ALTPADCFGL` reader"]
pub struct R(crate::R<ALTPADCFGL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ALTPADCFGL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ALTPADCFGL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ALTPADCFGL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ALTPADCFGL` writer"]
pub struct W(crate::W<ALTPADCFGL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ALTPADCFGL_SPEC>;
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
impl From<crate::W<ALTPADCFGL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ALTPADCFGL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Pad 47 slew rate selection.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD47_SR_A {
    #[doc = "1: Enables Slew rate control on pad value."]
    SR_EN = 1,
}
impl From<PAD47_SR_A> for bool {
    #[inline(always)]
    fn from(variant: PAD47_SR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PAD47_SR` reader - Pad 47 slew rate selection."]
pub struct PAD47_SR_R(crate::FieldReader<bool, PAD47_SR_A>);
impl PAD47_SR_R {
    pub(crate) fn new(bits: bool) -> Self {
        PAD47_SR_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PAD47_SR_A> {
        match self.bits {
            true => Some(PAD47_SR_A::SR_EN),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `SR_EN`"]
    #[inline(always)]
    pub fn is_sr_en(&self) -> bool {
        **self == PAD47_SR_A::SR_EN
    }
}
impl core::ops::Deref for PAD47_SR_R {
    type Target = crate::FieldReader<bool, PAD47_SR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PAD47_SR` writer - Pad 47 slew rate selection."]
pub struct PAD47_SR_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD47_SR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD47_SR_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Enables Slew rate control on pad value."]
    #[inline(always)]
    pub fn sr_en(self) -> &'a mut W {
        self.variant(PAD47_SR_A::SR_EN)
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
#[doc = "Field `PAD47_DS1` reader - Pad 47 high order drive strength selection. Used in conjunction with PAD47STRNG field to set the pad drive strength."]
pub struct PAD47_DS1_R(crate::FieldReader<bool, bool>);
impl PAD47_DS1_R {
    pub(crate) fn new(bits: bool) -> Self {
        PAD47_DS1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PAD47_DS1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PAD47_DS1` writer - Pad 47 high order drive strength selection. Used in conjunction with PAD47STRNG field to set the pad drive strength."]
pub struct PAD47_DS1_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD47_DS1_W<'a> {
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
#[doc = "Pad 46 slew rate selection.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD46_SR_A {
    #[doc = "1: Enables Slew rate control on pad value."]
    SR_EN = 1,
}
impl From<PAD46_SR_A> for bool {
    #[inline(always)]
    fn from(variant: PAD46_SR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PAD46_SR` reader - Pad 46 slew rate selection."]
pub struct PAD46_SR_R(crate::FieldReader<bool, PAD46_SR_A>);
impl PAD46_SR_R {
    pub(crate) fn new(bits: bool) -> Self {
        PAD46_SR_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PAD46_SR_A> {
        match self.bits {
            true => Some(PAD46_SR_A::SR_EN),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `SR_EN`"]
    #[inline(always)]
    pub fn is_sr_en(&self) -> bool {
        **self == PAD46_SR_A::SR_EN
    }
}
impl core::ops::Deref for PAD46_SR_R {
    type Target = crate::FieldReader<bool, PAD46_SR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PAD46_SR` writer - Pad 46 slew rate selection."]
pub struct PAD46_SR_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD46_SR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD46_SR_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Enables Slew rate control on pad value."]
    #[inline(always)]
    pub fn sr_en(self) -> &'a mut W {
        self.variant(PAD46_SR_A::SR_EN)
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
#[doc = "Field `PAD46_DS1` reader - Pad 46 high order drive strength selection. Used in conjunction with PAD46STRNG field to set the pad drive strength."]
pub struct PAD46_DS1_R(crate::FieldReader<bool, bool>);
impl PAD46_DS1_R {
    pub(crate) fn new(bits: bool) -> Self {
        PAD46_DS1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PAD46_DS1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PAD46_DS1` writer - Pad 46 high order drive strength selection. Used in conjunction with PAD46STRNG field to set the pad drive strength."]
pub struct PAD46_DS1_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD46_DS1_W<'a> {
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
#[doc = "Pad 45 slew rate selection.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD45_SR_A {
    #[doc = "1: Enables Slew rate control on pad value."]
    SR_EN = 1,
}
impl From<PAD45_SR_A> for bool {
    #[inline(always)]
    fn from(variant: PAD45_SR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PAD45_SR` reader - Pad 45 slew rate selection."]
pub struct PAD45_SR_R(crate::FieldReader<bool, PAD45_SR_A>);
impl PAD45_SR_R {
    pub(crate) fn new(bits: bool) -> Self {
        PAD45_SR_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PAD45_SR_A> {
        match self.bits {
            true => Some(PAD45_SR_A::SR_EN),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `SR_EN`"]
    #[inline(always)]
    pub fn is_sr_en(&self) -> bool {
        **self == PAD45_SR_A::SR_EN
    }
}
impl core::ops::Deref for PAD45_SR_R {
    type Target = crate::FieldReader<bool, PAD45_SR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PAD45_SR` writer - Pad 45 slew rate selection."]
pub struct PAD45_SR_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD45_SR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD45_SR_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Enables Slew rate control on pad value."]
    #[inline(always)]
    pub fn sr_en(self) -> &'a mut W {
        self.variant(PAD45_SR_A::SR_EN)
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
#[doc = "Field `PAD45_DS1` reader - Pad 45 high order drive strength selection. Used in conjunction with PAD45STRNG field to set the pad drive strength."]
pub struct PAD45_DS1_R(crate::FieldReader<bool, bool>);
impl PAD45_DS1_R {
    pub(crate) fn new(bits: bool) -> Self {
        PAD45_DS1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PAD45_DS1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PAD45_DS1` writer - Pad 45 high order drive strength selection. Used in conjunction with PAD45STRNG field to set the pad drive strength."]
pub struct PAD45_DS1_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD45_DS1_W<'a> {
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
#[doc = "Pad 44 slew rate selection.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD44_SR_A {
    #[doc = "1: Enables Slew rate control on pad value."]
    SR_EN = 1,
}
impl From<PAD44_SR_A> for bool {
    #[inline(always)]
    fn from(variant: PAD44_SR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PAD44_SR` reader - Pad 44 slew rate selection."]
pub struct PAD44_SR_R(crate::FieldReader<bool, PAD44_SR_A>);
impl PAD44_SR_R {
    pub(crate) fn new(bits: bool) -> Self {
        PAD44_SR_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PAD44_SR_A> {
        match self.bits {
            true => Some(PAD44_SR_A::SR_EN),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `SR_EN`"]
    #[inline(always)]
    pub fn is_sr_en(&self) -> bool {
        **self == PAD44_SR_A::SR_EN
    }
}
impl core::ops::Deref for PAD44_SR_R {
    type Target = crate::FieldReader<bool, PAD44_SR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PAD44_SR` writer - Pad 44 slew rate selection."]
pub struct PAD44_SR_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD44_SR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD44_SR_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Enables Slew rate control on pad value."]
    #[inline(always)]
    pub fn sr_en(self) -> &'a mut W {
        self.variant(PAD44_SR_A::SR_EN)
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
#[doc = "Field `PAD44_DS1` reader - Pad 44 high order drive strength selection. Used in conjunction with PAD44STRNG field to set the pad drive strength."]
pub struct PAD44_DS1_R(crate::FieldReader<bool, bool>);
impl PAD44_DS1_R {
    pub(crate) fn new(bits: bool) -> Self {
        PAD44_DS1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PAD44_DS1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PAD44_DS1` writer - Pad 44 high order drive strength selection. Used in conjunction with PAD44STRNG field to set the pad drive strength."]
pub struct PAD44_DS1_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD44_DS1_W<'a> {
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
    #[doc = "Bit 28 - Pad 47 slew rate selection."]
    #[inline(always)]
    pub fn pad47_sr(&self) -> PAD47_SR_R {
        PAD47_SR_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Pad 47 high order drive strength selection. Used in conjunction with PAD47STRNG field to set the pad drive strength."]
    #[inline(always)]
    pub fn pad47_ds1(&self) -> PAD47_DS1_R {
        PAD47_DS1_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Pad 46 slew rate selection."]
    #[inline(always)]
    pub fn pad46_sr(&self) -> PAD46_SR_R {
        PAD46_SR_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Pad 46 high order drive strength selection. Used in conjunction with PAD46STRNG field to set the pad drive strength."]
    #[inline(always)]
    pub fn pad46_ds1(&self) -> PAD46_DS1_R {
        PAD46_DS1_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Pad 45 slew rate selection."]
    #[inline(always)]
    pub fn pad45_sr(&self) -> PAD45_SR_R {
        PAD45_SR_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Pad 45 high order drive strength selection. Used in conjunction with PAD45STRNG field to set the pad drive strength."]
    #[inline(always)]
    pub fn pad45_ds1(&self) -> PAD45_DS1_R {
        PAD45_DS1_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Pad 44 slew rate selection."]
    #[inline(always)]
    pub fn pad44_sr(&self) -> PAD44_SR_R {
        PAD44_SR_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Pad 44 high order drive strength selection. Used in conjunction with PAD44STRNG field to set the pad drive strength."]
    #[inline(always)]
    pub fn pad44_ds1(&self) -> PAD44_DS1_R {
        PAD44_DS1_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 28 - Pad 47 slew rate selection."]
    #[inline(always)]
    pub fn pad47_sr(&mut self) -> PAD47_SR_W {
        PAD47_SR_W { w: self }
    }
    #[doc = "Bit 24 - Pad 47 high order drive strength selection. Used in conjunction with PAD47STRNG field to set the pad drive strength."]
    #[inline(always)]
    pub fn pad47_ds1(&mut self) -> PAD47_DS1_W {
        PAD47_DS1_W { w: self }
    }
    #[doc = "Bit 20 - Pad 46 slew rate selection."]
    #[inline(always)]
    pub fn pad46_sr(&mut self) -> PAD46_SR_W {
        PAD46_SR_W { w: self }
    }
    #[doc = "Bit 16 - Pad 46 high order drive strength selection. Used in conjunction with PAD46STRNG field to set the pad drive strength."]
    #[inline(always)]
    pub fn pad46_ds1(&mut self) -> PAD46_DS1_W {
        PAD46_DS1_W { w: self }
    }
    #[doc = "Bit 12 - Pad 45 slew rate selection."]
    #[inline(always)]
    pub fn pad45_sr(&mut self) -> PAD45_SR_W {
        PAD45_SR_W { w: self }
    }
    #[doc = "Bit 8 - Pad 45 high order drive strength selection. Used in conjunction with PAD45STRNG field to set the pad drive strength."]
    #[inline(always)]
    pub fn pad45_ds1(&mut self) -> PAD45_DS1_W {
        PAD45_DS1_W { w: self }
    }
    #[doc = "Bit 4 - Pad 44 slew rate selection."]
    #[inline(always)]
    pub fn pad44_sr(&mut self) -> PAD44_SR_W {
        PAD44_SR_W { w: self }
    }
    #[doc = "Bit 0 - Pad 44 high order drive strength selection. Used in conjunction with PAD44STRNG field to set the pad drive strength."]
    #[inline(always)]
    pub fn pad44_ds1(&mut self) -> PAD44_DS1_W {
        PAD44_DS1_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Alternate Pad Configuration reg11 (Pads 47,46,45,44)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [altpadcfgl](index.html) module"]
pub struct ALTPADCFGL_SPEC;
impl crate::RegisterSpec for ALTPADCFGL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [altpadcfgl::R](R) reader structure"]
impl crate::Readable for ALTPADCFGL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [altpadcfgl::W](W) writer structure"]
impl crate::Writable for ALTPADCFGL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ALTPADCFGL to value 0"]
impl crate::Resettable for ALTPADCFGL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
