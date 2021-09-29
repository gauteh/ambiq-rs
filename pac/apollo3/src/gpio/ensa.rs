#[doc = "Register `ENSA` reader"]
pub struct R(crate::R<ENSA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ENSA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ENSA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ENSA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ENSA` writer"]
pub struct W(crate::W<ENSA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ENSA_SPEC>;
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
impl From<crate::W<ENSA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ENSA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ENSA` reader - Set the GPIO31-0 output enables"]
pub struct ENSA_R(crate::FieldReader<u32, u32>);
impl ENSA_R {
    pub(crate) fn new(bits: u32) -> Self {
        ENSA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ENSA_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ENSA` writer - Set the GPIO31-0 output enables"]
pub struct ENSA_W<'a> {
    w: &'a mut W,
}
impl<'a> ENSA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | (value as u32 & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Set the GPIO31-0 output enables"]
    #[inline(always)]
    pub fn ensa(&self) -> ENSA_R {
        ENSA_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Set the GPIO31-0 output enables"]
    #[inline(always)]
    pub fn ensa(&mut self) -> ENSA_W {
        ENSA_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPIO Enable Register A Set\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ensa](index.html) module"]
pub struct ENSA_SPEC;
impl crate::RegisterSpec for ENSA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ensa::R](R) reader structure"]
impl crate::Readable for ENSA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ensa::W](W) writer structure"]
impl crate::Writable for ENSA_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ENSA to value 0"]
impl crate::Resettable for ENSA_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
