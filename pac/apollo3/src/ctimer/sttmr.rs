#[doc = "Register `STTMR` reader"]
pub struct R(crate::R<STTMR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STTMR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STTMR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STTMR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `STTMR` writer"]
pub struct W(crate::W<STTMR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<STTMR_SPEC>;
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
impl From<crate::W<STTMR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<STTMR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `STTMR` reader - Value of the 32-bit counter as it ticks over."]
pub struct STTMR_R(crate::FieldReader<u32, u32>);
impl STTMR_R {
    pub(crate) fn new(bits: u32) -> Self {
        STTMR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for STTMR_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `STTMR` writer - Value of the 32-bit counter as it ticks over."]
pub struct STTMR_W<'a> {
    w: &'a mut W,
}
impl<'a> STTMR_W<'a> {
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
    pub fn sttmr(&self) -> STTMR_R {
        STTMR_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Value of the 32-bit counter as it ticks over."]
    #[inline(always)]
    pub fn sttmr(&mut self) -> STTMR_W {
        STTMR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "System Timer Count Register (Real Time Counter)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sttmr](index.html) module"]
pub struct STTMR_SPEC;
impl crate::RegisterSpec for STTMR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sttmr::R](R) reader structure"]
impl crate::Readable for STTMR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sttmr::W](W) writer structure"]
impl crate::Writable for STTMR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets STTMR to value 0"]
impl crate::Resettable for STTMR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
