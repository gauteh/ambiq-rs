#[doc = "Register `IBRD` reader"]
pub struct R(crate::R<IBRD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IBRD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IBRD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IBRD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IBRD` writer"]
pub struct W(crate::W<IBRD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IBRD_SPEC>;
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
impl From<crate::W<IBRD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IBRD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DIVINT` reader - These bits hold the baud integer divisor."]
pub struct DIVINT_R(crate::FieldReader<u16, u16>);
impl DIVINT_R {
    pub(crate) fn new(bits: u16) -> Self {
        DIVINT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DIVINT_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DIVINT` writer - These bits hold the baud integer divisor."]
pub struct DIVINT_W<'a> {
    w: &'a mut W,
}
impl<'a> DIVINT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - These bits hold the baud integer divisor."]
    #[inline(always)]
    pub fn divint(&self) -> DIVINT_R {
        DIVINT_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - These bits hold the baud integer divisor."]
    #[inline(always)]
    pub fn divint(&mut self) -> DIVINT_W {
        DIVINT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Integer Baud Rate Divisor\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ibrd](index.html) module"]
pub struct IBRD_SPEC;
impl crate::RegisterSpec for IBRD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ibrd::R](R) reader structure"]
impl crate::Readable for IBRD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ibrd::W](W) writer structure"]
impl crate::Writable for IBRD_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets IBRD to value 0"]
impl crate::Resettable for IBRD_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
