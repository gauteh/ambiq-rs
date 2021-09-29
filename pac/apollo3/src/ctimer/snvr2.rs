#[doc = "Register `SNVR2` reader"]
pub struct R(crate::R<SNVR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SNVR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SNVR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SNVR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SNVR2` writer"]
pub struct W(crate::W<SNVR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SNVR2_SPEC>;
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
impl From<crate::W<SNVR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SNVR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SNVR2` reader - Value of the 32-bit counter as it ticks over."]
pub struct SNVR2_R(crate::FieldReader<u32, u32>);
impl SNVR2_R {
    pub(crate) fn new(bits: u32) -> Self {
        SNVR2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SNVR2_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SNVR2` writer - Value of the 32-bit counter as it ticks over."]
pub struct SNVR2_W<'a> {
    w: &'a mut W,
}
impl<'a> SNVR2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | (value as u32 & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Value of the 32-bit counter as it ticks over."]
    #[inline(always)]
    pub fn snvr2(&self) -> SNVR2_R {
        SNVR2_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Value of the 32-bit counter as it ticks over."]
    #[inline(always)]
    pub fn snvr2(&mut self) -> SNVR2_W {
        SNVR2_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "System Timer NVRAM_C Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [snvr2](index.html) module"]
pub struct SNVR2_SPEC;
impl crate::RegisterSpec for SNVR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [snvr2::R](R) reader structure"]
impl crate::Readable for SNVR2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [snvr2::W](W) writer structure"]
impl crate::Writable for SNVR2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SNVR2 to value 0"]
impl crate::Resettable for SNVR2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
