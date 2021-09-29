#[doc = "Register `CMPRAUXB3` reader"]
pub struct R(crate::R<CMPRAUXB3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CMPRAUXB3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CMPRAUXB3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CMPRAUXB3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CMPRAUXB3` writer"]
pub struct W(crate::W<CMPRAUXB3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CMPRAUXB3_SPEC>;
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
impl From<crate::W<CMPRAUXB3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CMPRAUXB3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CMPR3B3` reader - Counter/Timer B3 Compare Register 3. Holds the upper limit for timer half B."]
pub struct CMPR3B3_R(crate::FieldReader<u16, u16>);
impl CMPR3B3_R {
    pub(crate) fn new(bits: u16) -> Self {
        CMPR3B3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CMPR3B3_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CMPR3B3` writer - Counter/Timer B3 Compare Register 3. Holds the upper limit for timer half B."]
pub struct CMPR3B3_W<'a> {
    w: &'a mut W,
}
impl<'a> CMPR3B3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | ((value as u32 & 0xffff) << 16);
        self.w
    }
}
#[doc = "Field `CMPR2B3` reader - Counter/Timer B3 Compare Register 2. Holds the lower limit for timer half B."]
pub struct CMPR2B3_R(crate::FieldReader<u16, u16>);
impl CMPR2B3_R {
    pub(crate) fn new(bits: u16) -> Self {
        CMPR2B3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CMPR2B3_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CMPR2B3` writer - Counter/Timer B3 Compare Register 2. Holds the lower limit for timer half B."]
pub struct CMPR2B3_W<'a> {
    w: &'a mut W,
}
impl<'a> CMPR2B3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 16:31 - Counter/Timer B3 Compare Register 3. Holds the upper limit for timer half B."]
    #[inline(always)]
    pub fn cmpr3b3(&self) -> CMPR3B3_R {
        CMPR3B3_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
    #[doc = "Bits 0:15 - Counter/Timer B3 Compare Register 2. Holds the lower limit for timer half B."]
    #[inline(always)]
    pub fn cmpr2b3(&self) -> CMPR2B3_R {
        CMPR2B3_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 16:31 - Counter/Timer B3 Compare Register 3. Holds the upper limit for timer half B."]
    #[inline(always)]
    pub fn cmpr3b3(&mut self) -> CMPR3B3_W {
        CMPR3B3_W { w: self }
    }
    #[doc = "Bits 0:15 - Counter/Timer B3 Compare Register 2. Holds the lower limit for timer half B."]
    #[inline(always)]
    pub fn cmpr2b3(&mut self) -> CMPR2B3_W {
        CMPR2B3_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Counter/Timer B3 Compare Registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmprauxb3](index.html) module"]
pub struct CMPRAUXB3_SPEC;
impl crate::RegisterSpec for CMPRAUXB3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cmprauxb3::R](R) reader structure"]
impl crate::Readable for CMPRAUXB3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cmprauxb3::W](W) writer structure"]
impl crate::Writable for CMPRAUXB3_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CMPRAUXB3 to value 0"]
impl crate::Resettable for CMPRAUXB3_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
