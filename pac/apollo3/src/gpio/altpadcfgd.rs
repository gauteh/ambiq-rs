#[doc = "Register `ALTPADCFGD` reader"]
pub struct R(crate::R<ALTPADCFGD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ALTPADCFGD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ALTPADCFGD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ALTPADCFGD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ALTPADCFGD` writer"]
pub struct W(crate::W<ALTPADCFGD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ALTPADCFGD_SPEC>;
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
impl From<crate::W<ALTPADCFGD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ALTPADCFGD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Pad 15 slew rate selection.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD15_SR_A {
    #[doc = "1: Enables Slew rate control on pad value."]
    SR_EN = 1,
}
impl From<PAD15_SR_A> for bool {
    #[inline(always)]
    fn from(variant: PAD15_SR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PAD15_SR` reader - Pad 15 slew rate selection."]
pub struct PAD15_SR_R(crate::FieldReader<bool, PAD15_SR_A>);
impl PAD15_SR_R {
    pub(crate) fn new(bits: bool) -> Self {
        PAD15_SR_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PAD15_SR_A> {
        match self.bits {
            true => Some(PAD15_SR_A::SR_EN),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `SR_EN`"]
    #[inline(always)]
    pub fn is_sr_en(&self) -> bool {
        **self == PAD15_SR_A::SR_EN
    }
}
impl core::ops::Deref for PAD15_SR_R {
    type Target = crate::FieldReader<bool, PAD15_SR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PAD15_SR` writer - Pad 15 slew rate selection."]
pub struct PAD15_SR_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD15_SR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD15_SR_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Enables Slew rate control on pad value."]
    #[inline(always)]
    pub fn sr_en(self) -> &'a mut W {
        self.variant(PAD15_SR_A::SR_EN)
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
#[doc = "Field `PAD15_DS1` reader - Pad 15 high order drive strength selection. Used in conjunction with PAD15STRNG field to set the pad drive strength."]
pub struct PAD15_DS1_R(crate::FieldReader<bool, bool>);
impl PAD15_DS1_R {
    pub(crate) fn new(bits: bool) -> Self {
        PAD15_DS1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PAD15_DS1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PAD15_DS1` writer - Pad 15 high order drive strength selection. Used in conjunction with PAD15STRNG field to set the pad drive strength."]
pub struct PAD15_DS1_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD15_DS1_W<'a> {
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
#[doc = "Pad 14 slew rate selection.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD14_SR_A {
    #[doc = "1: Enables Slew rate control on pad value."]
    SR_EN = 1,
}
impl From<PAD14_SR_A> for bool {
    #[inline(always)]
    fn from(variant: PAD14_SR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PAD14_SR` reader - Pad 14 slew rate selection."]
pub struct PAD14_SR_R(crate::FieldReader<bool, PAD14_SR_A>);
impl PAD14_SR_R {
    pub(crate) fn new(bits: bool) -> Self {
        PAD14_SR_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PAD14_SR_A> {
        match self.bits {
            true => Some(PAD14_SR_A::SR_EN),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `SR_EN`"]
    #[inline(always)]
    pub fn is_sr_en(&self) -> bool {
        **self == PAD14_SR_A::SR_EN
    }
}
impl core::ops::Deref for PAD14_SR_R {
    type Target = crate::FieldReader<bool, PAD14_SR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PAD14_SR` writer - Pad 14 slew rate selection."]
pub struct PAD14_SR_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD14_SR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD14_SR_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Enables Slew rate control on pad value."]
    #[inline(always)]
    pub fn sr_en(self) -> &'a mut W {
        self.variant(PAD14_SR_A::SR_EN)
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
#[doc = "Field `PAD14_DS1` reader - Pad 14 high order drive strength selection. Used in conjunction with PAD14STRNG field to set the pad drive strength."]
pub struct PAD14_DS1_R(crate::FieldReader<bool, bool>);
impl PAD14_DS1_R {
    pub(crate) fn new(bits: bool) -> Self {
        PAD14_DS1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PAD14_DS1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PAD14_DS1` writer - Pad 14 high order drive strength selection. Used in conjunction with PAD14STRNG field to set the pad drive strength."]
pub struct PAD14_DS1_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD14_DS1_W<'a> {
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
#[doc = "Pad 13 slew rate selection.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD13_SR_A {
    #[doc = "1: Enables Slew rate control on pad value."]
    SR_EN = 1,
}
impl From<PAD13_SR_A> for bool {
    #[inline(always)]
    fn from(variant: PAD13_SR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PAD13_SR` reader - Pad 13 slew rate selection."]
pub struct PAD13_SR_R(crate::FieldReader<bool, PAD13_SR_A>);
impl PAD13_SR_R {
    pub(crate) fn new(bits: bool) -> Self {
        PAD13_SR_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PAD13_SR_A> {
        match self.bits {
            true => Some(PAD13_SR_A::SR_EN),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `SR_EN`"]
    #[inline(always)]
    pub fn is_sr_en(&self) -> bool {
        **self == PAD13_SR_A::SR_EN
    }
}
impl core::ops::Deref for PAD13_SR_R {
    type Target = crate::FieldReader<bool, PAD13_SR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PAD13_SR` writer - Pad 13 slew rate selection."]
pub struct PAD13_SR_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD13_SR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD13_SR_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Enables Slew rate control on pad value."]
    #[inline(always)]
    pub fn sr_en(self) -> &'a mut W {
        self.variant(PAD13_SR_A::SR_EN)
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
#[doc = "Field `PAD13_DS1` reader - Pad 13 high order drive strength selection. Used in conjunction with PAD13STRNG field to set the pad drive strength."]
pub struct PAD13_DS1_R(crate::FieldReader<bool, bool>);
impl PAD13_DS1_R {
    pub(crate) fn new(bits: bool) -> Self {
        PAD13_DS1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PAD13_DS1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PAD13_DS1` writer - Pad 13 high order drive strength selection. Used in conjunction with PAD13STRNG field to set the pad drive strength."]
pub struct PAD13_DS1_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD13_DS1_W<'a> {
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
#[doc = "Pad 12 slew rate selection.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD12_SR_A {
    #[doc = "1: Enables Slew rate control on pad value."]
    SR_EN = 1,
}
impl From<PAD12_SR_A> for bool {
    #[inline(always)]
    fn from(variant: PAD12_SR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PAD12_SR` reader - Pad 12 slew rate selection."]
pub struct PAD12_SR_R(crate::FieldReader<bool, PAD12_SR_A>);
impl PAD12_SR_R {
    pub(crate) fn new(bits: bool) -> Self {
        PAD12_SR_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PAD12_SR_A> {
        match self.bits {
            true => Some(PAD12_SR_A::SR_EN),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `SR_EN`"]
    #[inline(always)]
    pub fn is_sr_en(&self) -> bool {
        **self == PAD12_SR_A::SR_EN
    }
}
impl core::ops::Deref for PAD12_SR_R {
    type Target = crate::FieldReader<bool, PAD12_SR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PAD12_SR` writer - Pad 12 slew rate selection."]
pub struct PAD12_SR_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD12_SR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD12_SR_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Enables Slew rate control on pad value."]
    #[inline(always)]
    pub fn sr_en(self) -> &'a mut W {
        self.variant(PAD12_SR_A::SR_EN)
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
#[doc = "Field `PAD12_DS1` reader - Pad 12 high order drive strength selection. Used in conjunction with PAD12STRNG field to set the pad drive strength."]
pub struct PAD12_DS1_R(crate::FieldReader<bool, bool>);
impl PAD12_DS1_R {
    pub(crate) fn new(bits: bool) -> Self {
        PAD12_DS1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PAD12_DS1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PAD12_DS1` writer - Pad 12 high order drive strength selection. Used in conjunction with PAD12STRNG field to set the pad drive strength."]
pub struct PAD12_DS1_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD12_DS1_W<'a> {
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
    #[doc = "Bit 28 - Pad 15 slew rate selection."]
    #[inline(always)]
    pub fn pad15_sr(&self) -> PAD15_SR_R {
        PAD15_SR_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Pad 15 high order drive strength selection. Used in conjunction with PAD15STRNG field to set the pad drive strength."]
    #[inline(always)]
    pub fn pad15_ds1(&self) -> PAD15_DS1_R {
        PAD15_DS1_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Pad 14 slew rate selection."]
    #[inline(always)]
    pub fn pad14_sr(&self) -> PAD14_SR_R {
        PAD14_SR_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Pad 14 high order drive strength selection. Used in conjunction with PAD14STRNG field to set the pad drive strength."]
    #[inline(always)]
    pub fn pad14_ds1(&self) -> PAD14_DS1_R {
        PAD14_DS1_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Pad 13 slew rate selection."]
    #[inline(always)]
    pub fn pad13_sr(&self) -> PAD13_SR_R {
        PAD13_SR_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Pad 13 high order drive strength selection. Used in conjunction with PAD13STRNG field to set the pad drive strength."]
    #[inline(always)]
    pub fn pad13_ds1(&self) -> PAD13_DS1_R {
        PAD13_DS1_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Pad 12 slew rate selection."]
    #[inline(always)]
    pub fn pad12_sr(&self) -> PAD12_SR_R {
        PAD12_SR_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Pad 12 high order drive strength selection. Used in conjunction with PAD12STRNG field to set the pad drive strength."]
    #[inline(always)]
    pub fn pad12_ds1(&self) -> PAD12_DS1_R {
        PAD12_DS1_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 28 - Pad 15 slew rate selection."]
    #[inline(always)]
    pub fn pad15_sr(&mut self) -> PAD15_SR_W {
        PAD15_SR_W { w: self }
    }
    #[doc = "Bit 24 - Pad 15 high order drive strength selection. Used in conjunction with PAD15STRNG field to set the pad drive strength."]
    #[inline(always)]
    pub fn pad15_ds1(&mut self) -> PAD15_DS1_W {
        PAD15_DS1_W { w: self }
    }
    #[doc = "Bit 20 - Pad 14 slew rate selection."]
    #[inline(always)]
    pub fn pad14_sr(&mut self) -> PAD14_SR_W {
        PAD14_SR_W { w: self }
    }
    #[doc = "Bit 16 - Pad 14 high order drive strength selection. Used in conjunction with PAD14STRNG field to set the pad drive strength."]
    #[inline(always)]
    pub fn pad14_ds1(&mut self) -> PAD14_DS1_W {
        PAD14_DS1_W { w: self }
    }
    #[doc = "Bit 12 - Pad 13 slew rate selection."]
    #[inline(always)]
    pub fn pad13_sr(&mut self) -> PAD13_SR_W {
        PAD13_SR_W { w: self }
    }
    #[doc = "Bit 8 - Pad 13 high order drive strength selection. Used in conjunction with PAD13STRNG field to set the pad drive strength."]
    #[inline(always)]
    pub fn pad13_ds1(&mut self) -> PAD13_DS1_W {
        PAD13_DS1_W { w: self }
    }
    #[doc = "Bit 4 - Pad 12 slew rate selection."]
    #[inline(always)]
    pub fn pad12_sr(&mut self) -> PAD12_SR_W {
        PAD12_SR_W { w: self }
    }
    #[doc = "Bit 0 - Pad 12 high order drive strength selection. Used in conjunction with PAD12STRNG field to set the pad drive strength."]
    #[inline(always)]
    pub fn pad12_ds1(&mut self) -> PAD12_DS1_W {
        PAD12_DS1_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Alternate Pad Configuration reg3 (Pads 15,14,13,12)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [altpadcfgd](index.html) module"]
pub struct ALTPADCFGD_SPEC;
impl crate::RegisterSpec for ALTPADCFGD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [altpadcfgd::R](R) reader structure"]
impl crate::Readable for ALTPADCFGD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [altpadcfgd::W](W) writer structure"]
impl crate::Writable for ALTPADCFGD_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ALTPADCFGD to value 0"]
impl crate::Resettable for ALTPADCFGD_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
