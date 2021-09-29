#[doc = "Register `DMON1` reader"]
pub struct R(crate::R<DMON1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMON1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMON1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMON1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DMON1` writer"]
pub struct W(crate::W<DMON1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMON1_SPEC>;
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
impl From<crate::W<DMON1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DMON1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DLOOKUP_COUNT` reader - Total tag lookups from data cache."]
pub struct DLOOKUP_COUNT_R(crate::FieldReader<u32, u32>);
impl DLOOKUP_COUNT_R {
    pub(crate) fn new(bits: u32) -> Self {
        DLOOKUP_COUNT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DLOOKUP_COUNT_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DLOOKUP_COUNT` writer - Total tag lookups from data cache."]
pub struct DLOOKUP_COUNT_W<'a> {
    w: &'a mut W,
}
impl<'a> DLOOKUP_COUNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | (value as u32 & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Total tag lookups from data cache."]
    #[inline(always)]
    pub fn dlookup_count(&self) -> DLOOKUP_COUNT_R {
        DLOOKUP_COUNT_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Total tag lookups from data cache."]
    #[inline(always)]
    pub fn dlookup_count(&mut self) -> DLOOKUP_COUNT_W {
        DLOOKUP_COUNT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Data Cache Tag Lookups\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmon1](index.html) module"]
pub struct DMON1_SPEC;
impl crate::RegisterSpec for DMON1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dmon1::R](R) reader structure"]
impl crate::Readable for DMON1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dmon1::W](W) writer structure"]
impl crate::Writable for DMON1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DMON1 to value 0"]
impl crate::Resettable for DMON1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
