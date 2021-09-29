#[doc = "Register `PWDKEY` reader"]
pub struct R(crate::R<PWDKEY_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PWDKEY_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PWDKEY_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PWDKEY_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PWDKEY` writer"]
pub struct W(crate::W<PWDKEY_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PWDKEY_SPEC>;
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
impl From<crate::W<PWDKEY_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PWDKEY_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Key register value.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u32)]
pub enum PWDKEY_A {
    #[doc = "55: Key value."]
    KEY = 55,
}
impl From<PWDKEY_A> for u32 {
    #[inline(always)]
    fn from(variant: PWDKEY_A) -> Self {
        variant as _
    }
}
#[doc = "Field `PWDKEY` reader - Key register value."]
pub struct PWDKEY_R(crate::FieldReader<u32, PWDKEY_A>);
impl PWDKEY_R {
    pub(crate) fn new(bits: u32) -> Self {
        PWDKEY_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PWDKEY_A> {
        match self.bits {
            55 => Some(PWDKEY_A::KEY),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `KEY`"]
    #[inline(always)]
    pub fn is_key(&self) -> bool {
        **self == PWDKEY_A::KEY
    }
}
impl core::ops::Deref for PWDKEY_R {
    type Target = crate::FieldReader<u32, PWDKEY_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PWDKEY` writer - Key register value."]
pub struct PWDKEY_W<'a> {
    w: &'a mut W,
}
impl<'a> PWDKEY_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PWDKEY_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Key value."]
    #[inline(always)]
    pub fn key(self) -> &'a mut W {
        self.variant(PWDKEY_A::KEY)
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
    pub fn pwdkey(&self) -> PWDKEY_R {
        PWDKEY_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Key register value."]
    #[inline(always)]
    pub fn pwdkey(&mut self) -> PWDKEY_W {
        PWDKEY_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Key Register for Powering Down the Voltage Comparator\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwdkey](index.html) module"]
pub struct PWDKEY_SPEC;
impl crate::RegisterSpec for PWDKEY_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pwdkey::R](R) reader structure"]
impl crate::Readable for PWDKEY_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pwdkey::W](W) writer structure"]
impl crate::Writable for PWDKEY_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PWDKEY to value 0"]
impl crate::Resettable for PWDKEY_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
