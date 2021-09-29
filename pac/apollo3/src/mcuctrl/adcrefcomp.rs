#[doc = "Register `ADCREFCOMP` reader"]
pub struct R(crate::R<ADCREFCOMP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADCREFCOMP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADCREFCOMP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADCREFCOMP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADCREFCOMP` writer"]
pub struct W(crate::W<ADCREFCOMP_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADCREFCOMP_SPEC>;
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
impl From<crate::W<ADCREFCOMP_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADCREFCOMP_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADCRFCMPEN` reader - ADC Reference comparator power down"]
pub struct ADCRFCMPEN_R(crate::FieldReader<bool, bool>);
impl ADCRFCMPEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        ADCRFCMPEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADCRFCMPEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADCRFCMPEN` writer - ADC Reference comparator power down"]
pub struct ADCRFCMPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> ADCRFCMPEN_W<'a> {
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
#[doc = "Field `ADCREFKEEPTRIM` reader - ADC Reference Keeper Trim"]
pub struct ADCREFKEEPTRIM_R(crate::FieldReader<u8, u8>);
impl ADCREFKEEPTRIM_R {
    pub(crate) fn new(bits: u8) -> Self {
        ADCREFKEEPTRIM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADCREFKEEPTRIM_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADCREFKEEPTRIM` writer - ADC Reference Keeper Trim"]
pub struct ADCREFKEEPTRIM_W<'a> {
    w: &'a mut W,
}
impl<'a> ADCREFKEEPTRIM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 8)) | ((value as u32 & 0x1f) << 8);
        self.w
    }
}
#[doc = "Field `ADC_REFCOMP_OUT` reader - Output of the ADC reference comparator"]
pub struct ADC_REFCOMP_OUT_R(crate::FieldReader<bool, bool>);
impl ADC_REFCOMP_OUT_R {
    pub(crate) fn new(bits: bool) -> Self {
        ADC_REFCOMP_OUT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADC_REFCOMP_OUT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADC_REFCOMP_OUT` writer - Output of the ADC reference comparator"]
pub struct ADC_REFCOMP_OUT_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC_REFCOMP_OUT_W<'a> {
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
    #[doc = "Bit 16 - ADC Reference comparator power down"]
    #[inline(always)]
    pub fn adcrfcmpen(&self) -> ADCRFCMPEN_R {
        ADCRFCMPEN_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bits 8:12 - ADC Reference Keeper Trim"]
    #[inline(always)]
    pub fn adcrefkeeptrim(&self) -> ADCREFKEEPTRIM_R {
        ADCREFKEEPTRIM_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bit 0 - Output of the ADC reference comparator"]
    #[inline(always)]
    pub fn adc_refcomp_out(&self) -> ADC_REFCOMP_OUT_R {
        ADC_REFCOMP_OUT_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 16 - ADC Reference comparator power down"]
    #[inline(always)]
    pub fn adcrfcmpen(&mut self) -> ADCRFCMPEN_W {
        ADCRFCMPEN_W { w: self }
    }
    #[doc = "Bits 8:12 - ADC Reference Keeper Trim"]
    #[inline(always)]
    pub fn adcrefkeeptrim(&mut self) -> ADCREFKEEPTRIM_W {
        ADCREFKEEPTRIM_W { w: self }
    }
    #[doc = "Bit 0 - Output of the ADC reference comparator"]
    #[inline(always)]
    pub fn adc_refcomp_out(&mut self) -> ADC_REFCOMP_OUT_W {
        ADC_REFCOMP_OUT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ADC Referece Keeper and Comparator Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adcrefcomp](index.html) module"]
pub struct ADCREFCOMP_SPEC;
impl crate::RegisterSpec for ADCREFCOMP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [adcrefcomp::R](R) reader structure"]
impl crate::Readable for ADCREFCOMP_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adcrefcomp::W](W) writer structure"]
impl crate::Writable for ADCREFCOMP_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ADCREFCOMP to value 0"]
impl crate::Resettable for ADCREFCOMP_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
