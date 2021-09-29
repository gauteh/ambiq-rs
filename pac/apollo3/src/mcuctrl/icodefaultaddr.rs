#[doc = "Register `ICODEFAULTADDR` reader"]
pub struct R(crate::R<ICODEFAULTADDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ICODEFAULTADDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ICODEFAULTADDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ICODEFAULTADDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ICODEFAULTADDR` writer"]
pub struct W(crate::W<ICODEFAULTADDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ICODEFAULTADDR_SPEC>;
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
impl From<crate::W<ICODEFAULTADDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ICODEFAULTADDR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ICODEFAULTADDR` reader - The ICODE bus address observed when a Bus Fault occurred. Once an address is captured in this field, it is held until the corresponding Fault Observed bit is cleared in the FAULTSTATUS register."]
pub struct ICODEFAULTADDR_R(crate::FieldReader<u32, u32>);
impl ICODEFAULTADDR_R {
    pub(crate) fn new(bits: u32) -> Self {
        ICODEFAULTADDR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ICODEFAULTADDR_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ICODEFAULTADDR` writer - The ICODE bus address observed when a Bus Fault occurred. Once an address is captured in this field, it is held until the corresponding Fault Observed bit is cleared in the FAULTSTATUS register."]
pub struct ICODEFAULTADDR_W<'a> {
    w: &'a mut W,
}
impl<'a> ICODEFAULTADDR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | (value as u32 & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - The ICODE bus address observed when a Bus Fault occurred. Once an address is captured in this field, it is held until the corresponding Fault Observed bit is cleared in the FAULTSTATUS register."]
    #[inline(always)]
    pub fn icodefaultaddr(&self) -> ICODEFAULTADDR_R {
        ICODEFAULTADDR_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - The ICODE bus address observed when a Bus Fault occurred. Once an address is captured in this field, it is held until the corresponding Fault Observed bit is cleared in the FAULTSTATUS register."]
    #[inline(always)]
    pub fn icodefaultaddr(&mut self) -> ICODEFAULTADDR_W {
        ICODEFAULTADDR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ICODE bus address which was present when a bus fault occurred.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [icodefaultaddr](index.html) module"]
pub struct ICODEFAULTADDR_SPEC;
impl crate::RegisterSpec for ICODEFAULTADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [icodefaultaddr::R](R) reader structure"]
impl crate::Readable for ICODEFAULTADDR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [icodefaultaddr::W](W) writer structure"]
impl crate::Writable for ICODEFAULTADDR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ICODEFAULTADDR to value 0"]
impl crate::Resettable for ICODEFAULTADDR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
