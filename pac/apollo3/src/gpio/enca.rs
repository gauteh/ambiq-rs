#[doc = "Register `ENCA` reader"]
pub struct R(crate::R<ENCA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ENCA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ENCA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ENCA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ENCA` writer"]
pub struct W(crate::W<ENCA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ENCA_SPEC>;
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
impl From<crate::W<ENCA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ENCA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ENCA` reader - Clear the GPIO31-0 output enables"]
pub struct ENCA_R(crate::FieldReader<u32, u32>);
impl ENCA_R {
    pub(crate) fn new(bits: u32) -> Self {
        ENCA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ENCA_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ENCA` writer - Clear the GPIO31-0 output enables"]
pub struct ENCA_W<'a> {
    w: &'a mut W,
}
impl<'a> ENCA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | (value as u32 & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Clear the GPIO31-0 output enables"]
    #[inline(always)]
    pub fn enca(&self) -> ENCA_R {
        ENCA_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Clear the GPIO31-0 output enables"]
    #[inline(always)]
    pub fn enca(&mut self) -> ENCA_W {
        ENCA_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPIO Enable Register A Clear\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [enca](index.html) module"]
pub struct ENCA_SPEC;
impl crate::RegisterSpec for ENCA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [enca::R](R) reader structure"]
impl crate::Readable for ENCA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [enca::W](W) writer structure"]
impl crate::Writable for ENCA_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ENCA to value 0"]
impl crate::Resettable for ENCA_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
