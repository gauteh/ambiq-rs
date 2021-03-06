#[doc = "Register `DMON2` reader"]
pub struct R(crate::R<DMON2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMON2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMON2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMON2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DMON2` writer"]
pub struct W(crate::W<DMON2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMON2_SPEC>;
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
impl From<crate::W<DMON2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DMON2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DHIT_COUNT` reader - Cache hits from lookup operations."]
pub struct DHIT_COUNT_R(crate::FieldReader<u32, u32>);
impl DHIT_COUNT_R {
    pub(crate) fn new(bits: u32) -> Self {
        DHIT_COUNT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DHIT_COUNT_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DHIT_COUNT` writer - Cache hits from lookup operations."]
pub struct DHIT_COUNT_W<'a> {
    w: &'a mut W,
}
impl<'a> DHIT_COUNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | (value as u32 & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Cache hits from lookup operations."]
    #[inline(always)]
    pub fn dhit_count(&self) -> DHIT_COUNT_R {
        DHIT_COUNT_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Cache hits from lookup operations."]
    #[inline(always)]
    pub fn dhit_count(&mut self) -> DHIT_COUNT_W {
        DHIT_COUNT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Data Cache Hits\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmon2](index.html) module"]
pub struct DMON2_SPEC;
impl crate::RegisterSpec for DMON2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dmon2::R](R) reader structure"]
impl crate::Readable for DMON2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dmon2::W](W) writer structure"]
impl crate::Writable for DMON2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DMON2 to value 0"]
impl crate::Resettable for DMON2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
