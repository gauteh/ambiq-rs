#[doc = "Register `ADCCAL` reader"]
pub struct R(crate::R<ADCCAL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADCCAL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADCCAL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADCCAL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADCCAL` writer"]
pub struct W(crate::W<ADCCAL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADCCAL_SPEC>;
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
impl From<crate::W<ADCCAL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADCCAL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Status for ADC Calibration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADCCALIBRATED_A {
    #[doc = "0: ADC is not calibrated value."]
    FALSE = 0,
    #[doc = "1: ADC is calibrated value."]
    TRUE = 1,
}
impl From<ADCCALIBRATED_A> for bool {
    #[inline(always)]
    fn from(variant: ADCCALIBRATED_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADCCALIBRATED` reader - Status for ADC Calibration"]
pub struct ADCCALIBRATED_R(crate::FieldReader<bool, ADCCALIBRATED_A>);
impl ADCCALIBRATED_R {
    pub(crate) fn new(bits: bool) -> Self {
        ADCCALIBRATED_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADCCALIBRATED_A {
        match self.bits {
            false => ADCCALIBRATED_A::FALSE,
            true => ADCCALIBRATED_A::TRUE,
        }
    }
    #[doc = "Checks if the value of the field is `FALSE`"]
    #[inline(always)]
    pub fn is_false(&self) -> bool {
        **self == ADCCALIBRATED_A::FALSE
    }
    #[doc = "Checks if the value of the field is `TRUE`"]
    #[inline(always)]
    pub fn is_true(&self) -> bool {
        **self == ADCCALIBRATED_A::TRUE
    }
}
impl core::ops::Deref for ADCCALIBRATED_R {
    type Target = crate::FieldReader<bool, ADCCALIBRATED_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADCCALIBRATED` writer - Status for ADC Calibration"]
pub struct ADCCALIBRATED_W<'a> {
    w: &'a mut W,
}
impl<'a> ADCCALIBRATED_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADCCALIBRATED_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "ADC is not calibrated value."]
    #[inline(always)]
    pub fn false_(self) -> &'a mut W {
        self.variant(ADCCALIBRATED_A::FALSE)
    }
    #[doc = "ADC is calibrated value."]
    #[inline(always)]
    pub fn true_(self) -> &'a mut W {
        self.variant(ADCCALIBRATED_A::TRUE)
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
#[doc = "Run ADC Calibration on initial power up sequence\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CALONPWRUP_A {
    #[doc = "0: Disable automatic calibration on initial power up value."]
    DIS = 0,
    #[doc = "1: Enable automatic calibration on initial power up value."]
    EN = 1,
}
impl From<CALONPWRUP_A> for bool {
    #[inline(always)]
    fn from(variant: CALONPWRUP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CALONPWRUP` reader - Run ADC Calibration on initial power up sequence"]
pub struct CALONPWRUP_R(crate::FieldReader<bool, CALONPWRUP_A>);
impl CALONPWRUP_R {
    pub(crate) fn new(bits: bool) -> Self {
        CALONPWRUP_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CALONPWRUP_A {
        match self.bits {
            false => CALONPWRUP_A::DIS,
            true => CALONPWRUP_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        **self == CALONPWRUP_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        **self == CALONPWRUP_A::EN
    }
}
impl core::ops::Deref for CALONPWRUP_R {
    type Target = crate::FieldReader<bool, CALONPWRUP_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CALONPWRUP` writer - Run ADC Calibration on initial power up sequence"]
pub struct CALONPWRUP_W<'a> {
    w: &'a mut W,
}
impl<'a> CALONPWRUP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CALONPWRUP_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable automatic calibration on initial power up value."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(CALONPWRUP_A::DIS)
    }
    #[doc = "Enable automatic calibration on initial power up value."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(CALONPWRUP_A::EN)
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
    #[doc = "Bit 1 - Status for ADC Calibration"]
    #[inline(always)]
    pub fn adccalibrated(&self) -> ADCCALIBRATED_R {
        ADCCALIBRATED_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Run ADC Calibration on initial power up sequence"]
    #[inline(always)]
    pub fn calonpwrup(&self) -> CALONPWRUP_R {
        CALONPWRUP_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Status for ADC Calibration"]
    #[inline(always)]
    pub fn adccalibrated(&mut self) -> ADCCALIBRATED_W {
        ADCCALIBRATED_W { w: self }
    }
    #[doc = "Bit 0 - Run ADC Calibration on initial power up sequence"]
    #[inline(always)]
    pub fn calonpwrup(&mut self) -> CALONPWRUP_W {
        CALONPWRUP_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ADC Calibration Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adccal](index.html) module"]
pub struct ADCCAL_SPEC;
impl crate::RegisterSpec for ADCCAL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [adccal::R](R) reader structure"]
impl crate::Readable for ADCCAL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adccal::W](W) writer structure"]
impl crate::Writable for ADCCAL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ADCCAL to value 0x01"]
impl crate::Resettable for ADCCAL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x01
    }
}
