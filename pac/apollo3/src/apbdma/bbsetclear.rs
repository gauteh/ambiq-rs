#[doc = "Register `BBSETCLEAR` reader"]
pub struct R(crate::R<BBSETCLEAR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BBSETCLEAR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BBSETCLEAR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BBSETCLEAR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BBSETCLEAR` writer"]
pub struct W(crate::W<BBSETCLEAR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BBSETCLEAR_SPEC>;
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
impl From<crate::W<BBSETCLEAR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BBSETCLEAR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CLEAR` reader - Write 1 to Clear PIO value"]
pub struct CLEAR_R(crate::FieldReader<u8, u8>);
impl CLEAR_R {
    pub(crate) fn new(bits: u8) -> Self {
        CLEAR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CLEAR_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CLEAR` writer - Write 1 to Clear PIO value"]
pub struct CLEAR_W<'a> {
    w: &'a mut W,
}
impl<'a> CLEAR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | ((value as u32 & 0xff) << 16);
        self.w
    }
}
#[doc = "Field `SET` reader - Write 1 to Set PIO value (set hier priority than clear if both bit set)"]
pub struct SET_R(crate::FieldReader<u8, u8>);
impl SET_R {
    pub(crate) fn new(bits: u8) -> Self {
        SET_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SET_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SET` writer - Write 1 to Set PIO value (set hier priority than clear if both bit set)"]
pub struct SET_W<'a> {
    w: &'a mut W,
}
impl<'a> SET_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 16:23 - Write 1 to Clear PIO value"]
    #[inline(always)]
    pub fn clear(&self) -> CLEAR_R {
        CLEAR_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 0:7 - Write 1 to Set PIO value (set hier priority than clear if both bit set)"]
    #[inline(always)]
    pub fn set(&self) -> SET_R {
        SET_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 16:23 - Write 1 to Clear PIO value"]
    #[inline(always)]
    pub fn clear(&mut self) -> CLEAR_W {
        CLEAR_W { w: self }
    }
    #[doc = "Bits 0:7 - Write 1 to Set PIO value (set hier priority than clear if both bit set)"]
    #[inline(always)]
    pub fn set(&mut self) -> SET_W {
        SET_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Set/Clear Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bbsetclear](index.html) module"]
pub struct BBSETCLEAR_SPEC;
impl crate::RegisterSpec for BBSETCLEAR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [bbsetclear::R](R) reader structure"]
impl crate::Readable for BBSETCLEAR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [bbsetclear::W](W) writer structure"]
impl crate::Writable for BBSETCLEAR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets BBSETCLEAR to value 0"]
impl crate::Resettable for BBSETCLEAR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
