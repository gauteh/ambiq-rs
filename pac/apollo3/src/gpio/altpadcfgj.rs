#[doc = "Register `ALTPADCFGJ` reader"]
pub struct R(crate::R<ALTPADCFGJ_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ALTPADCFGJ_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ALTPADCFGJ_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ALTPADCFGJ_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ALTPADCFGJ` writer"]
pub struct W(crate::W<ALTPADCFGJ_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ALTPADCFGJ_SPEC>;
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
impl From<crate::W<ALTPADCFGJ_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ALTPADCFGJ_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Pad 39 slew rate selection.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD39_SR_A {
    #[doc = "1: Enables Slew rate control on pad value."]
    SR_EN = 1,
}
impl From<PAD39_SR_A> for bool {
    #[inline(always)]
    fn from(variant: PAD39_SR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PAD39_SR` reader - Pad 39 slew rate selection."]
pub struct PAD39_SR_R(crate::FieldReader<bool, PAD39_SR_A>);
impl PAD39_SR_R {
    pub(crate) fn new(bits: bool) -> Self {
        PAD39_SR_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PAD39_SR_A> {
        match self.bits {
            true => Some(PAD39_SR_A::SR_EN),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `SR_EN`"]
    #[inline(always)]
    pub fn is_sr_en(&self) -> bool {
        **self == PAD39_SR_A::SR_EN
    }
}
impl core::ops::Deref for PAD39_SR_R {
    type Target = crate::FieldReader<bool, PAD39_SR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PAD39_SR` writer - Pad 39 slew rate selection."]
pub struct PAD39_SR_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD39_SR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD39_SR_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Enables Slew rate control on pad value."]
    #[inline(always)]
    pub fn sr_en(self) -> &'a mut W {
        self.variant(PAD39_SR_A::SR_EN)
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
#[doc = "Field `PAD39_DS1` reader - Pad 39 high order drive strength selection. Used in conjunction with PAD39STRNG field to set the pad drive strength."]
pub struct PAD39_DS1_R(crate::FieldReader<bool, bool>);
impl PAD39_DS1_R {
    pub(crate) fn new(bits: bool) -> Self {
        PAD39_DS1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PAD39_DS1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PAD39_DS1` writer - Pad 39 high order drive strength selection. Used in conjunction with PAD39STRNG field to set the pad drive strength."]
pub struct PAD39_DS1_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD39_DS1_W<'a> {
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
#[doc = "Pad 38 slew rate selection.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD38_SR_A {
    #[doc = "1: Enables Slew rate control on pad value."]
    SR_EN = 1,
}
impl From<PAD38_SR_A> for bool {
    #[inline(always)]
    fn from(variant: PAD38_SR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PAD38_SR` reader - Pad 38 slew rate selection."]
pub struct PAD38_SR_R(crate::FieldReader<bool, PAD38_SR_A>);
impl PAD38_SR_R {
    pub(crate) fn new(bits: bool) -> Self {
        PAD38_SR_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PAD38_SR_A> {
        match self.bits {
            true => Some(PAD38_SR_A::SR_EN),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `SR_EN`"]
    #[inline(always)]
    pub fn is_sr_en(&self) -> bool {
        **self == PAD38_SR_A::SR_EN
    }
}
impl core::ops::Deref for PAD38_SR_R {
    type Target = crate::FieldReader<bool, PAD38_SR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PAD38_SR` writer - Pad 38 slew rate selection."]
pub struct PAD38_SR_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD38_SR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD38_SR_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Enables Slew rate control on pad value."]
    #[inline(always)]
    pub fn sr_en(self) -> &'a mut W {
        self.variant(PAD38_SR_A::SR_EN)
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
#[doc = "Field `PAD38_DS1` reader - Pad 38 high order drive strength selection. Used in conjunction with PAD38STRNG field to set the pad drive strength."]
pub struct PAD38_DS1_R(crate::FieldReader<bool, bool>);
impl PAD38_DS1_R {
    pub(crate) fn new(bits: bool) -> Self {
        PAD38_DS1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PAD38_DS1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PAD38_DS1` writer - Pad 38 high order drive strength selection. Used in conjunction with PAD38STRNG field to set the pad drive strength."]
pub struct PAD38_DS1_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD38_DS1_W<'a> {
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
#[doc = "Pad 37 slew rate selection.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD37_SR_A {
    #[doc = "1: Enables Slew rate control on pad value."]
    SR_EN = 1,
}
impl From<PAD37_SR_A> for bool {
    #[inline(always)]
    fn from(variant: PAD37_SR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PAD37_SR` reader - Pad 37 slew rate selection."]
pub struct PAD37_SR_R(crate::FieldReader<bool, PAD37_SR_A>);
impl PAD37_SR_R {
    pub(crate) fn new(bits: bool) -> Self {
        PAD37_SR_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PAD37_SR_A> {
        match self.bits {
            true => Some(PAD37_SR_A::SR_EN),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `SR_EN`"]
    #[inline(always)]
    pub fn is_sr_en(&self) -> bool {
        **self == PAD37_SR_A::SR_EN
    }
}
impl core::ops::Deref for PAD37_SR_R {
    type Target = crate::FieldReader<bool, PAD37_SR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PAD37_SR` writer - Pad 37 slew rate selection."]
pub struct PAD37_SR_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD37_SR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD37_SR_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Enables Slew rate control on pad value."]
    #[inline(always)]
    pub fn sr_en(self) -> &'a mut W {
        self.variant(PAD37_SR_A::SR_EN)
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
#[doc = "Field `PAD37_DS1` reader - Pad 37 high order drive strength selection. Used in conjunction with PAD37STRNG field to set the pad drive strength."]
pub struct PAD37_DS1_R(crate::FieldReader<bool, bool>);
impl PAD37_DS1_R {
    pub(crate) fn new(bits: bool) -> Self {
        PAD37_DS1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PAD37_DS1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PAD37_DS1` writer - Pad 37 high order drive strength selection. Used in conjunction with PAD37STRNG field to set the pad drive strength."]
pub struct PAD37_DS1_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD37_DS1_W<'a> {
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
#[doc = "Pad 36 slew rate selection.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD36_SR_A {
    #[doc = "1: Enables Slew rate control on pad value."]
    SR_EN = 1,
}
impl From<PAD36_SR_A> for bool {
    #[inline(always)]
    fn from(variant: PAD36_SR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PAD36_SR` reader - Pad 36 slew rate selection."]
pub struct PAD36_SR_R(crate::FieldReader<bool, PAD36_SR_A>);
impl PAD36_SR_R {
    pub(crate) fn new(bits: bool) -> Self {
        PAD36_SR_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PAD36_SR_A> {
        match self.bits {
            true => Some(PAD36_SR_A::SR_EN),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `SR_EN`"]
    #[inline(always)]
    pub fn is_sr_en(&self) -> bool {
        **self == PAD36_SR_A::SR_EN
    }
}
impl core::ops::Deref for PAD36_SR_R {
    type Target = crate::FieldReader<bool, PAD36_SR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PAD36_SR` writer - Pad 36 slew rate selection."]
pub struct PAD36_SR_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD36_SR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD36_SR_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Enables Slew rate control on pad value."]
    #[inline(always)]
    pub fn sr_en(self) -> &'a mut W {
        self.variant(PAD36_SR_A::SR_EN)
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
#[doc = "Field `PAD36_DS1` reader - Pad 36 high order drive strength selection. Used in conjunction with PAD36STRNG field to set the pad drive strength."]
pub struct PAD36_DS1_R(crate::FieldReader<bool, bool>);
impl PAD36_DS1_R {
    pub(crate) fn new(bits: bool) -> Self {
        PAD36_DS1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PAD36_DS1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PAD36_DS1` writer - Pad 36 high order drive strength selection. Used in conjunction with PAD36STRNG field to set the pad drive strength."]
pub struct PAD36_DS1_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD36_DS1_W<'a> {
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
    #[doc = "Bit 28 - Pad 39 slew rate selection."]
    #[inline(always)]
    pub fn pad39_sr(&self) -> PAD39_SR_R {
        PAD39_SR_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Pad 39 high order drive strength selection. Used in conjunction with PAD39STRNG field to set the pad drive strength."]
    #[inline(always)]
    pub fn pad39_ds1(&self) -> PAD39_DS1_R {
        PAD39_DS1_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Pad 38 slew rate selection."]
    #[inline(always)]
    pub fn pad38_sr(&self) -> PAD38_SR_R {
        PAD38_SR_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Pad 38 high order drive strength selection. Used in conjunction with PAD38STRNG field to set the pad drive strength."]
    #[inline(always)]
    pub fn pad38_ds1(&self) -> PAD38_DS1_R {
        PAD38_DS1_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Pad 37 slew rate selection."]
    #[inline(always)]
    pub fn pad37_sr(&self) -> PAD37_SR_R {
        PAD37_SR_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Pad 37 high order drive strength selection. Used in conjunction with PAD37STRNG field to set the pad drive strength."]
    #[inline(always)]
    pub fn pad37_ds1(&self) -> PAD37_DS1_R {
        PAD37_DS1_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Pad 36 slew rate selection."]
    #[inline(always)]
    pub fn pad36_sr(&self) -> PAD36_SR_R {
        PAD36_SR_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Pad 36 high order drive strength selection. Used in conjunction with PAD36STRNG field to set the pad drive strength."]
    #[inline(always)]
    pub fn pad36_ds1(&self) -> PAD36_DS1_R {
        PAD36_DS1_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 28 - Pad 39 slew rate selection."]
    #[inline(always)]
    pub fn pad39_sr(&mut self) -> PAD39_SR_W {
        PAD39_SR_W { w: self }
    }
    #[doc = "Bit 24 - Pad 39 high order drive strength selection. Used in conjunction with PAD39STRNG field to set the pad drive strength."]
    #[inline(always)]
    pub fn pad39_ds1(&mut self) -> PAD39_DS1_W {
        PAD39_DS1_W { w: self }
    }
    #[doc = "Bit 20 - Pad 38 slew rate selection."]
    #[inline(always)]
    pub fn pad38_sr(&mut self) -> PAD38_SR_W {
        PAD38_SR_W { w: self }
    }
    #[doc = "Bit 16 - Pad 38 high order drive strength selection. Used in conjunction with PAD38STRNG field to set the pad drive strength."]
    #[inline(always)]
    pub fn pad38_ds1(&mut self) -> PAD38_DS1_W {
        PAD38_DS1_W { w: self }
    }
    #[doc = "Bit 12 - Pad 37 slew rate selection."]
    #[inline(always)]
    pub fn pad37_sr(&mut self) -> PAD37_SR_W {
        PAD37_SR_W { w: self }
    }
    #[doc = "Bit 8 - Pad 37 high order drive strength selection. Used in conjunction with PAD37STRNG field to set the pad drive strength."]
    #[inline(always)]
    pub fn pad37_ds1(&mut self) -> PAD37_DS1_W {
        PAD37_DS1_W { w: self }
    }
    #[doc = "Bit 4 - Pad 36 slew rate selection."]
    #[inline(always)]
    pub fn pad36_sr(&mut self) -> PAD36_SR_W {
        PAD36_SR_W { w: self }
    }
    #[doc = "Bit 0 - Pad 36 high order drive strength selection. Used in conjunction with PAD36STRNG field to set the pad drive strength."]
    #[inline(always)]
    pub fn pad36_ds1(&mut self) -> PAD36_DS1_W {
        PAD36_DS1_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Alternate Pad Configuration reg9 (Pads 39,38,37,36)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [altpadcfgj](index.html) module"]
pub struct ALTPADCFGJ_SPEC;
impl crate::RegisterSpec for ALTPADCFGJ_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [altpadcfgj::R](R) reader structure"]
impl crate::Readable for ALTPADCFGJ_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [altpadcfgj::W](W) writer structure"]
impl crate::Writable for ALTPADCFGJ_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ALTPADCFGJ to value 0"]
impl crate::Resettable for ALTPADCFGJ_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
