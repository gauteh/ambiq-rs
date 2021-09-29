#[doc = "Register `CMPRA4` reader"]
pub struct R(crate::R<CMPRA4_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CMPRA4_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CMPRA4_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CMPRA4_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CMPRA4` writer"]
pub struct W(crate::W<CMPRA4_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CMPRA4_SPEC>;
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
impl From<crate::W<CMPRA4_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CMPRA4_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CMPR1A4` reader - Counter/Timer A4 Compare Register 1. Holds the upper limit for timer half A."]
pub struct CMPR1A4_R(crate::FieldReader<u16, u16>);
impl CMPR1A4_R {
    pub(crate) fn new(bits: u16) -> Self {
        CMPR1A4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CMPR1A4_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CMPR1A4` writer - Counter/Timer A4 Compare Register 1. Holds the upper limit for timer half A."]
pub struct CMPR1A4_W<'a> {
    w: &'a mut W,
}
impl<'a> CMPR1A4_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | ((value as u32 & 0xffff) << 16);
        self.w
    }
}
#[doc = "Field `CMPR0A4` reader - Counter/Timer A4 Compare Register 0. Holds the lower limit for timer half A."]
pub struct CMPR0A4_R(crate::FieldReader<u16, u16>);
impl CMPR0A4_R {
    pub(crate) fn new(bits: u16) -> Self {
        CMPR0A4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CMPR0A4_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CMPR0A4` writer - Counter/Timer A4 Compare Register 0. Holds the lower limit for timer half A."]
pub struct CMPR0A4_W<'a> {
    w: &'a mut W,
}
impl<'a> CMPR0A4_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 16:31 - Counter/Timer A4 Compare Register 1. Holds the upper limit for timer half A."]
    #[inline(always)]
    pub fn cmpr1a4(&self) -> CMPR1A4_R {
        CMPR1A4_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
    #[doc = "Bits 0:15 - Counter/Timer A4 Compare Register 0. Holds the lower limit for timer half A."]
    #[inline(always)]
    pub fn cmpr0a4(&self) -> CMPR0A4_R {
        CMPR0A4_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 16:31 - Counter/Timer A4 Compare Register 1. Holds the upper limit for timer half A."]
    #[inline(always)]
    pub fn cmpr1a4(&mut self) -> CMPR1A4_W {
        CMPR1A4_W { w: self }
    }
    #[doc = "Bits 0:15 - Counter/Timer A4 Compare Register 0. Holds the lower limit for timer half A."]
    #[inline(always)]
    pub fn cmpr0a4(&mut self) -> CMPR0A4_W {
        CMPR0A4_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Counter/Timer A4 Compare Registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmpra4](index.html) module"]
pub struct CMPRA4_SPEC;
impl crate::RegisterSpec for CMPRA4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cmpra4::R](R) reader structure"]
impl crate::Readable for CMPRA4_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cmpra4::W](W) writer structure"]
impl crate::Writable for CMPRA4_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CMPRA4 to value 0"]
impl crate::Resettable for CMPRA4_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
