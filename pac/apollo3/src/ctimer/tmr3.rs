#[doc = "Register `TMR3` reader"]
pub struct R(crate::R<TMR3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TMR3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TMR3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TMR3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TMR3` writer"]
pub struct W(crate::W<TMR3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TMR3_SPEC>;
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
impl From<crate::W<TMR3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TMR3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CTTMRB3` reader - Counter/Timer B3."]
pub struct CTTMRB3_R(crate::FieldReader<u16, u16>);
impl CTTMRB3_R {
    pub(crate) fn new(bits: u16) -> Self {
        CTTMRB3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CTTMRB3_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CTTMRB3` writer - Counter/Timer B3."]
pub struct CTTMRB3_W<'a> {
    w: &'a mut W,
}
impl<'a> CTTMRB3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | ((value as u32 & 0xffff) << 16);
        self.w
    }
}
#[doc = "Field `CTTMRA3` reader - Counter/Timer A3."]
pub struct CTTMRA3_R(crate::FieldReader<u16, u16>);
impl CTTMRA3_R {
    pub(crate) fn new(bits: u16) -> Self {
        CTTMRA3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CTTMRA3_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CTTMRA3` writer - Counter/Timer A3."]
pub struct CTTMRA3_W<'a> {
    w: &'a mut W,
}
impl<'a> CTTMRA3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 16:31 - Counter/Timer B3."]
    #[inline(always)]
    pub fn cttmrb3(&self) -> CTTMRB3_R {
        CTTMRB3_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
    #[doc = "Bits 0:15 - Counter/Timer A3."]
    #[inline(always)]
    pub fn cttmra3(&self) -> CTTMRA3_R {
        CTTMRA3_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 16:31 - Counter/Timer B3."]
    #[inline(always)]
    pub fn cttmrb3(&mut self) -> CTTMRB3_W {
        CTTMRB3_W { w: self }
    }
    #[doc = "Bits 0:15 - Counter/Timer A3."]
    #[inline(always)]
    pub fn cttmra3(&mut self) -> CTTMRA3_W {
        CTTMRA3_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Counter/Timer Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tmr3](index.html) module"]
pub struct TMR3_SPEC;
impl crate::RegisterSpec for TMR3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tmr3::R](R) reader structure"]
impl crate::Readable for TMR3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tmr3::W](W) writer structure"]
impl crate::Writable for TMR3_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TMR3 to value 0"]
impl crate::Resettable for TMR3_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
