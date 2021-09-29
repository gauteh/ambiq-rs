#[doc = "Register `WTCB` reader"]
pub struct R(crate::R<WTCB_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WTCB_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WTCB_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WTCB_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `WTCB` writer"]
pub struct W(crate::W<WTCB_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WTCB_SPEC>;
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
impl From<crate::W<WTCB_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WTCB_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WTCB` reader - Clear the GPIO49-32 write data."]
pub struct WTCB_R(crate::FieldReader<u32, u32>);
impl WTCB_R {
    pub(crate) fn new(bits: u32) -> Self {
        WTCB_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WTCB_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WTCB` writer - Clear the GPIO49-32 write data."]
pub struct WTCB_W<'a> {
    w: &'a mut W,
}
impl<'a> WTCB_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0003_ffff) | (value as u32 & 0x0003_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:17 - Clear the GPIO49-32 write data."]
    #[inline(always)]
    pub fn wtcb(&self) -> WTCB_R {
        WTCB_R::new((self.bits & 0x0003_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:17 - Clear the GPIO49-32 write data."]
    #[inline(always)]
    pub fn wtcb(&mut self) -> WTCB_W {
        WTCB_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPIO Output Register B Clear\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wtcb](index.html) module"]
pub struct WTCB_SPEC;
impl crate::RegisterSpec for WTCB_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [wtcb::R](R) reader structure"]
impl crate::Readable for WTCB_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [wtcb::W](W) writer structure"]
impl crate::Writable for WTCB_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets WTCB to value 0"]
impl crate::Resettable for WTCB_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
