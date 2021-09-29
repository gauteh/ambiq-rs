#[doc = "Register `LOCKCTRL` reader"]
pub struct R(crate::R<LOCKCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LOCKCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LOCKCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LOCKCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LOCKCTRL` writer"]
pub struct W(crate::W<LOCKCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LOCKCTRL_SPEC>;
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
impl From<crate::W<LOCKCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LOCKCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "LOCK Function Select register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SELECT_A {
    #[doc = "1: Unlock Customer Key (access to top half of info0) value."]
    CUSTOMER_KEY = 1,
    #[doc = "0: Lock Control should be set to NONE when not in use. value."]
    NONE = 0,
}
impl From<SELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: SELECT_A) -> Self {
        variant as _
    }
}
#[doc = "Field `SELECT` reader - LOCK Function Select register."]
pub struct SELECT_R(crate::FieldReader<u8, SELECT_A>);
impl SELECT_R {
    pub(crate) fn new(bits: u8) -> Self {
        SELECT_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SELECT_A> {
        match self.bits {
            1 => Some(SELECT_A::CUSTOMER_KEY),
            0 => Some(SELECT_A::NONE),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `CUSTOMER_KEY`"]
    #[inline(always)]
    pub fn is_customer_key(&self) -> bool {
        **self == SELECT_A::CUSTOMER_KEY
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        **self == SELECT_A::NONE
    }
}
impl core::ops::Deref for SELECT_R {
    type Target = crate::FieldReader<u8, SELECT_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SELECT` writer - LOCK Function Select register."]
pub struct SELECT_W<'a> {
    w: &'a mut W,
}
impl<'a> SELECT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SELECT_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Unlock Customer Key (access to top half of info0) value."]
    #[inline(always)]
    pub fn customer_key(self) -> &'a mut W {
        self.variant(SELECT_A::CUSTOMER_KEY)
    }
    #[doc = "Lock Control should be set to NONE when not in use. value."]
    #[inline(always)]
    pub fn none(self) -> &'a mut W {
        self.variant(SELECT_A::NONE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - LOCK Function Select register."]
    #[inline(always)]
    pub fn select(&self) -> SELECT_R {
        SELECT_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - LOCK Function Select register."]
    #[inline(always)]
    pub fn select(&mut self) -> SELECT_W {
        SELECT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "LOCK Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lockctrl](index.html) module"]
pub struct LOCKCTRL_SPEC;
impl crate::RegisterSpec for LOCKCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lockctrl::R](R) reader structure"]
impl crate::Readable for LOCKCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lockctrl::W](W) writer structure"]
impl crate::Writable for LOCKCTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets LOCKCTRL to value 0"]
impl crate::Resettable for LOCKCTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
