#[doc = "Register `STATUS` reader"]
pub struct R(crate::R<STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `STATUS` writer"]
pub struct W(crate::W<STATUS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<STATUS_SPEC>;
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
impl From<crate::W<STATUS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<STATUS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OSCF` reader - XT Oscillator is enabled but not oscillating"]
pub struct OSCF_R(crate::FieldReader<bool, bool>);
impl OSCF_R {
    pub(crate) fn new(bits: bool) -> Self {
        OSCF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OSCF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OSCF` writer - XT Oscillator is enabled but not oscillating"]
pub struct OSCF_W<'a> {
    w: &'a mut W,
}
impl<'a> OSCF_W<'a> {
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
#[doc = "Field `OMODE` reader - Current RTC oscillator (1 => LFRC, 0 => XT). After an RTC oscillator change, it may take up to 2 seconds for this field to reflect the new oscillator."]
pub struct OMODE_R(crate::FieldReader<bool, bool>);
impl OMODE_R {
    pub(crate) fn new(bits: bool) -> Self {
        OMODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OMODE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OMODE` writer - Current RTC oscillator (1 => LFRC, 0 => XT). After an RTC oscillator change, it may take up to 2 seconds for this field to reflect the new oscillator."]
pub struct OMODE_W<'a> {
    w: &'a mut W,
}
impl<'a> OMODE_W<'a> {
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
    #[doc = "Bit 1 - XT Oscillator is enabled but not oscillating"]
    #[inline(always)]
    pub fn oscf(&self) -> OSCF_R {
        OSCF_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Current RTC oscillator (1 => LFRC, 0 => XT). After an RTC oscillator change, it may take up to 2 seconds for this field to reflect the new oscillator."]
    #[inline(always)]
    pub fn omode(&self) -> OMODE_R {
        OMODE_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - XT Oscillator is enabled but not oscillating"]
    #[inline(always)]
    pub fn oscf(&mut self) -> OSCF_W {
        OSCF_W { w: self }
    }
    #[doc = "Bit 0 - Current RTC oscillator (1 => LFRC, 0 => XT). After an RTC oscillator change, it may take up to 2 seconds for this field to reflect the new oscillator."]
    #[inline(always)]
    pub fn omode(&mut self) -> OMODE_W {
        OMODE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Clock Generator Status\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [status](index.html) module"]
pub struct STATUS_SPEC;
impl crate::RegisterSpec for STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [status::R](R) reader structure"]
impl crate::Readable for STATUS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [status::W](W) writer structure"]
impl crate::Writable for STATUS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets STATUS to value 0"]
impl crate::Resettable for STATUS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
