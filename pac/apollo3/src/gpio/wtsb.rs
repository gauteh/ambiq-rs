#[doc = "Register `WTSB` reader"]
pub struct R(crate::R<WTSB_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WTSB_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WTSB_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WTSB_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `WTSB` writer"]
pub struct W(crate::W<WTSB_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WTSB_SPEC>;
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
impl From<crate::W<WTSB_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WTSB_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WTSB` reader - Set the GPIO49-32 write data."]
pub struct WTSB_R(crate::FieldReader<u32, u32>);
impl WTSB_R {
    pub(crate) fn new(bits: u32) -> Self {
        WTSB_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WTSB_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WTSB` writer - Set the GPIO49-32 write data."]
pub struct WTSB_W<'a> {
    w: &'a mut W,
}
impl<'a> WTSB_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0003_ffff) | (value as u32 & 0x0003_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:17 - Set the GPIO49-32 write data."]
    #[inline(always)]
    pub fn wtsb(&self) -> WTSB_R {
        WTSB_R::new((self.bits & 0x0003_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:17 - Set the GPIO49-32 write data."]
    #[inline(always)]
    pub fn wtsb(&mut self) -> WTSB_W {
        WTSB_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPIO Output Register B Set\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wtsb](index.html) module"]
pub struct WTSB_SPEC;
impl crate::RegisterSpec for WTSB_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [wtsb::R](R) reader structure"]
impl crate::Readable for WTSB_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [wtsb::W](W) writer structure"]
impl crate::Writable for WTSB_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets WTSB to value 0"]
impl crate::Resettable for WTSB_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
