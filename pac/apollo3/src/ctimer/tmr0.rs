#[doc = "Register `TMR0` reader"]
pub struct R(crate::R<TMR0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TMR0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TMR0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TMR0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TMR0` writer"]
pub struct W(crate::W<TMR0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TMR0_SPEC>;
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
impl From<crate::W<TMR0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TMR0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CTTMRB0` reader - Counter/Timer B0."]
pub struct CTTMRB0_R(crate::FieldReader<u16, u16>);
impl CTTMRB0_R {
    pub(crate) fn new(bits: u16) -> Self {
        CTTMRB0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CTTMRB0_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CTTMRB0` writer - Counter/Timer B0."]
pub struct CTTMRB0_W<'a> {
    w: &'a mut W,
}
impl<'a> CTTMRB0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | ((value as u32 & 0xffff) << 16);
        self.w
    }
}
#[doc = "Field `CTTMRA0` reader - Counter/Timer A0."]
pub struct CTTMRA0_R(crate::FieldReader<u16, u16>);
impl CTTMRA0_R {
    pub(crate) fn new(bits: u16) -> Self {
        CTTMRA0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CTTMRA0_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CTTMRA0` writer - Counter/Timer A0."]
pub struct CTTMRA0_W<'a> {
    w: &'a mut W,
}
impl<'a> CTTMRA0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 16:31 - Counter/Timer B0."]
    #[inline(always)]
    pub fn cttmrb0(&self) -> CTTMRB0_R {
        CTTMRB0_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
    #[doc = "Bits 0:15 - Counter/Timer A0."]
    #[inline(always)]
    pub fn cttmra0(&self) -> CTTMRA0_R {
        CTTMRA0_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 16:31 - Counter/Timer B0."]
    #[inline(always)]
    pub fn cttmrb0(&mut self) -> CTTMRB0_W {
        CTTMRB0_W { w: self }
    }
    #[doc = "Bits 0:15 - Counter/Timer A0."]
    #[inline(always)]
    pub fn cttmra0(&mut self) -> CTTMRA0_W {
        CTTMRA0_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Counter/Timer Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tmr0](index.html) module"]
pub struct TMR0_SPEC;
impl crate::RegisterSpec for TMR0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tmr0::R](R) reader structure"]
impl crate::Readable for TMR0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tmr0::W](W) writer structure"]
impl crate::Writable for TMR0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TMR0 to value 0"]
impl crate::Resettable for TMR0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
