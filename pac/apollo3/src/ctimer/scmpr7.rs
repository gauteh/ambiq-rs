#[doc = "Register `SCMPR7` reader"]
pub struct R(crate::R<SCMPR7_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SCMPR7_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SCMPR7_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SCMPR7_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SCMPR7` writer"]
pub struct W(crate::W<SCMPR7_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SCMPR7_SPEC>;
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
impl From<crate::W<SCMPR7_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SCMPR7_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SCMPR7` reader - Compare this value to the value in the COUNTER register according to the match criterion, as selected in the COMPARE_H_EN bit in the REG_CTIMER_STCGF register."]
pub struct SCMPR7_R(crate::FieldReader<u32, u32>);
impl SCMPR7_R {
    pub(crate) fn new(bits: u32) -> Self {
        SCMPR7_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SCMPR7_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SCMPR7` writer - Compare this value to the value in the COUNTER register according to the match criterion, as selected in the COMPARE_H_EN bit in the REG_CTIMER_STCGF register."]
pub struct SCMPR7_W<'a> {
    w: &'a mut W,
}
impl<'a> SCMPR7_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | (value as u32 & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Compare this value to the value in the COUNTER register according to the match criterion, as selected in the COMPARE_H_EN bit in the REG_CTIMER_STCGF register."]
    #[inline(always)]
    pub fn scmpr7(&self) -> SCMPR7_R {
        SCMPR7_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Compare this value to the value in the COUNTER register according to the match criterion, as selected in the COMPARE_H_EN bit in the REG_CTIMER_STCGF register."]
    #[inline(always)]
    pub fn scmpr7(&mut self) -> SCMPR7_W {
        SCMPR7_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Compare Register H\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [scmpr7](index.html) module"]
pub struct SCMPR7_SPEC;
impl crate::RegisterSpec for SCMPR7_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [scmpr7::R](R) reader structure"]
impl crate::Readable for SCMPR7_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [scmpr7::W](W) writer structure"]
impl crate::Writable for SCMPR7_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SCMPR7 to value 0"]
impl crate::Resettable for SCMPR7_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
