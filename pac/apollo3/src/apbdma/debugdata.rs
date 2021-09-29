#[doc = "Register `DEBUGDATA` reader"]
pub struct R(crate::R<DEBUGDATA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DEBUGDATA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DEBUGDATA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DEBUGDATA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DEBUGDATA` writer"]
pub struct W(crate::W<DEBUGDATA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DEBUGDATA_SPEC>;
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
impl From<crate::W<DEBUGDATA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DEBUGDATA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DEBUGDATA` reader - Debug Data"]
pub struct DEBUGDATA_R(crate::FieldReader<u32, u32>);
impl DEBUGDATA_R {
    pub(crate) fn new(bits: u32) -> Self {
        DEBUGDATA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DEBUGDATA_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DEBUGDATA` writer - Debug Data"]
pub struct DEBUGDATA_W<'a> {
    w: &'a mut W,
}
impl<'a> DEBUGDATA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | (value as u32 & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Debug Data"]
    #[inline(always)]
    pub fn debugdata(&self) -> DEBUGDATA_R {
        DEBUGDATA_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Debug Data"]
    #[inline(always)]
    pub fn debugdata(&mut self) -> DEBUGDATA_W {
        DEBUGDATA_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PIO Input Values\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [debugdata](index.html) module"]
pub struct DEBUGDATA_SPEC;
impl crate::RegisterSpec for DEBUGDATA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [debugdata::R](R) reader structure"]
impl crate::Readable for DEBUGDATA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [debugdata::W](W) writer structure"]
impl crate::Writable for DEBUGDATA_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DEBUGDATA to value 0"]
impl crate::Resettable for DEBUGDATA_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
