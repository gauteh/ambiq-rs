#[doc = "Register `CTRLOW` reader"]
pub struct R(crate::R<CTRLOW_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTRLOW_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTRLOW_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTRLOW_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTRLOW` writer"]
pub struct W(crate::W<CTRLOW_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTRLOW_SPEC>;
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
impl From<crate::W<CTRLOW_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTRLOW_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CTRHR` reader - Hours Counter"]
pub struct CTRHR_R(crate::FieldReader<u8, u8>);
impl CTRHR_R {
    pub(crate) fn new(bits: u8) -> Self {
        CTRHR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CTRHR_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CTRHR` writer - Hours Counter"]
pub struct CTRHR_W<'a> {
    w: &'a mut W,
}
impl<'a> CTRHR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 24)) | ((value as u32 & 0x3f) << 24);
        self.w
    }
}
#[doc = "Field `CTRMIN` reader - Minutes Counter"]
pub struct CTRMIN_R(crate::FieldReader<u8, u8>);
impl CTRMIN_R {
    pub(crate) fn new(bits: u8) -> Self {
        CTRMIN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CTRMIN_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CTRMIN` writer - Minutes Counter"]
pub struct CTRMIN_W<'a> {
    w: &'a mut W,
}
impl<'a> CTRMIN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 16)) | ((value as u32 & 0x7f) << 16);
        self.w
    }
}
#[doc = "Field `CTRSEC` reader - Seconds Counter"]
pub struct CTRSEC_R(crate::FieldReader<u8, u8>);
impl CTRSEC_R {
    pub(crate) fn new(bits: u8) -> Self {
        CTRSEC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CTRSEC_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CTRSEC` writer - Seconds Counter"]
pub struct CTRSEC_W<'a> {
    w: &'a mut W,
}
impl<'a> CTRSEC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 8)) | ((value as u32 & 0x7f) << 8);
        self.w
    }
}
#[doc = "Field `CTR100` reader - 100ths of a second Counter"]
pub struct CTR100_R(crate::FieldReader<u8, u8>);
impl CTR100_R {
    pub(crate) fn new(bits: u8) -> Self {
        CTR100_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CTR100_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CTR100` writer - 100ths of a second Counter"]
pub struct CTR100_W<'a> {
    w: &'a mut W,
}
impl<'a> CTR100_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 24:29 - Hours Counter"]
    #[inline(always)]
    pub fn ctrhr(&self) -> CTRHR_R {
        CTRHR_R::new(((self.bits >> 24) & 0x3f) as u8)
    }
    #[doc = "Bits 16:22 - Minutes Counter"]
    #[inline(always)]
    pub fn ctrmin(&self) -> CTRMIN_R {
        CTRMIN_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bits 8:14 - Seconds Counter"]
    #[inline(always)]
    pub fn ctrsec(&self) -> CTRSEC_R {
        CTRSEC_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bits 0:7 - 100ths of a second Counter"]
    #[inline(always)]
    pub fn ctr100(&self) -> CTR100_R {
        CTR100_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 24:29 - Hours Counter"]
    #[inline(always)]
    pub fn ctrhr(&mut self) -> CTRHR_W {
        CTRHR_W { w: self }
    }
    #[doc = "Bits 16:22 - Minutes Counter"]
    #[inline(always)]
    pub fn ctrmin(&mut self) -> CTRMIN_W {
        CTRMIN_W { w: self }
    }
    #[doc = "Bits 8:14 - Seconds Counter"]
    #[inline(always)]
    pub fn ctrsec(&mut self) -> CTRSEC_W {
        CTRSEC_W { w: self }
    }
    #[doc = "Bits 0:7 - 100ths of a second Counter"]
    #[inline(always)]
    pub fn ctr100(&mut self) -> CTR100_W {
        CTR100_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RTC Counters Lower\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrlow](index.html) module"]
pub struct CTRLOW_SPEC;
impl crate::RegisterSpec for CTRLOW_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctrlow::R](R) reader structure"]
impl crate::Readable for CTRLOW_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctrlow::W](W) writer structure"]
impl crate::Writable for CTRLOW_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CTRLOW to value 0x0100_0000"]
impl crate::Resettable for CTRLOW_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0100_0000
    }
}
