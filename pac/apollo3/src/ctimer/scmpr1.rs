#[doc = "Register `SCMPR1` reader"]
pub struct R(crate::R<SCMPR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SCMPR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SCMPR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SCMPR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SCMPR1` writer"]
pub struct W(crate::W<SCMPR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SCMPR1_SPEC>;
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
impl From<crate::W<SCMPR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SCMPR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SCMPR1` reader - Compare this value to the value in the COUNTER register according to the match criterion, as selected in the COMPARE_B_EN bit in the REG_CTIMER_STCGF register."]
pub struct SCMPR1_R(crate::FieldReader<u32, u32>);
impl SCMPR1_R {
    pub(crate) fn new(bits: u32) -> Self {
        SCMPR1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SCMPR1_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SCMPR1` writer - Compare this value to the value in the COUNTER register according to the match criterion, as selected in the COMPARE_B_EN bit in the REG_CTIMER_STCGF register."]
pub struct SCMPR1_W<'a> {
    w: &'a mut W,
}
impl<'a> SCMPR1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | (value as u32 & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Compare this value to the value in the COUNTER register according to the match criterion, as selected in the COMPARE_B_EN bit in the REG_CTIMER_STCGF register."]
    #[inline(always)]
    pub fn scmpr1(&self) -> SCMPR1_R {
        SCMPR1_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Compare this value to the value in the COUNTER register according to the match criterion, as selected in the COMPARE_B_EN bit in the REG_CTIMER_STCGF register."]
    #[inline(always)]
    pub fn scmpr1(&mut self) -> SCMPR1_W {
        SCMPR1_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Compare Register B\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [scmpr1](index.html) module"]
pub struct SCMPR1_SPEC;
impl crate::RegisterSpec for SCMPR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [scmpr1::R](R) reader structure"]
impl crate::Readable for SCMPR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [scmpr1::W](W) writer structure"]
impl crate::Writable for SCMPR1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SCMPR1 to value 0"]
impl crate::Resettable for SCMPR1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
