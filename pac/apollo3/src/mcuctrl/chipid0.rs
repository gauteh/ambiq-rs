#[doc = "Register `CHIPID0` reader"]
pub struct R(crate::R<CHIPID0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CHIPID0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CHIPID0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CHIPID0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CHIPID0` writer"]
pub struct W(crate::W<CHIPID0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CHIPID0_SPEC>;
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
impl From<crate::W<CHIPID0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CHIPID0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Unique chip ID 0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u32)]
pub enum CHIPID0_A {
    #[doc = "0: Apollo3 CHIPID0. value."]
    APOLLO3 = 0,
}
impl From<CHIPID0_A> for u32 {
    #[inline(always)]
    fn from(variant: CHIPID0_A) -> Self {
        variant as _
    }
}
#[doc = "Field `CHIPID0` reader - Unique chip ID 0."]
pub struct CHIPID0_R(crate::FieldReader<u32, CHIPID0_A>);
impl CHIPID0_R {
    pub(crate) fn new(bits: u32) -> Self {
        CHIPID0_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CHIPID0_A> {
        match self.bits {
            0 => Some(CHIPID0_A::APOLLO3),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `APOLLO3`"]
    #[inline(always)]
    pub fn is_apollo3(&self) -> bool {
        **self == CHIPID0_A::APOLLO3
    }
}
impl core::ops::Deref for CHIPID0_R {
    type Target = crate::FieldReader<u32, CHIPID0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CHIPID0` writer - Unique chip ID 0."]
pub struct CHIPID0_W<'a> {
    w: &'a mut W,
}
impl<'a> CHIPID0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CHIPID0_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Apollo3 CHIPID0. value."]
    #[inline(always)]
    pub fn apollo3(self) -> &'a mut W {
        self.variant(CHIPID0_A::APOLLO3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | (value as u32 & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Unique chip ID 0."]
    #[inline(always)]
    pub fn chipid0(&self) -> CHIPID0_R {
        CHIPID0_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Unique chip ID 0."]
    #[inline(always)]
    pub fn chipid0(&mut self) -> CHIPID0_W {
        CHIPID0_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Unique Chip ID 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [chipid0](index.html) module"]
pub struct CHIPID0_SPEC;
impl crate::RegisterSpec for CHIPID0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [chipid0::R](R) reader structure"]
impl crate::Readable for CHIPID0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [chipid0::W](W) writer structure"]
impl crate::Writable for CHIPID0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CHIPID0 to value 0"]
impl crate::Resettable for CHIPID0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
