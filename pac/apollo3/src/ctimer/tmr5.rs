#[doc = "Register `TMR5` reader"]
pub struct R(crate::R<TMR5_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TMR5_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TMR5_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TMR5_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TMR5` writer"]
pub struct W(crate::W<TMR5_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TMR5_SPEC>;
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
impl From<crate::W<TMR5_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TMR5_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CTTMRB5` reader - Counter/Timer B5."]
pub struct CTTMRB5_R(crate::FieldReader<u16, u16>);
impl CTTMRB5_R {
    pub(crate) fn new(bits: u16) -> Self {
        CTTMRB5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CTTMRB5_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CTTMRB5` writer - Counter/Timer B5."]
pub struct CTTMRB5_W<'a> {
    w: &'a mut W,
}
impl<'a> CTTMRB5_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | ((value as u32 & 0xffff) << 16);
        self.w
    }
}
#[doc = "Field `CTTMRA5` reader - Counter/Timer A5."]
pub struct CTTMRA5_R(crate::FieldReader<u16, u16>);
impl CTTMRA5_R {
    pub(crate) fn new(bits: u16) -> Self {
        CTTMRA5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CTTMRA5_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CTTMRA5` writer - Counter/Timer A5."]
pub struct CTTMRA5_W<'a> {
    w: &'a mut W,
}
impl<'a> CTTMRA5_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 16:31 - Counter/Timer B5."]
    #[inline(always)]
    pub fn cttmrb5(&self) -> CTTMRB5_R {
        CTTMRB5_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
    #[doc = "Bits 0:15 - Counter/Timer A5."]
    #[inline(always)]
    pub fn cttmra5(&self) -> CTTMRA5_R {
        CTTMRA5_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 16:31 - Counter/Timer B5."]
    #[inline(always)]
    pub fn cttmrb5(&mut self) -> CTTMRB5_W {
        CTTMRB5_W { w: self }
    }
    #[doc = "Bits 0:15 - Counter/Timer A5."]
    #[inline(always)]
    pub fn cttmra5(&mut self) -> CTTMRA5_W {
        CTTMRA5_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Counter/Timer Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tmr5](index.html) module"]
pub struct TMR5_SPEC;
impl crate::RegisterSpec for TMR5_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tmr5::R](R) reader structure"]
impl crate::Readable for TMR5_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tmr5::W](W) writer structure"]
impl crate::Writable for TMR5_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TMR5 to value 0"]
impl crate::Resettable for TMR5_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
