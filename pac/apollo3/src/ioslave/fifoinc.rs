#[doc = "Register `FIFOINC` reader"]
pub struct R(crate::R<FIFOINC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FIFOINC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FIFOINC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FIFOINC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FIFOINC` writer"]
pub struct W(crate::W<FIFOINC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FIFOINC_SPEC>;
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
impl From<crate::W<FIFOINC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FIFOINC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FIFOINC` reader - Increment the Overall FIFO Counter by this value on a write"]
pub struct FIFOINC_R(crate::FieldReader<u16, u16>);
impl FIFOINC_R {
    pub(crate) fn new(bits: u16) -> Self {
        FIFOINC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FIFOINC_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FIFOINC` writer - Increment the Overall FIFO Counter by this value on a write"]
pub struct FIFOINC_W<'a> {
    w: &'a mut W,
}
impl<'a> FIFOINC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03ff) | (value as u32 & 0x03ff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:9 - Increment the Overall FIFO Counter by this value on a write"]
    #[inline(always)]
    pub fn fifoinc(&self) -> FIFOINC_R {
        FIFOINC_R::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - Increment the Overall FIFO Counter by this value on a write"]
    #[inline(always)]
    pub fn fifoinc(&mut self) -> FIFOINC_W {
        FIFOINC_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Overall FIFO Counter Increment\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fifoinc](index.html) module"]
pub struct FIFOINC_SPEC;
impl crate::RegisterSpec for FIFOINC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fifoinc::R](R) reader structure"]
impl crate::Readable for FIFOINC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fifoinc::W](W) writer structure"]
impl crate::Writable for FIFOINC_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FIFOINC to value 0"]
impl crate::Resettable for FIFOINC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
