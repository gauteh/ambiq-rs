#[doc = "Register `ALTPADCFGM` reader"]
pub struct R(crate::R<ALTPADCFGM_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ALTPADCFGM_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ALTPADCFGM_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ALTPADCFGM_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ALTPADCFGM` writer"]
pub struct W(crate::W<ALTPADCFGM_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ALTPADCFGM_SPEC>;
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
impl From<crate::W<ALTPADCFGM_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ALTPADCFGM_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Pad 49 slew rate selection.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD49_SR_A {
    #[doc = "1: Enables Slew rate control on pad value."]
    SR_EN = 1,
}
impl From<PAD49_SR_A> for bool {
    #[inline(always)]
    fn from(variant: PAD49_SR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PAD49_SR` reader - Pad 49 slew rate selection."]
pub struct PAD49_SR_R(crate::FieldReader<bool, PAD49_SR_A>);
impl PAD49_SR_R {
    pub(crate) fn new(bits: bool) -> Self {
        PAD49_SR_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PAD49_SR_A> {
        match self.bits {
            true => Some(PAD49_SR_A::SR_EN),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `SR_EN`"]
    #[inline(always)]
    pub fn is_sr_en(&self) -> bool {
        **self == PAD49_SR_A::SR_EN
    }
}
impl core::ops::Deref for PAD49_SR_R {
    type Target = crate::FieldReader<bool, PAD49_SR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PAD49_SR` writer - Pad 49 slew rate selection."]
pub struct PAD49_SR_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD49_SR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD49_SR_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Enables Slew rate control on pad value."]
    #[inline(always)]
    pub fn sr_en(self) -> &'a mut W {
        self.variant(PAD49_SR_A::SR_EN)
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
#[doc = "Field `PAD49_DS1` reader - Pad 49 high order drive strength selection. Used in conjunction with PAD49STRNG field to set the pad drive strength."]
pub struct PAD49_DS1_R(crate::FieldReader<bool, bool>);
impl PAD49_DS1_R {
    pub(crate) fn new(bits: bool) -> Self {
        PAD49_DS1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PAD49_DS1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PAD49_DS1` writer - Pad 49 high order drive strength selection. Used in conjunction with PAD49STRNG field to set the pad drive strength."]
pub struct PAD49_DS1_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD49_DS1_W<'a> {
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
#[doc = "Pad 48 slew rate selection.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD48_SR_A {
    #[doc = "1: Enables Slew rate control on pad value."]
    SR_EN = 1,
}
impl From<PAD48_SR_A> for bool {
    #[inline(always)]
    fn from(variant: PAD48_SR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PAD48_SR` reader - Pad 48 slew rate selection."]
pub struct PAD48_SR_R(crate::FieldReader<bool, PAD48_SR_A>);
impl PAD48_SR_R {
    pub(crate) fn new(bits: bool) -> Self {
        PAD48_SR_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PAD48_SR_A> {
        match self.bits {
            true => Some(PAD48_SR_A::SR_EN),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `SR_EN`"]
    #[inline(always)]
    pub fn is_sr_en(&self) -> bool {
        **self == PAD48_SR_A::SR_EN
    }
}
impl core::ops::Deref for PAD48_SR_R {
    type Target = crate::FieldReader<bool, PAD48_SR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PAD48_SR` writer - Pad 48 slew rate selection."]
pub struct PAD48_SR_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD48_SR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD48_SR_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Enables Slew rate control on pad value."]
    #[inline(always)]
    pub fn sr_en(self) -> &'a mut W {
        self.variant(PAD48_SR_A::SR_EN)
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
#[doc = "Field `PAD48_DS1` reader - Pad 48 high order drive strength selection. Used in conjunction with PAD48STRNG field to set the pad drive strength."]
pub struct PAD48_DS1_R(crate::FieldReader<bool, bool>);
impl PAD48_DS1_R {
    pub(crate) fn new(bits: bool) -> Self {
        PAD48_DS1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PAD48_DS1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PAD48_DS1` writer - Pad 48 high order drive strength selection. Used in conjunction with PAD48STRNG field to set the pad drive strength."]
pub struct PAD48_DS1_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD48_DS1_W<'a> {
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
    #[doc = "Bit 12 - Pad 49 slew rate selection."]
    #[inline(always)]
    pub fn pad49_sr(&self) -> PAD49_SR_R {
        PAD49_SR_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Pad 49 high order drive strength selection. Used in conjunction with PAD49STRNG field to set the pad drive strength."]
    #[inline(always)]
    pub fn pad49_ds1(&self) -> PAD49_DS1_R {
        PAD49_DS1_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Pad 48 slew rate selection."]
    #[inline(always)]
    pub fn pad48_sr(&self) -> PAD48_SR_R {
        PAD48_SR_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Pad 48 high order drive strength selection. Used in conjunction with PAD48STRNG field to set the pad drive strength."]
    #[inline(always)]
    pub fn pad48_ds1(&self) -> PAD48_DS1_R {
        PAD48_DS1_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 12 - Pad 49 slew rate selection."]
    #[inline(always)]
    pub fn pad49_sr(&mut self) -> PAD49_SR_W {
        PAD49_SR_W { w: self }
    }
    #[doc = "Bit 8 - Pad 49 high order drive strength selection. Used in conjunction with PAD49STRNG field to set the pad drive strength."]
    #[inline(always)]
    pub fn pad49_ds1(&mut self) -> PAD49_DS1_W {
        PAD49_DS1_W { w: self }
    }
    #[doc = "Bit 4 - Pad 48 slew rate selection."]
    #[inline(always)]
    pub fn pad48_sr(&mut self) -> PAD48_SR_W {
        PAD48_SR_W { w: self }
    }
    #[doc = "Bit 0 - Pad 48 high order drive strength selection. Used in conjunction with PAD48STRNG field to set the pad drive strength."]
    #[inline(always)]
    pub fn pad48_ds1(&mut self) -> PAD48_DS1_W {
        PAD48_DS1_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Alternate Pad Configuration reg12 (Pads 49,48)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [altpadcfgm](index.html) module"]
pub struct ALTPADCFGM_SPEC;
impl crate::RegisterSpec for ALTPADCFGM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [altpadcfgm::R](R) reader structure"]
impl crate::Readable for ALTPADCFGM_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [altpadcfgm::W](W) writer structure"]
impl crate::Writable for ALTPADCFGM_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ALTPADCFGM to value 0"]
impl crate::Resettable for ALTPADCFGM_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
