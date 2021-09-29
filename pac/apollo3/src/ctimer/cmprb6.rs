#[doc = "Register `CMPRB6` reader"]
pub struct R(crate::R<CMPRB6_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CMPRB6_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CMPRB6_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CMPRB6_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CMPRB6` writer"]
pub struct W(crate::W<CMPRB6_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CMPRB6_SPEC>;
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
impl From<crate::W<CMPRB6_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CMPRB6_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CMPR1B6` reader - Counter/Timer B6 Compare Register 1."]
pub struct CMPR1B6_R(crate::FieldReader<u16, u16>);
impl CMPR1B6_R {
    pub(crate) fn new(bits: u16) -> Self {
        CMPR1B6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CMPR1B6_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CMPR1B6` writer - Counter/Timer B6 Compare Register 1."]
pub struct CMPR1B6_W<'a> {
    w: &'a mut W,
}
impl<'a> CMPR1B6_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | ((value as u32 & 0xffff) << 16);
        self.w
    }
}
#[doc = "Field `CMPR0B6` reader - Counter/Timer B6 Compare Register 0."]
pub struct CMPR0B6_R(crate::FieldReader<u16, u16>);
impl CMPR0B6_R {
    pub(crate) fn new(bits: u16) -> Self {
        CMPR0B6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CMPR0B6_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CMPR0B6` writer - Counter/Timer B6 Compare Register 0."]
pub struct CMPR0B6_W<'a> {
    w: &'a mut W,
}
impl<'a> CMPR0B6_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 16:31 - Counter/Timer B6 Compare Register 1."]
    #[inline(always)]
    pub fn cmpr1b6(&self) -> CMPR1B6_R {
        CMPR1B6_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
    #[doc = "Bits 0:15 - Counter/Timer B6 Compare Register 0."]
    #[inline(always)]
    pub fn cmpr0b6(&self) -> CMPR0B6_R {
        CMPR0B6_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 16:31 - Counter/Timer B6 Compare Register 1."]
    #[inline(always)]
    pub fn cmpr1b6(&mut self) -> CMPR1B6_W {
        CMPR1B6_W { w: self }
    }
    #[doc = "Bits 0:15 - Counter/Timer B6 Compare Register 0."]
    #[inline(always)]
    pub fn cmpr0b6(&mut self) -> CMPR0B6_W {
        CMPR0B6_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Counter/Timer B6 Compare Registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmprb6](index.html) module"]
pub struct CMPRB6_SPEC;
impl crate::RegisterSpec for CMPRB6_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cmprb6::R](R) reader structure"]
impl crate::Readable for CMPRB6_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cmprb6::W](W) writer structure"]
impl crate::Writable for CMPRB6_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CMPRB6 to value 0"]
impl crate::Resettable for CMPRB6_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
