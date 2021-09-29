#[doc = "Register `CLKKEY` reader"]
pub struct R(crate::R<CLKKEY_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CLKKEY_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CLKKEY_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CLKKEY_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CLKKEY` writer"]
pub struct W(crate::W<CLKKEY_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CLKKEY_SPEC>;
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
impl From<crate::W<CLKKEY_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CLKKEY_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Key register value.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u32)]
pub enum CLKKEY_A {
    #[doc = "71: Key value."]
    KEY = 71,
}
impl From<CLKKEY_A> for u32 {
    #[inline(always)]
    fn from(variant: CLKKEY_A) -> Self {
        variant as _
    }
}
#[doc = "Field `CLKKEY` reader - Key register value."]
pub struct CLKKEY_R(crate::FieldReader<u32, CLKKEY_A>);
impl CLKKEY_R {
    pub(crate) fn new(bits: u32) -> Self {
        CLKKEY_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CLKKEY_A> {
        match self.bits {
            71 => Some(CLKKEY_A::KEY),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `KEY`"]
    #[inline(always)]
    pub fn is_key(&self) -> bool {
        **self == CLKKEY_A::KEY
    }
}
impl core::ops::Deref for CLKKEY_R {
    type Target = crate::FieldReader<u32, CLKKEY_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CLKKEY` writer - Key register value."]
pub struct CLKKEY_W<'a> {
    w: &'a mut W,
}
impl<'a> CLKKEY_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CLKKEY_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Key value."]
    #[inline(always)]
    pub fn key(self) -> &'a mut W {
        self.variant(CLKKEY_A::KEY)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | (value as u32 & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Key register value."]
    #[inline(always)]
    pub fn clkkey(&self) -> CLKKEY_R {
        CLKKEY_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Key register value."]
    #[inline(always)]
    pub fn clkkey(&mut self) -> CLKKEY_W {
        CLKKEY_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Key Register for Clock Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clkkey](index.html) module"]
pub struct CLKKEY_SPEC;
impl crate::RegisterSpec for CLKKEY_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [clkkey::R](R) reader structure"]
impl crate::Readable for CLKKEY_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [clkkey::W](W) writer structure"]
impl crate::Writable for CLKKEY_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CLKKEY to value 0"]
impl crate::Resettable for CLKKEY_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
