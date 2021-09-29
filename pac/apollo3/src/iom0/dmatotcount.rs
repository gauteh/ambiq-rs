#[doc = "Register `DMATOTCOUNT` reader"]
pub struct R(crate::R<DMATOTCOUNT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMATOTCOUNT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMATOTCOUNT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMATOTCOUNT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DMATOTCOUNT` writer"]
pub struct W(crate::W<DMATOTCOUNT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMATOTCOUNT_SPEC>;
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
impl From<crate::W<DMATOTCOUNT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DMATOTCOUNT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TOTCOUNT` reader - Triggered DMA from Command complete event occured. Bit is read only and can be cleared by disabling the DTHR trigger enable or by disabling DMA."]
pub struct TOTCOUNT_R(crate::FieldReader<u16, u16>);
impl TOTCOUNT_R {
    pub(crate) fn new(bits: u16) -> Self {
        TOTCOUNT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TOTCOUNT_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TOTCOUNT` writer - Triggered DMA from Command complete event occured. Bit is read only and can be cleared by disabling the DTHR trigger enable or by disabling DMA."]
pub struct TOTCOUNT_W<'a> {
    w: &'a mut W,
}
impl<'a> TOTCOUNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0fff) | (value as u32 & 0x0fff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:11 - Triggered DMA from Command complete event occured. Bit is read only and can be cleared by disabling the DTHR trigger enable or by disabling DMA."]
    #[inline(always)]
    pub fn totcount(&self) -> TOTCOUNT_R {
        TOTCOUNT_R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - Triggered DMA from Command complete event occured. Bit is read only and can be cleared by disabling the DTHR trigger enable or by disabling DMA."]
    #[inline(always)]
    pub fn totcount(&mut self) -> TOTCOUNT_W {
        TOTCOUNT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DMA Total Transfer Count\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmatotcount](index.html) module"]
pub struct DMATOTCOUNT_SPEC;
impl crate::RegisterSpec for DMATOTCOUNT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dmatotcount::R](R) reader structure"]
impl crate::Readable for DMATOTCOUNT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dmatotcount::W](W) writer structure"]
impl crate::Writable for DMATOTCOUNT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DMATOTCOUNT to value 0"]
impl crate::Resettable for DMATOTCOUNT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
