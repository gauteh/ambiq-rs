#[doc = "Register `BBINPUT` reader"]
pub struct R(crate::R<BBINPUT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BBINPUT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BBINPUT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BBINPUT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BBINPUT` writer"]
pub struct W(crate::W<BBINPUT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BBINPUT_SPEC>;
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
impl From<crate::W<BBINPUT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BBINPUT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DATAIN` reader - PIO values"]
pub struct DATAIN_R(crate::FieldReader<u8, u8>);
impl DATAIN_R {
    pub(crate) fn new(bits: u8) -> Self {
        DATAIN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DATAIN_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DATAIN` writer - PIO values"]
pub struct DATAIN_W<'a> {
    w: &'a mut W,
}
impl<'a> DATAIN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - PIO values"]
    #[inline(always)]
    pub fn datain(&self) -> DATAIN_R {
        DATAIN_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - PIO values"]
    #[inline(always)]
    pub fn datain(&mut self) -> DATAIN_W {
        DATAIN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PIO Input Values\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bbinput](index.html) module"]
pub struct BBINPUT_SPEC;
impl crate::RegisterSpec for BBINPUT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [bbinput::R](R) reader structure"]
impl crate::Readable for BBINPUT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [bbinput::W](W) writer structure"]
impl crate::Writable for BBINPUT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets BBINPUT to value 0"]
impl crate::Resettable for BBINPUT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
