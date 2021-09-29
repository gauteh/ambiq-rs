#[doc = "Register `ADCBATTLOAD` reader"]
pub struct R(crate::R<ADCBATTLOAD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADCBATTLOAD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADCBATTLOAD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADCBATTLOAD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADCBATTLOAD` writer"]
pub struct W(crate::W<ADCBATTLOAD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADCBATTLOAD_SPEC>;
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
impl From<crate::W<ADCBATTLOAD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADCBATTLOAD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Enable the ADC battery load resistor\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BATTLOAD_A {
    #[doc = "0: Battery load is disconnected value."]
    DIS = 0,
    #[doc = "1: Battery load is enabled value."]
    EN = 1,
}
impl From<BATTLOAD_A> for bool {
    #[inline(always)]
    fn from(variant: BATTLOAD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BATTLOAD` reader - Enable the ADC battery load resistor"]
pub struct BATTLOAD_R(crate::FieldReader<bool, BATTLOAD_A>);
impl BATTLOAD_R {
    pub(crate) fn new(bits: bool) -> Self {
        BATTLOAD_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BATTLOAD_A {
        match self.bits {
            false => BATTLOAD_A::DIS,
            true => BATTLOAD_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        **self == BATTLOAD_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        **self == BATTLOAD_A::EN
    }
}
impl core::ops::Deref for BATTLOAD_R {
    type Target = crate::FieldReader<bool, BATTLOAD_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BATTLOAD` writer - Enable the ADC battery load resistor"]
pub struct BATTLOAD_W<'a> {
    w: &'a mut W,
}
impl<'a> BATTLOAD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BATTLOAD_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Battery load is disconnected value."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(BATTLOAD_A::DIS)
    }
    #[doc = "Battery load is enabled value."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(BATTLOAD_A::EN)
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
    #[doc = "Bit 0 - Enable the ADC battery load resistor"]
    #[inline(always)]
    pub fn battload(&self) -> BATTLOAD_R {
        BATTLOAD_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable the ADC battery load resistor"]
    #[inline(always)]
    pub fn battload(&mut self) -> BATTLOAD_W {
        BATTLOAD_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ADC Battery Load Enable\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adcbattload](index.html) module"]
pub struct ADCBATTLOAD_SPEC;
impl crate::RegisterSpec for ADCBATTLOAD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [adcbattload::R](R) reader structure"]
impl crate::Readable for ADCBATTLOAD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adcbattload::W](W) writer structure"]
impl crate::Writable for ADCBATTLOAD_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ADCBATTLOAD to value 0"]
impl crate::Resettable for ADCBATTLOAD_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
