#[doc = "Register `CMPRA7` reader"]
pub struct R(crate::R<CMPRA7_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CMPRA7_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CMPRA7_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CMPRA7_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CMPRA7` writer"]
pub struct W(crate::W<CMPRA7_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CMPRA7_SPEC>;
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
impl From<crate::W<CMPRA7_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CMPRA7_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CMPR1A7` reader - Counter/Timer A7 Compare Register 1."]
pub struct CMPR1A7_R(crate::FieldReader<u16, u16>);
impl CMPR1A7_R {
    pub(crate) fn new(bits: u16) -> Self {
        CMPR1A7_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CMPR1A7_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CMPR1A7` writer - Counter/Timer A7 Compare Register 1."]
pub struct CMPR1A7_W<'a> {
    w: &'a mut W,
}
impl<'a> CMPR1A7_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | ((value as u32 & 0xffff) << 16);
        self.w
    }
}
#[doc = "Field `CMPR0A7` reader - Counter/Timer A7 Compare Register 0."]
pub struct CMPR0A7_R(crate::FieldReader<u16, u16>);
impl CMPR0A7_R {
    pub(crate) fn new(bits: u16) -> Self {
        CMPR0A7_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CMPR0A7_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CMPR0A7` writer - Counter/Timer A7 Compare Register 0."]
pub struct CMPR0A7_W<'a> {
    w: &'a mut W,
}
impl<'a> CMPR0A7_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 16:31 - Counter/Timer A7 Compare Register 1."]
    #[inline(always)]
    pub fn cmpr1a7(&self) -> CMPR1A7_R {
        CMPR1A7_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
    #[doc = "Bits 0:15 - Counter/Timer A7 Compare Register 0."]
    #[inline(always)]
    pub fn cmpr0a7(&self) -> CMPR0A7_R {
        CMPR0A7_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 16:31 - Counter/Timer A7 Compare Register 1."]
    #[inline(always)]
    pub fn cmpr1a7(&mut self) -> CMPR1A7_W {
        CMPR1A7_W { w: self }
    }
    #[doc = "Bits 0:15 - Counter/Timer A7 Compare Register 0."]
    #[inline(always)]
    pub fn cmpr0a7(&mut self) -> CMPR0A7_W {
        CMPR0A7_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Counter/Timer A7 Compare Registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmpra7](index.html) module"]
pub struct CMPRA7_SPEC;
impl crate::RegisterSpec for CMPRA7_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cmpra7::R](R) reader structure"]
impl crate::Readable for CMPRA7_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cmpra7::W](W) writer structure"]
impl crate::Writable for CMPRA7_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CMPRA7 to value 0"]
impl crate::Resettable for CMPRA7_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
