#[doc = "Register `SWPOR` reader"]
pub struct R(crate::R<SWPOR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SWPOR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SWPOR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SWPOR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SWPOR` writer"]
pub struct W(crate::W<SWPOR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SWPOR_SPEC>;
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
impl From<crate::W<SWPOR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SWPOR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "0xD4 generates a software POR reset.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SWPORKEY_A {
    #[doc = "212: Writing 0xD4 key value generates a software POR reset. value."]
    KEYVALUE = 212,
}
impl From<SWPORKEY_A> for u8 {
    #[inline(always)]
    fn from(variant: SWPORKEY_A) -> Self {
        variant as _
    }
}
#[doc = "Field `SWPORKEY` reader - 0xD4 generates a software POR reset."]
pub struct SWPORKEY_R(crate::FieldReader<u8, SWPORKEY_A>);
impl SWPORKEY_R {
    pub(crate) fn new(bits: u8) -> Self {
        SWPORKEY_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SWPORKEY_A> {
        match self.bits {
            212 => Some(SWPORKEY_A::KEYVALUE),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `KEYVALUE`"]
    #[inline(always)]
    pub fn is_keyvalue(&self) -> bool {
        **self == SWPORKEY_A::KEYVALUE
    }
}
impl core::ops::Deref for SWPORKEY_R {
    type Target = crate::FieldReader<u8, SWPORKEY_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SWPORKEY` writer - 0xD4 generates a software POR reset."]
pub struct SWPORKEY_W<'a> {
    w: &'a mut W,
}
impl<'a> SWPORKEY_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SWPORKEY_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Writing 0xD4 key value generates a software POR reset. value."]
    #[inline(always)]
    pub fn keyvalue(self) -> &'a mut W {
        self.variant(SWPORKEY_A::KEYVALUE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - 0xD4 generates a software POR reset."]
    #[inline(always)]
    pub fn swporkey(&self) -> SWPORKEY_R {
        SWPORKEY_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 0xD4 generates a software POR reset."]
    #[inline(always)]
    pub fn swporkey(&mut self) -> SWPORKEY_W {
        SWPORKEY_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Software POR Reset\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [swpor](index.html) module"]
pub struct SWPOR_SPEC;
impl crate::RegisterSpec for SWPOR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [swpor::R](R) reader structure"]
impl crate::Readable for SWPOR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [swpor::W](W) writer structure"]
impl crate::Writable for SWPOR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SWPOR to value 0"]
impl crate::Resettable for SWPOR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
