#[doc = "Register `CAPTURECONTROL` reader"]
pub struct R(crate::R<CAPTURECONTROL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CAPTURECONTROL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CAPTURECONTROL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CAPTURECONTROL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CAPTURECONTROL` writer"]
pub struct W(crate::W<CAPTURECONTROL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CAPTURECONTROL_SPEC>;
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
impl From<crate::W<CAPTURECONTROL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CAPTURECONTROL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Selects whether capture is enabled for the specified capture register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CAPTURE3_A {
    #[doc = "0: Capture function disabled. value."]
    DISABLE = 0,
    #[doc = "1: Capture function enabled. value."]
    ENABLE = 1,
}
impl From<CAPTURE3_A> for bool {
    #[inline(always)]
    fn from(variant: CAPTURE3_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CAPTURE3` reader - Selects whether capture is enabled for the specified capture register."]
pub struct CAPTURE3_R(crate::FieldReader<bool, CAPTURE3_A>);
impl CAPTURE3_R {
    pub(crate) fn new(bits: bool) -> Self {
        CAPTURE3_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CAPTURE3_A {
        match self.bits {
            false => CAPTURE3_A::DISABLE,
            true => CAPTURE3_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        **self == CAPTURE3_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == CAPTURE3_A::ENABLE
    }
}
impl core::ops::Deref for CAPTURE3_R {
    type Target = crate::FieldReader<bool, CAPTURE3_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CAPTURE3` writer - Selects whether capture is enabled for the specified capture register."]
pub struct CAPTURE3_W<'a> {
    w: &'a mut W,
}
impl<'a> CAPTURE3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CAPTURE3_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Capture function disabled. value."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(CAPTURE3_A::DISABLE)
    }
    #[doc = "Capture function enabled. value."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(CAPTURE3_A::ENABLE)
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u32 & 0x01) << 3);
        self.w
    }
}
#[doc = "Selects whether capture is enabled for the specified capture register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CAPTURE2_A {
    #[doc = "0: Capture function disabled. value."]
    DISABLE = 0,
    #[doc = "1: Capture function enabled. value."]
    ENABLE = 1,
}
impl From<CAPTURE2_A> for bool {
    #[inline(always)]
    fn from(variant: CAPTURE2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CAPTURE2` reader - Selects whether capture is enabled for the specified capture register."]
pub struct CAPTURE2_R(crate::FieldReader<bool, CAPTURE2_A>);
impl CAPTURE2_R {
    pub(crate) fn new(bits: bool) -> Self {
        CAPTURE2_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CAPTURE2_A {
        match self.bits {
            false => CAPTURE2_A::DISABLE,
            true => CAPTURE2_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        **self == CAPTURE2_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == CAPTURE2_A::ENABLE
    }
}
impl core::ops::Deref for CAPTURE2_R {
    type Target = crate::FieldReader<bool, CAPTURE2_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CAPTURE2` writer - Selects whether capture is enabled for the specified capture register."]
pub struct CAPTURE2_W<'a> {
    w: &'a mut W,
}
impl<'a> CAPTURE2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CAPTURE2_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Capture function disabled. value."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(CAPTURE2_A::DISABLE)
    }
    #[doc = "Capture function enabled. value."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(CAPTURE2_A::ENABLE)
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
        self.w
    }
}
#[doc = "Selects whether capture is enabled for the specified capture register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CAPTURE1_A {
    #[doc = "0: Capture function disabled. value."]
    DISABLE = 0,
    #[doc = "1: Capture function enabled. value."]
    ENABLE = 1,
}
impl From<CAPTURE1_A> for bool {
    #[inline(always)]
    fn from(variant: CAPTURE1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CAPTURE1` reader - Selects whether capture is enabled for the specified capture register."]
pub struct CAPTURE1_R(crate::FieldReader<bool, CAPTURE1_A>);
impl CAPTURE1_R {
    pub(crate) fn new(bits: bool) -> Self {
        CAPTURE1_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CAPTURE1_A {
        match self.bits {
            false => CAPTURE1_A::DISABLE,
            true => CAPTURE1_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        **self == CAPTURE1_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == CAPTURE1_A::ENABLE
    }
}
impl core::ops::Deref for CAPTURE1_R {
    type Target = crate::FieldReader<bool, CAPTURE1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CAPTURE1` writer - Selects whether capture is enabled for the specified capture register."]
pub struct CAPTURE1_W<'a> {
    w: &'a mut W,
}
impl<'a> CAPTURE1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CAPTURE1_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Capture function disabled. value."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(CAPTURE1_A::DISABLE)
    }
    #[doc = "Capture function enabled. value."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(CAPTURE1_A::ENABLE)
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
#[doc = "Selects whether capture is enabled for the specified capture register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CAPTURE0_A {
    #[doc = "0: Capture function disabled. value."]
    DISABLE = 0,
    #[doc = "1: Capture function enabled. value."]
    ENABLE = 1,
}
impl From<CAPTURE0_A> for bool {
    #[inline(always)]
    fn from(variant: CAPTURE0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CAPTURE0` reader - Selects whether capture is enabled for the specified capture register."]
pub struct CAPTURE0_R(crate::FieldReader<bool, CAPTURE0_A>);
impl CAPTURE0_R {
    pub(crate) fn new(bits: bool) -> Self {
        CAPTURE0_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CAPTURE0_A {
        match self.bits {
            false => CAPTURE0_A::DISABLE,
            true => CAPTURE0_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        **self == CAPTURE0_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == CAPTURE0_A::ENABLE
    }
}
impl core::ops::Deref for CAPTURE0_R {
    type Target = crate::FieldReader<bool, CAPTURE0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CAPTURE0` writer - Selects whether capture is enabled for the specified capture register."]
pub struct CAPTURE0_W<'a> {
    w: &'a mut W,
}
impl<'a> CAPTURE0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CAPTURE0_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Capture function disabled. value."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(CAPTURE0_A::DISABLE)
    }
    #[doc = "Capture function enabled. value."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(CAPTURE0_A::ENABLE)
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
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
impl R {
    #[doc = "Bit 3 - Selects whether capture is enabled for the specified capture register."]
    #[inline(always)]
    pub fn capture3(&self) -> CAPTURE3_R {
        CAPTURE3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Selects whether capture is enabled for the specified capture register."]
    #[inline(always)]
    pub fn capture2(&self) -> CAPTURE2_R {
        CAPTURE2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - Selects whether capture is enabled for the specified capture register."]
    #[inline(always)]
    pub fn capture1(&self) -> CAPTURE1_R {
        CAPTURE1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Selects whether capture is enabled for the specified capture register."]
    #[inline(always)]
    pub fn capture0(&self) -> CAPTURE0_R {
        CAPTURE0_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 3 - Selects whether capture is enabled for the specified capture register."]
    #[inline(always)]
    pub fn capture3(&mut self) -> CAPTURE3_W {
        CAPTURE3_W { w: self }
    }
    #[doc = "Bit 2 - Selects whether capture is enabled for the specified capture register."]
    #[inline(always)]
    pub fn capture2(&mut self) -> CAPTURE2_W {
        CAPTURE2_W { w: self }
    }
    #[doc = "Bit 1 - Selects whether capture is enabled for the specified capture register."]
    #[inline(always)]
    pub fn capture1(&mut self) -> CAPTURE1_W {
        CAPTURE1_W { w: self }
    }
    #[doc = "Bit 0 - Selects whether capture is enabled for the specified capture register."]
    #[inline(always)]
    pub fn capture0(&mut self) -> CAPTURE0_W {
        CAPTURE0_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Capture Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [capturecontrol](index.html) module"]
pub struct CAPTURECONTROL_SPEC;
impl crate::RegisterSpec for CAPTURECONTROL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [capturecontrol::R](R) reader structure"]
impl crate::Readable for CAPTURECONTROL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [capturecontrol::W](W) writer structure"]
impl crate::Writable for CAPTURECONTROL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CAPTURECONTROL to value 0"]
impl crate::Resettable for CAPTURECONTROL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
