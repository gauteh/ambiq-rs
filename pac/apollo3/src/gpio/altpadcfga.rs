#[doc = "Register `ALTPADCFGA` reader"]
pub struct R(crate::R<ALTPADCFGA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ALTPADCFGA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ALTPADCFGA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ALTPADCFGA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ALTPADCFGA` writer"]
pub struct W(crate::W<ALTPADCFGA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ALTPADCFGA_SPEC>;
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
impl From<crate::W<ALTPADCFGA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ALTPADCFGA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Pad 3 slew rate selection.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD3_SR_A {
    #[doc = "1: Enables Slew rate control on pad value."]
    SR_EN = 1,
}
impl From<PAD3_SR_A> for bool {
    #[inline(always)]
    fn from(variant: PAD3_SR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PAD3_SR` reader - Pad 3 slew rate selection."]
pub struct PAD3_SR_R(crate::FieldReader<bool, PAD3_SR_A>);
impl PAD3_SR_R {
    pub(crate) fn new(bits: bool) -> Self {
        PAD3_SR_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PAD3_SR_A> {
        match self.bits {
            true => Some(PAD3_SR_A::SR_EN),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `SR_EN`"]
    #[inline(always)]
    pub fn is_sr_en(&self) -> bool {
        **self == PAD3_SR_A::SR_EN
    }
}
impl core::ops::Deref for PAD3_SR_R {
    type Target = crate::FieldReader<bool, PAD3_SR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PAD3_SR` writer - Pad 3 slew rate selection."]
pub struct PAD3_SR_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD3_SR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD3_SR_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Enables Slew rate control on pad value."]
    #[inline(always)]
    pub fn sr_en(self) -> &'a mut W {
        self.variant(PAD3_SR_A::SR_EN)
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
#[doc = "Field `PAD3_DS1` reader - Pad 3 high order drive strength selection. Used in conjunction with PAD3STRNG field to set the pad drive strength."]
pub struct PAD3_DS1_R(crate::FieldReader<bool, bool>);
impl PAD3_DS1_R {
    pub(crate) fn new(bits: bool) -> Self {
        PAD3_DS1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PAD3_DS1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PAD3_DS1` writer - Pad 3 high order drive strength selection. Used in conjunction with PAD3STRNG field to set the pad drive strength."]
pub struct PAD3_DS1_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD3_DS1_W<'a> {
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
#[doc = "Pad 2 slew rate selection.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD2_SR_A {
    #[doc = "1: Enables Slew rate control on pad value."]
    SR_EN = 1,
}
impl From<PAD2_SR_A> for bool {
    #[inline(always)]
    fn from(variant: PAD2_SR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PAD2_SR` reader - Pad 2 slew rate selection."]
pub struct PAD2_SR_R(crate::FieldReader<bool, PAD2_SR_A>);
impl PAD2_SR_R {
    pub(crate) fn new(bits: bool) -> Self {
        PAD2_SR_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PAD2_SR_A> {
        match self.bits {
            true => Some(PAD2_SR_A::SR_EN),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `SR_EN`"]
    #[inline(always)]
    pub fn is_sr_en(&self) -> bool {
        **self == PAD2_SR_A::SR_EN
    }
}
impl core::ops::Deref for PAD2_SR_R {
    type Target = crate::FieldReader<bool, PAD2_SR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PAD2_SR` writer - Pad 2 slew rate selection."]
pub struct PAD2_SR_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD2_SR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD2_SR_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Enables Slew rate control on pad value."]
    #[inline(always)]
    pub fn sr_en(self) -> &'a mut W {
        self.variant(PAD2_SR_A::SR_EN)
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
#[doc = "Field `PAD2_DS1` reader - Pad 2 high order drive strength selection. Used in conjunction with PAD2STRNG field to set the pad drive strength."]
pub struct PAD2_DS1_R(crate::FieldReader<bool, bool>);
impl PAD2_DS1_R {
    pub(crate) fn new(bits: bool) -> Self {
        PAD2_DS1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PAD2_DS1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PAD2_DS1` writer - Pad 2 high order drive strength selection. Used in conjunction with PAD2STRNG field to set the pad drive strength."]
pub struct PAD2_DS1_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD2_DS1_W<'a> {
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
#[doc = "Pad 1 slew rate selection.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD1_SR_A {
    #[doc = "1: Enables Slew rate control on pad value."]
    SR_EN = 1,
}
impl From<PAD1_SR_A> for bool {
    #[inline(always)]
    fn from(variant: PAD1_SR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PAD1_SR` reader - Pad 1 slew rate selection."]
pub struct PAD1_SR_R(crate::FieldReader<bool, PAD1_SR_A>);
impl PAD1_SR_R {
    pub(crate) fn new(bits: bool) -> Self {
        PAD1_SR_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PAD1_SR_A> {
        match self.bits {
            true => Some(PAD1_SR_A::SR_EN),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `SR_EN`"]
    #[inline(always)]
    pub fn is_sr_en(&self) -> bool {
        **self == PAD1_SR_A::SR_EN
    }
}
impl core::ops::Deref for PAD1_SR_R {
    type Target = crate::FieldReader<bool, PAD1_SR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PAD1_SR` writer - Pad 1 slew rate selection."]
pub struct PAD1_SR_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD1_SR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD1_SR_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Enables Slew rate control on pad value."]
    #[inline(always)]
    pub fn sr_en(self) -> &'a mut W {
        self.variant(PAD1_SR_A::SR_EN)
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
#[doc = "Field `PAD1_DS1` reader - Pad 1 high order drive strength selection. Used in conjunction with PAD1STRNG field to set the pad drive strength."]
pub struct PAD1_DS1_R(crate::FieldReader<bool, bool>);
impl PAD1_DS1_R {
    pub(crate) fn new(bits: bool) -> Self {
        PAD1_DS1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PAD1_DS1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PAD1_DS1` writer - Pad 1 high order drive strength selection. Used in conjunction with PAD1STRNG field to set the pad drive strength."]
pub struct PAD1_DS1_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD1_DS1_W<'a> {
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
#[doc = "Pad 0 slew rate selection.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD0_SR_A {
    #[doc = "1: Enables Slew rate control on pad value."]
    SR_EN = 1,
}
impl From<PAD0_SR_A> for bool {
    #[inline(always)]
    fn from(variant: PAD0_SR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PAD0_SR` reader - Pad 0 slew rate selection."]
pub struct PAD0_SR_R(crate::FieldReader<bool, PAD0_SR_A>);
impl PAD0_SR_R {
    pub(crate) fn new(bits: bool) -> Self {
        PAD0_SR_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PAD0_SR_A> {
        match self.bits {
            true => Some(PAD0_SR_A::SR_EN),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `SR_EN`"]
    #[inline(always)]
    pub fn is_sr_en(&self) -> bool {
        **self == PAD0_SR_A::SR_EN
    }
}
impl core::ops::Deref for PAD0_SR_R {
    type Target = crate::FieldReader<bool, PAD0_SR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PAD0_SR` writer - Pad 0 slew rate selection."]
pub struct PAD0_SR_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD0_SR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAD0_SR_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Enables Slew rate control on pad value."]
    #[inline(always)]
    pub fn sr_en(self) -> &'a mut W {
        self.variant(PAD0_SR_A::SR_EN)
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
#[doc = "Field `PAD0_DS1` reader - Pad 0 high order drive strength selection. Used in conjunction with PAD0STRNG field to set the pad drive strength."]
pub struct PAD0_DS1_R(crate::FieldReader<bool, bool>);
impl PAD0_DS1_R {
    pub(crate) fn new(bits: bool) -> Self {
        PAD0_DS1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PAD0_DS1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PAD0_DS1` writer - Pad 0 high order drive strength selection. Used in conjunction with PAD0STRNG field to set the pad drive strength."]
pub struct PAD0_DS1_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD0_DS1_W<'a> {
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
    #[doc = "Bit 28 - Pad 3 slew rate selection."]
    #[inline(always)]
    pub fn pad3_sr(&self) -> PAD3_SR_R {
        PAD3_SR_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Pad 3 high order drive strength selection. Used in conjunction with PAD3STRNG field to set the pad drive strength."]
    #[inline(always)]
    pub fn pad3_ds1(&self) -> PAD3_DS1_R {
        PAD3_DS1_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Pad 2 slew rate selection."]
    #[inline(always)]
    pub fn pad2_sr(&self) -> PAD2_SR_R {
        PAD2_SR_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Pad 2 high order drive strength selection. Used in conjunction with PAD2STRNG field to set the pad drive strength."]
    #[inline(always)]
    pub fn pad2_ds1(&self) -> PAD2_DS1_R {
        PAD2_DS1_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Pad 1 slew rate selection."]
    #[inline(always)]
    pub fn pad1_sr(&self) -> PAD1_SR_R {
        PAD1_SR_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Pad 1 high order drive strength selection. Used in conjunction with PAD1STRNG field to set the pad drive strength."]
    #[inline(always)]
    pub fn pad1_ds1(&self) -> PAD1_DS1_R {
        PAD1_DS1_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Pad 0 slew rate selection."]
    #[inline(always)]
    pub fn pad0_sr(&self) -> PAD0_SR_R {
        PAD0_SR_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Pad 0 high order drive strength selection. Used in conjunction with PAD0STRNG field to set the pad drive strength."]
    #[inline(always)]
    pub fn pad0_ds1(&self) -> PAD0_DS1_R {
        PAD0_DS1_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 28 - Pad 3 slew rate selection."]
    #[inline(always)]
    pub fn pad3_sr(&mut self) -> PAD3_SR_W {
        PAD3_SR_W { w: self }
    }
    #[doc = "Bit 24 - Pad 3 high order drive strength selection. Used in conjunction with PAD3STRNG field to set the pad drive strength."]
    #[inline(always)]
    pub fn pad3_ds1(&mut self) -> PAD3_DS1_W {
        PAD3_DS1_W { w: self }
    }
    #[doc = "Bit 20 - Pad 2 slew rate selection."]
    #[inline(always)]
    pub fn pad2_sr(&mut self) -> PAD2_SR_W {
        PAD2_SR_W { w: self }
    }
    #[doc = "Bit 16 - Pad 2 high order drive strength selection. Used in conjunction with PAD2STRNG field to set the pad drive strength."]
    #[inline(always)]
    pub fn pad2_ds1(&mut self) -> PAD2_DS1_W {
        PAD2_DS1_W { w: self }
    }
    #[doc = "Bit 12 - Pad 1 slew rate selection."]
    #[inline(always)]
    pub fn pad1_sr(&mut self) -> PAD1_SR_W {
        PAD1_SR_W { w: self }
    }
    #[doc = "Bit 8 - Pad 1 high order drive strength selection. Used in conjunction with PAD1STRNG field to set the pad drive strength."]
    #[inline(always)]
    pub fn pad1_ds1(&mut self) -> PAD1_DS1_W {
        PAD1_DS1_W { w: self }
    }
    #[doc = "Bit 4 - Pad 0 slew rate selection."]
    #[inline(always)]
    pub fn pad0_sr(&mut self) -> PAD0_SR_W {
        PAD0_SR_W { w: self }
    }
    #[doc = "Bit 0 - Pad 0 high order drive strength selection. Used in conjunction with PAD0STRNG field to set the pad drive strength."]
    #[inline(always)]
    pub fn pad0_ds1(&mut self) -> PAD0_DS1_W {
        PAD0_DS1_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Alternate Pad Configuration reg0 (Pads 3,2,1,0)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [altpadcfga](index.html) module"]
pub struct ALTPADCFGA_SPEC;
impl crate::RegisterSpec for ALTPADCFGA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [altpadcfga::R](R) reader structure"]
impl crate::Readable for ALTPADCFGA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [altpadcfga::W](W) writer structure"]
impl crate::Writable for ALTPADCFGA_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ALTPADCFGA to value 0"]
impl crate::Resettable for ALTPADCFGA_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
