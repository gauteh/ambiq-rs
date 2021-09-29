#[doc = "Register `SWPOI` reader"]
pub struct R(crate::R<SWPOI_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SWPOI_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SWPOI_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SWPOI_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SWPOI` writer"]
pub struct W(crate::W<SWPOI_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SWPOI_SPEC>;
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
impl From<crate::W<SWPOI_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SWPOI_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "0x1B generates a software POI reset. This is a write-only register. Reading from this register will yield only all 0s.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SWPOIKEY_A {
    #[doc = "27: Writing 0x1B key value generates a software POI reset. value."]
    KEYVALUE = 27,
}
impl From<SWPOIKEY_A> for u8 {
    #[inline(always)]
    fn from(variant: SWPOIKEY_A) -> Self {
        variant as _
    }
}
#[doc = "Field `SWPOIKEY` reader - 0x1B generates a software POI reset. This is a write-only register. Reading from this register will yield only all 0s."]
pub struct SWPOIKEY_R(crate::FieldReader<u8, SWPOIKEY_A>);
impl SWPOIKEY_R {
    pub(crate) fn new(bits: u8) -> Self {
        SWPOIKEY_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SWPOIKEY_A> {
        match self.bits {
            27 => Some(SWPOIKEY_A::KEYVALUE),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `KEYVALUE`"]
    #[inline(always)]
    pub fn is_keyvalue(&self) -> bool {
        **self == SWPOIKEY_A::KEYVALUE
    }
}
impl core::ops::Deref for SWPOIKEY_R {
    type Target = crate::FieldReader<u8, SWPOIKEY_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SWPOIKEY` writer - 0x1B generates a software POI reset. This is a write-only register. Reading from this register will yield only all 0s."]
pub struct SWPOIKEY_W<'a> {
    w: &'a mut W,
}
impl<'a> SWPOIKEY_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SWPOIKEY_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Writing 0x1B key value generates a software POI reset. value."]
    #[inline(always)]
    pub fn keyvalue(self) -> &'a mut W {
        self.variant(SWPOIKEY_A::KEYVALUE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - 0x1B generates a software POI reset. This is a write-only register. Reading from this register will yield only all 0s."]
    #[inline(always)]
    pub fn swpoikey(&self) -> SWPOIKEY_R {
        SWPOIKEY_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 0x1B generates a software POI reset. This is a write-only register. Reading from this register will yield only all 0s."]
    #[inline(always)]
    pub fn swpoikey(&mut self) -> SWPOIKEY_W {
        SWPOIKEY_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Software POI Reset\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [swpoi](index.html) module"]
pub struct SWPOI_SPEC;
impl crate::RegisterSpec for SWPOI_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [swpoi::R](R) reader structure"]
impl crate::Readable for SWPOI_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [swpoi::W](W) writer structure"]
impl crate::Writable for SWPOI_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SWPOI to value 0"]
impl crate::Resettable for SWPOI_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
