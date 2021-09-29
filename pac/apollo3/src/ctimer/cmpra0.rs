#[doc = "Register `CMPRA0` reader"]
pub struct R(crate::R<CMPRA0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CMPRA0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CMPRA0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CMPRA0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CMPRA0` writer"]
pub struct W(crate::W<CMPRA0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CMPRA0_SPEC>;
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
impl From<crate::W<CMPRA0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CMPRA0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CMPR1A0` reader - Counter/Timer A0 Compare Register 1. Holds the upper limit for timer half A."]
pub struct CMPR1A0_R(crate::FieldReader<u16, u16>);
impl CMPR1A0_R {
    pub(crate) fn new(bits: u16) -> Self {
        CMPR1A0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CMPR1A0_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CMPR1A0` writer - Counter/Timer A0 Compare Register 1. Holds the upper limit for timer half A."]
pub struct CMPR1A0_W<'a> {
    w: &'a mut W,
}
impl<'a> CMPR1A0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | ((value as u32 & 0xffff) << 16);
        self.w
    }
}
#[doc = "Field `CMPR0A0` reader - Counter/Timer A0 Compare Register 0. Holds the lower limit for timer half A."]
pub struct CMPR0A0_R(crate::FieldReader<u16, u16>);
impl CMPR0A0_R {
    pub(crate) fn new(bits: u16) -> Self {
        CMPR0A0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CMPR0A0_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CMPR0A0` writer - Counter/Timer A0 Compare Register 0. Holds the lower limit for timer half A."]
pub struct CMPR0A0_W<'a> {
    w: &'a mut W,
}
impl<'a> CMPR0A0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 16:31 - Counter/Timer A0 Compare Register 1. Holds the upper limit for timer half A."]
    #[inline(always)]
    pub fn cmpr1a0(&self) -> CMPR1A0_R {
        CMPR1A0_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
    #[doc = "Bits 0:15 - Counter/Timer A0 Compare Register 0. Holds the lower limit for timer half A."]
    #[inline(always)]
    pub fn cmpr0a0(&self) -> CMPR0A0_R {
        CMPR0A0_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 16:31 - Counter/Timer A0 Compare Register 1. Holds the upper limit for timer half A."]
    #[inline(always)]
    pub fn cmpr1a0(&mut self) -> CMPR1A0_W {
        CMPR1A0_W { w: self }
    }
    #[doc = "Bits 0:15 - Counter/Timer A0 Compare Register 0. Holds the lower limit for timer half A."]
    #[inline(always)]
    pub fn cmpr0a0(&mut self) -> CMPR0A0_W {
        CMPR0A0_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Counter/Timer A0 Compare Registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmpra0](index.html) module"]
pub struct CMPRA0_SPEC;
impl crate::RegisterSpec for CMPRA0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cmpra0::R](R) reader structure"]
impl crate::Readable for CMPRA0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cmpra0::W](W) writer structure"]
impl crate::Writable for CMPRA0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CMPRA0 to value 0"]
impl crate::Resettable for CMPRA0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
