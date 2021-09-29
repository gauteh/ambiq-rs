#[doc = "Register `TMR1` reader"]
pub struct R(crate::R<TMR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TMR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TMR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TMR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TMR1` writer"]
pub struct W(crate::W<TMR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TMR1_SPEC>;
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
impl From<crate::W<TMR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TMR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CTTMRB1` reader - Counter/Timer B1."]
pub struct CTTMRB1_R(crate::FieldReader<u16, u16>);
impl CTTMRB1_R {
    pub(crate) fn new(bits: u16) -> Self {
        CTTMRB1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CTTMRB1_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CTTMRB1` writer - Counter/Timer B1."]
pub struct CTTMRB1_W<'a> {
    w: &'a mut W,
}
impl<'a> CTTMRB1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | ((value as u32 & 0xffff) << 16);
        self.w
    }
}
#[doc = "Field `CTTMRA1` reader - Counter/Timer A1."]
pub struct CTTMRA1_R(crate::FieldReader<u16, u16>);
impl CTTMRA1_R {
    pub(crate) fn new(bits: u16) -> Self {
        CTTMRA1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CTTMRA1_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CTTMRA1` writer - Counter/Timer A1."]
pub struct CTTMRA1_W<'a> {
    w: &'a mut W,
}
impl<'a> CTTMRA1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 16:31 - Counter/Timer B1."]
    #[inline(always)]
    pub fn cttmrb1(&self) -> CTTMRB1_R {
        CTTMRB1_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
    #[doc = "Bits 0:15 - Counter/Timer A1."]
    #[inline(always)]
    pub fn cttmra1(&self) -> CTTMRA1_R {
        CTTMRA1_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 16:31 - Counter/Timer B1."]
    #[inline(always)]
    pub fn cttmrb1(&mut self) -> CTTMRB1_W {
        CTTMRB1_W { w: self }
    }
    #[doc = "Bits 0:15 - Counter/Timer A1."]
    #[inline(always)]
    pub fn cttmra1(&mut self) -> CTTMRA1_W {
        CTTMRA1_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Counter/Timer Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tmr1](index.html) module"]
pub struct TMR1_SPEC;
impl crate::RegisterSpec for TMR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tmr1::R](R) reader structure"]
impl crate::Readable for TMR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tmr1::W](W) writer structure"]
impl crate::Writable for TMR1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TMR1 to value 0"]
impl crate::Resettable for TMR1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
