#[doc = "Register `ALMUP` reader"]
pub struct R(crate::R<ALMUP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ALMUP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ALMUP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ALMUP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ALMUP` writer"]
pub struct W(crate::W<ALMUP_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ALMUP_SPEC>;
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
impl From<crate::W<ALMUP_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ALMUP_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ALMWKDY` reader - Weekdays Alarm"]
pub struct ALMWKDY_R(crate::FieldReader<u8, u8>);
impl ALMWKDY_R {
    pub(crate) fn new(bits: u8) -> Self {
        ALMWKDY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ALMWKDY_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ALMWKDY` writer - Weekdays Alarm"]
pub struct ALMWKDY_W<'a> {
    w: &'a mut W,
}
impl<'a> ALMWKDY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 16)) | ((value as u32 & 0x07) << 16);
        self.w
    }
}
#[doc = "Field `ALMMO` reader - Months Alarm"]
pub struct ALMMO_R(crate::FieldReader<u8, u8>);
impl ALMMO_R {
    pub(crate) fn new(bits: u8) -> Self {
        ALMMO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ALMMO_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ALMMO` writer - Months Alarm"]
pub struct ALMMO_W<'a> {
    w: &'a mut W,
}
impl<'a> ALMMO_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 8)) | ((value as u32 & 0x1f) << 8);
        self.w
    }
}
#[doc = "Field `ALMDATE` reader - Date Alarm"]
pub struct ALMDATE_R(crate::FieldReader<u8, u8>);
impl ALMDATE_R {
    pub(crate) fn new(bits: u8) -> Self {
        ALMDATE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ALMDATE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ALMDATE` writer - Date Alarm"]
pub struct ALMDATE_W<'a> {
    w: &'a mut W,
}
impl<'a> ALMDATE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | (value as u32 & 0x3f);
        self.w
    }
}
impl R {
    #[doc = "Bits 16:18 - Weekdays Alarm"]
    #[inline(always)]
    pub fn almwkdy(&self) -> ALMWKDY_R {
        ALMWKDY_R::new(((self.bits >> 16) & 0x07) as u8)
    }
    #[doc = "Bits 8:12 - Months Alarm"]
    #[inline(always)]
    pub fn almmo(&self) -> ALMMO_R {
        ALMMO_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 0:5 - Date Alarm"]
    #[inline(always)]
    pub fn almdate(&self) -> ALMDATE_R {
        ALMDATE_R::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 16:18 - Weekdays Alarm"]
    #[inline(always)]
    pub fn almwkdy(&mut self) -> ALMWKDY_W {
        ALMWKDY_W { w: self }
    }
    #[doc = "Bits 8:12 - Months Alarm"]
    #[inline(always)]
    pub fn almmo(&mut self) -> ALMMO_W {
        ALMMO_W { w: self }
    }
    #[doc = "Bits 0:5 - Date Alarm"]
    #[inline(always)]
    pub fn almdate(&mut self) -> ALMDATE_W {
        ALMDATE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RTC Alarms Upper\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [almup](index.html) module"]
pub struct ALMUP_SPEC;
impl crate::RegisterSpec for ALMUP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [almup::R](R) reader structure"]
impl crate::Readable for ALMUP_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [almup::W](W) writer structure"]
impl crate::Writable for ALMUP_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ALMUP to value 0"]
impl crate::Resettable for ALMUP_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
