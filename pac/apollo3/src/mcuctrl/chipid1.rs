#[doc = "Register `CHIPID1` reader"]
pub struct R(crate::R<CHIPID1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CHIPID1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CHIPID1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CHIPID1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CHIPID1` writer"]
pub struct W(crate::W<CHIPID1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CHIPID1_SPEC>;
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
impl From<crate::W<CHIPID1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CHIPID1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Unique chip ID 1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u32)]
pub enum CHIPID1_A {
    #[doc = "0: Apollo3 CHIPID1. value."]
    APOLLO3 = 0,
}
impl From<CHIPID1_A> for u32 {
    #[inline(always)]
    fn from(variant: CHIPID1_A) -> Self {
        variant as _
    }
}
#[doc = "Field `CHIPID1` reader - Unique chip ID 1."]
pub struct CHIPID1_R(crate::FieldReader<u32, CHIPID1_A>);
impl CHIPID1_R {
    pub(crate) fn new(bits: u32) -> Self {
        CHIPID1_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CHIPID1_A> {
        match self.bits {
            0 => Some(CHIPID1_A::APOLLO3),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `APOLLO3`"]
    #[inline(always)]
    pub fn is_apollo3(&self) -> bool {
        **self == CHIPID1_A::APOLLO3
    }
}
impl core::ops::Deref for CHIPID1_R {
    type Target = crate::FieldReader<u32, CHIPID1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CHIPID1` writer - Unique chip ID 1."]
pub struct CHIPID1_W<'a> {
    w: &'a mut W,
}
impl<'a> CHIPID1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CHIPID1_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Apollo3 CHIPID1. value."]
    #[inline(always)]
    pub fn apollo3(self) -> &'a mut W {
        self.variant(CHIPID1_A::APOLLO3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | (value as u32 & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Unique chip ID 1."]
    #[inline(always)]
    pub fn chipid1(&self) -> CHIPID1_R {
        CHIPID1_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Unique chip ID 1."]
    #[inline(always)]
    pub fn chipid1(&mut self) -> CHIPID1_W {
        CHIPID1_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Unique Chip ID 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [chipid1](index.html) module"]
pub struct CHIPID1_SPEC;
impl crate::RegisterSpec for CHIPID1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [chipid1::R](R) reader structure"]
impl crate::Readable for CHIPID1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [chipid1::W](W) writer structure"]
impl crate::Writable for CHIPID1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CHIPID1 to value 0"]
impl crate::Resettable for CHIPID1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
