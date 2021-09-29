#[doc = "Register `WULIM` reader"]
pub struct R(crate::R<WULIM_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WULIM_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WULIM_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WULIM_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `WULIM` writer"]
pub struct W(crate::W<WULIM_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WULIM_SPEC>;
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
impl From<crate::W<WULIM_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WULIM_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ULIM` reader - Sets the upper limit for the window comparator."]
pub struct ULIM_R(crate::FieldReader<u32, u32>);
impl ULIM_R {
    pub(crate) fn new(bits: u32) -> Self {
        ULIM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ULIM_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ULIM` writer - Sets the upper limit for the window comparator."]
pub struct ULIM_W<'a> {
    w: &'a mut W,
}
impl<'a> ULIM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x000f_ffff) | (value as u32 & 0x000f_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:19 - Sets the upper limit for the window comparator."]
    #[inline(always)]
    pub fn ulim(&self) -> ULIM_R {
        ULIM_R::new((self.bits & 0x000f_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:19 - Sets the upper limit for the window comparator."]
    #[inline(always)]
    pub fn ulim(&mut self) -> ULIM_W {
        ULIM_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Window Comparator Upper Limits Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wulim](index.html) module"]
pub struct WULIM_SPEC;
impl crate::RegisterSpec for WULIM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [wulim::R](R) reader structure"]
impl crate::Readable for WULIM_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [wulim::W](W) writer structure"]
impl crate::Writable for WULIM_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets WULIM to value 0"]
impl crate::Resettable for WULIM_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
