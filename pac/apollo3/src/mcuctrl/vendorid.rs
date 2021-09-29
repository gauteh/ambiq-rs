#[doc = "Register `VENDORID` reader"]
pub struct R(crate::R<VENDORID_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<VENDORID_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<VENDORID_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<VENDORID_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `VENDORID` writer"]
pub struct W(crate::W<VENDORID_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<VENDORID_SPEC>;
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
impl From<crate::W<VENDORID_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<VENDORID_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Unique Vendor ID\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u32)]
pub enum VENDORID_A {
    #[doc = "1095582289: Ambiq Vendor ID value."]
    AMBIQ = 1095582289,
}
impl From<VENDORID_A> for u32 {
    #[inline(always)]
    fn from(variant: VENDORID_A) -> Self {
        variant as _
    }
}
#[doc = "Field `VENDORID` reader - Unique Vendor ID"]
pub struct VENDORID_R(crate::FieldReader<u32, VENDORID_A>);
impl VENDORID_R {
    pub(crate) fn new(bits: u32) -> Self {
        VENDORID_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<VENDORID_A> {
        match self.bits {
            1095582289 => Some(VENDORID_A::AMBIQ),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `AMBIQ`"]
    #[inline(always)]
    pub fn is_ambiq(&self) -> bool {
        **self == VENDORID_A::AMBIQ
    }
}
impl core::ops::Deref for VENDORID_R {
    type Target = crate::FieldReader<u32, VENDORID_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VENDORID` writer - Unique Vendor ID"]
pub struct VENDORID_W<'a> {
    w: &'a mut W,
}
impl<'a> VENDORID_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: VENDORID_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Ambiq Vendor ID value."]
    #[inline(always)]
    pub fn ambiq(self) -> &'a mut W {
        self.variant(VENDORID_A::AMBIQ)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | (value as u32 & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Unique Vendor ID"]
    #[inline(always)]
    pub fn vendorid(&self) -> VENDORID_R {
        VENDORID_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Unique Vendor ID"]
    #[inline(always)]
    pub fn vendorid(&mut self) -> VENDORID_W {
        VENDORID_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Unique Vendor ID\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [vendorid](index.html) module"]
pub struct VENDORID_SPEC;
impl crate::RegisterSpec for VENDORID_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [vendorid::R](R) reader structure"]
impl crate::Readable for VENDORID_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [vendorid::W](W) writer structure"]
impl crate::Writable for VENDORID_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets VENDORID to value 0"]
impl crate::Resettable for VENDORID_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
