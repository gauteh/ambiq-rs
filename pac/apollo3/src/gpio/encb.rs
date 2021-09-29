#[doc = "Register `ENCB` reader"]
pub struct R(crate::R<ENCB_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ENCB_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ENCB_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ENCB_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ENCB` writer"]
pub struct W(crate::W<ENCB_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ENCB_SPEC>;
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
impl From<crate::W<ENCB_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ENCB_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ENCB` reader - Clear the GPIO49-32 output enables"]
pub struct ENCB_R(crate::FieldReader<u32, u32>);
impl ENCB_R {
    pub(crate) fn new(bits: u32) -> Self {
        ENCB_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ENCB_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ENCB` writer - Clear the GPIO49-32 output enables"]
pub struct ENCB_W<'a> {
    w: &'a mut W,
}
impl<'a> ENCB_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0003_ffff) | (value as u32 & 0x0003_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:17 - Clear the GPIO49-32 output enables"]
    #[inline(always)]
    pub fn encb(&self) -> ENCB_R {
        ENCB_R::new((self.bits & 0x0003_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:17 - Clear the GPIO49-32 output enables"]
    #[inline(always)]
    pub fn encb(&mut self) -> ENCB_W {
        ENCB_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPIO Enable Register B Clear\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [encb](index.html) module"]
pub struct ENCB_SPEC;
impl crate::RegisterSpec for ENCB_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [encb::R](R) reader structure"]
impl crate::Readable for ENCB_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [encb::W](W) writer structure"]
impl crate::Writable for ENCB_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ENCB to value 0"]
impl crate::Resettable for ENCB_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
