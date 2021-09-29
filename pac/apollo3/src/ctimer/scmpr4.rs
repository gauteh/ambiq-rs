#[doc = "Register `SCMPR4` reader"]
pub struct R(crate::R<SCMPR4_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SCMPR4_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SCMPR4_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SCMPR4_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SCMPR4` writer"]
pub struct W(crate::W<SCMPR4_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SCMPR4_SPEC>;
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
impl From<crate::W<SCMPR4_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SCMPR4_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SCMPR4` reader - Compare this value to the value in the COUNTER register according to the match criterion, as selected in the COMPARE_E_EN bit in the REG_CTIMER_STCGF register."]
pub struct SCMPR4_R(crate::FieldReader<u32, u32>);
impl SCMPR4_R {
    pub(crate) fn new(bits: u32) -> Self {
        SCMPR4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SCMPR4_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SCMPR4` writer - Compare this value to the value in the COUNTER register according to the match criterion, as selected in the COMPARE_E_EN bit in the REG_CTIMER_STCGF register."]
pub struct SCMPR4_W<'a> {
    w: &'a mut W,
}
impl<'a> SCMPR4_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | (value as u32 & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Compare this value to the value in the COUNTER register according to the match criterion, as selected in the COMPARE_E_EN bit in the REG_CTIMER_STCGF register."]
    #[inline(always)]
    pub fn scmpr4(&self) -> SCMPR4_R {
        SCMPR4_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Compare this value to the value in the COUNTER register according to the match criterion, as selected in the COMPARE_E_EN bit in the REG_CTIMER_STCGF register."]
    #[inline(always)]
    pub fn scmpr4(&mut self) -> SCMPR4_W {
        SCMPR4_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Compare Register E\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [scmpr4](index.html) module"]
pub struct SCMPR4_SPEC;
impl crate::RegisterSpec for SCMPR4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [scmpr4::R](R) reader structure"]
impl crate::Readable for SCMPR4_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [scmpr4::W](W) writer structure"]
impl crate::Writable for SCMPR4_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SCMPR4 to value 0"]
impl crate::Resettable for SCMPR4_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
