#[doc = "Register `STAT` reader"]
pub struct R(crate::R<STAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STAT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STAT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `STAT` writer"]
pub struct W(crate::W<STAT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<STAT_SPEC>;
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
impl From<crate::W<STAT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<STAT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "This bit indicates the power down state of the voltage comparator.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWDSTAT_A {
    #[doc = "1: The voltage comparator is powered down. value."]
    POWERED_DOWN = 1,
}
impl From<PWDSTAT_A> for bool {
    #[inline(always)]
    fn from(variant: PWDSTAT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PWDSTAT` reader - This bit indicates the power down state of the voltage comparator."]
pub struct PWDSTAT_R(crate::FieldReader<bool, PWDSTAT_A>);
impl PWDSTAT_R {
    pub(crate) fn new(bits: bool) -> Self {
        PWDSTAT_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PWDSTAT_A> {
        match self.bits {
            true => Some(PWDSTAT_A::POWERED_DOWN),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `POWERED_DOWN`"]
    #[inline(always)]
    pub fn is_powered_down(&self) -> bool {
        **self == PWDSTAT_A::POWERED_DOWN
    }
}
impl core::ops::Deref for PWDSTAT_R {
    type Target = crate::FieldReader<bool, PWDSTAT_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PWDSTAT` writer - This bit indicates the power down state of the voltage comparator."]
pub struct PWDSTAT_W<'a> {
    w: &'a mut W,
}
impl<'a> PWDSTAT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PWDSTAT_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The voltage comparator is powered down. value."]
    #[inline(always)]
    pub fn powered_down(self) -> &'a mut W {
        self.variant(PWDSTAT_A::POWERED_DOWN)
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
#[doc = "This bit is 1 if the positive input of the comparator is greater than the negative input.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CMPOUT_A {
    #[doc = "0: The negative input of the comparator is greater than the positive input. value."]
    VOUT_LOW = 0,
    #[doc = "1: The positive input of the comparator is greater than the negative input. value."]
    VOUT_HIGH = 1,
}
impl From<CMPOUT_A> for bool {
    #[inline(always)]
    fn from(variant: CMPOUT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CMPOUT` reader - This bit is 1 if the positive input of the comparator is greater than the negative input."]
pub struct CMPOUT_R(crate::FieldReader<bool, CMPOUT_A>);
impl CMPOUT_R {
    pub(crate) fn new(bits: bool) -> Self {
        CMPOUT_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMPOUT_A {
        match self.bits {
            false => CMPOUT_A::VOUT_LOW,
            true => CMPOUT_A::VOUT_HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `VOUT_LOW`"]
    #[inline(always)]
    pub fn is_vout_low(&self) -> bool {
        **self == CMPOUT_A::VOUT_LOW
    }
    #[doc = "Checks if the value of the field is `VOUT_HIGH`"]
    #[inline(always)]
    pub fn is_vout_high(&self) -> bool {
        **self == CMPOUT_A::VOUT_HIGH
    }
}
impl core::ops::Deref for CMPOUT_R {
    type Target = crate::FieldReader<bool, CMPOUT_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CMPOUT` writer - This bit is 1 if the positive input of the comparator is greater than the negative input."]
pub struct CMPOUT_W<'a> {
    w: &'a mut W,
}
impl<'a> CMPOUT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CMPOUT_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The negative input of the comparator is greater than the positive input. value."]
    #[inline(always)]
    pub fn vout_low(self) -> &'a mut W {
        self.variant(CMPOUT_A::VOUT_LOW)
    }
    #[doc = "The positive input of the comparator is greater than the negative input. value."]
    #[inline(always)]
    pub fn vout_high(self) -> &'a mut W {
        self.variant(CMPOUT_A::VOUT_HIGH)
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
    #[doc = "Bit 1 - This bit indicates the power down state of the voltage comparator."]
    #[inline(always)]
    pub fn pwdstat(&self) -> PWDSTAT_R {
        PWDSTAT_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - This bit is 1 if the positive input of the comparator is greater than the negative input."]
    #[inline(always)]
    pub fn cmpout(&self) -> CMPOUT_R {
        CMPOUT_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - This bit indicates the power down state of the voltage comparator."]
    #[inline(always)]
    pub fn pwdstat(&mut self) -> PWDSTAT_W {
        PWDSTAT_W { w: self }
    }
    #[doc = "Bit 0 - This bit is 1 if the positive input of the comparator is greater than the negative input."]
    #[inline(always)]
    pub fn cmpout(&mut self) -> CMPOUT_W {
        CMPOUT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [stat](index.html) module"]
pub struct STAT_SPEC;
impl crate::RegisterSpec for STAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [stat::R](R) reader structure"]
impl crate::Readable for STAT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [stat::W](W) writer structure"]
impl crate::Writable for STAT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets STAT to value 0"]
impl crate::Resettable for STAT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
