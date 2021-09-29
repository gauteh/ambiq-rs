#[doc = "Register `CMPRAUXB1` reader"]
pub struct R(crate::R<CMPRAUXB1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CMPRAUXB1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CMPRAUXB1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CMPRAUXB1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CMPRAUXB1` writer"]
pub struct W(crate::W<CMPRAUXB1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CMPRAUXB1_SPEC>;
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
impl From<crate::W<CMPRAUXB1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CMPRAUXB1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CMPR3B1` reader - Counter/Timer B1 Compare Register 3. Holds the upper limit for timer half B."]
pub struct CMPR3B1_R(crate::FieldReader<u16, u16>);
impl CMPR3B1_R {
    pub(crate) fn new(bits: u16) -> Self {
        CMPR3B1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CMPR3B1_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CMPR3B1` writer - Counter/Timer B1 Compare Register 3. Holds the upper limit for timer half B."]
pub struct CMPR3B1_W<'a> {
    w: &'a mut W,
}
impl<'a> CMPR3B1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | ((value as u32 & 0xffff) << 16);
        self.w
    }
}
#[doc = "Field `CMPR2B1` reader - Counter/Timer B1 Compare Register 2. Holds the lower limit for timer half B."]
pub struct CMPR2B1_R(crate::FieldReader<u16, u16>);
impl CMPR2B1_R {
    pub(crate) fn new(bits: u16) -> Self {
        CMPR2B1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CMPR2B1_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CMPR2B1` writer - Counter/Timer B1 Compare Register 2. Holds the lower limit for timer half B."]
pub struct CMPR2B1_W<'a> {
    w: &'a mut W,
}
impl<'a> CMPR2B1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 16:31 - Counter/Timer B1 Compare Register 3. Holds the upper limit for timer half B."]
    #[inline(always)]
    pub fn cmpr3b1(&self) -> CMPR3B1_R {
        CMPR3B1_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
    #[doc = "Bits 0:15 - Counter/Timer B1 Compare Register 2. Holds the lower limit for timer half B."]
    #[inline(always)]
    pub fn cmpr2b1(&self) -> CMPR2B1_R {
        CMPR2B1_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 16:31 - Counter/Timer B1 Compare Register 3. Holds the upper limit for timer half B."]
    #[inline(always)]
    pub fn cmpr3b1(&mut self) -> CMPR3B1_W {
        CMPR3B1_W { w: self }
    }
    #[doc = "Bits 0:15 - Counter/Timer B1 Compare Register 2. Holds the lower limit for timer half B."]
    #[inline(always)]
    pub fn cmpr2b1(&mut self) -> CMPR2B1_W {
        CMPR2B1_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Counter/Timer B1 Compare Registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmprauxb1](index.html) module"]
pub struct CMPRAUXB1_SPEC;
impl crate::RegisterSpec for CMPRAUXB1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cmprauxb1::R](R) reader structure"]
impl crate::Readable for CMPRAUXB1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cmprauxb1::W](W) writer structure"]
impl crate::Writable for CMPRAUXB1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CMPRAUXB1 to value 0"]
impl crate::Resettable for CMPRAUXB1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
