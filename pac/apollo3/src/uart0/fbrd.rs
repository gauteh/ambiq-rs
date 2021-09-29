#[doc = "Register `FBRD` reader"]
pub struct R(crate::R<FBRD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FBRD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FBRD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FBRD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FBRD` writer"]
pub struct W(crate::W<FBRD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FBRD_SPEC>;
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
impl From<crate::W<FBRD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FBRD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DIVFRAC` reader - These bits hold the baud fractional divisor."]
pub struct DIVFRAC_R(crate::FieldReader<u8, u8>);
impl DIVFRAC_R {
    pub(crate) fn new(bits: u8) -> Self {
        DIVFRAC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DIVFRAC_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DIVFRAC` writer - These bits hold the baud fractional divisor."]
pub struct DIVFRAC_W<'a> {
    w: &'a mut W,
}
impl<'a> DIVFRAC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | (value as u32 & 0x3f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:5 - These bits hold the baud fractional divisor."]
    #[inline(always)]
    pub fn divfrac(&self) -> DIVFRAC_R {
        DIVFRAC_R::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - These bits hold the baud fractional divisor."]
    #[inline(always)]
    pub fn divfrac(&mut self) -> DIVFRAC_W {
        DIVFRAC_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Fractional Baud Rate Divisor\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fbrd](index.html) module"]
pub struct FBRD_SPEC;
impl crate::RegisterSpec for FBRD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fbrd::R](R) reader structure"]
impl crate::Readable for FBRD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fbrd::W](W) writer structure"]
impl crate::Writable for FBRD_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FBRD to value 0"]
impl crate::Resettable for FBRD_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
