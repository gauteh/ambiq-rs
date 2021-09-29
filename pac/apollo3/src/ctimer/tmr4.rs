#[doc = "Register `TMR4` reader"]
pub struct R(crate::R<TMR4_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TMR4_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TMR4_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TMR4_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TMR4` writer"]
pub struct W(crate::W<TMR4_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TMR4_SPEC>;
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
impl From<crate::W<TMR4_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TMR4_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CTTMRB4` reader - Counter/Timer B4."]
pub struct CTTMRB4_R(crate::FieldReader<u16, u16>);
impl CTTMRB4_R {
    pub(crate) fn new(bits: u16) -> Self {
        CTTMRB4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CTTMRB4_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CTTMRB4` writer - Counter/Timer B4."]
pub struct CTTMRB4_W<'a> {
    w: &'a mut W,
}
impl<'a> CTTMRB4_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | ((value as u32 & 0xffff) << 16);
        self.w
    }
}
#[doc = "Field `CTTMRA4` reader - Counter/Timer A4."]
pub struct CTTMRA4_R(crate::FieldReader<u16, u16>);
impl CTTMRA4_R {
    pub(crate) fn new(bits: u16) -> Self {
        CTTMRA4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CTTMRA4_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CTTMRA4` writer - Counter/Timer A4."]
pub struct CTTMRA4_W<'a> {
    w: &'a mut W,
}
impl<'a> CTTMRA4_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 16:31 - Counter/Timer B4."]
    #[inline(always)]
    pub fn cttmrb4(&self) -> CTTMRB4_R {
        CTTMRB4_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
    #[doc = "Bits 0:15 - Counter/Timer A4."]
    #[inline(always)]
    pub fn cttmra4(&self) -> CTTMRA4_R {
        CTTMRA4_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 16:31 - Counter/Timer B4."]
    #[inline(always)]
    pub fn cttmrb4(&mut self) -> CTTMRB4_W {
        CTTMRB4_W { w: self }
    }
    #[doc = "Bits 0:15 - Counter/Timer A4."]
    #[inline(always)]
    pub fn cttmra4(&mut self) -> CTTMRA4_W {
        CTTMRA4_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Counter/Timer Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tmr4](index.html) module"]
pub struct TMR4_SPEC;
impl crate::RegisterSpec for TMR4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tmr4::R](R) reader structure"]
impl crate::Readable for TMR4_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tmr4::W](W) writer structure"]
impl crate::Writable for TMR4_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TMR4 to value 0"]
impl crate::Resettable for TMR4_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
