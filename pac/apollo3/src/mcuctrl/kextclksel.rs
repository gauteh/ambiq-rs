#[doc = "Register `KEXTCLKSEL` reader"]
pub struct R(crate::R<KEXTCLKSEL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<KEXTCLKSEL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<KEXTCLKSEL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<KEXTCLKSEL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `KEXTCLKSEL` writer"]
pub struct W(crate::W<KEXTCLKSEL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<KEXTCLKSEL_SPEC>;
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
impl From<crate::W<KEXTCLKSEL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<KEXTCLKSEL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Key register value.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u32)]
pub enum KEXTCLKSEL_A {
    #[doc = "83: Key value."]
    KEY = 83,
}
impl From<KEXTCLKSEL_A> for u32 {
    #[inline(always)]
    fn from(variant: KEXTCLKSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `KEXTCLKSEL` reader - Key register value."]
pub struct KEXTCLKSEL_R(crate::FieldReader<u32, KEXTCLKSEL_A>);
impl KEXTCLKSEL_R {
    pub(crate) fn new(bits: u32) -> Self {
        KEXTCLKSEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<KEXTCLKSEL_A> {
        match self.bits {
            83 => Some(KEXTCLKSEL_A::KEY),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `KEY`"]
    #[inline(always)]
    pub fn is_key(&self) -> bool {
        **self == KEXTCLKSEL_A::KEY
    }
}
impl core::ops::Deref for KEXTCLKSEL_R {
    type Target = crate::FieldReader<u32, KEXTCLKSEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `KEXTCLKSEL` writer - Key register value."]
pub struct KEXTCLKSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> KEXTCLKSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: KEXTCLKSEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Key value."]
    #[inline(always)]
    pub fn key(self) -> &'a mut W {
        self.variant(KEXTCLKSEL_A::KEY)
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
    pub fn kextclksel(&self) -> KEXTCLKSEL_R {
        KEXTCLKSEL_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Key register value."]
    #[inline(always)]
    pub fn kextclksel(&mut self) -> KEXTCLKSEL_W {
        KEXTCLKSEL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Key Register to enable the use of external clock selects via the EXTCLKSEL reg\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [kextclksel](index.html) module"]
pub struct KEXTCLKSEL_SPEC;
impl crate::RegisterSpec for KEXTCLKSEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [kextclksel::R](R) reader structure"]
impl crate::Readable for KEXTCLKSEL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [kextclksel::W](W) writer structure"]
impl crate::Writable for KEXTCLKSEL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets KEXTCLKSEL to value 0"]
impl crate::Resettable for KEXTCLKSEL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
