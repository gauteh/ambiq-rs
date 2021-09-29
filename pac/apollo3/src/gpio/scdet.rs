#[doc = "Register `SCDET` reader"]
pub struct R(crate::R<SCDET_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SCDET_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SCDET_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SCDET_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SCDET` writer"]
pub struct W(crate::W<SCDET_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SCDET_SPEC>;
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
impl From<crate::W<SCDET_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SCDET_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SCDET` reader - SCARD card detect pad select."]
pub struct SCDET_R(crate::FieldReader<u8, u8>);
impl SCDET_R {
    pub(crate) fn new(bits: u8) -> Self {
        SCDET_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SCDET_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SCDET` writer - SCARD card detect pad select."]
pub struct SCDET_W<'a> {
    w: &'a mut W,
}
impl<'a> SCDET_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | (value as u32 & 0x3f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:5 - SCARD card detect pad select."]
    #[inline(always)]
    pub fn scdet(&self) -> SCDET_R {
        SCDET_R::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - SCARD card detect pad select."]
    #[inline(always)]
    pub fn scdet(&mut self) -> SCDET_W {
        SCDET_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SCARD Card Detect select\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [scdet](index.html) module"]
pub struct SCDET_SPEC;
impl crate::RegisterSpec for SCDET_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [scdet::R](R) reader structure"]
impl crate::Readable for SCDET_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [scdet::W](W) writer structure"]
impl crate::Writable for SCDET_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SCDET to value 0x3f"]
impl crate::Resettable for SCDET_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x3f
    }
}
