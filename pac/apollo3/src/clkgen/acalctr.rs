#[doc = "Register `ACALCTR` reader"]
pub struct R(crate::R<ACALCTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ACALCTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ACALCTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ACALCTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ACALCTR` writer"]
pub struct W(crate::W<ACALCTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ACALCTR_SPEC>;
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
impl From<crate::W<ACALCTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ACALCTR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ACALCTR` reader - Autocalibration Counter result. Bits 17 down to 0 of this is feed directly to the CALRC register if ACAL register in OCTRL register is set to 1024SEC or 512SEC."]
pub struct ACALCTR_R(crate::FieldReader<u32, u32>);
impl ACALCTR_R {
    pub(crate) fn new(bits: u32) -> Self {
        ACALCTR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ACALCTR_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ACALCTR` writer - Autocalibration Counter result. Bits 17 down to 0 of this is feed directly to the CALRC register if ACAL register in OCTRL register is set to 1024SEC or 512SEC."]
pub struct ACALCTR_W<'a> {
    w: &'a mut W,
}
impl<'a> ACALCTR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x00ff_ffff) | (value as u32 & 0x00ff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:23 - Autocalibration Counter result. Bits 17 down to 0 of this is feed directly to the CALRC register if ACAL register in OCTRL register is set to 1024SEC or 512SEC."]
    #[inline(always)]
    pub fn acalctr(&self) -> ACALCTR_R {
        ACALCTR_R::new((self.bits & 0x00ff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:23 - Autocalibration Counter result. Bits 17 down to 0 of this is feed directly to the CALRC register if ACAL register in OCTRL register is set to 1024SEC or 512SEC."]
    #[inline(always)]
    pub fn acalctr(&mut self) -> ACALCTR_W {
        ACALCTR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Autocalibration Counter\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [acalctr](index.html) module"]
pub struct ACALCTR_SPEC;
impl crate::RegisterSpec for ACALCTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [acalctr::R](R) reader structure"]
impl crate::Readable for ACALCTR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [acalctr::W](W) writer structure"]
impl crate::Writable for ACALCTR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ACALCTR to value 0"]
impl crate::Resettable for ACALCTR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
