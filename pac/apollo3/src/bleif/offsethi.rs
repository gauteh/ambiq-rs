#[doc = "Register `OFFSETHI` reader"]
pub struct R(crate::R<OFFSETHI_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OFFSETHI_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OFFSETHI_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OFFSETHI_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OFFSETHI` writer"]
pub struct W(crate::W<OFFSETHI_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OFFSETHI_SPEC>;
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
impl From<crate::W<OFFSETHI_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OFFSETHI_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OFFSETHI` reader - Holds the high order bytes of the 2 or 3 byte offset phase of a transaction."]
pub struct OFFSETHI_R(crate::FieldReader<u16, u16>);
impl OFFSETHI_R {
    pub(crate) fn new(bits: u16) -> Self {
        OFFSETHI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OFFSETHI_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OFFSETHI` writer - Holds the high order bytes of the 2 or 3 byte offset phase of a transaction."]
pub struct OFFSETHI_W<'a> {
    w: &'a mut W,
}
impl<'a> OFFSETHI_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Holds the high order bytes of the 2 or 3 byte offset phase of a transaction."]
    #[inline(always)]
    pub fn offsethi(&self) -> OFFSETHI_R {
        OFFSETHI_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Holds the high order bytes of the 2 or 3 byte offset phase of a transaction."]
    #[inline(always)]
    pub fn offsethi(&mut self) -> OFFSETHI_W {
        OFFSETHI_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "High order offset bytes\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [offsethi](index.html) module"]
pub struct OFFSETHI_SPEC;
impl crate::RegisterSpec for OFFSETHI_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [offsethi::R](R) reader structure"]
impl crate::Readable for OFFSETHI_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [offsethi::W](W) writer structure"]
impl crate::Writable for OFFSETHI_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets OFFSETHI to value 0"]
impl crate::Resettable for OFFSETHI_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
