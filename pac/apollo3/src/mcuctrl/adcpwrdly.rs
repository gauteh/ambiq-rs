#[doc = "Register `ADCPWRDLY` reader"]
pub struct R(crate::R<ADCPWRDLY_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADCPWRDLY_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADCPWRDLY_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADCPWRDLY_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADCPWRDLY` writer"]
pub struct W(crate::W<ADCPWRDLY_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADCPWRDLY_SPEC>;
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
impl From<crate::W<ADCPWRDLY_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADCPWRDLY_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADCPWR1` reader - ADC Reference Keeper enable delay in 16 ADC CLK increments for ADC_CLKSEL = 0x1, 8 ADC CLOCK increments for ADC_CLKSEL = 0x2."]
pub struct ADCPWR1_R(crate::FieldReader<u8, u8>);
impl ADCPWR1_R {
    pub(crate) fn new(bits: u8) -> Self {
        ADCPWR1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADCPWR1_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADCPWR1` writer - ADC Reference Keeper enable delay in 16 ADC CLK increments for ADC_CLKSEL = 0x1, 8 ADC CLOCK increments for ADC_CLKSEL = 0x2."]
pub struct ADCPWR1_W<'a> {
    w: &'a mut W,
}
impl<'a> ADCPWR1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | ((value as u32 & 0xff) << 8);
        self.w
    }
}
#[doc = "Field `ADCPWR0` reader - ADC Reference Buffer Power Enable delay in 64 ADC CLK increments for ADC_CLKSEL = 0x1, 32 ADC CLOCK increments for ADC_CLKSEL = 0x2."]
pub struct ADCPWR0_R(crate::FieldReader<u8, u8>);
impl ADCPWR0_R {
    pub(crate) fn new(bits: u8) -> Self {
        ADCPWR0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADCPWR0_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADCPWR0` writer - ADC Reference Buffer Power Enable delay in 64 ADC CLK increments for ADC_CLKSEL = 0x1, 32 ADC CLOCK increments for ADC_CLKSEL = 0x2."]
pub struct ADCPWR0_W<'a> {
    w: &'a mut W,
}
impl<'a> ADCPWR0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 8:15 - ADC Reference Keeper enable delay in 16 ADC CLK increments for ADC_CLKSEL = 0x1, 8 ADC CLOCK increments for ADC_CLKSEL = 0x2."]
    #[inline(always)]
    pub fn adcpwr1(&self) -> ADCPWR1_R {
        ADCPWR1_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 0:7 - ADC Reference Buffer Power Enable delay in 64 ADC CLK increments for ADC_CLKSEL = 0x1, 32 ADC CLOCK increments for ADC_CLKSEL = 0x2."]
    #[inline(always)]
    pub fn adcpwr0(&self) -> ADCPWR0_R {
        ADCPWR0_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 8:15 - ADC Reference Keeper enable delay in 16 ADC CLK increments for ADC_CLKSEL = 0x1, 8 ADC CLOCK increments for ADC_CLKSEL = 0x2."]
    #[inline(always)]
    pub fn adcpwr1(&mut self) -> ADCPWR1_W {
        ADCPWR1_W { w: self }
    }
    #[doc = "Bits 0:7 - ADC Reference Buffer Power Enable delay in 64 ADC CLK increments for ADC_CLKSEL = 0x1, 32 ADC CLOCK increments for ADC_CLKSEL = 0x2."]
    #[inline(always)]
    pub fn adcpwr0(&mut self) -> ADCPWR0_W {
        ADCPWR0_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ADC Power Up Delay Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adcpwrdly](index.html) module"]
pub struct ADCPWRDLY_SPEC;
impl crate::RegisterSpec for ADCPWRDLY_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [adcpwrdly::R](R) reader structure"]
impl crate::Readable for ADCPWRDLY_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adcpwrdly::W](W) writer structure"]
impl crate::Writable for ADCPWRDLY_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ADCPWRDLY to value 0"]
impl crate::Resettable for ADCPWRDLY_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
