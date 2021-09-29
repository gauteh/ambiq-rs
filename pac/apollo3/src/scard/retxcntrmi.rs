#[doc = "Register `RETXCNTRMI` reader"]
pub struct R(crate::R<RETXCNTRMI_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RETXCNTRMI_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RETXCNTRMI_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RETXCNTRMI_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RETXCNTRMI` writer"]
pub struct W(crate::W<RETXCNTRMI_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RETXCNTRMI_SPEC>;
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
impl From<crate::W<RETXCNTRMI_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RETXCNTRMI_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RETXCNTRMI` reader - Resent count inquiry register."]
pub struct RETXCNTRMI_R(crate::FieldReader<u8, u8>);
impl RETXCNTRMI_R {
    pub(crate) fn new(bits: u8) -> Self {
        RETXCNTRMI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RETXCNTRMI_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RETXCNTRMI` writer - Resent count inquiry register."]
pub struct RETXCNTRMI_W<'a> {
    w: &'a mut W,
}
impl<'a> RETXCNTRMI_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | (value as u32 & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - Resent count inquiry register."]
    #[inline(always)]
    pub fn retxcntrmi(&self) -> RETXCNTRMI_R {
        RETXCNTRMI_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Resent count inquiry register."]
    #[inline(always)]
    pub fn retxcntrmi(&mut self) -> RETXCNTRMI_W {
        RETXCNTRMI_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ISO7816 resent count inquiry\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [retxcntrmi](index.html) module"]
pub struct RETXCNTRMI_SPEC;
impl crate::RegisterSpec for RETXCNTRMI_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [retxcntrmi::R](R) reader structure"]
impl crate::Readable for RETXCNTRMI_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [retxcntrmi::W](W) writer structure"]
impl crate::Writable for RETXCNTRMI_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RETXCNTRMI to value 0"]
impl crate::Resettable for RETXCNTRMI_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
