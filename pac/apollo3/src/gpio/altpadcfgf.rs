#[doc = "Register `ALTPADCFGF` reader"]
pub struct R(crate::R<ALTPADCFGF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ALTPADCFGF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ALTPADCFGF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ALTPADCFGF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ALTPADCFGF` writer"]
pub struct W(crate::W<ALTPADCFGF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ALTPADCFGF_SPEC>;
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
impl From<crate::W<ALTPADCFGF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ALTPADCFGF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Pad 23 slew rate selection.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD23_SR_A {
    #[doc = "1: Enables Slew rate control on pad value."]
    SR_EN = 1,
}
impl From<PAD23_SR_A> for bool {
    #[inline(always)]
    fn from(variant: PAD23_SR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PAD23_SR` reader - Pad 23 slew rate selection."]
pub struct PAD23_SR_R(crate::FieldReader<bool, PAD23_SR_A>);
impl PAD23_SR_R {
    pub(crate) fn new(bits: bool) -> Self {
        PAD23_SR_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PAD23_SR_A> {
        match self.bits {
            true => Some(PAD23_SR_A::SR_EN),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `SR_EN`"]
    #[inline(always)]
    pub fn is_sr_en(&self) -> bool {
        **self == PAD23_SR_A::SR_EN
    }
}
impl core::ops::Deref for PAD23_SR_R {
    type Target = crate::FieldReader<bool, PAD23_SR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PAD23_SR` writer - Pad 23 slew rate selection."]
pub struct PAD23_SR_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD23_SR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD23_SR_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Enables Slew rate control on pad value."]
    #[inline(always)]
    pub fn sr_en(self) -> &'a mut W {
        self.variant(PAD23_SR_A::SR_EN)
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
#[doc = "Field `PAD23_DS1` reader - Pad 23 high order drive strength selection. Used in conjunction with PAD23STRNG field to set the pad drive strength."]
pub struct PAD23_DS1_R(crate::FieldReader<bool, bool>);
impl PAD23_DS1_R {
    pub(crate) fn new(bits: bool) -> Self {
        PAD23_DS1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PAD23_DS1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PAD23_DS1` writer - Pad 23 high order drive strength selection. Used in conjunction with PAD23STRNG field to set the pad drive strength."]
pub struct PAD23_DS1_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD23_DS1_W<'a> {
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
#[doc = "Pad 22 slew rate selection.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD22_SR_A {
    #[doc = "1: Enables Slew rate control on pad value."]
    SR_EN = 1,
}
impl From<PAD22_SR_A> for bool {
    #[inline(always)]
    fn from(variant: PAD22_SR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PAD22_SR` reader - Pad 22 slew rate selection."]
pub struct PAD22_SR_R(crate::FieldReader<bool, PAD22_SR_A>);
impl PAD22_SR_R {
    pub(crate) fn new(bits: bool) -> Self {
        PAD22_SR_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PAD22_SR_A> {
        match self.bits {
            true => Some(PAD22_SR_A::SR_EN),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `SR_EN`"]
    #[inline(always)]
    pub fn is_sr_en(&self) -> bool {
        **self == PAD22_SR_A::SR_EN
    }
}
impl core::ops::Deref for PAD22_SR_R {
    type Target = crate::FieldReader<bool, PAD22_SR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PAD22_SR` writer - Pad 22 slew rate selection."]
pub struct PAD22_SR_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD22_SR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD22_SR_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Enables Slew rate control on pad value."]
    #[inline(always)]
    pub fn sr_en(self) -> &'a mut W {
        self.variant(PAD22_SR_A::SR_EN)
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
#[doc = "Field `PAD22_DS1` reader - Pad 22 high order drive strength selection. Used in conjunction with PAD22STRNG field to set the pad drive strength."]
pub struct PAD22_DS1_R(crate::FieldReader<bool, bool>);
impl PAD22_DS1_R {
    pub(crate) fn new(bits: bool) -> Self {
        PAD22_DS1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PAD22_DS1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PAD22_DS1` writer - Pad 22 high order drive strength selection. Used in conjunction with PAD22STRNG field to set the pad drive strength."]
pub struct PAD22_DS1_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD22_DS1_W<'a> {
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
#[doc = "Pad 21 slew rate selection.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD21_SR_A {
    #[doc = "1: Enables Slew rate control on pad value."]
    SR_EN = 1,
}
impl From<PAD21_SR_A> for bool {
    #[inline(always)]
    fn from(variant: PAD21_SR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PAD21_SR` reader - Pad 21 slew rate selection."]
pub struct PAD21_SR_R(crate::FieldReader<bool, PAD21_SR_A>);
impl PAD21_SR_R {
    pub(crate) fn new(bits: bool) -> Self {
        PAD21_SR_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PAD21_SR_A> {
        match self.bits {
            true => Some(PAD21_SR_A::SR_EN),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `SR_EN`"]
    #[inline(always)]
    pub fn is_sr_en(&self) -> bool {
        **self == PAD21_SR_A::SR_EN
    }
}
impl core::ops::Deref for PAD21_SR_R {
    type Target = crate::FieldReader<bool, PAD21_SR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PAD21_SR` writer - Pad 21 slew rate selection."]
pub struct PAD21_SR_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD21_SR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD21_SR_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Enables Slew rate control on pad value."]
    #[inline(always)]
    pub fn sr_en(self) -> &'a mut W {
        self.variant(PAD21_SR_A::SR_EN)
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
#[doc = "Field `PAD21_DS1` reader - Pad 21 high order drive strength selection. Used in conjunction with PAD21STRNG field to set the pad drive strength."]
pub struct PAD21_DS1_R(crate::FieldReader<bool, bool>);
impl PAD21_DS1_R {
    pub(crate) fn new(bits: bool) -> Self {
        PAD21_DS1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PAD21_DS1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PAD21_DS1` writer - Pad 21 high order drive strength selection. Used in conjunction with PAD21STRNG field to set the pad drive strength."]
pub struct PAD21_DS1_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD21_DS1_W<'a> {
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
#[doc = "Pad 20 slew rate selection.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD20_SR_A {
    #[doc = "1: Enables Slew rate control on pad value."]
    SR_EN = 1,
}
impl From<PAD20_SR_A> for bool {
    #[inline(always)]
    fn from(variant: PAD20_SR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PAD20_SR` reader - Pad 20 slew rate selection."]
pub struct PAD20_SR_R(crate::FieldReader<bool, PAD20_SR_A>);
impl PAD20_SR_R {
    pub(crate) fn new(bits: bool) -> Self {
        PAD20_SR_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PAD20_SR_A> {
        match self.bits {
            true => Some(PAD20_SR_A::SR_EN),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `SR_EN`"]
    #[inline(always)]
    pub fn is_sr_en(&self) -> bool {
        **self == PAD20_SR_A::SR_EN
    }
}
impl core::ops::Deref for PAD20_SR_R {
    type Target = crate::FieldReader<bool, PAD20_SR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PAD20_SR` writer - Pad 20 slew rate selection."]
pub struct PAD20_SR_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD20_SR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD20_SR_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Enables Slew rate control on pad value."]
    #[inline(always)]
    pub fn sr_en(self) -> &'a mut W {
        self.variant(PAD20_SR_A::SR_EN)
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
#[doc = "Field `PAD20_DS1` reader - Pad 20 high order drive strength selection. Used in conjunction with PAD20STRNG field to set the pad drive strength."]
pub struct PAD20_DS1_R(crate::FieldReader<bool, bool>);
impl PAD20_DS1_R {
    pub(crate) fn new(bits: bool) -> Self {
        PAD20_DS1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PAD20_DS1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PAD20_DS1` writer - Pad 20 high order drive strength selection. Used in conjunction with PAD20STRNG field to set the pad drive strength."]
pub struct PAD20_DS1_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD20_DS1_W<'a> {
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
    #[doc = "Bit 28 - Pad 23 slew rate selection."]
    #[inline(always)]
    pub fn pad23_sr(&self) -> PAD23_SR_R {
        PAD23_SR_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Pad 23 high order drive strength selection. Used in conjunction with PAD23STRNG field to set the pad drive strength."]
    #[inline(always)]
    pub fn pad23_ds1(&self) -> PAD23_DS1_R {
        PAD23_DS1_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Pad 22 slew rate selection."]
    #[inline(always)]
    pub fn pad22_sr(&self) -> PAD22_SR_R {
        PAD22_SR_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Pad 22 high order drive strength selection. Used in conjunction with PAD22STRNG field to set the pad drive strength."]
    #[inline(always)]
    pub fn pad22_ds1(&self) -> PAD22_DS1_R {
        PAD22_DS1_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Pad 21 slew rate selection."]
    #[inline(always)]
    pub fn pad21_sr(&self) -> PAD21_SR_R {
        PAD21_SR_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Pad 21 high order drive strength selection. Used in conjunction with PAD21STRNG field to set the pad drive strength."]
    #[inline(always)]
    pub fn pad21_ds1(&self) -> PAD21_DS1_R {
        PAD21_DS1_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Pad 20 slew rate selection."]
    #[inline(always)]
    pub fn pad20_sr(&self) -> PAD20_SR_R {
        PAD20_SR_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Pad 20 high order drive strength selection. Used in conjunction with PAD20STRNG field to set the pad drive strength."]
    #[inline(always)]
    pub fn pad20_ds1(&self) -> PAD20_DS1_R {
        PAD20_DS1_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 28 - Pad 23 slew rate selection."]
    #[inline(always)]
    pub fn pad23_sr(&mut self) -> PAD23_SR_W {
        PAD23_SR_W { w: self }
    }
    #[doc = "Bit 24 - Pad 23 high order drive strength selection. Used in conjunction with PAD23STRNG field to set the pad drive strength."]
    #[inline(always)]
    pub fn pad23_ds1(&mut self) -> PAD23_DS1_W {
        PAD23_DS1_W { w: self }
    }
    #[doc = "Bit 20 - Pad 22 slew rate selection."]
    #[inline(always)]
    pub fn pad22_sr(&mut self) -> PAD22_SR_W {
        PAD22_SR_W { w: self }
    }
    #[doc = "Bit 16 - Pad 22 high order drive strength selection. Used in conjunction with PAD22STRNG field to set the pad drive strength."]
    #[inline(always)]
    pub fn pad22_ds1(&mut self) -> PAD22_DS1_W {
        PAD22_DS1_W { w: self }
    }
    #[doc = "Bit 12 - Pad 21 slew rate selection."]
    #[inline(always)]
    pub fn pad21_sr(&mut self) -> PAD21_SR_W {
        PAD21_SR_W { w: self }
    }
    #[doc = "Bit 8 - Pad 21 high order drive strength selection. Used in conjunction with PAD21STRNG field to set the pad drive strength."]
    #[inline(always)]
    pub fn pad21_ds1(&mut self) -> PAD21_DS1_W {
        PAD21_DS1_W { w: self }
    }
    #[doc = "Bit 4 - Pad 20 slew rate selection."]
    #[inline(always)]
    pub fn pad20_sr(&mut self) -> PAD20_SR_W {
        PAD20_SR_W { w: self }
    }
    #[doc = "Bit 0 - Pad 20 high order drive strength selection. Used in conjunction with PAD20STRNG field to set the pad drive strength."]
    #[inline(always)]
    pub fn pad20_ds1(&mut self) -> PAD20_DS1_W {
        PAD20_DS1_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Alternate Pad Configuration reg5 (Pads 23,22,21,20)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [altpadcfgf](index.html) module"]
pub struct ALTPADCFGF_SPEC;
impl crate::RegisterSpec for ALTPADCFGF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [altpadcfgf::R](R) reader structure"]
impl crate::Readable for ALTPADCFGF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [altpadcfgf::W](W) writer structure"]
impl crate::Writable for ALTPADCFGF_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ALTPADCFGF to value 0"]
impl crate::Resettable for ALTPADCFGF_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
