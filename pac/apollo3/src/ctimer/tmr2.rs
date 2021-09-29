#[doc = "Register `TMR2` reader"]
pub struct R(crate::R<TMR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TMR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TMR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TMR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TMR2` writer"]
pub struct W(crate::W<TMR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TMR2_SPEC>;
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
impl From<crate::W<TMR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TMR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CTTMRB2` reader - Counter/Timer B2."]
pub struct CTTMRB2_R(crate::FieldReader<u16, u16>);
impl CTTMRB2_R {
    pub(crate) fn new(bits: u16) -> Self {
        CTTMRB2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CTTMRB2_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CTTMRB2` writer - Counter/Timer B2."]
pub struct CTTMRB2_W<'a> {
    w: &'a mut W,
}
impl<'a> CTTMRB2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | ((value as u32 & 0xffff) << 16);
        self.w
    }
}
#[doc = "Field `CTTMRA2` reader - Counter/Timer A2."]
pub struct CTTMRA2_R(crate::FieldReader<u16, u16>);
impl CTTMRA2_R {
    pub(crate) fn new(bits: u16) -> Self {
        CTTMRA2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CTTMRA2_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CTTMRA2` writer - Counter/Timer A2."]
pub struct CTTMRA2_W<'a> {
    w: &'a mut W,
}
impl<'a> CTTMRA2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 16:31 - Counter/Timer B2."]
    #[inline(always)]
    pub fn cttmrb2(&self) -> CTTMRB2_R {
        CTTMRB2_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
    #[doc = "Bits 0:15 - Counter/Timer A2."]
    #[inline(always)]
    pub fn cttmra2(&self) -> CTTMRA2_R {
        CTTMRA2_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 16:31 - Counter/Timer B2."]
    #[inline(always)]
    pub fn cttmrb2(&mut self) -> CTTMRB2_W {
        CTTMRB2_W { w: self }
    }
    #[doc = "Bits 0:15 - Counter/Timer A2."]
    #[inline(always)]
    pub fn cttmra2(&mut self) -> CTTMRA2_W {
        CTTMRA2_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Counter/Timer Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tmr2](index.html) module"]
pub struct TMR2_SPEC;
impl crate::RegisterSpec for TMR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tmr2::R](R) reader structure"]
impl crate::Readable for TMR2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tmr2::W](W) writer structure"]
impl crate::Writable for TMR2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TMR2 to value 0"]
impl crate::Resettable for TMR2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
