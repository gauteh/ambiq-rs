#[doc = "Register `ILPR` reader"]
pub struct R(crate::R<ILPR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ILPR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ILPR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ILPR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ILPR` writer"]
pub struct W(crate::W<ILPR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ILPR_SPEC>;
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
impl From<crate::W<ILPR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ILPR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ILPDVSR` reader - These bits hold the IrDA counter divisor."]
pub struct ILPDVSR_R(crate::FieldReader<u8, u8>);
impl ILPDVSR_R {
    pub(crate) fn new(bits: u8) -> Self {
        ILPDVSR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ILPDVSR_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ILPDVSR` writer - These bits hold the IrDA counter divisor."]
pub struct ILPDVSR_W<'a> {
    w: &'a mut W,
}
impl<'a> ILPDVSR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - These bits hold the IrDA counter divisor."]
    #[inline(always)]
    pub fn ilpdvsr(&self) -> ILPDVSR_R {
        ILPDVSR_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - These bits hold the IrDA counter divisor."]
    #[inline(always)]
    pub fn ilpdvsr(&mut self) -> ILPDVSR_W {
        ILPDVSR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "IrDA Counter\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ilpr](index.html) module"]
pub struct ILPR_SPEC;
impl crate::RegisterSpec for ILPR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ilpr::R](R) reader structure"]
impl crate::Readable for ILPR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ilpr::W](W) writer structure"]
impl crate::Writable for ILPR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ILPR to value 0"]
impl crate::Resettable for ILPR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
