#[doc = "Register `BBVALUE` reader"]
pub struct R(crate::R<BBVALUE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BBVALUE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BBVALUE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BBVALUE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BBVALUE` writer"]
pub struct W(crate::W<BBVALUE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BBVALUE_SPEC>;
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
impl From<crate::W<BBVALUE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BBVALUE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PIN` reader - PIO values"]
pub struct PIN_R(crate::FieldReader<u8, u8>);
impl PIN_R {
    pub(crate) fn new(bits: u8) -> Self {
        PIN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PIN_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PIN` writer - PIO values"]
pub struct PIN_W<'a> {
    w: &'a mut W,
}
impl<'a> PIN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | ((value as u32 & 0xff) << 16);
        self.w
    }
}
#[doc = "Field `DATAOUT` reader - Data Output Values"]
pub struct DATAOUT_R(crate::FieldReader<u8, u8>);
impl DATAOUT_R {
    pub(crate) fn new(bits: u8) -> Self {
        DATAOUT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DATAOUT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DATAOUT` writer - Data Output Values"]
pub struct DATAOUT_W<'a> {
    w: &'a mut W,
}
impl<'a> DATAOUT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 16:23 - PIO values"]
    #[inline(always)]
    pub fn pin(&self) -> PIN_R {
        PIN_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 0:7 - Data Output Values"]
    #[inline(always)]
    pub fn dataout(&self) -> DATAOUT_R {
        DATAOUT_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 16:23 - PIO values"]
    #[inline(always)]
    pub fn pin(&mut self) -> PIN_W {
        PIN_W { w: self }
    }
    #[doc = "Bits 0:7 - Data Output Values"]
    #[inline(always)]
    pub fn dataout(&mut self) -> DATAOUT_W {
        DATAOUT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bbvalue](index.html) module"]
pub struct BBVALUE_SPEC;
impl crate::RegisterSpec for BBVALUE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [bbvalue::R](R) reader structure"]
impl crate::Readable for BBVALUE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [bbvalue::W](W) writer structure"]
impl crate::Writable for BBVALUE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets BBVALUE to value 0"]
impl crate::Resettable for BBVALUE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
