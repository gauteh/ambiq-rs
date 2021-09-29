#[doc = "Register `ENSB` reader"]
pub struct R(crate::R<ENSB_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ENSB_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ENSB_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ENSB_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ENSB` writer"]
pub struct W(crate::W<ENSB_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ENSB_SPEC>;
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
impl From<crate::W<ENSB_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ENSB_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ENSB` reader - Set the GPIO49-32 output enables"]
pub struct ENSB_R(crate::FieldReader<u32, u32>);
impl ENSB_R {
    pub(crate) fn new(bits: u32) -> Self {
        ENSB_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ENSB_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ENSB` writer - Set the GPIO49-32 output enables"]
pub struct ENSB_W<'a> {
    w: &'a mut W,
}
impl<'a> ENSB_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0003_ffff) | (value as u32 & 0x0003_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:17 - Set the GPIO49-32 output enables"]
    #[inline(always)]
    pub fn ensb(&self) -> ENSB_R {
        ENSB_R::new((self.bits & 0x0003_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:17 - Set the GPIO49-32 output enables"]
    #[inline(always)]
    pub fn ensb(&mut self) -> ENSB_W {
        ENSB_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPIO Enable Register B Set\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ensb](index.html) module"]
pub struct ENSB_SPEC;
impl crate::RegisterSpec for ENSB_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ensb::R](R) reader structure"]
impl crate::Readable for ENSB_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ensb::W](W) writer structure"]
impl crate::Writable for ENSB_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ENSB to value 0"]
impl crate::Resettable for ENSB_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
