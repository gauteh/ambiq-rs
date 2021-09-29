#[doc = "Register `SCMPR5` reader"]
pub struct R(crate::R<SCMPR5_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SCMPR5_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SCMPR5_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SCMPR5_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SCMPR5` writer"]
pub struct W(crate::W<SCMPR5_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SCMPR5_SPEC>;
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
impl From<crate::W<SCMPR5_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SCMPR5_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SCMPR5` reader - Compare this value to the value in the COUNTER register according to the match criterion, as selected in the COMPARE_F_EN bit in the REG_CTIMER_STCGF register."]
pub struct SCMPR5_R(crate::FieldReader<u32, u32>);
impl SCMPR5_R {
    pub(crate) fn new(bits: u32) -> Self {
        SCMPR5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SCMPR5_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SCMPR5` writer - Compare this value to the value in the COUNTER register according to the match criterion, as selected in the COMPARE_F_EN bit in the REG_CTIMER_STCGF register."]
pub struct SCMPR5_W<'a> {
    w: &'a mut W,
}
impl<'a> SCMPR5_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | (value as u32 & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Compare this value to the value in the COUNTER register according to the match criterion, as selected in the COMPARE_F_EN bit in the REG_CTIMER_STCGF register."]
    #[inline(always)]
    pub fn scmpr5(&self) -> SCMPR5_R {
        SCMPR5_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Compare this value to the value in the COUNTER register according to the match criterion, as selected in the COMPARE_F_EN bit in the REG_CTIMER_STCGF register."]
    #[inline(always)]
    pub fn scmpr5(&mut self) -> SCMPR5_W {
        SCMPR5_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Compare Register F\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [scmpr5](index.html) module"]
pub struct SCMPR5_SPEC;
impl crate::RegisterSpec for SCMPR5_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [scmpr5::R](R) reader structure"]
impl crate::Readable for SCMPR5_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [scmpr5::W](W) writer structure"]
impl crate::Writable for SCMPR5_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SCMPR5 to value 0"]
impl crate::Resettable for SCMPR5_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
