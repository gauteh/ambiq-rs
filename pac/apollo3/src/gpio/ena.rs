#[doc = "Register `ENA` reader"]
pub struct R(crate::R<ENA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ENA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ENA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ENA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ENA` writer"]
pub struct W(crate::W<ENA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ENA_SPEC>;
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
impl From<crate::W<ENA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ENA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ENA` reader - GPIO31-0 output enables"]
pub struct ENA_R(crate::FieldReader<u32, u32>);
impl ENA_R {
    pub(crate) fn new(bits: u32) -> Self {
        ENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ENA_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ENA` writer - GPIO31-0 output enables"]
pub struct ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> ENA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | (value as u32 & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - GPIO31-0 output enables"]
    #[inline(always)]
    pub fn ena(&self) -> ENA_R {
        ENA_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - GPIO31-0 output enables"]
    #[inline(always)]
    pub fn ena(&mut self) -> ENA_W {
        ENA_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPIO Enable Register A\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ena](index.html) module"]
pub struct ENA_SPEC;
impl crate::RegisterSpec for ENA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ena::R](R) reader structure"]
impl crate::Readable for ENA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ena::W](W) writer structure"]
impl crate::Writable for ENA_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ENA to value 0"]
impl crate::Resettable for ENA_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
