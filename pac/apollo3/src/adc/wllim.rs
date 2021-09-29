#[doc = "Register `WLLIM` reader"]
pub struct R(crate::R<WLLIM_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WLLIM_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WLLIM_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WLLIM_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `WLLIM` writer"]
pub struct W(crate::W<WLLIM_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WLLIM_SPEC>;
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
impl From<crate::W<WLLIM_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WLLIM_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LLIM` reader - Sets the lower limit for the window comparator."]
pub struct LLIM_R(crate::FieldReader<u32, u32>);
impl LLIM_R {
    pub(crate) fn new(bits: u32) -> Self {
        LLIM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LLIM_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LLIM` writer - Sets the lower limit for the window comparator."]
pub struct LLIM_W<'a> {
    w: &'a mut W,
}
impl<'a> LLIM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x000f_ffff) | (value as u32 & 0x000f_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:19 - Sets the lower limit for the window comparator."]
    #[inline(always)]
    pub fn llim(&self) -> LLIM_R {
        LLIM_R::new((self.bits & 0x000f_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:19 - Sets the lower limit for the window comparator."]
    #[inline(always)]
    pub fn llim(&mut self) -> LLIM_W {
        LLIM_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Window Comparator Lower Limits Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wllim](index.html) module"]
pub struct WLLIM_SPEC;
impl crate::RegisterSpec for WLLIM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [wllim::R](R) reader structure"]
impl crate::Readable for WLLIM_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [wllim::W](W) writer structure"]
impl crate::Writable for WLLIM_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets WLLIM to value 0"]
impl crate::Resettable for WLLIM_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
