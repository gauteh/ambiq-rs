#[doc = "Register `IMON3` reader"]
pub struct R(crate::R<IMON3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IMON3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IMON3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IMON3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IMON3` writer"]
pub struct W(crate::W<IMON3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IMON3_SPEC>;
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
impl From<crate::W<IMON3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IMON3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ILINE_COUNT` reader - Cache hits from line cache"]
pub struct ILINE_COUNT_R(crate::FieldReader<u32, u32>);
impl ILINE_COUNT_R {
    pub(crate) fn new(bits: u32) -> Self {
        ILINE_COUNT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ILINE_COUNT_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ILINE_COUNT` writer - Cache hits from line cache"]
pub struct ILINE_COUNT_W<'a> {
    w: &'a mut W,
}
impl<'a> ILINE_COUNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | (value as u32 & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Cache hits from line cache"]
    #[inline(always)]
    pub fn iline_count(&self) -> ILINE_COUNT_R {
        ILINE_COUNT_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Cache hits from line cache"]
    #[inline(always)]
    pub fn iline_count(&mut self) -> ILINE_COUNT_W {
        ILINE_COUNT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Instruction Cache Line Hits\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [imon3](index.html) module"]
pub struct IMON3_SPEC;
impl crate::RegisterSpec for IMON3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [imon3::R](R) reader structure"]
impl crate::Readable for IMON3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [imon3::W](W) writer structure"]
impl crate::Writable for IMON3_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets IMON3 to value 0"]
impl crate::Resettable for IMON3_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
