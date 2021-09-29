#[doc = "Register `SCMPR0` reader"]
pub struct R(crate::R<SCMPR0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SCMPR0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SCMPR0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SCMPR0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SCMPR0` writer"]
pub struct W(crate::W<SCMPR0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SCMPR0_SPEC>;
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
impl From<crate::W<SCMPR0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SCMPR0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SCMPR0` reader - Compare this value to the value in the COUNTER register according to the match criterion, as selected in the COMPARE_A_EN bit in the REG_CTIMER_STCGF register."]
pub struct SCMPR0_R(crate::FieldReader<u32, u32>);
impl SCMPR0_R {
    pub(crate) fn new(bits: u32) -> Self {
        SCMPR0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SCMPR0_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SCMPR0` writer - Compare this value to the value in the COUNTER register according to the match criterion, as selected in the COMPARE_A_EN bit in the REG_CTIMER_STCGF register."]
pub struct SCMPR0_W<'a> {
    w: &'a mut W,
}
impl<'a> SCMPR0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | (value as u32 & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Compare this value to the value in the COUNTER register according to the match criterion, as selected in the COMPARE_A_EN bit in the REG_CTIMER_STCGF register."]
    #[inline(always)]
    pub fn scmpr0(&self) -> SCMPR0_R {
        SCMPR0_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Compare this value to the value in the COUNTER register according to the match criterion, as selected in the COMPARE_A_EN bit in the REG_CTIMER_STCGF register."]
    #[inline(always)]
    pub fn scmpr0(&mut self) -> SCMPR0_W {
        SCMPR0_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Compare Register A\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [scmpr0](index.html) module"]
pub struct SCMPR0_SPEC;
impl crate::RegisterSpec for SCMPR0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [scmpr0::R](R) reader structure"]
impl crate::Readable for SCMPR0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [scmpr0::W](W) writer structure"]
impl crate::Writable for SCMPR0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SCMPR0 to value 0"]
impl crate::Resettable for SCMPR0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
