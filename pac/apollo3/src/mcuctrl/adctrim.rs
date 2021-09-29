#[doc = "Register `ADCTRIM` reader"]
pub struct R(crate::R<ADCTRIM_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADCTRIM_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADCTRIM_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADCTRIM_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADCTRIM` writer"]
pub struct W(crate::W<ADCTRIM_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADCTRIM_SPEC>;
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
impl From<crate::W<ADCTRIM_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADCTRIM_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADCRFBUFIBTRIM` reader - ADC reference buffer input bias trim"]
pub struct ADCRFBUFIBTRIM_R(crate::FieldReader<u8, u8>);
impl ADCRFBUFIBTRIM_R {
    pub(crate) fn new(bits: u8) -> Self {
        ADCRFBUFIBTRIM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADCRFBUFIBTRIM_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADCRFBUFIBTRIM` writer - ADC reference buffer input bias trim"]
pub struct ADCRFBUFIBTRIM_W<'a> {
    w: &'a mut W,
}
impl<'a> ADCRFBUFIBTRIM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 11)) | ((value as u32 & 0x03) << 11);
        self.w
    }
}
#[doc = "Field `ADCREFBUFTRIM` reader - ADC Reference buffer trim"]
pub struct ADCREFBUFTRIM_R(crate::FieldReader<u8, u8>);
impl ADCREFBUFTRIM_R {
    pub(crate) fn new(bits: u8) -> Self {
        ADCREFBUFTRIM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADCREFBUFTRIM_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADCREFBUFTRIM` writer - ADC Reference buffer trim"]
pub struct ADCREFBUFTRIM_W<'a> {
    w: &'a mut W,
}
impl<'a> ADCREFBUFTRIM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 6)) | ((value as u32 & 0x1f) << 6);
        self.w
    }
}
#[doc = "Field `ADCREFKEEPIBTRIM` reader - ADC Reference Ibias trim"]
pub struct ADCREFKEEPIBTRIM_R(crate::FieldReader<u8, u8>);
impl ADCREFKEEPIBTRIM_R {
    pub(crate) fn new(bits: u8) -> Self {
        ADCREFKEEPIBTRIM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADCREFKEEPIBTRIM_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADCREFKEEPIBTRIM` writer - ADC Reference Ibias trim"]
pub struct ADCREFKEEPIBTRIM_W<'a> {
    w: &'a mut W,
}
impl<'a> ADCREFKEEPIBTRIM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bits 11:12 - ADC reference buffer input bias trim"]
    #[inline(always)]
    pub fn adcrfbufibtrim(&self) -> ADCRFBUFIBTRIM_R {
        ADCRFBUFIBTRIM_R::new(((self.bits >> 11) & 0x03) as u8)
    }
    #[doc = "Bits 6:10 - ADC Reference buffer trim"]
    #[inline(always)]
    pub fn adcrefbuftrim(&self) -> ADCREFBUFTRIM_R {
        ADCREFBUFTRIM_R::new(((self.bits >> 6) & 0x1f) as u8)
    }
    #[doc = "Bits 0:1 - ADC Reference Ibias trim"]
    #[inline(always)]
    pub fn adcrefkeepibtrim(&self) -> ADCREFKEEPIBTRIM_R {
        ADCREFKEEPIBTRIM_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 11:12 - ADC reference buffer input bias trim"]
    #[inline(always)]
    pub fn adcrfbufibtrim(&mut self) -> ADCRFBUFIBTRIM_W {
        ADCRFBUFIBTRIM_W { w: self }
    }
    #[doc = "Bits 6:10 - ADC Reference buffer trim"]
    #[inline(always)]
    pub fn adcrefbuftrim(&mut self) -> ADCREFBUFTRIM_W {
        ADCREFBUFTRIM_W { w: self }
    }
    #[doc = "Bits 0:1 - ADC Reference Ibias trim"]
    #[inline(always)]
    pub fn adcrefkeepibtrim(&mut self) -> ADCREFKEEPIBTRIM_W {
        ADCREFKEEPIBTRIM_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ADC Trims\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adctrim](index.html) module"]
pub struct ADCTRIM_SPEC;
impl crate::RegisterSpec for ADCTRIM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [adctrim::R](R) reader structure"]
impl crate::Readable for ADCTRIM_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adctrim::W](W) writer structure"]
impl crate::Writable for ADCTRIM_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ADCTRIM to value 0x0200"]
impl crate::Resettable for ADCTRIM_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0200
    }
}
