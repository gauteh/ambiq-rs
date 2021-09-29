#[doc = "Register `WTCA` reader"]
pub struct R(crate::R<WTCA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WTCA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WTCA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WTCA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `WTCA` writer"]
pub struct W(crate::W<WTCA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WTCA_SPEC>;
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
impl From<crate::W<WTCA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WTCA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WTCA` reader - Clear the GPIO31-0 write data."]
pub struct WTCA_R(crate::FieldReader<u32, u32>);
impl WTCA_R {
    pub(crate) fn new(bits: u32) -> Self {
        WTCA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WTCA_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WTCA` writer - Clear the GPIO31-0 write data."]
pub struct WTCA_W<'a> {
    w: &'a mut W,
}
impl<'a> WTCA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | (value as u32 & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Clear the GPIO31-0 write data."]
    #[inline(always)]
    pub fn wtca(&self) -> WTCA_R {
        WTCA_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Clear the GPIO31-0 write data."]
    #[inline(always)]
    pub fn wtca(&mut self) -> WTCA_W {
        WTCA_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPIO Output Register A Clear\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wtca](index.html) module"]
pub struct WTCA_SPEC;
impl crate::RegisterSpec for WTCA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [wtca::R](R) reader structure"]
impl crate::Readable for WTCA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [wtca::W](W) writer structure"]
impl crate::Writable for WTCA_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets WTCA to value 0"]
impl crate::Resettable for WTCA_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
