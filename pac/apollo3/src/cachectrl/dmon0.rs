#[doc = "Register `DMON0` reader"]
pub struct R(crate::R<DMON0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMON0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMON0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMON0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DMON0` writer"]
pub struct W(crate::W<DMON0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMON0_SPEC>;
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
impl From<crate::W<DMON0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DMON0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DACCESS_COUNT` reader - Total accesses to data cache. All performance metrics should be relative to the number of accesses performed."]
pub struct DACCESS_COUNT_R(crate::FieldReader<u32, u32>);
impl DACCESS_COUNT_R {
    pub(crate) fn new(bits: u32) -> Self {
        DACCESS_COUNT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DACCESS_COUNT_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DACCESS_COUNT` writer - Total accesses to data cache. All performance metrics should be relative to the number of accesses performed."]
pub struct DACCESS_COUNT_W<'a> {
    w: &'a mut W,
}
impl<'a> DACCESS_COUNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | (value as u32 & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Total accesses to data cache. All performance metrics should be relative to the number of accesses performed."]
    #[inline(always)]
    pub fn daccess_count(&self) -> DACCESS_COUNT_R {
        DACCESS_COUNT_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Total accesses to data cache. All performance metrics should be relative to the number of accesses performed."]
    #[inline(always)]
    pub fn daccess_count(&mut self) -> DACCESS_COUNT_W {
        DACCESS_COUNT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Data Cache Total Accesses\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmon0](index.html) module"]
pub struct DMON0_SPEC;
impl crate::RegisterSpec for DMON0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dmon0::R](R) reader structure"]
impl crate::Readable for DMON0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dmon0::W](W) writer structure"]
impl crate::Writable for DMON0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DMON0 to value 0"]
impl crate::Resettable for DMON0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
