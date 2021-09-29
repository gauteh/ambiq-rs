#[doc = "Register `DCODEFAULTADDR` reader"]
pub struct R(crate::R<DCODEFAULTADDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DCODEFAULTADDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DCODEFAULTADDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DCODEFAULTADDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DCODEFAULTADDR` writer"]
pub struct W(crate::W<DCODEFAULTADDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DCODEFAULTADDR_SPEC>;
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
impl From<crate::W<DCODEFAULTADDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DCODEFAULTADDR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DCODEFAULTADDR` reader - The DCODE bus address observed when a Bus Fault occurred. Once an address is captured in this field, it is held until the corresponding Fault Observed bit is cleared in the FAULTSTATUS register."]
pub struct DCODEFAULTADDR_R(crate::FieldReader<u32, u32>);
impl DCODEFAULTADDR_R {
    pub(crate) fn new(bits: u32) -> Self {
        DCODEFAULTADDR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DCODEFAULTADDR_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DCODEFAULTADDR` writer - The DCODE bus address observed when a Bus Fault occurred. Once an address is captured in this field, it is held until the corresponding Fault Observed bit is cleared in the FAULTSTATUS register."]
pub struct DCODEFAULTADDR_W<'a> {
    w: &'a mut W,
}
impl<'a> DCODEFAULTADDR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | (value as u32 & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - The DCODE bus address observed when a Bus Fault occurred. Once an address is captured in this field, it is held until the corresponding Fault Observed bit is cleared in the FAULTSTATUS register."]
    #[inline(always)]
    pub fn dcodefaultaddr(&self) -> DCODEFAULTADDR_R {
        DCODEFAULTADDR_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - The DCODE bus address observed when a Bus Fault occurred. Once an address is captured in this field, it is held until the corresponding Fault Observed bit is cleared in the FAULTSTATUS register."]
    #[inline(always)]
    pub fn dcodefaultaddr(&mut self) -> DCODEFAULTADDR_W {
        DCODEFAULTADDR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DCODE bus address which was present when a bus fault occurred.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dcodefaultaddr](index.html) module"]
pub struct DCODEFAULTADDR_SPEC;
impl crate::RegisterSpec for DCODEFAULTADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dcodefaultaddr::R](R) reader structure"]
impl crate::Readable for DCODEFAULTADDR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dcodefaultaddr::W](W) writer structure"]
impl crate::Writable for DCODEFAULTADDR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DCODEFAULTADDR to value 0"]
impl crate::Resettable for DCODEFAULTADDR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
