#[doc = "Register `CQCURIDX` reader"]
pub struct R(crate::R<CQCURIDX_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CQCURIDX_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CQCURIDX_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CQCURIDX_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CQCURIDX` writer"]
pub struct W(crate::W<CQCURIDX_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CQCURIDX_SPEC>;
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
impl From<crate::W<CQCURIDX_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CQCURIDX_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CQCURIDX` reader - Can be used to indicate the current position of the command queue by having CQ operations write this field. A CQ hardware status flag indicates when CURIDX and ENDIDX are not equal, allowing SW to pause the CQ processing until the end index is updated."]
pub struct CQCURIDX_R(crate::FieldReader<u8, u8>);
impl CQCURIDX_R {
    pub(crate) fn new(bits: u8) -> Self {
        CQCURIDX_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CQCURIDX_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CQCURIDX` writer - Can be used to indicate the current position of the command queue by having CQ operations write this field. A CQ hardware status flag indicates when CURIDX and ENDIDX are not equal, allowing SW to pause the CQ processing until the end index is updated."]
pub struct CQCURIDX_W<'a> {
    w: &'a mut W,
}
impl<'a> CQCURIDX_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Can be used to indicate the current position of the command queue by having CQ operations write this field. A CQ hardware status flag indicates when CURIDX and ENDIDX are not equal, allowing SW to pause the CQ processing until the end index is updated."]
    #[inline(always)]
    pub fn cqcuridx(&self) -> CQCURIDX_R {
        CQCURIDX_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Can be used to indicate the current position of the command queue by having CQ operations write this field. A CQ hardware status flag indicates when CURIDX and ENDIDX are not equal, allowing SW to pause the CQ processing until the end index is updated."]
    #[inline(always)]
    pub fn cqcuridx(&mut self) -> CQCURIDX_W {
        CQCURIDX_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Command Queue Current Index\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cqcuridx](index.html) module"]
pub struct CQCURIDX_SPEC;
impl crate::RegisterSpec for CQCURIDX_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cqcuridx::R](R) reader structure"]
impl crate::Readable for CQCURIDX_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cqcuridx::W](W) writer structure"]
impl crate::Writable for CQCURIDX_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CQCURIDX to value 0"]
impl crate::Resettable for CQCURIDX_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
