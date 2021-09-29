#[doc = "Register `PADKEY` reader"]
pub struct R(crate::R<PADKEY_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PADKEY_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PADKEY_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PADKEY_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PADKEY` writer"]
pub struct W(crate::W<PADKEY_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PADKEY_SPEC>;
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
impl From<crate::W<PADKEY_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PADKEY_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Key register value.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u32)]
pub enum PADKEY_A {
    #[doc = "115: Key value."]
    KEY = 115,
}
impl From<PADKEY_A> for u32 {
    #[inline(always)]
    fn from(variant: PADKEY_A) -> Self {
        variant as _
    }
}
#[doc = "Field `PADKEY` reader - Key register value."]
pub struct PADKEY_R(crate::FieldReader<u32, PADKEY_A>);
impl PADKEY_R {
    pub(crate) fn new(bits: u32) -> Self {
        PADKEY_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PADKEY_A> {
        match self.bits {
            115 => Some(PADKEY_A::KEY),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `KEY`"]
    #[inline(always)]
    pub fn is_key(&self) -> bool {
        **self == PADKEY_A::KEY
    }
}
impl core::ops::Deref for PADKEY_R {
    type Target = crate::FieldReader<u32, PADKEY_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PADKEY` writer - Key register value."]
pub struct PADKEY_W<'a> {
    w: &'a mut W,
}
impl<'a> PADKEY_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PADKEY_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Key value."]
    #[inline(always)]
    pub fn key(self) -> &'a mut W {
        self.variant(PADKEY_A::KEY)
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
    pub fn padkey(&self) -> PADKEY_R {
        PADKEY_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Key register value."]
    #[inline(always)]
    pub fn padkey(&mut self) -> PADKEY_W {
        PADKEY_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Key Register for all pad configuration registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [padkey](index.html) module"]
pub struct PADKEY_SPEC;
impl crate::RegisterSpec for PADKEY_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [padkey::R](R) reader structure"]
impl crate::Readable for PADKEY_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [padkey::W](W) writer structure"]
impl crate::Writable for PADKEY_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PADKEY to value 0"]
impl crate::Resettable for PADKEY_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
