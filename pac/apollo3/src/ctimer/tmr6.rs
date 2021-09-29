#[doc = "Register `TMR6` reader"]
pub struct R(crate::R<TMR6_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TMR6_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TMR6_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TMR6_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TMR6` writer"]
pub struct W(crate::W<TMR6_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TMR6_SPEC>;
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
impl From<crate::W<TMR6_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TMR6_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CTTMRB6` reader - Counter/Timer B6."]
pub struct CTTMRB6_R(crate::FieldReader<u16, u16>);
impl CTTMRB6_R {
    pub(crate) fn new(bits: u16) -> Self {
        CTTMRB6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CTTMRB6_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CTTMRB6` writer - Counter/Timer B6."]
pub struct CTTMRB6_W<'a> {
    w: &'a mut W,
}
impl<'a> CTTMRB6_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | ((value as u32 & 0xffff) << 16);
        self.w
    }
}
#[doc = "Field `CTTMRA6` reader - Counter/Timer A6."]
pub struct CTTMRA6_R(crate::FieldReader<u16, u16>);
impl CTTMRA6_R {
    pub(crate) fn new(bits: u16) -> Self {
        CTTMRA6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CTTMRA6_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CTTMRA6` writer - Counter/Timer A6."]
pub struct CTTMRA6_W<'a> {
    w: &'a mut W,
}
impl<'a> CTTMRA6_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 16:31 - Counter/Timer B6."]
    #[inline(always)]
    pub fn cttmrb6(&self) -> CTTMRB6_R {
        CTTMRB6_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
    #[doc = "Bits 0:15 - Counter/Timer A6."]
    #[inline(always)]
    pub fn cttmra6(&self) -> CTTMRA6_R {
        CTTMRA6_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 16:31 - Counter/Timer B6."]
    #[inline(always)]
    pub fn cttmrb6(&mut self) -> CTTMRB6_W {
        CTTMRB6_W { w: self }
    }
    #[doc = "Bits 0:15 - Counter/Timer A6."]
    #[inline(always)]
    pub fn cttmra6(&mut self) -> CTTMRA6_W {
        CTTMRA6_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Counter/Timer Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tmr6](index.html) module"]
pub struct TMR6_SPEC;
impl crate::RegisterSpec for TMR6_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tmr6::R](R) reader structure"]
impl crate::Readable for TMR6_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tmr6::W](W) writer structure"]
impl crate::Writable for TMR6_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TMR6 to value 0"]
impl crate::Resettable for TMR6_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
