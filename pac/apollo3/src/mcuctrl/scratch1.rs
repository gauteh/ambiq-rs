#[doc = "Register `SCRATCH1` reader"]
pub struct R(crate::R<SCRATCH1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SCRATCH1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SCRATCH1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SCRATCH1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SCRATCH1` writer"]
pub struct W(crate::W<SCRATCH1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SCRATCH1_SPEC>;
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
impl From<crate::W<SCRATCH1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SCRATCH1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SCRATCH1` reader - Scratch register 1."]
pub struct SCRATCH1_R(crate::FieldReader<u32, u32>);
impl SCRATCH1_R {
    pub(crate) fn new(bits: u32) -> Self {
        SCRATCH1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SCRATCH1_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SCRATCH1` writer - Scratch register 1."]
pub struct SCRATCH1_W<'a> {
    w: &'a mut W,
}
impl<'a> SCRATCH1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | (value as u32 & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Scratch register 1."]
    #[inline(always)]
    pub fn scratch1(&self) -> SCRATCH1_R {
        SCRATCH1_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Scratch register 1."]
    #[inline(always)]
    pub fn scratch1(&mut self) -> SCRATCH1_W {
        SCRATCH1_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Scratch register that is not reset by any reset\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [scratch1](index.html) module"]
pub struct SCRATCH1_SPEC;
impl crate::RegisterSpec for SCRATCH1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [scratch1::R](R) reader structure"]
impl crate::Readable for SCRATCH1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [scratch1::W](W) writer structure"]
impl crate::Writable for SCRATCH1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SCRATCH1 to value 0"]
impl crate::Resettable for SCRATCH1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
