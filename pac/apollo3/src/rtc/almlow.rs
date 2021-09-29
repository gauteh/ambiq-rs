#[doc = "Register `ALMLOW` reader"]
pub struct R(crate::R<ALMLOW_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ALMLOW_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ALMLOW_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ALMLOW_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ALMLOW` writer"]
pub struct W(crate::W<ALMLOW_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ALMLOW_SPEC>;
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
impl From<crate::W<ALMLOW_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ALMLOW_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ALMHR` reader - Hours Alarm"]
pub struct ALMHR_R(crate::FieldReader<u8, u8>);
impl ALMHR_R {
    pub(crate) fn new(bits: u8) -> Self {
        ALMHR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ALMHR_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ALMHR` writer - Hours Alarm"]
pub struct ALMHR_W<'a> {
    w: &'a mut W,
}
impl<'a> ALMHR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 24)) | ((value as u32 & 0x3f) << 24);
        self.w
    }
}
#[doc = "Field `ALMMIN` reader - Minutes Alarm"]
pub struct ALMMIN_R(crate::FieldReader<u8, u8>);
impl ALMMIN_R {
    pub(crate) fn new(bits: u8) -> Self {
        ALMMIN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ALMMIN_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ALMMIN` writer - Minutes Alarm"]
pub struct ALMMIN_W<'a> {
    w: &'a mut W,
}
impl<'a> ALMMIN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 16)) | ((value as u32 & 0x7f) << 16);
        self.w
    }
}
#[doc = "Field `ALMSEC` reader - Seconds Alarm"]
pub struct ALMSEC_R(crate::FieldReader<u8, u8>);
impl ALMSEC_R {
    pub(crate) fn new(bits: u8) -> Self {
        ALMSEC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ALMSEC_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ALMSEC` writer - Seconds Alarm"]
pub struct ALMSEC_W<'a> {
    w: &'a mut W,
}
impl<'a> ALMSEC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 8)) | ((value as u32 & 0x7f) << 8);
        self.w
    }
}
#[doc = "Field `ALM100` reader - 100ths of a second Alarm"]
pub struct ALM100_R(crate::FieldReader<u8, u8>);
impl ALM100_R {
    pub(crate) fn new(bits: u8) -> Self {
        ALM100_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ALM100_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ALM100` writer - 100ths of a second Alarm"]
pub struct ALM100_W<'a> {
    w: &'a mut W,
}
impl<'a> ALM100_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 24:29 - Hours Alarm"]
    #[inline(always)]
    pub fn almhr(&self) -> ALMHR_R {
        ALMHR_R::new(((self.bits >> 24) & 0x3f) as u8)
    }
    #[doc = "Bits 16:22 - Minutes Alarm"]
    #[inline(always)]
    pub fn almmin(&self) -> ALMMIN_R {
        ALMMIN_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bits 8:14 - Seconds Alarm"]
    #[inline(always)]
    pub fn almsec(&self) -> ALMSEC_R {
        ALMSEC_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bits 0:7 - 100ths of a second Alarm"]
    #[inline(always)]
    pub fn alm100(&self) -> ALM100_R {
        ALM100_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 24:29 - Hours Alarm"]
    #[inline(always)]
    pub fn almhr(&mut self) -> ALMHR_W {
        ALMHR_W { w: self }
    }
    #[doc = "Bits 16:22 - Minutes Alarm"]
    #[inline(always)]
    pub fn almmin(&mut self) -> ALMMIN_W {
        ALMMIN_W { w: self }
    }
    #[doc = "Bits 8:14 - Seconds Alarm"]
    #[inline(always)]
    pub fn almsec(&mut self) -> ALMSEC_W {
        ALMSEC_W { w: self }
    }
    #[doc = "Bits 0:7 - 100ths of a second Alarm"]
    #[inline(always)]
    pub fn alm100(&mut self) -> ALM100_W {
        ALM100_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RTC Alarms Lower\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [almlow](index.html) module"]
pub struct ALMLOW_SPEC;
impl crate::RegisterSpec for ALMLOW_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [almlow::R](R) reader structure"]
impl crate::Readable for ALMLOW_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [almlow::W](W) writer structure"]
impl crate::Writable for ALMLOW_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ALMLOW to value 0"]
impl crate::Resettable for ALMLOW_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
