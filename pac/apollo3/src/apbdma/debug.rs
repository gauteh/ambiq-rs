#[doc = "Register `DEBUG` reader"]
pub struct R(crate::R<DEBUG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DEBUG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DEBUG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DEBUG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DEBUG` writer"]
pub struct W(crate::W<DEBUG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DEBUG_SPEC>;
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
impl From<crate::W<DEBUG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DEBUG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Debug Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum DEBUGEN_A {
    #[doc = "0: Debug Disabled value."]
    OFF = 0,
    #[doc = "1: Debug Arb values value."]
    ARB = 1,
}
impl From<DEBUGEN_A> for u8 {
    #[inline(always)]
    fn from(variant: DEBUGEN_A) -> Self {
        variant as _
    }
}
#[doc = "Field `DEBUGEN` reader - Debug Enable"]
pub struct DEBUGEN_R(crate::FieldReader<u8, DEBUGEN_A>);
impl DEBUGEN_R {
    pub(crate) fn new(bits: u8) -> Self {
        DEBUGEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<DEBUGEN_A> {
        match self.bits {
            0 => Some(DEBUGEN_A::OFF),
            1 => Some(DEBUGEN_A::ARB),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `OFF`"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        **self == DEBUGEN_A::OFF
    }
    #[doc = "Checks if the value of the field is `ARB`"]
    #[inline(always)]
    pub fn is_arb(&self) -> bool {
        **self == DEBUGEN_A::ARB
    }
}
impl core::ops::Deref for DEBUGEN_R {
    type Target = crate::FieldReader<u8, DEBUGEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DEBUGEN` writer - Debug Enable"]
pub struct DEBUGEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DEBUGEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DEBUGEN_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Debug Disabled value."]
    #[inline(always)]
    pub fn off(self) -> &'a mut W {
        self.variant(DEBUGEN_A::OFF)
    }
    #[doc = "Debug Arb values value."]
    #[inline(always)]
    pub fn arb(self) -> &'a mut W {
        self.variant(DEBUGEN_A::ARB)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | (value as u32 & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - Debug Enable"]
    #[inline(always)]
    pub fn debugen(&self) -> DEBUGEN_R {
        DEBUGEN_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Debug Enable"]
    #[inline(always)]
    pub fn debugen(&mut self) -> DEBUGEN_W {
        DEBUGEN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PIO Input Values\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [debug](index.html) module"]
pub struct DEBUG_SPEC;
impl crate::RegisterSpec for DEBUG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [debug::R](R) reader structure"]
impl crate::Readable for DEBUG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [debug::W](W) writer structure"]
impl crate::Writable for DEBUG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DEBUG to value 0"]
impl crate::Resettable for DEBUG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
