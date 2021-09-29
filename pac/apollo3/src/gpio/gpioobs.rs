#[doc = "Register `GPIOOBS` reader"]
pub struct R(crate::R<GPIOOBS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GPIOOBS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GPIOOBS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GPIOOBS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GPIOOBS` writer"]
pub struct W(crate::W<GPIOOBS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GPIOOBS_SPEC>;
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
impl From<crate::W<GPIOOBS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GPIOOBS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OBS_DATA` reader - Sample of the data output on the GPIO observation port. May have async sampling issues, as the data is not synronized to the read operation. Intended for debug purposes only"]
pub struct OBS_DATA_R(crate::FieldReader<u16, u16>);
impl OBS_DATA_R {
    pub(crate) fn new(bits: u16) -> Self {
        OBS_DATA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OBS_DATA_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OBS_DATA` writer - Sample of the data output on the GPIO observation port. May have async sampling issues, as the data is not synronized to the read operation. Intended for debug purposes only"]
pub struct OBS_DATA_W<'a> {
    w: &'a mut W,
}
impl<'a> OBS_DATA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Sample of the data output on the GPIO observation port. May have async sampling issues, as the data is not synronized to the read operation. Intended for debug purposes only"]
    #[inline(always)]
    pub fn obs_data(&self) -> OBS_DATA_R {
        OBS_DATA_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Sample of the data output on the GPIO observation port. May have async sampling issues, as the data is not synronized to the read operation. Intended for debug purposes only"]
    #[inline(always)]
    pub fn obs_data(&mut self) -> OBS_DATA_W {
        OBS_DATA_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPIO Observation Mode Sample register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpioobs](index.html) module"]
pub struct GPIOOBS_SPEC;
impl crate::RegisterSpec for GPIOOBS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gpioobs::R](R) reader structure"]
impl crate::Readable for GPIOOBS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gpioobs::W](W) writer structure"]
impl crate::Writable for GPIOOBS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets GPIOOBS to value 0"]
impl crate::Resettable for GPIOOBS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
